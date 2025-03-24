// src/handlers/health_data/gps_location.rs
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::Claims;
use crate::models::sensor_data::{GpsLocationDataUpload, HealthDataResponse, HealthDataTimeQuery};

#[tracing::instrument(
    name = "Upload GPS location data",
    skip(data, pool, claims),
    fields(
        username = %claims.username,
        data_type = %data.data_type
    )
)]
pub async fn upload_gps_location_data(
    data: web::Json<GpsLocationDataUpload>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("GPS location upload handler called with data_type: {}", data.data_type);
    
    // Validate data_type
    if data.data_type != "gps_location" {
        tracing::warn!("Invalid data type received: {}", data.data_type);
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid data type. Expected 'gps_location'."
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
    tracing::info!("Generated new UUID for GPS location data: {}", id);
    
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
            tracing::info!("Successfully inserted GPS location data: {}", id);
            HttpResponse::Ok().json(HealthDataResponse {
                id: id.to_string(),
                status: "success".to_string(),
                message: Some("GPS location data uploaded successfully".to_string()),
            })
        },
        Err(e) => {
            tracing::error!("Failed to insert GPS location data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to store GPS location data"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Get GPS location data",
    skip(pool, claims),
    fields(
        username = %claims.username,
    )
)]
pub async fn get_user_gps_location_data(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("GPS location get handler called");
    
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
    
    tracing::info!("Beginning database query for GPS location data");
    
    // Get all GPS location data for this user
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
        WHERE user_id = $1 AND data_type = 'gps_location'
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(records) => {
            tracing::info!("Successfully retrieved {} GPS location records", records.len());
            
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
            tracing::error!("Failed to fetch GPS location data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve GPS location data"
            }))
        }
    }
}

// New function to get health data from a specific time period with GPS locations
#[tracing::instrument(
    name = "Get health data with GPS locations",
    skip(pool, claims, params),
    fields(
        username = %claims.username,
        data_type = %params.data_type
    )
)]
pub async fn get_health_data_with_gps(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
    web::Query(params): web::Query<HealthDataTimeQuery>
) -> HttpResponse {
    tracing::info!("Health data with GPS handler called for data_type: {}", params.data_type);
    
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
    
    // First, get the requested health data
    let health_data_result = sqlx::query!(
        r#"
        SELECT 
            id, 
            data_type, 
            device_info as "device_info: serde_json::Value", 
            sampling_rate_hz, 
            start_time,
            end_time, 
            data as "data: serde_json::Value", 
            created_at
        FROM health_data 
        WHERE user_id = $1 
          AND data_type = $2 
          AND start_time >= $3 
          AND start_time <= $4
        ORDER BY start_time ASC
        "#,
        user_id,
        params.data_type,
        params.start_time,
        params.end_time
    )
    .fetch_all(pool.get_ref())
    .await;
    
    if let Err(e) = health_data_result {
        tracing::error!("Failed to fetch health data: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to retrieve health data"
        }));
    }
    
    let health_data = health_data_result.unwrap();
    
    if health_data.is_empty() {
        return HttpResponse::Ok().json(json!({
            "status": "success",
            "count": 0,
            "data": []
        }));
    }
    
    // Now, get GPS data for the same time period
    let gps_data_result = sqlx::query!(
        r#"
        SELECT 
            id, 
            data_type, 
            start_time,
            end_time,
            data as "data: serde_json::Value", 
            created_at
        FROM health_data 
        WHERE user_id = $1 
          AND data_type = 'gps_location' 
          AND (
            (start_time <= $2 AND end_time >= $2) OR
            (start_time <= $3 AND end_time >= $3) OR
            (start_time >= $2 AND end_time <= $3)
          )
        ORDER BY start_time ASC
        "#,
        user_id,
        params.start_time,
        params.end_time
    )
    .fetch_all(pool.get_ref())
    .await;
    
    if let Err(e) = gps_data_result {
        tracing::error!("Failed to fetch GPS data: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to retrieve GPS data"
        }));
    }
    
    let gps_data = gps_data_result.unwrap();
    
    // Combine the data
    let mut result = Vec::new();
    
    for health_record in health_data {
        // Find GPS locations for this health data record's timeframe
        let mut relevant_gps_samples = Vec::new();
        
        for gps_record in &gps_data {
            // Check if there's any overlap in the timeframes
            let timeframes_overlap = (gps_record.start_time <= health_record.start_time && gps_record.end_time >= health_record.start_time) ||
                                     (gps_record.start_time <= health_record.end_time && gps_record.end_time >= health_record.end_time) ||
                                     (gps_record.start_time >= health_record.start_time && gps_record.end_time <= health_record.end_time);
            
            if timeframes_overlap {
                if let Some(samples) = gps_record.data.get("samples") {
                    if let Some(samples_array) = samples.as_array() {
                        for sample in samples_array {
                            if let Some(timestamp_str) = sample.get("timestamp").and_then(|t| t.as_str()) {
                                if let Ok(timestamp) = timestamp_str.parse::<DateTime<Utc>>() {
                                    // Only include GPS samples that fall within the health data timeframe
                                    if timestamp >= health_record.start_time && timestamp <= health_record.end_time {
                                        relevant_gps_samples.push(sample.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        result.push(json!({
            "id": health_record.id.to_string(),
            "data_type": health_record.data_type,
            "device_info": health_record.device_info,
            "sampling_rate_hz": health_record.sampling_rate_hz,
            "start_time": health_record.start_time,
            "end_time": health_record.end_time,
            "data": health_record.data,
            "created_at": health_record.created_at,
            "gps_data": relevant_gps_samples
        }));
    }
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "count": result.len(),
        "data": result
    }))
}