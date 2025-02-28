use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;

use crate::models::user::AuthRequest;
use crate::utils::password::hash_password;

pub async fn register_user(
    user_form: web::Json<AuthRequest>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let hashed_password = hash_password(&user_form.password);
    let id = uuid::Uuid::new_v4();

    match sqlx::query!(
        r#"
        INSERT INTO users (id, username, password, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        &user_form.username,
        &hashed_password,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}