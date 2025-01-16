use anyhow::Result;
use log::info;

mod error;
mod github;
mod protocol;
mod server;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting GitHub MCP Server...");

    let server = server::Server::new("github-mcp-server", "0.1.0");
    let transport = transport::StdioTransport::new();

    server.run(transport).await?;
    Ok(())
}
