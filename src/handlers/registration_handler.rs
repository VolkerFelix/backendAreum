use actix_web::{web, HttpResponse};
use secrecy::ExposeSecret;
use serde_json::json;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

use crate::models::user::RegistrationRequest;
use crate::utils::password::hash_password;

#[tracing::instrument(
    name = "Adding a new user",
    // Don't show arguments
    skip(user_form, pool),
    fields(
        username = %user_form.username,
        email = %user_form
    )
)]
pub async fn register_user(
    user_form: web::Json<RegistrationRequest>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    tracing::info!("Received registration request for username: {}", user_form.username);
    // Validate input data
    if user_form.username.is_empty() {
        tracing::error!("Username cannot be empty");
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Username cannot be empty" }));
    }

    if user_form.username.len() < 3 || user_form.username.len() > 50 {
        tracing::error!("Username must be between 3 and 50 characters");
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Username must be between 3 and 50 characters" }));
    }

    if user_form.password.expose_secret().is_empty() {
        tracing::error!("Password cannot be empty");
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Password cannot be empty" }));
    }

    if user_form.password.expose_secret().len() < 8 {
        tracing::error!("Password must be at least 8 characters long");
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Password must be at least 8 characters long" }));
    }

    if user_form.email.is_empty() || !user_form.email.contains('@') {
        tracing::error!("Valid email address is required");
        return HttpResponse::BadRequest()
            .json(json!({ "error": "Valid email address is required" }));
    }

    // Proceed with user registration if validation passes
    match insert_user(&user_form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // Check for specific database errors like unique constraint violations
            if let Some(db_error) = e.as_database_error() {
                if db_error.constraint().is_some() {
                    // This is likely a duplicate username or email
                    return HttpResponse::BadRequest()
                        .json(json!({ "error": "Username or email already exists" }));
                }
            }
            // Log other errors and return generic error
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert_user(
    user_form: &web::Json<RegistrationRequest>,
    pool: &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, password_hash, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        &user_form.username,
        &hash_password(&user_form.password.expose_secret()),
        &user_form.email,
        Utc::now(),
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