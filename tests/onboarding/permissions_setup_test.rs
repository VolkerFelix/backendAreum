use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use crate::common::utils::spawn_app;
use super::common::register_and_login_user;

#[tokio::test]
async fn submit_permissions_setup_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    let permissions = json!({
        "heart_rate_enabled": true,
        "temperature_enabled": true,
        "spo2_enabled": true,
        "accelerometer_enabled": true,
        "notifications_enabled": true,
        "background_usage_enabled": true,
        "third_party_connections": [
            {
                "connection_type": "apple_health",
                "connection_data": {
                    "permissions": ["read", "write"]
                }
            }
        ]
    });

    // Act - Try to submit permissions without authentication
    let response = client
        .post(&format!("{}/onboarding/permissions_setup", &test_app.address))
        .json(&permissions)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn submit_permissions_setup_returns_200_with_valid_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let token = register_and_login_user(&client, &test_app, "permissionsuser").await;

    // Prepare permissions data
    let permissions = json!({
        "heart_rate_enabled": true,
        "temperature_enabled": true,
        "spo2_enabled": true,
        "accelerometer_enabled": true,
        "notifications_enabled": true,
        "background_usage_enabled": true,
        "third_party_connections": [
            {
                "connection_type": "apple_health",
                "connection_data": {
                    "permissions": ["read", "write"]
                }
            }
        ]
    });

    // Act - Submit permissions
    let submit_response = client
        .post(&format!("{}/onboarding/permissions_setup", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&permissions)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, submit_response.status().as_u16(), "Should return 200 OK");
    
    let submit_json = submit_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(submit_json["status"], "success", "Response status should be 'success'");
    
    // Verify the data was saved by getting it back
    let get_response = client
        .get(&format!("{}/onboarding/permissions_setup", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK");
    
    let get_json = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(get_json["status"], "success", "Response status should be 'success'");
    assert_eq!(get_json["data"]["heart_rate_enabled"], true, "Heart rate permission should match");
    assert_eq!(get_json["data"]["temperature_enabled"], true, "Temperature permission should match");
    assert_eq!(get_json["data"]["spo2_enabled"], true, "SpO2 permission should match");
    assert_eq!(get_json["data"]["accelerometer_enabled"], true, "Accelerometer permission should match");
    assert_eq!(get_json["data"]["notifications_enabled"], true, "Notifications permission should match");
    assert_eq!(get_json["data"]["background_usage_enabled"], true, "Background usage permission should match");
    assert_eq!(get_json["data"]["third_party_connections"][0]["connection_type"], "apple_health", "Third party connection type should match");
} 