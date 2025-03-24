// tests/sleep_data_test.rs
use reqwest::Client;
use serde_json::json;
use uuid::Uuid;
use chrono::{Utc, Duration};

mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn get_sleep_data_returns_404_when_no_data_exists() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user first
    let username = format!("sleeptestuser{}", Uuid::new_v4());
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

    // Act - Try to get sleep data for a date where none exists
    let test_date = "2025-03-24";
    let get_response = client
        .get(&format!("{}/health/sleep_data?date={}", &test_app.address, test_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(404, get_response.status().as_u16(), "Should return 404 Not Found when no sleep data exists");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "error", "Response status should be 'error'");
    assert!(response_body["message"].as_str().unwrap().contains("No sleep data found"), 
            "Error message should indicate no sleep data was found");
}

#[tokio::test]
async fn get_sleep_data_returns_200_when_data_exists() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    // Register user
    let register_response = client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({
            "username": username,
            "password": password,
            "email": email
        }))
        .send()
        .await
        .expect("Failed to execute registration request.");

    assert_eq!(200, register_response.status().as_u16());

    // Login to get a token
    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .expect("Failed to execute login request.");

    let login_json = login_response.json::<serde_json::Value>().await.unwrap();
    let token = login_json["token"].as_str().unwrap();

    // Get the user's UUID to insert test data
    let user = sqlx::query!(
        r#"SELECT id FROM users WHERE username = $1"#,
        username
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch user.");

    // Insert test sleep data directly into the database
    let test_date = "2025-03-24";
    let sleep_data_id = Uuid::new_v4();
    let now = Utc::now();
    let sleep_start = now - Duration::hours(8);
    let sleep_end = now - Duration::hours(1);
    
    // Create test data
    let test_data = json!({
        "id": sleep_data_id.to_string(),
        "user_id": user.id.to_string(),
        "night_date": test_date,
        "start_time": sleep_start,
        "end_time": sleep_end,
        "samples": [
            {
                "timestamp": sleep_start.to_rfc3339(),
                "stage": "awake",
                "confidence": 0.95,
                "duration_seconds": 600
            },
            {
                "timestamp": (sleep_start + Duration::minutes(10)).to_rfc3339(),
                "stage": "light",
                "confidence": 0.92,
                "duration_seconds": 1800
            }
        ],
        "metrics": {
            "sleep_efficiency": 92.5,
            "sleep_latency_seconds": 600,
            "awakenings": 3,
            "time_in_bed_seconds": 29700,
            "total_sleep_seconds": 27500,
            "light_sleep_seconds": 14400,
            "deep_sleep_seconds": 7200,
            "rem_sleep_seconds": 5900,
            "awake_seconds": 2200
        },
        "sleep_score": 85
    });

    // Insert test data into the database
    sqlx::query!(
        r#"
        INSERT INTO processed_sleep_data
        (id, user_id, data_type, night_date, data, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        sleep_data_id,
        user.id,
        "sleep_stages",
        test_date,
        test_data,
        Utc::now()
    )
    .execute(&test_app.db_pool)
    .await
    .expect("Failed to insert test sleep data");

    // Act - Get sleep data
    let get_response = client
        .get(&format!("{}/health/sleep_data?date={}", &test_app.address, test_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK when sleep data exists");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "success", "Response status should be 'success'");
    assert!(response_body["data"].is_object(), "Response should include data object");
    assert_eq!(response_body["data"]["id"], sleep_data_id.to_string(), "Sleep data ID should match");
    assert_eq!(response_body["data"]["sleep_score"], 85, "Sleep score should match");
}

#[tokio::test]
async fn get_sleep_data_returns_401_without_token() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();
    let test_date = "2025-03-24";

    // Act - Try to get sleep data without authentication
    let response = client
        .get(&format!("{}/health/sleep_data?date={}", &test_app.address, test_date))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16(), "Should return 401 Unauthorized");
}

#[tokio::test]
async fn get_sleep_data_returns_400_with_invalid_date_format() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register and login a user
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    // Register and login
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ "username": username, "password": password, "email": email }))
        .send()
        .await
        .expect("Failed to execute registration request.");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to execute login request.");

    let login_json = login_response.json::<serde_json::Value>().await.unwrap();
    let token = login_json["token"].as_str().unwrap();

    // Act - Try to get sleep data with invalid date format
    let invalid_date = "24-03-2025"; // DD-MM-YYYY instead of YYYY-MM-DD
    let get_response = client
        .get(&format!("{}/health/sleep_data?date={}", &test_app.address, invalid_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(400, get_response.status().as_u16(), "Should return 400 Bad Request with invalid date format");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "error", "Response status should be 'error'");
    assert!(response_body["message"].as_str().unwrap().contains("Invalid date format"), 
            "Error message should indicate invalid date format");
}

