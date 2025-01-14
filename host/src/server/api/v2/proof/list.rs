use axum::{extract::State, routing::get, Json, Router};
use serde_json::Value;
use utoipa::OpenApi;

use crate::interfaces::HostResult;
use raiko_reqactor::Gateway;

#[utoipa::path(post, path = "/proof/list",
    tag = "Proving",
    responses (
        (status = 200, description = "Successfully listed all proofs & Ids", body = CancelStatus)
    )
)]
async fn list_handler(State(_gateway): State<Gateway>) -> HostResult<Json<Value>> {
    todo!()
}

#[derive(OpenApi)]
#[openapi(paths(list_handler))]
struct Docs;

pub fn create_docs() -> utoipa::openapi::OpenApi {
    Docs::openapi()
}

pub fn create_router() -> Router<Gateway> {
    Router::new().route("/", get(list_handler::<P>))
}
