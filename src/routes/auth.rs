use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::handlers::auth_handler::register_user;
use crate::models::user::AuthRequest;

#[post("/register")]
async fn register(pool: web::Data<sqlx::PgPool>, user: web::Json<AuthRequest>) -> impl Responder {
    register_user(pool, user).await
}