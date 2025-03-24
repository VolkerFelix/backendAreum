// tests/gps_location_data_test.rs
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn upload_gps_location_data_returns_200_for_authenticated_user() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user first
    let username = format!("gpsuser{}", Uuid::new_v4());
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

    // Create dummy GPS location data
    let gps_location_data = json!({
        "data_type": "gps_location",
        "device_info": {
            "device_type": "smartphone",
            "model": "iPhone 14",
            "os_version": "iOS 16.5"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-10T12:00:00Z",
        "end_time": "2025-03-10T12:05:00Z",
        "samples": [
            {"timestamp": "2025-03-10T12:00:00Z", "latitude": 40.7128, "longitude": -74.0060, "altitude": 10.5, "accuracy": 5.0, "speed": 0.0, "bearing": 0.0},
            {"timestamp": "2025-03-10T12:01:00Z", "latitude": 40.7129, "longitude": -74.0061, "altitude": 10.5, "accuracy": 5.0, "speed": 1.2, "bearing": 45.0},
            {"timestamp": "2025-03-10T12:02:00Z", "latitude": 40.7130, "longitude": -74.0062, "altitude": 10.5, "accuracy": 4.5, "speed": 1.3, "bearing": 45.0},
            {"timestamp": "2025-03-10T12:03:00Z", "latitude": 40.7131, "longitude": -74.0063, "altitude": 10.5, "accuracy": 4.5, "speed": 1.3, "bearing": 45.0}
        ],
        "metadata": {"activity": "walking"}
    });

    // Act - Upload GPS location data
    let upload_response = client
        .post(&format!("{}/health/upload_gps_location", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&gps_location_data)
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
    assert_eq!(saved.data_type, "gps_location", "Data type should be gps_location");
    assert_eq!(saved.sampling_rate_hz, 1, "Sampling rate should match");
    
    // Now try to retrieve the data with GET endpoint
    let get_response = client
        .get(&format!("{}/health/gps_location_data", &test_app.address))
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
async fn upload_gps_location_data_without_optional_fields_succeeds() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user
    let username = format!("gpsuser{}", Uuid::new_v4());
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

    // Create GPS location data without optional fields
    let gps_location_data = json!({
        "data_type": "gps_location",
        "device_info": {
            "device_type": "smartphone",
            "model": "Google Pixel 7",
            "os_version": "Android 13"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-10T14:00:00Z",
        "end_time": "2025-03-10T14:02:00Z",
        "samples": [
            {"timestamp": "2025-03-10T14:00:00Z", "latitude": 37.7749, "longitude": -122.4194},
            {"timestamp": "2025-03-10T14:01:00Z", "latitude": 37.7750, "longitude": -122.4195},
            {"timestamp": "2025-03-10T14:02:00Z", "latitude": 37.7751, "longitude": -122.4196}
        ]
    });

    // Act - Upload GPS location data
    let upload_response = client
        .post(&format!("{}/health/upload_gps_location", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&gps_location_data)
        .send()
        .await
        .expect("Failed to execute upload request.");

    // Assert
    assert_eq!(200, upload_response.status().as_u16(), "Upload should succeed even with missing optional fields");
}

#[tokio::test]
async fn test_health_data_with_gps_endpoint() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user
    let username = format!("healthgpsuser{}", Uuid::new_v4());
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

    // Upload GPS data
    let gps_data = json!({
        "data_type": "gps_location",
        "device_info": {
            "device_type": "smartphone",
            "model": "iPhone 14",
            "os_version": "iOS 16.5"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-15T10:00:00Z",
        "end_time": "2025-03-15T10:30:00Z",
        "samples": [
            {"timestamp": "2025-03-15T10:00:00Z", "latitude": 40.7128, "longitude": -74.0060},
            {"timestamp": "2025-03-15T10:05:00Z", "latitude": 40.7129, "longitude": -74.0061},
            {"timestamp": "2025-03-15T10:10:00Z", "latitude": 40.7130, "longitude": -74.0062},
            {"timestamp": "2025-03-15T10:15:00Z", "latitude": 40.7131, "longitude": -74.0063},
            {"timestamp": "2025-03-15T10:20:00Z", "latitude": 40.7132, "longitude": -74.0064},
            {"timestamp": "2025-03-15T10:25:00Z", "latitude": 40.7133, "longitude": -74.0065},
            {"timestamp": "2025-03-15T10:30:00Z", "latitude": 40.7134, "longitude": -74.0066}
        ]
    });

    let _ = client
        .post(&format!("{}/health/upload_gps_location", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&gps_data)
        .send()
        .await
        .expect("Failed to execute GPS upload request.");

    // Upload heart rate data
    let heart_rate_data = json!({
        "data_type": "heart_rate",
        "device_info": {
            "device_type": "smartwatch",
            "model": "Apple Watch Series 8",
            "os_version": "watchOS 10.1"
        },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-15T10:05:00Z",
        "end_time": "2025-03-15T10:20:00Z",
        "samples": [
            {"timestamp": "2025-03-15T10:05:00Z", "heart_rate": 72},
            {"timestamp": "2025-03-15T10:10:00Z", "heart_rate": 75},
            {"timestamp": "2025-03-15T10:15:00Z", "heart_rate": 78},
            {"timestamp": "2025-03-15T10:20:00Z", "heart_rate": 74}
        ]
    });

    let _ = client
        .post(&format!("{}/health/upload_heart_rate", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .json(&heart_rate_data)
        .send()
        .await
        .expect("Failed to execute heart rate upload request.");

    // Now query for health data with GPS
    let query_params = format!(
        "data_type=heart_rate&start_time=2025-03-15T10:00:00Z&end_time=2025-03-15T11:00:00Z"
    );

    let get_response = client
        .get(&format!("{}/health/health_data_with_gps?{}", &test_app.address, query_params))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Verify response
    assert_eq!(200, get_response.status().as_u16(), "GET request should succeed");

    let get_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse GET response as JSON");

    assert_eq!(get_body["status"], "success", "GET status should be success");
    assert_eq!(get_body["count"], 1, "Should have 1 record");
    assert!(get_body["data"].is_array(), "Data field should be an array");
    assert!(get_body["data"][0]["gps_data"].is_array(), "GPS data field should be an array");
    assert!(get_body["data"][0]["gps_data"].as_array().unwrap().len() > 0, "GPS data should contain samples");
}