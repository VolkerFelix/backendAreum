// src/handlers/health_data/acceleration.rs
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::Claims;
use crate::models::health_data::{AccelerationDataUpload, HealthDataRecord, HealthDataResponse};

#[tracing::instrument(
    name = "Uploading acceleration data",
    skip(data, pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn upload_acceleration_data(
    data: web::Json<AccelerationDataUpload>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    // Validate data_type
    if data.data_type != "acceleration" {
        tracing::warn!("Invalid data type: {}", data.data_type);
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid data type. Expected 'acceleration'."
        }));
    }

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse user_id as UUID: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Invalid user ID format"
            }));
        }
    };
    
    // Generate a unique ID for this data upload
    let id = Uuid::new_v4();
    
    // Calculate end time based on samples
    let end_time = if let Some(last_sample) = data.samples.last() {
        last_sample.timestamp
    } else {
        data.start_time
    };

    // Create JSON data payload
    let data_json = match serde_json::to_value(json!({
        "samples": &data.samples,
        "metadata": &data.metadata
    })) {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("Failed to serialize data to JSON: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to process acceleration data"
            }));
        }
    };

    // Create device info JSON
    let device_info_json = match serde_json::to_value(&data.device_info) {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("Failed to serialize device info to JSON: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to process device information"
            }));
        }
    };

    // Insert into database
    let result = sqlx::query!(
        r#"
        INSERT INTO health_data (
            id, user_id, data_type, device_info, sampling_rate_hz, 
            start_time, end_time, data, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        id,
        user_id,
        data.data_type,
        device_info_json,
        data.sampling_rate_hz,
        data.start_time,
        end_time,
        data_json,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(HealthDataResponse {
                id: id.to_string(),
                status: "success".to_string(),
                message: Some("Acceleration data uploaded successfully".to_string()),
            })
        },
        Err(e) => {
            tracing::error!("Failed to insert acceleration data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to store acceleration data"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Getting user acceleration data",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_user_acceleration_data(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to parse user_id as UUID: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Invalid user ID format"
            }));
        }
    };
    
    // Get all acceleration data for this user
    let result = sqlx::query_as!(
        HealthDataRecord,
        r#"
        SELECT 
            id, 
            user_id,
            data_type,
            device_info as "device_info: serde_json::Value", 
            sampling_rate_hz, 
            start_time,
            end_time,
            data as "data: serde_json::Value",
            created_at
        FROM health_data 
        WHERE user_id = $1 AND data_type = 'acceleration'
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(records) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "count": records.len(),
                "data": records
            }))
        },
        Err(e) => {
            tracing::error!("Failed to fetch acceleration data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve acceleration data"
            }))
        }
    }
}