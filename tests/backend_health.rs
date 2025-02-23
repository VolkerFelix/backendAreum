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
    //assert_eq!(Some(0), response.content_length());

}

fn spawn_app(){
    let server = run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}