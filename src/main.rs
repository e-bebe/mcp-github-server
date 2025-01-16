use anyhow::Result;
use tracing::{error, info};

mod error;
mod github;
mod protocol;
mod server;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    // enable logger, this is global, so initialize it in main function
    tracing_subscriber::fmt()
        .with_ansi(true) // ANSIカラーを有効化
        .with_target(true) // モジュールパスを表示
        .with_thread_ids(true) // スレッドIDを表示
        .with_line_number(true) // 行番号を表示
        .with_file(false) // ファイル名を表示
        .with_level(true) // ログレベルを表示
        .try_init()
        .expect("Failed to initialize logger");
    info!("Starting GitHub MCP Server...");

    let server = server::Server::new("github-mcp-server", "0.1.0");
    let transport = transport::StdioTransport::new();

    // サーバーを実行し、Ctrl+Cで終了できるようにする
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
