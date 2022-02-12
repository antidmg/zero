use axum::{body::Body, http::Request};
use hyper::StatusCode;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;
use uuid::Uuid;
use zero2prod::config::{get_config, DatabaseSettings};
use zero2prod::startup::get_app;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// Ensure that the `tracing` setup is only done once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    // To reduce noise in the test output, disable logs by default unless we pass this flag as true.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

#[tokio::test]
async fn health_check_200_success() {
    let listener = get_listener().expect("Failed to create TCP listener");
    let addr = listener.local_addr().unwrap();
    spawn_app(listener).await;
    let client = hyper::Client::new();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/health_check", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
}

#[tokio::test]
async fn valid_form_data_subscribe_200_success() {
    let listener = get_listener().expect("Faild to create TCP listener");
    let addr = listener.local_addr().unwrap();
    let conn_str = spawn_app(listener).await;

    let client = hyper::Client::new();

    let body = "name=tiny%20cat&email=tiny_cat%40gmail.com";
    let response = client
        .request(
            Request::post(format!("http://{addr}/subscriptions"))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    let mut connection = PgConnection::connect(&conn_str)
        .await
        .expect("Failed to connect to Postgres.");
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "tiny_cat@gmail.com");
    assert_eq!(saved.name, "tiny cat");
}

#[tokio::test]
async fn missing_form_data_400_bad_request() {
    let listener = get_listener().expect("Faild to create TCP listener");
    let addr = listener.local_addr().unwrap();

    spawn_app(listener).await;
    let client = hyper::Client::new();

    let test_cases = vec![
        ("name=tiny%20cat", "missing email"),
        ("email=tiny_cat@gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = client
            .request(
                Request::post(format!("http://{addr}/subscriptions"))
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(Body::from(invalid_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "API did not fail with 400 when payload was {}",
            error_msg
        );
    }
}

fn get_listener() -> std::io::Result<TcpListener> {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    Ok(listener)
}

async fn spawn_app(listener: TcpListener) -> String {
    // This will be skipped after the first time.
    Lazy::force(&TRACING);

    let mut config = get_config().expect("Failed to read configuration.");
    config.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_db(&config.database)
        .await
        .expect("Failed to configure database.");

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(get_app(pool).into_make_service())
            .await
            .unwrap();
    });

    config.database.connection_string()
}

async fn configure_db(config: &DatabaseSettings) -> Result<PgPool, sqlx::error::Error> {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    println!("config db name: {}", config.database_name);
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to create DB pool.");

    // Ensure that tables are created for this test DB.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database.");
    Ok(pool)
}
