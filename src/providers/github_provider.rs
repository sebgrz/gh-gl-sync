use super::{Project, ProviderActions, ProviderError};
use crate::config::ProviderConfig;
use async_trait::async_trait;

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
    async fn create_project(&self, name: String) -> Result<Project, ProviderError> {
        let username = &self.username;
        let octo = octocrab::Octocrab::builder()
            .personal_token(self.token.to_string())
            .build()
            .map_err(|e| ProviderError(e.to_string()))?;

        octo.current()
            .create_repo(&name.to_string())
            .private(true)
            .send()
            .await
            .map_err(|e| ProviderError(e.to_string()))?;

        let repo = octo
            .repos(username, name.to_string())
            .get()
            .await
            .map_err(|e| ProviderError(e.to_string()))?;

        Ok(Project {
            name: repo.name,
            ssh_url: repo.ssh_url.unwrap(),
        })
    }

    async fn get_all_projects(&self) -> Result<Vec<super::Project>, ProviderError> {
        let octo = octocrab::Octocrab::builder()
            .personal_token(self.token.to_string())
            .build()
            .map_err(|e| ProviderError(e.to_string()))?;

        // TODO pagination
        let mut repos = octo
            .current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .sort("created")
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
