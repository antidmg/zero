use std::time::Duration;

use axum::routing::{get, post};
use axum::{AddExtensionLayer, Router};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::routes::subscribe;
use crate::{config::get_config, routes::health_check};

pub async fn run(addr: &str) {
    let config = get_config().expect("Failed to read configuration");
    let conn_str = config.database.connection_string();

    let pool = PgPoolOptions::new()
        .max_connections(1000)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(conn_str.as_str())
        .await
        .expect("Failed to create DB pool.");

    let app = get_app(pool);

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn get_app(pool: Pool<Postgres>) -> axum::routing::Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(pool))
                .into_inner(),
        )
}
