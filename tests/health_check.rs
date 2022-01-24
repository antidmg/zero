use axum::{body::Body, http::Request};
use hyper::StatusCode;
use sqlx::{Connection, PgConnection};
use std::net::{SocketAddr, TcpListener};
use zero2prod::config::get_config;
use zero2prod::startup::get_app;

#[tokio::test]
async fn health_check() {
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
async fn valid_form_data_subscribe_success() {
    let listener = get_listener().expect("Faild to create TCP listener");
    let addr = listener.local_addr().unwrap();
    spawn_app(listener).await;

    let config = get_config().expect("Failed to read configuration");
    let conn_str = config.database.connection_string();
    let mut connection = PgConnection::connect(&conn_str)
        .await
        .expect("Failed to connect to Postgres.");

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

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "tiny_cat@gmail.com");
    assert_eq!(saved.name, "tiny cat");
}

#[tokio::test]
async fn missing_form_data_bad_request() {
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

async fn spawn_app(listener: TcpListener) {
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(get_app().into_make_service())
            .await
            .unwrap();
    });
}
