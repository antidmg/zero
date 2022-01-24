use axum::{extract::Form, response::IntoResponse};
use hyper::StatusCode;
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}
