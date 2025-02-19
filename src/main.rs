use actix_web::{App, HttpServer};

mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod utils;

use crate::routes::init_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}