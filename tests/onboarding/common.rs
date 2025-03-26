use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use crate::common::utils::spawn_app;

pub async fn register_and_login_user(client: &Client, test_app: &crate::common::utils::TestApp, username_prefix: &str) -> String {
    let username = format!("{}{}", username_prefix, Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    println!("Attempting to register user: {} and email: {}", username, email);

    // Register user
    let user_request = json!({
        "username": username,
        "password": password,
        "email": email
    });

    println!("Sending request body: {}", serde_json::to_string_pretty(&user_request).unwrap());

    let register_response = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute registration request.");

    if register_response.status() != 200 {
        println!("Registration failed with status: {}", register_response.status().as_u16());
        println!("Response body: {}", register_response.text().await.unwrap());
        panic!("Registration failed");
    }

    assert_eq!(200, register_response.status().as_u16(), "Registration should succeed");

    // Login to get a token
    let login_request = json!({
        "username": username,
        "password": password
    });

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&login_request)
        .send()
        .await
        .expect("Failed to execute login request.");

    assert_eq!(200, login_response.status().as_u16(), "Login should succeed");

    let login_json = login_response.json::<serde_json::Value>().await
        .expect("Failed to parse login response as JSON");
    login_json["token"].as_str().expect("Token not found in response").to_string()
} 