use thiserror::Error;

#[derive(Error, Debug)]
pub enum MCPError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("GitHub API error: {0}")]
    GitHub(String),

    #[error("Transport error: {0}")]
    Transport(String),
}
