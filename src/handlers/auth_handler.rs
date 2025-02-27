use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::user::AuthRequest;
use crate::utils::password::hash_password;

pub async fn register_user(user_form: web::Json<AuthRequest>) -> HttpResponse {
    let hashed_password = hash_password(&user_form.password);
    let id = uuid::Uuid::new_v4();

    // let result = sqlx::query!(
    //     "INSERT INTO users (id, username, password) VALUES ($1, $2, $3)",
    //     id, &user.username, &hashed_password
    // )
    // .execute(pool.get_ref())
    // .await;

    // match result {
    //     Ok(_) => HttpResponse::Ok().json("User registered successfully"),
    //     Err(_) => HttpResponse::InternalServerError().json("Error registering user"),
    // }
    HttpResponse::Ok().finish()
}