#[tokio::test]
async fn get_sleep_summary_returns_404_when_no_summary_exists() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register, login and get token
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    let email = format!("{}@example.com", username);

    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ "username": username, "password": password, "email": email }))
        .send()
        .await
        .expect("Failed to execute registration request.");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to execute login request.");

    let token = login_response.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // Act - Try to get sleep summary for a date where none exists
    let test_date = "2025-03-24";
    let get_response = client
        .get(&format!("{}/health/sleep_summary?date={}", &test_app.address, test_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(404, get_response.status().as_u16(), "Should return 404 Not Found when no sleep summary exists");
}

#[tokio::test]
async fn get_sleep_summary_returns_200_when_summary_exists() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register, login and get user ID
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ 
            "username": username, 
            "password": password, 
            "email": format!("{}@example.com", username) 
        }))
        .send()
        .await
        .expect("Failed to execute registration request.");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to execute login request.");

    let token = login_response.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // Get user ID
    let user = sqlx::query!(
        r#"SELECT id FROM users WHERE username = $1"#,
        username
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch user.");

    // Insert test sleep summary
    let test_date = "2025-03-24";
    let summary_id = Uuid::new_v4();
    
    let test_summary = json!({
        "id": summary_id.to_string(),
        "user_id": user.id.to_string(),
        "night_date": test_date,
        "sleep_metrics": {
            "sleep_efficiency": 92.5,
            "sleep_latency_seconds": 600,
            "awakenings": 3,
            "time_in_bed_seconds": 29700,
            "total_sleep_seconds": 27500,
            "light_sleep_seconds": 14400,
            "deep_sleep_seconds": 7200,
            "rem_sleep_seconds": 5900,
            "awake_seconds": 2200
        },
        "sleep_score": 85,
        "overall_quality": "Good",
        "highlights": [
            "Excellent deep sleep duration",
            "Consistent sleep schedule",
            "Good sleep efficiency"
        ],
        "issues": [
            "Slightly long time to fall asleep",
            "Brief awakening at 3:20 AM"
        ],
        "stage_distribution": {
            "awake_percentage": 7.5,
            "light_percentage": 48.5,
            "deep_percentage": 24.2,
            "rem_percentage": 19.8
        },
        "recommendations": [
            "Consider relaxation techniques before bedtime to reduce sleep latency",
            "Maintain your consistent sleep schedule",
            "Your deep sleep is excellent - continue with your current exercise routine"
        ],
        "created_at": Utc::now()
    });

    sqlx::query!(
        r#"
        INSERT INTO processed_sleep_data
        (id, user_id, data_type, night_date, data, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        summary_id,
        user.id,
        "sleep_summary",
        test_date,
        test_summary,
        Utc::now()
    )
    .execute(&test_app.db_pool)
    .await
    .expect("Failed to insert test sleep summary");

    // Act - Get sleep summary
    let get_response = client
        .get(&format!("{}/health/sleep_summary?date={}", &test_app.address, test_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK when sleep summary exists");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "success", "Response status should be 'success'");
    assert!(response_body["data"].is_object(), "Response should include data object");
    assert_eq!(response_body["data"]["sleep_score"], 85, "Sleep score should match");
    assert_eq!(response_body["data"]["overall_quality"], "Good", "Overall quality should match");
    assert!(response_body["data"]["recommendations"].is_array(), "Recommendations should be an array");
}

#[tokio::test]
async fn get_sleep_data_range_returns_empty_array_when_no_data_exists() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register, login and get token
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ 
            "username": username, 
            "password": password, 
            "email": format!("{}@example.com", username) 
        }))
        .send()
        .await
        .expect("Failed to register");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to login");

    let token = login_response.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // Act - Get sleep data range when no data exists
    let start_date = "2025-03-20";
    let end_date = "2025-03-27";
    let get_response = client
        .get(&format!("{}/health/sleep_data_range?start_date={}&end_date={}", 
                     &test_app.address, start_date, end_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK with empty array");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "success", "Response status should be 'success'");
    assert_eq!(response_body["count"], 0, "Count should be 0");
    assert!(response_body["data"].is_array(), "Data should be an array");
    assert_eq!(response_body["data"].as_array().unwrap().len(), 0, "Data array should be empty");
}

#[tokio::test]
async fn get_sleep_data_range_returns_400_with_invalid_date_range() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register, login and get token
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ 
            "username": username, 
            "password": password, 
            "email": format!("{}@example.com", username) 
        }))
        .send()
        .await
        .expect("Failed to register");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to login");

    let token = login_response.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // Act - Try to get sleep data with end date before start date
    let start_date = "2025-03-27";
    let end_date = "2025-03-20"; // Before start date
    let get_response = client
        .get(&format!("{}/health/sleep_data_range?start_date={}&end_date={}", 
                     &test_app.address, start_date, end_date))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(400, get_response.status().as_u16(), "Should return 400 Bad Request with invalid date range");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "error", "Response status should be 'error'");
    assert!(response_body["message"].as_str().unwrap().contains("End date must be"), 
            "Error message should explain date range issue");
}

