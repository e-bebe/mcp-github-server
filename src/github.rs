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
            .personal_token(std::env::var("GITHUB_TOKEN")?)
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
            .per_page(per_page)
            .page(page)
            .send()
            .await?;

        let items = result
            .items
            .into_iter()
            .map(|repo| Repository {
                name: repo.name,
                full_name: repo.full_name,
                description: repo.description,
                html_url: repo.html_url.to_string(),
                stargazers_count: repo.stargazers_count,
            })
            .collect();

        Ok(SearchRepositoriesResult {
            total_count: result.total_count,
            items,
        })
    }
}
