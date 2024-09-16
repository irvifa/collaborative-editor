use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

// Import the necessary items from your library crate
use collaborative_editor_client::{
    calculate_retry_delay, deserialize_edit, parse_user_input, serialize_edit,
};

async fn connect_to_server(
    url: &str,
) -> Result<
    (
        futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            Message,
        >,
        futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
        >,
    ),
    Box<dyn std::error::Error>,
> {
    let url = Url::parse(url)?;
    let (ws_stream, _) = connect_async(url).await?;
    info!("WebSocket handshake has been successfully completed");
    Ok(ws_stream.split())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let server_url = "ws://server:8080";
    let mut retry_count = 0;
    let max_retries = 5;

    let (mut write, mut read) = loop {
        match connect_to_server(server_url).await {
            Ok(streams) => break streams,
            Err(e) => {
                retry_count += 1;
                if retry_count > max_retries {
                    error!("Failed to connect after {} attempts: {}", max_retries, e);
                    return Err("Max retries exceeded".into());
                }
                let delay = calculate_retry_delay(retry_count);
                warn!(
                    "Connection attempt failed. Retrying in {} seconds...",
                    delay.as_secs()
                );
                tokio::time::sleep(delay).await;
            }
        }
    };

    let user_input = tokio::spawn(async move {
        loop {
            print!("Enter an edit (position,insert/delete): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                warn!("Failed to read input.");
                continue;
            }

            let edit = match parse_user_input(&input) {
                Ok(edit) => edit,
                Err(err_msg) => {
                    warn!("{}", err_msg);
                    continue;
                }
            };

            let edit_json = match serialize_edit(&edit) {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to serialize edit: {}", e);
                    continue;
                }
            };

            if let Err(e) = write.send(Message::Text(edit_json)).await {
                error!("Failed to send message: {}", e);
                break;
            }
        }
    });

    let receive_messages = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => match deserialize_edit(&text) {
                    Ok(edit) => info!("Received edit: {:?}", edit),
                    Err(e) => warn!("Failed to parse received edit: {}", e),
                },
                Ok(_) => warn!("Received non-text message"),
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = user_input => {
            error!("User input task ended.");
        },
        _ = receive_messages => {
            warn!("Message receiving task ended.");
        },
    }

    Ok(())
}
