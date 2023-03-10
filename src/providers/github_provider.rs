use async_trait::async_trait;
use octocrab::params::repos::{Sort, Type};

use crate::config::ProviderConfig;

use super::{Project, ProviderActions, ProviderError};

pub struct GithubProvider {
    username: String,
    token: String,
}

impl GithubProvider {
    pub fn new(config: &ProviderConfig) -> GithubProvider {
        GithubProvider {
            username: config.username.to_string(),
            token: config.token.to_string(),
        }
    }
}

#[async_trait]
impl ProviderActions for GithubProvider {
    async fn get_all_projects(&self) -> Result<Vec<super::Project>, ProviderError> {
        let octo = octocrab::Octocrab::builder()
            .personal_token(self.token.to_string())
            .build()
            .map_err(|e| ProviderError(e.to_string()))?;

        // TODO pagination
        let mut repos = octo
            .orgs("owner")
            .list_repos()
            .repo_type(Type::Sources)
            .per_page(100)
            .page(0u32)
            .sort(Sort::Created)
            .send()
            .await
            .map_err(|e| ProviderError(e.to_string()))?;

        let projects = repos
            .take_items()
            .iter()
            .map(|m| Project {
                name: m.name.to_string(),
                ssh_url: m.ssh_url.as_ref().unwrap().to_string(),
            })
            .collect();

        Ok(projects)
    }
}
