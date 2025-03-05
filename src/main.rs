use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use areum_backend::run;
use areum_backend::config::get_config;
use areum_backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "areum-backend".into(), "info".into(), std::io::stdout
    );
    init_subscriber(subscriber);

    // Panic if we can't read the config
    let config = get_config().expect("Failed to read the config.");
    // Only try to establish connection when actually used
    let conection_pool = PgPool::connect_lazy(
        &config.database.connection_string().expose_secret()
        )
        .expect("Failed to create Postgres connection pool");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&address)?;
    
    run(listener, conection_pool)?.await
}