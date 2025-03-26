use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use crate::common::utils::spawn_app;
use super::common::register_and_login_user;

#[tokio::test]
async fn submit_personalization_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    let personalization = json!({
        "stress_triggers": ["work_deadlines", "lack_of_sleep"],
        "work_type": "office",
        "timezone": "UTC",
        "location_data": {
            "country": "US",
            "city": "San Francisco"
        }
    });

    // Act - Try to submit personalization without authentication
    let response = client
        .post(&format!("{}/onboarding/personalization", &test_app.address))
        .json(&personalization)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn submit_personalization_returns_200_with_valid_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let token = register_and_login_user(&client, &test_app, "personalizationuser").await;

    // Prepare personalization data
    let personalization = json!({
        "stress_triggers": ["work_deadlines", "lack_of_sleep"],
        "work_type": "office",
        "timezone": "UTC",
        "location_data": {
            "country": "US",
            "city": "San Francisco"
        }
    });

    // Act - Submit personalization
    let submit_response = client
        .post(&format!("{}/onboarding/personalization", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&personalization)
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
        .get(&format!("{}/onboarding/personalization", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK");
    
    let get_json = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(get_json["status"], "success", "Response status should be 'success'");
    assert_eq!(get_json["data"]["stress_triggers"], json!(["work_deadlines", "lack_of_sleep"]), "Stress triggers should match");
    assert_eq!(get_json["data"]["work_type"], "office", "Work type should match");
    assert_eq!(get_json["data"]["timezone"], "UTC", "Timezone should match");
} 