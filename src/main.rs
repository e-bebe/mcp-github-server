use anyhow::Result;

mod error;
mod github;
mod protocol;
mod server;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting GitHub MCP Server...");

    let server = server::Server::new("github-mcp-server", "0.1.0");
    let transport = transport::StdioTransport::new();

    // サーバーを実行し、Ctrl+Cで終了できるようにする
    tokio::select! {
        result = server.run(transport) => {
            if let Err(e) = result {
                log::error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("Received interrupt signal, shutting down...");
        }
    }

    log::info!("Server shutdown complete");
    Ok(())
}
