use actix_web::{get, post, web, HttpResponse};

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    BasicInfoRequest, 
    LifestyleHealthRequest, 
    PermissionsSetupRequest, 
    PersonalizationRequest
};
use crate::handlers::onboarding::{
    basic_info,
    common,
    lifestyle_health,
    permissions,
    personalization,
    status
};

// Onboarding status endpoint
#[get("/status")]
pub async fn onboarding_status(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    status::get_onboarding_status(pool, claims).await
}

// Basic info submission endpoint
#[post("/basic_info")]
pub async fn submit_basic_info(
    data: web::Json<BasicInfoRequest>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::submit_basic_info(data, pool, claims).await
}

// Get basic info endpoint
#[get("/basic_info")]
pub async fn get_basic_info(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::get_basic_info(pool, claims).await
}

// Lifestyle and health submission endpoint
#[post("/lifestyle_health")]
pub async fn submit_lifestyle_health(
    data: web::Json<LifestyleHealthRequest>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::submit_lifestyle_health(data, pool, claims).await
}

// Get lifestyle health endpoint
#[get("/lifestyle_health")]
pub async fn get_lifestyle_health(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::get_lifestyle_health(pool, claims).await
}

// Permissions setup submission endpoint
#[post("/permissions_setup")]
pub async fn submit_permissions_setup(
    data: web::Json<PermissionsSetupRequest>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::submit_permissions_setup(data, pool, claims).await
}

// Get permissions setup endpoint
#[get("/permissions_setup")]
pub async fn get_permissions_setup(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::get_permissions_setup(pool, claims).await
}

// Personalization submission endpoint
#[post("/personalization")]
pub async fn submit_personalization(
    data: web::Json<PersonalizationRequest>,
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::submit_personalization(data, pool, claims).await
}

// Get personalization endpoint
#[get("/personalization")]
pub async fn get_personalization(
    pool: web::Data<sqlx::PgPool>,
    claims: web::ReqData<Claims>
) -> impl actix_web::Responder {
    super::get_personalization(pool, claims).await
}

// Configuration function to add onboarding routes to the service
pub fn init_onboarding_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/onboarding")
            .service(onboarding_status)
            .service(submit_basic_info)
            .service(get_basic_info)
            .service(submit_lifestyle_health)
            .service(get_lifestyle_health)
            .service(submit_permissions_setup)
            .service(get_permissions_setup)
            .service(submit_personalization)
            .service(get_personalization)
    );
}