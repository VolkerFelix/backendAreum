use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::health_data::{AccelerationDataUpload, HealthDataResponse, HealthDataRecord, DeviceInfo, HeartRateDataUpload};

#[tracing::instrument(
    name = "Uploading acceleration data",
    skip(data_upload, pool, claims),
    fields(
        user_id = %claims.sub,
        data_type = %data_upload.data_type
    )
)]
pub async fn upload_acceleration_data(
    data_upload: web::Json<AccelerationDataUpload>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    // Validate data type
    if data_upload.data_type != "acceleration" {
        return HttpResponse::BadRequest().json(HealthDataResponse {
            id: String::new(),
            status: "error".to_string(),
            message: Some("Invalid data type. Expected 'acceleration'".to_string()),
        });
    }

    // Convert the user ID from string to UUID
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid user ID format in JWT claims");
            return HttpResponse::InternalServerError().json(HealthDataResponse {
                id: String::new(),
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
            });
        }
    };

    // Generate a new ID for this health data record
    let health_data_id = Uuid::new_v4();
    
    // Convert samples to JSON for storage
    let samples_json = serde_json::to_value(&data_upload.samples)
        .unwrap_or_else(|_| json!([]));
    
    // Add metadata to the data if it exists
    let mut data_json = json!({
        "samples": samples_json
    });
    
    if let Some(metadata) = &data_upload.metadata {
        data_json["metadata"] = metadata.clone();
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO health_data (
            id, user_id, data_type, device_info, sampling_rate_hz, 
            start_time, data, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        health_data_id,
        user_id,
        data_upload.data_type,
        serde_json::to_value(&data_upload.device_info).unwrap(),
        data_upload.sampling_rate_hz,
        data_upload.start_time,
        samples_json,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(HealthDataResponse {
                id: health_data_id.to_string(),
                status: "success".to_string(),
                message: Some("Acceleration data uploaded successfully".to_string()),
            })
        }
        Err(e) => {
            tracing::error!("Failed to insert health data: {:?}", e);
            HttpResponse::InternalServerError().json(HealthDataResponse {
                id: String::new(),
                status: "error".to_string(),
                message: Some("Failed to store health data".to_string()),
            })
        }
    }
}

#[tracing::instrument(
    name = "Uploading heart rate data",
    skip(data_upload, pool, claims),
    fields(
        user_id = %claims.sub,
        data_type = %data_upload.data_type
    )
)]
pub async fn upload_heart_rate_data(
    data_upload: web::Json<HeartRateDataUpload>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    // Validate data type
    if data_upload.data_type != "heart_rate" {
        return HttpResponse::BadRequest().json(HealthDataResponse {
            id: String::new(),
            status: "error".to_string(),
            message: Some("Invalid data type. Expected 'heart_rate'".to_string()),
        });
    }

    // Convert the user ID from string to UUID
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid user ID format in JWT claims");
            return HttpResponse::InternalServerError().json(HealthDataResponse {
                id: String::new(),
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
            });
        }
    };

    // Generate a new ID for this health data record
    let health_data_id = Uuid::new_v4();
    
    // Convert samples to JSON for storage
    let samples_json = serde_json::to_value(&data_upload.samples)
        .unwrap_or_else(|_| json!([]));
    
    // Add metadata to the data if it exists
    let mut data_json = json!({
        "samples": samples_json
    });
    
    if let Some(metadata) = &data_upload.metadata {
        data_json["metadata"] = metadata.clone();
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO health_data (
            id, user_id, data_type, device_info, sampling_rate_hz, 
            start_time, data, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        health_data_id,
        user_id,
        data_upload.data_type,
        serde_json::to_value(&data_upload.device_info).unwrap(),
        data_upload.sampling_rate_hz,
        data_upload.start_time,
        samples_json,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(HealthDataResponse {
                id: health_data_id.to_string(),
                status: "success".to_string(),
                message: Some("Heart rate data uploaded successfully".to_string()),
            })
        }
        Err(e) => {
            tracing::error!("Failed to insert health data: {:?}", e);
            HttpResponse::InternalServerError().json(HealthDataResponse {
                id: String::new(),
                status: "error".to_string(),
                message: Some("Failed to store health data".to_string()),
            })
        }
    }
}

#[tracing::instrument(
    name = "Retrieving user acceleration data",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_user_acceleration_data(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    // Convert the user ID from string to UUID
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid user ID format in JWT claims");
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Invalid user ID format"
            }));
        }
    };

    // Retrieve the user's acceleration data from the database
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
            // Convert the database records to a response format
            let response = records.iter().map(|record| {
                let device_info: Result<DeviceInfo, _> = serde_json::from_value(record.device_info.clone());
                
                json!({
                    "id": record.id,
                    "data_type": record.data_type,
                    "device_info": device_info.unwrap_or_default(),
                    "sampling_rate_hz": record.sampling_rate_hz,
                    "start_time": record.start_time,
                    "data": record.data,
                    "created_at": record.created_at
                })
            }).collect::<Vec<_>>();

            HttpResponse::Ok().json(json!({
                "status": "success",
                "count": records.len(),
                "data": response
            }))
        },
        Err(e) => {
            tracing::error!("Failed to retrieve health data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve health data"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Retrieving user heart rate data",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_user_heart_rate_data(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    // Convert the user ID from string to UUID
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid user ID format in JWT claims");
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Invalid user ID format"
            }));
        }
    };

    // Retrieve the user's heart rate data from the database
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
            data as "data: serde_json::Value", 
            created_at
        FROM health_data 
        WHERE user_id = $1 AND data_type = 'heart_rate'
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(records) => {
            // Convert the database records to a response format
            let response = records.iter().map(|record| {
                let device_info: Result<DeviceInfo, _> = serde_json::from_value(record.device_info.clone());
                
                json!({
                    "id": record.id,
                    "data_type": record.data_type,
                    "device_info": device_info.unwrap_or_default(),
                    "sampling_rate_hz": record.sampling_rate_hz,
                    "start_time": record.start_time,
                    "data": record.data,
                    "created_at": record.created_at
                })
            }).collect::<Vec<_>>();

            HttpResponse::Ok().json(json!({
                "status": "success",
                "count": records.len(),
                "data": response
            }))
        },
        Err(e) => {
            tracing::error!("Failed to retrieve health data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve heart rate data"
            }))
        }
    }
}