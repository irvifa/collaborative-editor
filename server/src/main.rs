#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    collaborative_editor_server::run_server().await
}
