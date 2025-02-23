use actix_web::{App, HttpServer};
use actix_web::dev::Server;

mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod utils;

use crate::routes::init_routes;

pub fn run() -> Result<Server, std::io::Error> {
    //let pool = db::init_db().await;

    let server = HttpServer::new(move || {
        App::new()
            //.app_data(actix_web::web::Data::new(pool.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run();

    Ok(server)
}