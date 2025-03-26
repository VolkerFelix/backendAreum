mod common;
mod onboarding;

use common::utils::spawn_app;
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Act
    let response = client
        .get(&format!("{}/backend_health", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let json = response.json::<serde_json::Value>().await.expect("Failed to parse response as JSON");
    assert_eq!(json["status"], "UP", "Response status should be 'UP'");
}

#[tokio::test]
async fn register_user_returns_200_for_valid_data() {
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register a new user first
    let username = format!("protecteduser{}", uuid::Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    let user_request = json!({
        "username": username,
        "password": password,
        "email": email
    });

    let response = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT username, email FROM users WHERE username = $1",
        username
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch saved user.");

    assert_eq!(saved.username, username);
    assert_eq!(saved.email, email);
}

#[tokio::test]
async fn register_user_returns_400_for_invalid_data() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Act
    let response = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({
            "username": "",
            "password": "",
            "email": "invalid-email"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn login_returns_200_for_valid_credentials() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let username = format!("testuser{}", Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    // Register user first
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({
            "username": username,
            "password": password,
            "email": email
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Act - Login
    let response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let response_json = response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    assert!(response_json["token"].is_string(), "Response should contain a token");
}

#[tokio::test]
async fn login_returns_401_for_invalid_credentials() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Act
    let response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({
            "username": "nonexistent",
            "password": "wrongpassword"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
} 