use rouille::{Request, Response, try_or_400};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct AddProjectToSyncReq {
    provider: Provider,
    name: String
}

#[derive(Debug,Deserialize)]
enum Provider {
    GITLAB,
    GITHUB
}

pub fn add_project_to_sync(req: &Request) -> Response {
    let body: AddProjectToSyncReq = try_or_400!(rouille::input::json_input(req));
    println!("{body:?}");
    Response::text("todo")
}
