use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};

use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::models;

#[allow(unused)]
#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    form: Form<FormData>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> impl IntoResponse {
    models::subscribe(&pool, form).await;
}
