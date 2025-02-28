use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

pub mod config;
mod db;
mod routes;
mod handlers;
mod models;
mod utils;

use crate::routes::init_routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    //let pool = db::init_db().await;

    let server = HttpServer::new(move || {
        App::new()
            //.app_data(actix_web::web::Data::new(pool.clone()))
            .configure(init_routes)
    })
    .listen(listener)?
    .run();

    Ok(server)
}