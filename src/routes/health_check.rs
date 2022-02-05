use axum::response::IntoResponse;
use hyper::StatusCode;
use tracing::info;

pub async fn health_check() -> impl IntoResponse {
    info!("health check OK");
    StatusCode::OK
}
