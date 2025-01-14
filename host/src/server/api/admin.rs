use axum::{extract::State, routing::post, Router};

use crate::interfaces::HostResult;
use raiko_reqactor::Gateway;

pub fn create_router() -> Router<Gateway> {
    Router::new().route("/admin/pause", post(pause))
}

async fn pause(State(gateway): State<Gateway>) -> HostResult<&'static str> {
    gateway.pause().await.map_err(|e| anyhow::anyhow!(e))?;
    Ok("System paused successfully")
}
