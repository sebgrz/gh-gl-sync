use async_trait::async_trait;
use serde::Deserialize;

pub mod github_provider;

#[derive(Debug, Deserialize)]
pub enum ProviderType {
    GITLAB,
    GITHUB,
}

pub struct Project {
    name: String,
    ssh_url: String,
}

pub struct ProviderError(String);

#[async_trait]
pub trait ProviderActions {
    async fn get_all_projects(&self) -> Result<Vec<Project>, ProviderError>;
}
