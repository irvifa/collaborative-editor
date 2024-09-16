use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;
use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use serde_json::json;
use futures_util::TryStreamExt;
use futures_util::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

use collaborative_editor_server::{DocumentState, Edit, PeerMap, Tx};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    collaborative_editor_server::run_server().await
}
