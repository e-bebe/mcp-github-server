use crate::{
    error::MCPError,
    github::{GitHubClient, SearchRepositoriesParams},
    protocol::{Request, Response},
    transport::Transport,
};
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;

pub struct Server {
    name: String,
    version: String,
    github: Arc<GitHubClient>,
}

impl Server {
    pub fn new(name: &str, version: &str) -> Self {
        let github = GitHubClient::new().expect("Failed to initialize GitHub client");

        Self {
            name: name.to_string(),
            version: version.to_string(),
            github: Arc::new(github),
        }
    }

    pub async fn run<T: Transport>(&self, transport: T) -> Result<()> {
        loop {
            let message = transport.read_message().await?;
            let request: Request = serde_json::from_str(&message)?;

            let response = self.handle_request(request).await?;
            let response_json = serde_json::to_string(&response)?;

            transport.write_message(&response_json).await?;
        }
    }

    async fn handle_request(&self, request: Request) -> Result<Response<serde_json::Value>> {
        match request.method.as_str() {
            "listTools" => self.handle_list_tools(request).await,
            "callTool" => self.handle_call_tool(request).await,
            _ => Err(MCPError::MethodNotFound(request.method).into()),
        }
    }

    async fn handle_list_tools(&self, request: Request) -> Result<Response<serde_json::Value>> {
        let tools = json!({
            "tools": [{
                "name": "search_repositories",
                "description": "Search for GitHub repositories",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "page": {
                            "type": "integer",
                            "description": "Page number",
                            "minimum": 1
                        },
                        "per_page": {
                            "type": "integer",
                            "description": "Results per page",
                            "minimum": 1,
                            "maximum": 100
                        }
                    },
                    "required": ["query"]
                }
            }]
        });

        Ok(Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(tools),
            error: None,
        })
    }

    async fn handle_call_tool(&self, request: Request) -> Result<Response<serde_json::Value>> {
        let params = request
            .params
            .ok_or_else(|| MCPError::InvalidRequest("Missing params".to_string()))?;

        let tool_params = params
            .get("params")
            .ok_or_else(|| MCPError::InvalidRequest("Missing tool params".to_string()))?;

        match params.get("name").and_then(|v| v.as_str()) {
            Some("search_repositories") => {
                let search_params: SearchRepositoriesParams =
                    serde_json::from_value(tool_params.clone())?;

                let result = self.github.search_repositories(search_params).await?;

                Ok(Response {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!(result)),
                    error: None,
                })
            }
            _ => Err(MCPError::MethodNotFound("Unknown tool".to_string()).into()),
        }
    }
}
