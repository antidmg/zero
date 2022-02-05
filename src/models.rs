use axum::{extract::Form, response::IntoResponse};
use chrono::Utc;
use hyper::StatusCode;
use sqlx::{Pool, Postgres};
use tracing::{error, info, info_span, Instrument};
use uuid::Uuid;

use crate::routes::FormData;

pub async fn subscribe(pool: &Pool<Postgres>, form: Form<FormData>) -> impl IntoResponse {
    let request_id = Uuid::new_v4();

    let request_span = info_span!("Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // dont do this?
    let _request_span_guard = request_span.enter();

    info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id, form.email, form.name
    );
    let query_span = info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            info!(
                "request_id {} - New subscriber details successfully saved",
                request_id
            );
            StatusCode::OK
        }
        Err(e) => {
            error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
