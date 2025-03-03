use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

use crate::models::user::AuthRequest;
use crate::utils::password::hash_password;

#[tracing::instrument(
    name = "Adding a new user",
    // Don't show arguments
    skip(user_form, pool),
    fields(
        subscriber_email = %user_form.username,
    )
)]
pub async fn register_user(
    user_form: web::Json<AuthRequest>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    match insert_user(&user_form, &pool).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn insert_user(
    user_form: &web::Json<AuthRequest>,
    pool: &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, password, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        &user_form.username,
        &hash_password(&user_form.password),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}