use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};

use hyper::StatusCode;
use serde::Deserialize;
use sqlx::{types::time::OffsetDateTime, Pool, Postgres};
use tracing::error;
use uuid::Uuid;

#[allow(unused)]
#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

// Attaches a span to our function declaration
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(
    form: Form<FormData>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> impl IntoResponse {
    match insert_subscriber(&pool, form).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &Pool<Postgres>,
    form: Form<FormData>,
) -> Result<(), crate::error::Error> {
    let res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        OffsetDateTime::now_utc(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
    });

    // TODO: we will improve this error handling later
    match res {
        Ok(_) => Ok(()),
        _ => Err(crate::error::Error::Internal(
            "Error while executing DB query".into(),
        )),
    }
}
