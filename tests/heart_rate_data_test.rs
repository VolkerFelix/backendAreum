
use reqwest::StatusCode;
use serde_json::json;
use chrono::Utc;

mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn test_upload_heart_rate_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // First, register a user
    let response = client
        .post(&format!("{}/register_user", &app.address))
        .json(&json!({
            "username": "testuser",
            "password": "testpassword",
            "email": "test@example.com"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);

    // Login to get a token
    let response = client
        .post(&format!("{}/login", &app.address))
        .json(&json!({
            "username": "testuser",
            "password": "testpassword"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);
    let token: serde_json::Value = response.json().await.unwrap();
    let token = token["token"].as_str().unwrap();

    // Create test heart rate data
    let heart_rate_data = json!({
        "data_type": "heart_rate",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Test Watch",
            "os_version": "1.0.0",
            "device_id": "test-device-1"
        },
        "sampling_rate_hz": 1,
        "start_time": Utc::now().to_rfc3339(),
        "samples": [
            {
                "timestamp": Utc::now().to_rfc3339(),
                "heart_rate": 75,
                "confidence": 0.95
            }
        ],
        "metadata": {
            "session_id": "test-session-1",
            "notes": "Test measurement"
        }
    });

    // Act
    let response = client
        .post(&format!("{}/health/upload_heart_rate", &app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&heart_rate_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let response_body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(response_body["status"], "success");
    assert!(response_body["id"].is_string());
}

#[tokio::test]
async fn test_get_heart_rate_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // First, register a user
    let response = client
        .post(&format!("{}/register_user", &app.address))
        .json(&json!({
            "username": "testuser",
            "password": "testpassword",
            "email": "test@example.com"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);

    // Login to get a token
    let response = client
        .post(&format!("{}/login", &app.address))
        .json(&json!({
            "username": "testuser",
            "password": "testpassword"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);
    let token: serde_json::Value = response.json().await.unwrap();
    let token = token["token"].as_str().unwrap();

    // Upload some test heart rate data
    let heart_rate_data = json!({
        "data_type": "heart_rate",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Test Watch",
            "os_version": "1.0.0",
            "device_id": "test-device-1"
        },
        "sampling_rate_hz": 1,
        "start_time": Utc::now().to_rfc3339(),
        "samples": [
            {
                "timestamp": Utc::now().to_rfc3339(),
                "heart_rate": 75,
                "confidence": 0.95
            }
        ]
    });

    let response = client
        .post(&format!("{}/health/upload_heart_rate", &app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&heart_rate_data)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);

    // Act
    let response = client
        .get(&format!("{}/health/heart_rate_data", &app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let response_body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(response_body["status"], "success");
    assert!(response_body["count"].as_i64().unwrap() > 0);
    assert!(response_body["data"].is_array());
    
    let data = response_body["data"].as_array().unwrap();
    assert!(!data.is_empty());
    
    let first_record = &data[0];
    assert_eq!(first_record["data_type"], "heart_rate");
    assert_eq!(first_record["sampling_rate_hz"], 1);
    assert!(first_record["data"]["samples"].is_array());
}

#[tokio::test]
async fn test_upload_heart_rate_data_unauthorized() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let heart_rate_data = json!({
        "data_type": "heart_rate",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Test Watch",
            "os_version": "1.0.0"
        },
        "sampling_rate_hz": 1,
        "start_time": Utc::now().to_rfc3339(),
        "samples": [
            {
                "timestamp": Utc::now().to_rfc3339(),
                "heart_rate": 75
            }
        ]
    });

    // Act
    let response = client
        .post(&format!("{}/health/upload_heart_rate", &app.address))
        .json(&heart_rate_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_heart_rate_data_unauthorized() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health/heart_rate_data", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_upload_heart_rate_data_invalid_type() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // First, register and login
    let response = client
        .post(&format!("{}/register_user", &app.address))
        .json(&json!({
            "username": "testuser",
            "password": "testpassword",
            "email": "test@example.com"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .post(&format!("{}/login", &app.address))
        .json(&json!({
            "username": "testuser",
            "password": "testpassword"
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::OK);
    let token: serde_json::Value = response.json().await.unwrap();
    let token = token["token"].as_str().unwrap();

    // Create test data with invalid type
    let heart_rate_data = json!({
        "data_type": "invalid_type",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Test Watch",
            "os_version": "1.0.0"
        },
        "sampling_rate_hz": 1,
        "start_time": Utc::now().to_rfc3339(),
        "samples": [
            {
                "timestamp": Utc::now().to_rfc3339(),
                "heart_rate": 75
            }
        ]
    });

    // Act
    let response = client
        .post(&format!("{}/health/upload_heart_rate", &app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&heart_rate_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let response_body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(response_body["status"], "error");
    assert!(response_body["message"].as_str().unwrap().contains("Invalid data type"));
} 