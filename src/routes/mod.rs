use actix_web::web::{self, service};

pub mod auth;
pub mod backend_health;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::register)
        .service(backend_health::backend_health);
       //.service(auth::login)
       //.service(health::submit_health)
       //.service(health::get_wellness_insights);
}