#[tokio::test]
async fn get_weekly_sleep_trends_returns_success_with_empty_data() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register, login and get token
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ 
            "username": username, 
            "password": password, 
            "email": format!("{}@example.com", username) 
        }))
        .send()
        .await
        .expect("Failed to register");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to login");

    let token = login_response.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // Act - Get weekly sleep trends when no data exists
    let get_response = client
        .get(&format!("{}/health/sleep_trends", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "success", "Response status should be 'success'");
    assert!(response_body["message"].is_string(), "Response should include a message");
    assert_eq!(response_body["data"]["days_with_data"], 0, "Days with data should be 0");
}

#[tokio::test]
async fn get_weekly_sleep_trends_includes_trend_data_when_available() {
    // Arrange
    let test_app = spawn_app().await;
    let client = Client::new();

    // Register, login and get user ID
    let username = format!("sleeptestuser{}", Uuid::new_v4());
    let password = "password123";
    
    client
        .post(&format!("{}/register_user", &test_app.address))
        .json(&json!({ 
            "username": username, 
            "password": password, 
            "email": format!("{}@example.com", username) 
        }))
        .send()
        .await
        .expect("Failed to register");

    let login_response = client
        .post(&format!("{}/login", &test_app.address))
        .json(&json!({ "username": username, "password": password }))
        .send()
        .await
        .expect("Failed to login");

    let token = login_response.json::<serde_json::Value>().await.unwrap()["token"].as_str().unwrap().to_string();

    // Get user ID
    let user = sqlx::query!(
        r#"SELECT id FROM users WHERE username = $1"#,
        username
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch user.");

    // Insert test sleep summaries for the past week
    let now = Utc::now();
    
    // Insert 3 days of sleep data
    for days_ago in 1..4 {
        let date = (now - Duration::days(days_ago)).date_naive().format("%Y-%m-%d").to_string();
        let summary_id = Uuid::new_v4();
        
        let sleep_score = 80 + days_ago as i32; // Slightly different scores
        
        let test_summary = json!({
            "id": summary_id.to_string(),
            "user_id": user.id.to_string(),
            "night_date": date,
            "sleep_metrics": {
                "sleep_efficiency": 90.0,
                "sleep_latency_seconds": 600,
                "awakenings": 2,
                "time_in_bed_seconds": 28800,
                "total_sleep_seconds": 25920,
                "light_sleep_seconds": 12960,
                "deep_sleep_seconds": 6480,
                "rem_sleep_seconds": 6480,
                "awake_seconds": 2880
            },
            "sleep_score": sleep_score,
            "overall_quality": "Good",
            "highlights": ["Good sleep efficiency"],
            "issues": [],
            "stage_distribution": {
                "awake_percentage": 10.0,
                "light_percentage": 50.0,
                "deep_percentage": 25.0,
                "rem_percentage": 25.0
            },
            "recommendations": ["Keep up your good sleep habits"],
            "created_at": now - Duration::days(days_ago)
        });

        sqlx::query!(
            r#"
            INSERT INTO processed_sleep_data
            (id, user_id, data_type, night_date, data, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            summary_id,
            user.id,
            "sleep_summary",
            date,
            test_summary,
            now - Duration::days(days_ago)
        )
        .execute(&test_app.db_pool)
        .await
        .expect("Failed to insert test sleep summary");
    }

    // Act - Get weekly sleep trends
    let get_response = client
        .get(&format!("{}/health/sleep_trends", &test_app.address))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to execute get request.");

    // Assert
    assert_eq!(200, get_response.status().as_u16(), "Should return 200 OK");
    
    let response_body = get_response.json::<serde_json::Value>().await
        .expect("Failed to parse response as JSON");
    
    assert_eq!(response_body["status"], "success", "Response status should be 'success'");
    assert_eq!(response_body["data"]["days_with_data"], 3, "Days with data should be 3");
    assert!(response_body["data"]["average_sleep_score"].is_number(), "Average sleep score should be present");
    assert!(response_body["data"]["average_sleep_time_hours"].is_number(), "Average sleep time should be present");
    
    let daily_summaries = response_body["data"]["daily_summaries"].as_array().unwrap();
    assert_eq!(daily_summaries.len(), 3, "Should include 3 daily summaries");
    
    // Check the format of a daily summary
    let first_summary = &daily_summaries[0];
    assert!(first_summary["date"].is_string(), "Date should be present");
    assert!(first_summary["sleep_score"].is_number(), "Sleep score should be present");
    assert!(first_summary["overall_quality"].is_string(), "Overall quality should be present");
}