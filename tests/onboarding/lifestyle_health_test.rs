use reqwest::Client;
use serde_json::json;

use crate::common::utils::spawn_app;
use super::common::register_and_login_user;

#[tokio::test]
async fn submit_lifestyle_health_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    let lifestyle_health = json!({
        "activity_level": "active",
        "bedtime": "23:00",
        "wake_time": "07:00",
        "is_smoker": false,
        "alcohol_consumption": "occasional",
        "tracks_menstrual_cycle": false,
        "medical_conditions": ["asthma"]
    });

    // Act - Try to submit lifestyle health without authentication
    let response = client
        .post(&format!("{}/onboarding/lifestyle_health", &test_app.address))
        .json(&lifestyle_health)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn submit_lifestyle_health_returns_200_with_valid_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let token = register_and_login_user(&client, &test_app, "lifestyleuser").await;

    // Prepare lifestyle health data
    let lifestyle_health = json!({
        "activity_level": "active",
        "bedtime": "23:00",
        "wake_time": "07:00",
        "is_smoker": false,
        "alcohol_consumption": "occasional",
        "tracks_menstrual_cycle": false,
        "medical_conditions": ["asthma"]
    });

    // Act - Submit lifestyle health
    let submit_response = client
        .post(&format!("{}/onboarding/lifestyle_health", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&lifestyle_health)
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
        .get(&format!("{}/onboarding/lifestyle_health", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK");
    
    let get_json = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(get_json["status"], "success", "Response status should be 'success'");
    assert_eq!(get_json["data"]["activity_level"], "active", "Activity level should match");
    assert_eq!(get_json["data"]["bedtime"], "23:00", "Bedtime should match");
    assert_eq!(get_json["data"]["wake_time"], "07:00", "Wake time should match");
    assert_eq!(get_json["data"]["is_smoker"], false, "Smoking status should match");
    assert_eq!(get_json["data"]["alcohol_consumption"], "occasional", "Alcohol consumption should match");
    assert_eq!(get_json["data"]["tracks_menstrual_cycle"], false, "Menstrual cycle tracking should match");
    assert_eq!(get_json["data"]["medical_conditions"], json!(["asthma"]), "Medical conditions should match");
} 