use reqwest::Client;
use std::net::TcpListener;
use serde_json::json;
use sqlx::{PgConnection, Connection};

use areum_backend::run;
use areum_backend::config::get_config;

#[tokio::test]
async fn register_user_working() {
    let address = spawn_app();
    let config = get_config().expect("Failed to get config.");
    let config_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&config_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = Client::new();

    let user_request = json!({
        "username": "testuser",
        "password": "password123"
    });

    let response = client
        .post(&format!("{}/register_user", &address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT username FROM users",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, "testuser");

}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // Get port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}