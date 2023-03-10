use rouille::{try_or_400, Request, Response};
use serde::Deserialize;

use crate::providers::ProviderType;

#[derive(Debug, Deserialize)]
struct AddProjectToSyncReq {
    provider: ProviderType,
    name: String,
}

pub fn add_project_to_sync(req: &Request) -> Response {
    let body: AddProjectToSyncReq = try_or_400!(rouille::input::json_input(req));
    println!("{body:?}");
    Response::text("todo")
}
