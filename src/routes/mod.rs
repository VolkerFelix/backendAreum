use actix_web::web;

pub mod registration;
pub mod backend_health;
pub mod auth;
pub mod protected;
pub mod health_data;
pub mod onboarding;

use crate::middleware::auth::AuthMiddleware;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(registration::register)
        .service(backend_health::backend_health)
        .service(auth::login);

    cfg.service(
        web::scope("/protected")
            .service(protected::protected_resource)
    );

    cfg.service(
        web::scope("/health")
            .wrap(AuthMiddleware)
            .service(health_data::upload_acceleration)
            .service(health_data::get_acceleration_data)
            .service(health_data::upload_heart_rate)
            .service(health_data::get_heart_rate_data)
            .service(health_data::upload_blood_oxygen)
            .service(health_data::get_blood_oxygen_data)
            .service(health_data::upload_skin_temperature)
            .service(health_data::get_skin_temperature_data)
            .service(health_data::upload_gps_location)
            .service(health_data::get_gps_location_data)
            .service(health_data::get_health_with_gps)
            .service(health_data::get_sleep_data)
            .service(health_data::get_sleep_range)
            .service(health_data::get_sleep_summary)
            .service(health_data::get_sleep_trends)
    );
    cfg.service(
        web::scope("/onboarding")
            .service(onboarding::onboarding_status)
            .service(onboarding::submit_basic_info)
            .service(onboarding::get_basic_info)
            .service(onboarding::submit_lifestyle_health)
            .service(onboarding::get_lifestyle_health)
            .service(onboarding::submit_permissions_setup)
            .service(onboarding::get_permissions_setup)
            .service(onboarding::submit_personalization)
            .service(onboarding::get_personalization)
    );

}