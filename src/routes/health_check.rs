use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use hyper::StatusCode;

use super::subscriptions;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn get_app() -> axum::routing::Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
}
