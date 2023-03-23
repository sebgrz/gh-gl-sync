use std::sync::Arc;

use async_trait::async_trait;
use gitlab::{
    api::projects,
    api::{self, AsyncQuery, Query},
    types, AsyncGitlab, Gitlab,
};
use tokio::task::spawn_blocking;

use crate::config::ProviderConfig;

use super::{Project, ProviderActions, ProviderError};

pub struct GitlabProvider {
    pub client: Arc<AsyncGitlab>,
}

impl GitlabProvider {
    pub async fn new(config: &ProviderConfig) -> GitlabProvider {
        let client = if config.https {
            Gitlab::builder(config.hostname.to_string(), config.token.to_string())
                .build_async()
                .await
        } else {
            Gitlab::builder(config.hostname.to_string(), config.token.to_string())
                .insecure()
                .build_async()
                .await
        }
        .unwrap_or_else(|e| {
            panic!("gitlab crate client failed {e}");
        });

        GitlabProvider {
            client: Arc::new(client),
        }
    }
}

#[async_trait]
impl ProviderActions for GitlabProvider {
    async fn create_project(&self, name: String) -> Result<Project, ProviderError> {
        let client = self.client.clone();

        let new_project = projects::CreateProject::builder()
            .name(name.to_string())
            .visibility(api::common::VisibilityLevel::Private)
            .default_branch("master")
            .build()
            .map_err(|e| ProviderError(e.to_string()))?;

        let project: types::Project = new_project
            .query_async(client.as_ref())
            .await
            .map_err(|e| ProviderError(e.to_string()))?;

        Ok(Project {
            name: project.path_with_namespace.to_string(),
            ssh_url: project.ssh_url_to_repo.to_string(),
        })
    }

    async fn get_all_projects(&self) -> Result<Vec<Project>, ProviderError> {
        let client = self.client.clone();
        let projects_endpoint = projects::Projects::builder()
            .owned(true)
            .order_by(projects::ProjectOrderBy::CreatedAt)
            .build()
            .map_err(|e| ProviderError(e.to_string()))?;
        // TODO: pagination
        let projects: Vec<types::Project> = projects_endpoint
            .query_async(client.as_ref())
            .await
            .map_err(|e| ProviderError(e.to_string()))?;
        let result = projects
            .iter()
            .map(|p| Project {
                name: p.path_with_namespace.to_string(),
                ssh_url: p.ssh_url_to_repo.to_string(),
            })
            .collect();

        Ok(result)
    }
}
