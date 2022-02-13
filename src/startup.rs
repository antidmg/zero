use std::time::Duration;

use axum::routing::{get, post};
use axum::{AddExtensionLayer, Router};

use hyper::header::HeaderName;
use hyper::{Body, Request};
use secrecy::ExposeSecret;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::TraceLayer;
use tracing::info;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use crate::routes::subscribe;
use crate::{config::get_config, routes::health_check};

pub async fn run(addr: &str) {
    let config = get_config().expect("Failed to read configuration");

    let pool = PgPoolOptions::new()
        .max_connections(1000)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to create DB pool.");
    info!("Created DB connection pool");

    let app = get_app(pool);

    info!("Starting server on: {addr}");
    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn get_app(pool: Pool<Postgres>) -> axum::routing::Router {
    let x_request_id = HeaderName::from_static("x-request-id");
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(AddExtensionLayer::new(pool))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                    .unwrap_or_default();

                tracing::info_span!(
                    "HTTP",
                    http.method = %request.method(),
                    http.url = %request.uri(),
                    request_id = %request_id,
                )
            }),
        )
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id))
}

#[derive(Copy, Clone)]
struct MakeRequestUuid;

/// Implement the trait for producing a request ID from the incoming request.
/// In our case, we want to generate a new UUID that we can associate with a single request.
impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}
