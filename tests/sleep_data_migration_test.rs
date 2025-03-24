// tests/migration_test.rs
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

mod common;
use common::utils::spawn_app;

#[tokio::test]
async fn processed_sleep_data_table_exists_and_works() {
    // Arrange - Create test app which runs migrations
    let test_app = spawn_app().await;
    
    // Act & Assert - Verify we can insert and query data in the processed_sleep_data table
    
    // Generate test data
    let test_id = Uuid::new_v4();
    let user_id = Uuid::new_v4(); // Fake user ID for testing
    let data_type = "sleep_test";
    let night_date = "2025-03-24";
    let created_at = Utc::now();
    
    let test_data = json!({
        "test": true,
        "value": "migration test"
    });
    
    // Insert test data
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO processed_sleep_data
        (id, user_id, data_type, night_date, data, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        test_id,
        user_id,
        data_type,
        night_date,
        test_data,
        created_at
    )
    .execute(&test_app.db_pool)
    .await;
    
    // Verify insert succeeded
    assert!(insert_result.is_ok(), "Should be able to insert into processed_sleep_data table");
    
    // Query the inserted data
    let query_result = sqlx::query!(
        r#"
        SELECT id, data_type, night_date, data as "data: serde_json::Value"
        FROM processed_sleep_data
        WHERE id = $1
        "#,
        test_id
    )
    .fetch_one(&test_app.db_pool)
    .await;
    
    // Verify query succeeded
    assert!(query_result.is_ok(), "Should be able to query from processed_sleep_data table");
    
    let record = query_result.unwrap();
    assert_eq!(record.id, test_id);
    assert_eq!(record.data_type, data_type);
    assert_eq!(record.night_date, night_date);
    assert_eq!(record.data["test"], json!(true));
    assert_eq!(record.data["value"], json!("migration test"));
    
    // Test the indexes
    let explain_result = sqlx::query!(
        r#"
        EXPLAIN ANALYZE
        SELECT * FROM processed_sleep_data 
        WHERE user_id = $1 AND night_date = $2
        "#,
        user_id,
        night_date
    )
    .fetch_all(&test_app.db_pool)
    .await;
    
    assert!(explain_result.is_ok(), "Should be able to run EXPLAIN ANALYZE");
    
    // Convert explain result to string to check if index is used
    let explain_output = explain_result.unwrap()
        .iter()
        .map(|row| row.plan.clone().unwrap_or_default())
        .collect::<Vec<String>>()
        .join("\n");
    
    // Check if the query uses the index we created
    // Note: This is somewhat fragile and depends on the PostgreSQL version and query planner
    assert!(
        explain_output.contains("Index") || explain_output.contains("idx_processed_sleep_data"),
        "Query should use an index for user_id and night_date: {}",
        explain_output
    );
}

#[tokio::test]
async fn processed_sleep_data_enforces_foreign_key_constraint() {
    // Arrange - Create test app which runs migrations
    let test_app = spawn_app().await;
    
    // Generate test data with a non-existent user ID
    let test_id = Uuid::new_v4();
    let fake_user_id = Uuid::new_v4(); // Non-existent user ID
    let data_type = "sleep_test";
    let night_date = "2025-03-24";
    let created_at = Utc::now();
    
    let test_data = json!({
        "test": true,
        "value": "constraint test"
    });
    
    // Act - Try to insert with non-existent user ID
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO processed_sleep_data
        (id, user_id, data_type, night_date, data, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        test_id,
        fake_user_id,
        data_type,
        night_date,
        test_data,
        created_at
    )
    .execute(&test_app.db_pool)
    .await;
    
    // Assert - Verify foreign key constraint prevents insertion
    assert!(insert_result.is_err(), "Insert should fail due to foreign key constraint");
    
    // Check error message contains "foreign key constraint"
    let error_message = insert_result.unwrap_err().to_string().to_lowercase();
    assert!(
        error_message.contains("foreign key") || error_message.contains("violates"),
        "Error should mention foreign key constraint violation: {}",
        error_message
    );
}

#[tokio::test]
async fn processed_sleep_data_table_preserves_jsonb_data() {
    // Arrange - Create test app which runs migrations
    let test_app = spawn_app().await;
    
    // Create a real user
    let username = format!("testuser{}", Uuid::new_v4());
    let email = format!("{}@example.com", username);
    
    let user_id = sqlx::query!(
        r#"
        INSERT INTO users (id, username, password_hash, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        Uuid::new_v4(),
        username,
        "fake_password_hash",
        email,
        Utc::now(),
        Utc::now()
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to create test user")
    .id;
    
    // Create complex nested JSON data
    let test_id = Uuid::new_v4();
    let night_date = "2025-03-24";
    let created_at = Utc::now();
    
    let complex_json = json!({
        "sleep_score": 85,
        "metrics": {
            "sleep_efficiency": 92.5,
            "sleep_latency_seconds": 600,
            "time_in_bed_seconds": 29700,
            "total_sleep_seconds": 27500
        },
        "stage_distribution": {
            "awake": 7.5,
            "light": 48.5,
            "deep": 24.2,
            "rem": 19.8
        },
        "samples": [
            {
                "timestamp": "2025-03-24T22:30:00Z",
                "stage": "awake",
                "confidence": 0.95,
                "duration_seconds": 600
            },
            {
                "timestamp": "2025-03-24T22:40:00Z",
                "stage": "light",
                "confidence": 0.92,
                "duration_seconds": 1800
            }
        ],
        "recommendations": [
            "Consider relaxation techniques before bedtime",
            "Maintain your consistent sleep schedule"
        ]
    });
    
    // Act - Insert complex JSON data
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO processed_sleep_data
        (id, user_id, data_type, night_date, data, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        test_id,
        user_id,
        "sleep_summary",
        night_date,
        complex_json,
        created_at
    )
    .execute(&test_app.db_pool)
    .await;
    
    assert!(insert_result.is_ok(), "Should be able to insert complex JSON data");
    
    // Query the inserted data
    let record = sqlx::query!(
        r#"
        SELECT data as "data: serde_json::Value"
        FROM processed_sleep_data
        WHERE id = $1
        "#,
        test_id
    )
    .fetch_one(&test_app.db_pool)
    .await
    .expect("Failed to fetch inserted data");
    
    // Verify all parts of the complex JSON are preserved
    assert_eq!(record.data["sleep_score"], 85);
    assert_eq!(record.data["metrics"]["sleep_efficiency"], 92.5);
    assert_eq!(record.data["stage_distribution"]["deep"], 24.2);
    assert!(record.data["samples"].is_array());
    assert_eq!(record.data["samples"][0]["stage"], "awake");
    assert!(record.data["recommendations"].is_array());
    assert_eq!(record.data["recommendations"][0], "Consider relaxation techniques before bedtime");
}