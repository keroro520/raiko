use axum::{extract::State, routing::get, Json, Router};
use serde_json::Value;
use utoipa::OpenApi;

use crate::interfaces::HostResult;
use raiko_reqactor::Gateway;

#[utoipa::path(post, path = "/proof/report",
    tag = "Proving",
    responses (
        (status = 200, description = "Successfully listed all current tasks", body = CancelStatus)
    )
)]
// #[debug_handler(state = Gateway)]
/// List all tasks.
///
/// Retrieve a list of `{ chain_id, blockhash, prover_type, prover, status }` items.
async fn report_handler(State(_gateway): State<Gateway>) -> HostResult<Json<Value>> {
    todo!()
}

#[derive(OpenApi)]
#[openapi(paths(report_handler))]
struct Docs;

pub fn create_docs() -> utoipa::openapi::OpenApi {
    Docs::openapi()
}

pub fn create_router() -> Router<Gateway> {
    Router::new().route("/", get(report_handler::<P>))
}
