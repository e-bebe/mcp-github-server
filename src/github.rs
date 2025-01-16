use anyhow::Result;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchRepositoriesParams {
    pub query: String,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct SearchRepositoriesResult {
    pub total_count: u32,
    pub items: Vec<Repository>,
}

#[derive(Debug, Serialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
}

pub struct GitHubClient {
    client: Octocrab,
}

impl GitHubClient {
    pub fn new() -> Result<Self> {
        let client = Octocrab::builder()
            .personal_token(std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")?)
            .build()?;

        Ok(Self { client })
    }

    pub async fn search_repositories(
        &self,
        params: SearchRepositoriesParams,
    ) -> Result<SearchRepositoriesResult> {
        let per_page = params.per_page.unwrap_or(30);
        let page = params.page.unwrap_or(1);

        let result = self
            .client
            .search()
            .repositories(&params.query)
            .per_page(per_page as u8)
            .page(page as u8)
            .send()
            .await?;

        let items = result
            .items
            .into_iter()
            .map(|repo| Repository {
                name: repo.name,
                full_name: repo.full_name.unwrap_or_default(),
                description: repo.description,
                html_url: repo
                    .html_url
                    .map_or_else(String::new, |url| url.to_string()),
                stargazers_count: repo.stargazers_count.unwrap_or(0) as u32,
            })
            .collect();

        Ok(SearchRepositoriesResult {
            total_count: result.total_count.unwrap_or(0) as u32,
            items,
        })
    }
}
