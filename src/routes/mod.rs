use actix_web::web;

pub mod registration;
pub mod backend_health;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(registration::register)
        .service(backend_health::backend_health);
       //.service(auth::login)
       //.service(health::submit_health)
       //.service(health::get_wellness_insights);
}