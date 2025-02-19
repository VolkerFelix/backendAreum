use actix_web::web;

pub mod auth;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::register);
       //.service(auth::login)
       //.service(health::submit_health)
       //.service(health::get_wellness_insights);
}