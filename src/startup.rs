use std::time::Duration;

use axum::routing::{get, post};
use axum::Router;

use sqlx::Error;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::routes::subscribe;
use crate::{config::get_config, routes::health_check};

pub async fn run(addr: &str) {
    let config = get_config().expect("Failed to read configuration");
    let conn_str = config.database.connection_string();

    let pool = db_conn(&conn_str.as_str())
        .await
        .expect("Failed to initialize DB pool.");
    let app = get_app(&pool);

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub type DB = Pool<Postgres>;
pub async fn db_conn(database_url: &str) -> Result<DB, Error> {
    PgPoolOptions::new()
        .max_connections(1000)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(database_url)
        .await
}

pub fn get_app(pool: &DB) -> axum::routing::Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
    // .layer(
    //     ServiceBuilder::new()
    //         .timeout(Duration::from_secs(10))
    //         .layer(TraceLayer::new_for_http())
    //         .layer(AddExtensionLayer::new(pool))
    //         .into_inner(),
    // )
}
