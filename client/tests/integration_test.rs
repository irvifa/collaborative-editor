use collaborative_editor_client::{deserialize_edit, serialize_edit, Edit};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::test]
async fn test_client_integration() {
    // Set up the server
    let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap();

    // Spawn the server task
    let server_handle = tokio::spawn(async move {
        while let Ok((stream, _)) = server.accept().await {
            // Handle the connection
            tokio::spawn(async move {
                let ws_stream = accept_async(stream).await.unwrap();
                let (mut write, mut read) = ws_stream.split();

                while let Some(Ok(message)) = read.next().await {
                    // Echo back the message
                    if let Message::Text(text) = message {
                        write.send(Message::Text(text)).await.unwrap();
                    }
                }
            });
        }
    });

    // Client code
    let url = format!("ws://{}/", addr);
    let url = Url::parse(&url).unwrap();

    let (ws_stream, _) = connect_async(url).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    // Prepare an Edit message
    let edit = Edit {
        position: 0,
        insert: Some("Hello, World!".to_string()),
        delete: None,
    };
    let edit_json = serialize_edit(&edit).unwrap();

    // Send the Edit message
    write.send(Message::Text(edit_json.clone())).await.unwrap();

    // Read the echoed message from the server
    if let Some(Ok(Message::Text(received_text))) = read.next().await {
        assert_eq!(received_text, edit_json);

        // Deserialize the received message
        let received_edit: Edit = deserialize_edit(&received_text).unwrap();
        assert_eq!(received_edit, edit);
    } else {
        panic!("Did not receive expected message from server");
    }

    // Clean up
    write.close().await.unwrap();
    server_handle.abort();
}
