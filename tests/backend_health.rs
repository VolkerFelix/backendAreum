use actix_web::test;
use actix_web::dev::Server;
use reqwest::Client;

use areum_backend::run;

#[tokio::test]
async fn backend_health_works() {
    spawn_app();

    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8080/backend_health")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let body = response.text().await.expect("Cannot read response body.");
    let json_response: serde_json::Value = serde_json::from_str(&body).expect("Cannot turn into a json.");

    assert_eq!(json_response, serde_json::json!({
        "status": "UP"
    }));
}

fn spawn_app(){
    let server = run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}