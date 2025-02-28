use std::net::TcpListener;

use areum_backend::run;
use areum_backend::config::get_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read the config
    let config = get_config().expect("Failed to read the config.");

    let address = format!("127.0.0.1:{}", config.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}