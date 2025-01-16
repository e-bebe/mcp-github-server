use anyhow::Result;
use tracing::{error, info};

mod error;
mod github;
mod protocol;
mod server;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");
    info!("Starting GitHub MCP Server...");

    let server = server::Server::new("github-mcp-server", "0.1.0");
    let transport = transport::StdioTransport::new();

    tokio::select! {
        result = server.run(transport) => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received interrupt signal, shutting down...");
        }
    }

    info!("Server shutdown complete");
    Ok(())
}
