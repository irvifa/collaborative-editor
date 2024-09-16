use futures_util::StreamExt;
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub type Tx = mpsc::UnboundedSender<tokio_tungstenite::tungstenite::Message>;
pub type PeerMap = Arc<RwLock<HashMap<String, Tx>>>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Edit {
    pub position: usize,
    pub insert: Option<String>,
    pub delete: Option<usize>,
    pub version: usize,
}

pub struct DocumentState {
    pub content: String,
    pub version: usize,
}

impl DocumentState {
    pub fn new() -> Self {
        DocumentState {
            content: String::new(),
            version: 0,
        }
    }

    pub fn apply_edit(&mut self, edit: &Edit) -> Result<(), &'static str> {
        // Check for version consistency
        if edit.version != self.version {
            eprintln!(
                "Version mismatch: edit version {} does not match document version {}",
                edit.version, self.version
            );
            return Err("Version mismatch");
        }

        // Ensure valid UTF-8 character boundary for insertion
        if let Some(ref insert) = edit.insert {
            if !self.content.is_char_boundary(edit.position) {
                eprintln!(
                    "Insert position is not a valid UTF-8 boundary: {}",
                    edit.position
                );
                return Err("Insert position is not a valid UTF-8 boundary.");
            }
            self.content.insert_str(edit.position, insert);
        }
        // Ensure valid UTF-8 character boundary for deletion
        else if let Some(delete) = edit.delete {
            let end = edit.position + delete;
            if !self.content.is_char_boundary(edit.position) || !self.content.is_char_boundary(end)
            {
                eprintln!(
                    "Delete range is not valid UTF-8 boundaries: {} to {}",
                    edit.position, end
                );
                return Err("Delete range is not valid UTF-8 boundaries.");
            }
            self.content.replace_range(edit.position..end, "");
        }

        self.version += 1;
        Ok(())
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    let peers: PeerMap = Arc::new(RwLock::new(HashMap::new()));
    let document = Arc::new(RwLock::new(DocumentState::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, peers.clone(), document.clone()));
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    peers: PeerMap,
    document: Arc<RwLock<DocumentState>>,
) {
    let addr = stream
        .peer_addr()
        .expect("Connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {}", e); // Log the handshake failure
            return; // Don't panic, just return and let the server continue
        }
    };

    println!("New WebSocket connection: {}", addr);

    let (tx, rx) = mpsc::unbounded_channel();
    peers.write().await.insert(addr.to_string(), tx.clone());

    // Send the initial document state to the new client
    let initial_message = {
        let doc = document.read().await;
        json!({
            "type": "initial",
            "content": doc.content,
            "version": doc.version,
        })
    };
    if let Err(e) = tx.send(Message::Text(initial_message.to_string())) {
        eprintln!("Failed to send initial content to {}: {}", addr, e);
    }

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let peers = peers.clone();
        let document = document.clone();

        async move {
            match msg.to_text() {
                Ok(text) => {
                    let data: serde_json::Value = match serde_json::from_str(text) {
                        Ok(data) => data,
                        Err(e) => {
                            eprintln!("Failed to parse message: {}", e);
                            return Ok(());
                        }
                    };

                    if data["type"] == "edit" {
                        let edit_json = &data["edit"];
                        let edit: Edit = match serde_json::from_value(edit_json.clone()) {
                            Ok(edit) => edit,
                            Err(e) => {
                                eprintln!("Failed to parse edit: {}", e);
                                return Ok(());
                            }
                        };

                        let mut doc = document.write().await;
                        match doc.apply_edit(&edit) {
                            Ok(_) => {
                                let new_version = doc.version;
                                let broadcast_msg = json!({
                                    "type": "edit",
                                    "edit": {
                                        "position": edit.position,
                                        "insert": edit.insert,
                                        "delete": edit.delete,
                                        "version": new_version,
                                    }
                                })
                                .to_string();

                                // Broadcast to all peers except the sender
                                let peers_guard = peers.read().await;
                                for (peer_addr, recp) in peers_guard.iter() {
                                    if peer_addr != &addr.to_string() {
                                        if let Err(e) =
                                            recp.send(Message::Text(broadcast_msg.clone()))
                                        {
                                            eprintln!(
                                                "Failed to send message to {}: {}",
                                                peer_addr, e
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error applying edit: {}", e);
                                // Optionally, send error back to client
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Received non-text message: {}", e),
            }
            Ok(())
        }
    });

    let receive_from_others = UnboundedReceiverStream::new(rx).map(Ok).forward(outgoing);

    tokio::select! {
        _ = broadcast_incoming => (),
        _ = receive_from_others => (),
    }

    println!("{} disconnected", &addr);
    peers.write().await.remove(&addr.to_string());
}

impl Default for DocumentState {
    fn default() -> Self {
        Self::new()
    }
}
