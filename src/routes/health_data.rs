use actix_web::{post, get, web, HttpResponse};
use crate::handlers::health_data_handler::{upload_acceleration_data, get_user_acceleration_data};
use crate::middleware::auth::Claims;
use crate::models::health_data::AccelerationDataUpload;

#[post("/upload_acceleration")]
async fn upload_acceleration(
    data: web::Json<AccelerationDataUpload>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    upload_acceleration_data(data, pool, claims).await
}

#[get("/acceleration_data")]
async fn get_acceleration_data(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    get_user_acceleration_data(pool, claims).await
}