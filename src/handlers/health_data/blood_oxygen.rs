// Create a new file: src/handlers/health_data/blood_oxygen.rs
use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::Claims;
use crate::models::health_data::{BloodOxygenDataUpload, HealthDataResponse};

#[tracing::instrument(
    name = "Upload blood oxygen data",
    skip(data, pool, claims),
    fields(
        username = %claims.username,
        data_type = %data.data_type
    )
)]
pub async fn upload_blood_oxygen_data(
    data: web::Json<BloodOxygenDataUpload>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("Blood oxygen upload handler called with data_type: {}", data.data_type);
    
    // Validate data_type
    if data.data_type != "blood_oxygen" {
        tracing::warn!("Invalid data type received: {}", data.data_type);
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid data type. Expected 'blood_oxygen'."
        }));
    }
    
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => {
            tracing::info!("User ID parsed successfully: {}", id);
            id
        },
        Err(e) => {
            tracing::error!("Failed to parse user ID: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Invalid user ID"
            }));
        }
    };
    
    // Generate a unique ID for this data upload
    let id = Uuid::new_v4();
    tracing::info!("Generated new UUID for blood oxygen data: {}", id);
    
    // Calculate end time based on samples if not provided
    let end_time = if data.end_time != data.start_time {
        data.end_time
    } else if let Some(last_sample) = data.samples.last() {
        tracing::info!("Using last sample timestamp for end_time");
        last_sample.timestamp
    } else {
        tracing::info!("No samples found, using start_time for end_time");
        data.start_time
    };

    tracing::info!("Beginning database insert");
    
    // Create a safe way to serialize JSON data
    let device_info_json = match serde_json::to_value(&data.device_info) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Failed to serialize device_info: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to process device information"
            }));
        }
    };
    
    let data_json = match serde_json::to_value(json!({
        "samples": &data.samples,
        "metadata": &data.metadata
    })) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Failed to serialize data: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to process data"
            }));
        }
    };
    
    // Insert into database
    match sqlx::query!(
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
        Utc::now()
    )
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            tracing::info!("Successfully inserted blood oxygen data: {}", id);
            HttpResponse::Ok().json(HealthDataResponse {
                id: id.to_string(),
                status: "success".to_string(),
                message: Some("Blood oxygen data uploaded successfully".to_string()),
            })
        },
        Err(e) => {
            tracing::error!("Failed to insert blood oxygen data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to store blood oxygen data"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Get blood oxygen data",
    skip(pool, claims),
    fields(
        username = %claims.username,
    )
)]
pub async fn get_user_blood_oxygen_data(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("Blood oxygen get handler called");
    
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => {
            tracing::info!("User ID parsed successfully: {}", id);
            id
        },
        Err(e) => {
            tracing::error!("Failed to parse user ID: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Invalid user ID"
            }));
        }
    };
    
    tracing::info!("Beginning database query for blood oxygen data");
    
    // Get all blood oxygen data for this user
    match sqlx::query!(
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
        WHERE user_id = $1 AND data_type = 'blood_oxygen'
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(records) => {
            tracing::info!("Successfully retrieved {} blood oxygen records", records.len());
            
            // Transform records into serializable format
            let serializable_records: Vec<serde_json::Value> = records.iter().map(|record| {
                json!({
                    "id": record.id.to_string(),
                    "user_id": record.user_id.to_string(),
                    "data_type": &record.data_type,
                    "device_info": &record.device_info,
                    "sampling_rate_hz": record.sampling_rate_hz,
                    "start_time": record.start_time,
                    "end_time": record.end_time,
                    "data": &record.data,
                    "created_at": record.created_at
                })
            }).collect();
            
            HttpResponse::Ok().json(json!({
                "status": "success",
                "count": records.len(),
                "data": serializable_records
            }))
        },
        Err(e) => {
            tracing::error!("Failed to fetch blood oxygen data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve blood oxygen data"
            }))
        }
    }
}