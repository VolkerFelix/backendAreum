use reqwest::Client;
use serde_json::json;

use crate::common::utils::spawn_app;
use super::common::register_and_login_user;

#[tokio::test]
async fn onboarding_status_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Act - Try to get onboarding status without authentication
    let response = client
        .get(&format!("{}/onboarding/status", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn onboarding_status_returns_200_with_valid_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let token = register_and_login_user(&client, &test_app, "onboardinguser").await;

    // Act - Get onboarding status
    let status_response = client
        .get(&format!("{}/onboarding/status", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, status_response.status().as_u16(), "Should return 200 OK");
    
    let status_json = status_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(status_json["status"], "success", "Response status should be 'success'");
    assert_eq!(status_json["data"]["basic_info_completed"], false, "Basic info should not be completed");
    assert_eq!(status_json["data"]["lifestyle_health_completed"], false, "Lifestyle health should not be completed");
    assert_eq!(status_json["data"]["permissions_setup_completed"], false, "Permissions setup should not be completed");
    assert_eq!(status_json["data"]["personalization_completed"], false, "Personalization should not be completed");
    assert_eq!(status_json["data"]["onboarding_completed"], false, "Onboarding should not be completed");
    assert_eq!(status_json["data"]["current_step"], "basic_info", "Current step should be 'basic_info'");
} 