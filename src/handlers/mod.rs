use std::sync::Arc;

use rouille::{try_or_400, Request, Response};
use serde::Deserialize;

use crate::providers::{ProviderType, Providers};

#[derive(Debug, Deserialize)]
struct AddProjectToSyncReq {
    provider: ProviderType,
    name: String,
}

pub async fn add_project_to_sync(providers: Arc<Providers>, req: &Request) -> Response {
    let body: AddProjectToSyncReq = try_or_400!(rouille::input::json_input(req));
    Response::text("ok")
}

/// Fetch list of projects from both providers
/// and then save them to the database.
/// If project exists, ignore it.
pub async fn sync_providers_projects(providers: Arc<Providers>) {
    let github = &providers.providers[&ProviderType::GITHUB];
    let gh_projects = github.get_all_projects().await.unwrap_or_else(|e| {
        panic!("{e:?}");
    });
    println!("GITHUB\n{gh_projects:?}");

    let gitlab = &providers.providers[&ProviderType::GITLAB];
    let gl_projects = gitlab.get_all_projects().await.unwrap_or_else(|e| {
        panic!("{e:?}");
    });
    println!("GITLAB\n{gl_projects:?}");
}

pub async fn create_gh_repo(providers: Arc<Providers>) -> Response {
    let github = &providers.providers[&ProviderType::GITHUB];
    let project = github
        .create_project("test".to_string())
        .await
        .unwrap_or_else(|e| {
            panic!("{e:?}");
        });
    Response::json(&project)
}

pub async fn create_gl_repo(providers: Arc<Providers>) -> Response {
    let github = &providers.providers[&ProviderType::GITLAB];
    let project = github
        .create_project("test".to_string())
        .await
        .unwrap_or_else(|e| {
            panic!("{e:?}");
        });
    Response::json(&project)
}
