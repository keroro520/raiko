use axum::{body::Body, http::header, response::Response, routing::get, Router};
use prometheus::{Encoder, TextEncoder};
use utoipa::OpenApi;

use crate::interfaces::HostResult;
use raiko_reqactor::Actor;

#[utoipa::path(
    get,
    path = "/metrics",
    tag = "Metrics",
    responses (
        (status = 200, description = "The metrics have been captured successfully"),
    ),
)]
/// Get prometheus metrics
///
/// Currently available metrics are:
/// - concurrent_requests - the number of proof requests currently being served
/// - host_request_count - the number of requests sent to this machine in total
/// - host_error_count - the number of requests failed outside of guest execution
/// - guest_proof_request_count - the number of requests sent to this guest
/// - guest_proof_success_count - the number of successful proofs generated by this guest
/// - guest_proof_error_count - the number of failed proofs generated by this guest
/// - guest_proof_time_histogram - time taken for proof generation by this guest
/// - prepare_input_time_histogram - time taken for prepare input
/// - total_time_histogram - time taken for the whole proof request
/// - process_cpu_seconds_total - total user and system CPU time spent in seconds
/// - process_open_fds - number of open file descriptors
/// - process_max_fds - maximum number of open file descriptors
/// - process_virtual_memory_bytes - virtual memory size in bytes
/// - process_resident_memory_bytes - resident memory size in bytes
/// - process_start_time_seconds - start time of the process since unix epoch in seconds
/// - process_threads - number of threads
async fn metrics_handler() -> HostResult<Response> {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    let mf = prometheus::gather();
    encoder.encode(&mf, &mut buffer).unwrap();

    Response::builder()
        .header(header::CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .map_err(|e| anyhow::anyhow!(e).into())
}

#[derive(OpenApi)]
#[openapi(paths(metrics_handler))]
struct Docs;

pub fn create_docs() -> utoipa::openapi::OpenApi {
    Docs::openapi()
}

pub fn create_router() -> Router<Actor> {
    Router::new().route("/", get(metrics_handler))
}
