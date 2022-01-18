use std::borrow::Cow;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

pub async fn run() {
    // run it with hyper on localhost:3000
    let app = get_app();
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn get_app() -> axum::routing::Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn greet(Path(key): Path<String>) -> impl IntoResponse {
    let name = key;
    println!("Hello, {}", &name);
    (StatusCode::OK, Cow::from(format!("Hello, {name}")))
}
