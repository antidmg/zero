use axum::body::Bytes;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, routing::post, Router};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(Path(key): Path<String>) -> impl IntoResponse {
    let name = key;
    println!("Hello, {}", &name);
    (StatusCode::OK, Cow::from(format!("Hello, {name}")))
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
