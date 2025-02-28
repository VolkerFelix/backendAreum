use reqwest::Client;
use serde_json::json;

mod utils;
use utils::spawn_app;

#[tokio::test]
async fn register_user_working() {
    let test_app = spawn_app().await;
    let client = Client::new();

    let user_request = json!({
        "username": "testuser",
        "password": "password123"
    });

    let response = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT username FROM users",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, "testuser");
}