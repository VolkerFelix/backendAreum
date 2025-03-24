// Create tests/blood_oxygen_data_test.rs
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn upload_blood_oxygen_data_returns_200_for_authenticated_user() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user first
    let username = format!("oxygenuser{}", Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    // Register user
    let user_request = json!({
        "username": username,
        "password": password,
        "email": email
    });

    let register_response = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute registration request.");

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
    let token = login_json["token"].as_str().expect("Token not found in response");

    // Create dummy blood oxygen data
    let blood_oxygen_data = json!({
        "data_type": "blood_oxygen",
        "device_info": {
            "device_type": "smartwatch",
            "model": "AppleWatch Series 8",
            "os_version": "watchOS 10.1"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-10T12:00:00Z",
        "end_time": "2025-03-10T12:00:10Z",
        "samples": [
            {"timestamp": "2025-03-10T12:00:00Z", "spo2": 98.5, "confidence": 0.95},
            {"timestamp": "2025-03-10T12:00:01Z", "spo2": 98.6, "confidence": 0.96},
            {"timestamp": "2025-03-10T12:00:02Z", "spo2": 98.7, "confidence": 0.97},
            {"timestamp": "2025-03-10T12:00:03Z", "spo2": 98.5, "confidence": 0.95}
        ],
        "metadata": {"activity": "resting"}
    });

    // Act - Upload blood oxygen data
    let upload_response = client
        .post(&format!("{}/health/upload_blood_oxygen", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&blood_oxygen_data)
        .send()
        .await
        .expect("Failed to execute upload request.");

    // Assert
    assert_eq!(200, upload_response.status().as_u16(), "Upload should succeed");
    
    // Verify the response contains the expected structure
    let response_body = upload_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert!(response_body.get("id").is_some(), "Response should contain an id");
    assert_eq!(response_body["status"], "success", "Status should be success");

    // Verify the data was saved in the database
    let saved = sqlx::query!(
        r#"
        SELECT user_id, data_type, device_info, sampling_rate_hz 
        FROM health_data 
        WHERE id = $1
        "#,
        Uuid::parse_str(response_body["id"].as_str().unwrap()).unwrap()
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch saved health data.");

    // Get the user's UUID to compare with the saved user_id
    let user = sqlx::query!(
        r#"SELECT id FROM users WHERE username = $1"#,
        username
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch user.");

    assert_eq!(saved.user_id, user.id, "User ID should match");
    assert_eq!(saved.data_type, "blood_oxygen", "Data type should be blood_oxygen");
    assert_eq!(saved.sampling_rate_hz, 1, "Sampling rate should match");
    
    // Now try to retrieve the data with GET endpoint
    let get_response = client
        .get(&format!("{}/health/blood_oxygen_data", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");
    
    // Verify GET endpoint returns 200 OK and contains our data
    assert_eq!(200, get_response.status().as_u16(), "GET request should succeed");
    
    let get_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse GET response as JSON");
    
    assert_eq!(get_body["status"], "success", "GET status should be success");
    assert_eq!(get_body["count"], 1, "Should have 1 record");
    assert!(get_body["data"].is_array(), "Data field should be an array");
    assert_eq!(get_body["data"][0]["id"], response_body["id"], "Record ID should match");
}

#[tokio::test]
async fn upload_blood_oxygen_data_without_confidence_succeeds() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user
    let username = format!("oxygenuser{}", Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    // Register and login
    let user_request = json!({
        "username": username,
        "password": password,
        "email": email
    });

    let _ = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&user_request)
        .send()
        .await
        .expect("Failed to execute registration request.");

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

    let login_json = login_response.json::<serde_json::Value>().await
        .expect("Failed to parse login response as JSON");
    let token = login_json["token"].as_str().expect("Token not found in response");

    // Create blood oxygen data without confidence values
    let blood_oxygen_data = json!({
        "data_type": "blood_oxygen",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Fitbit Sense",
            "os_version": "Fitbit OS 5.3"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-10T14:00:00Z",
        "end_time": "2025-03-10T14:00:10Z",
        "samples": [
            {"timestamp": "2025-03-10T14:00:00Z", "spo2": 97.2},
            {"timestamp": "2025-03-10T14:00:01Z", "spo2": 97.1},
            {"timestamp": "2025-03-10T14:00:02Z", "spo2": 97.3}
        ]
    });

    // Act - Upload blood oxygen data
    let upload_response = client
        .post(&format!("{}/health/upload_blood_oxygen", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&blood_oxygen_data)
        .send()
        .await
        .expect("Failed to execute upload request.");

    // Assert
    assert_eq!(200, upload_response.status().as_u16(), "Upload should succeed even with missing confidence values");
}

#[tokio::test]
async fn get_blood_oxygen_data_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Act - Try to get blood oxygen data without authentication
    let response = client
        .get(&format!("{}/health/blood_oxygen_data", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn upload_blood_oxygen_data_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Create dummy blood oxygen data
    let blood_oxygen_data = json!({
        "data_type": "blood_oxygen",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Samsung Galaxy Watch",
            "os_version": "Wear OS 4.0"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-10T12:00:00Z",
        "end_time": "2025-03-10T12:00:10Z",
        "samples": [
            {"timestamp": "2025-03-10T12:00:00Z", "spo2": 99.1, "confidence": 0.95},
            {"timestamp": "2025-03-10T12:00:01Z", "spo2": 99.0, "confidence": 0.94}
        ]
    });

    // Act - Try to upload without authentication
    let response = client
        .post(&format!("{}/health/upload_blood_oxygen", &test_app.address))
        .json(&blood_oxygen_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}