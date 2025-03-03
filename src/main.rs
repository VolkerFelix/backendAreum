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
    let conection_pool = PgPool::connect(
        &config.database.connection_string().expose_secret()
        ).await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(&address)?;
    
    run(listener, conection_pool)?.await
}