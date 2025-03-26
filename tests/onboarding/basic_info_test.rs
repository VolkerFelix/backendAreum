use reqwest::Client;
use serde_json::json;

use crate::common::utils::spawn_app;
use super::common::register_and_login_user;

#[tokio::test]
async fn submit_basic_info_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    let basic_info = json!({
        "display_name": "Test User",
        "date_of_birth": "1990-01-01",
        "biological_sex": "male",
        "height_cm": 180.0,
        "weight_kg": 75.0,
        "goals": ["improve_sleep", "reduce_stress"]
    });

    // Act - Try to submit basic info without authentication
    let response = client
        .post(&format!("{}/onboarding/basic_info", &test_app.address))
        .json(&basic_info)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn submit_basic_info_returns_200_with_valid_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let token = register_and_login_user(&client, &test_app, "basicinfouser").await;

    // Prepare basic info data
    let basic_info = json!({
        "display_name": "Test User",
        "date_of_birth": "1990-01-01",
        "biological_sex": "male",
        "height_cm": 180.0,
        "weight_kg": 75.0,
        "goals": ["improve_sleep", "reduce_stress"]
    });

    // Act - Submit basic info
    let submit_response = client
        .post(&format!("{}/onboarding/basic_info", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&basic_info)
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
        .get(&format!("{}/onboarding/basic_info", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK");
    
    let get_json = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(get_json["status"], "success", "Response status should be 'success'");
    assert_eq!(get_json["data"]["display_name"], "Test User", "Display name should match");
    assert_eq!(get_json["data"]["date_of_birth"], "1990-01-01", "Date of birth should match");
    assert_eq!(get_json["data"]["biological_sex"], "male", "Biological sex should match");
    assert_eq!(get_json["data"]["height_cm"], 180.0, "Height should match");
    assert_eq!(get_json["data"]["weight_kg"], 75.0, "Weight should match");
    assert_eq!(get_json["data"]["goals"], json!(["improve_sleep", "reduce_stress"]), "Goals should match");
} 