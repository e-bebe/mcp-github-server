use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, Stdin, Stdout};
use tokio::sync::Mutex;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn read_message(&self) -> Result<String>;
    async fn write_message(&self, message: &str) -> Result<()>;
}

pub struct StdioTransport {
    reader: Arc<Mutex<BufReader<Stdin>>>,
    writer: Arc<Mutex<Stdout>>,
}

impl StdioTransport {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let stdout = io::stdout();
        Self {
            reader: Arc::new(Mutex::new(BufReader::new(stdin))),
            writer: Arc::new(Mutex::new(stdout)),
        }
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn read_message(&self) -> Result<String> {
        let mut line = String::new();
        let mut reader = self.reader.lock().await;

        loop {
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                // EOFの場合は少し待ってから再試行
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
            break;
        }

        Ok(line)
    }

    async fn write_message(&self, message: &str) -> Result<()> {
        let mut writer = self.writer.lock().await;
        writer.write_all(message.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }
}
