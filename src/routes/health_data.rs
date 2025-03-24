// Update src/routes/health_data.rs
use actix_web::{post, get, web, HttpResponse};
use crate::handlers::health_data::{
    acceleration::upload_acceleration_data, 
    acceleration::get_user_acceleration_data, 
    heart_rate::upload_heart_rate_data, 
    heart_rate::get_user_heart_rate_data,
    blood_oxygen::upload_blood_oxygen_data,
    blood_oxygen::get_user_blood_oxygen_data,
    skin_temperature::upload_skin_temperature_data,
    skin_temperature::get_user_skin_temperature_data,
    gps_location::upload_gps_location_data,
    gps_location::get_user_gps_location_data,
    gps_location::get_health_data_with_gps
};
use crate::middleware::auth::Claims;
use crate::models::health_data::{
    AccelerationDataUpload, 
    HeartRateDataUpload, 
    BloodOxygenDataUpload,
    SkinTemperatureDataUpload,
    GpsLocationDataUpload,
    HealthDataTimeQuery
};

#[post("/upload_acceleration")]
async fn upload_acceleration(
    data: web::Json<AccelerationDataUpload>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    upload_acceleration_data(data, pool, claims).await
}

#[post("/upload_heart_rate")]
async fn upload_heart_rate(
    data: web::Json<HeartRateDataUpload>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    upload_heart_rate_data(data, pool, claims).await
}

#[post("/upload_blood_oxygen")]
async fn upload_blood_oxygen(
    data: web::Json<BloodOxygenDataUpload>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    upload_blood_oxygen_data(data, pool, claims).await
}

#[get("/acceleration_data")]
async fn get_acceleration_data(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    get_user_acceleration_data(pool, claims).await
}

#[get("/heart_rate_data")]
async fn get_heart_rate_data(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    get_user_heart_rate_data(pool, claims).await
}

#[get("/blood_oxygen_data")]
async fn get_blood_oxygen_data(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    get_user_blood_oxygen_data(pool, claims).await
}

#[post("/upload_skin_temperature")]
async fn upload_skin_temperature(
    data: web::Json<SkinTemperatureDataUpload>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    upload_skin_temperature_data(data, pool, claims).await
}

#[get("/skin_temperature_data")]
async fn get_skin_temperature_data(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    get_user_skin_temperature_data(pool, claims).await
}

#[post("/upload_gps_location")]
async fn upload_gps_location(
    data: web::Json<GpsLocationDataUpload>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    upload_gps_location_data(data, pool, claims).await
}

#[get("/gps_location_data")]
async fn get_gps_location_data(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    get_user_gps_location_data(pool, claims).await
}

#[get("/health_data_with_gps")]
async fn get_health_with_gps(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>,
    params: web::Query<HealthDataTimeQuery>
) -> HttpResponse {
    get_health_data_with_gps(pool, claims, params).await
}