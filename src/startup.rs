use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::health_check;
use crate::routes::subscribe;

pub async fn run(addr: &str) {
    let app = get_app();
    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn get_app() -> axum::routing::Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}
