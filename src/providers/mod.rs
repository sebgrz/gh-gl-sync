use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::config;

use self::{github_provider::GithubProvider, gitlab_provider::GitlabProvider};

pub mod github_provider;
pub mod gitlab_provider;

pub struct Providers {
    pub providers: HashMap<ProviderType, Box<dyn ProviderActions + Sync + Send>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderType {
    GITLAB,
    GITHUB,
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub struct Project {
    name: String,
    ssh_url: String,
}

#[derive(Debug)]
pub struct ProviderError(String);

#[async_trait]
pub trait ProviderActions {
    async fn get_all_projects(&self) -> Result<Vec<Project>, ProviderError>;
    async fn create_project(&self, name: String) -> Result<Project, ProviderError>;
}

impl Providers {
    pub async fn new(config: &config::Config) -> Providers {
        let mut providers = Providers {
            providers: HashMap::new(),
        };

        providers.providers.insert(
            ProviderType::GITHUB,
            Box::new(GithubProvider::new(&config.provider.github)),
        );
        providers.providers.insert(
            ProviderType::GITLAB,
            Box::new(GitlabProvider::new(&config.provider.gitlab).await),
        );

        providers
    }
}
