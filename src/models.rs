use axum::extract::Form;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::routes::FormData;

pub async fn subscribe(pool: &Pool<Postgres>, form: Form<FormData>) {
    let _ = sqlx::query!(
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
    .await;
}
