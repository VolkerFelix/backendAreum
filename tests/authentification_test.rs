use reqwest::Client;
use serde_json::json;

mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn login_returns_200_for_valid_credentials() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register a new user first
    let username = "testuser";
    let password = "password123";
    let email = "testuser@example.com";

    let user_request = json!({
        "username": username,
        "password": password,
        "email": email
    });

    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute registration request.");

    // Act - Try to login
    let login_request = json!({
        "username": username,
        "password": password
    });

    let response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&login_request)
        .send()
        .await
        .expect("Failed to execute login request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    
    // Check that the response contains a token
    let login_response = response.json::<serde_json::Value>().await
        .expect("Failed to parse login response as JSON");
    assert!(login_response.get("token").is_some(), "Response should contain a token");
}

#[tokio::test]
async fn login_returns_401_for_invalid_credentials() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Act - Try to login with non-existent user
    let login_request = json!({
        "username": "nonexistentuser",
        "password": "wrongpassword"
    });

    let response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&login_request)
        .send()
        .await
        .expect("Failed to execute login request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}