use axum::{body::Body, http::Request};
use hyper::StatusCode;
use std::net::{SocketAddr, TcpListener};
use zero2prod::get_app;

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

    let client = hyper::Client::new();

    let body = "name=tiny%20cat&email=tiny_cat@gmail.com";
    let response = client
        .request(
            Request::post(format!("http://{addr}/subscriptions"))
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
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
