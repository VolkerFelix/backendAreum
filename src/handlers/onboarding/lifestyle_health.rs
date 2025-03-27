use actix_web::{web, HttpResponse};
use chrono::{NaiveTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, LifestyleHealthRequest, LifestyleHealthResponse
};
use crate::models::onboarding::{MedicalConditionType, LifestyleInfo};

pub async fn submit_lifestyle_health(
    data: web::Json<LifestyleHealthRequest>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
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

    // Create lifestyle info record first
    let lifestyle_id = Uuid::new_v4();
    let now = Utc::now();
    
    // Parse time strings to NaiveTime if provided
    let bedtime = data.bedtime.as_deref()
        .and_then(|t| NaiveTime::parse_from_str(t, "%H:%M").ok());
    let wake_time = data.wake_time.as_deref()
        .and_then(|t| NaiveTime::parse_from_str(t, "%H:%M").ok());
    
    let lifestyle_result = sqlx::query!(
        r#"
        INSERT INTO lifestyle_info (
            id, user_id, activity_level, bedtime, wake_time,
            is_smoker, alcohol_consumption, tracks_menstrual_cycle,
            menstrual_cycle_data, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
        ON CONFLICT (user_id) DO UPDATE
        SET
            activity_level = EXCLUDED.activity_level,
            bedtime = EXCLUDED.bedtime,
            wake_time = EXCLUDED.wake_time,
            is_smoker = EXCLUDED.is_smoker,
            alcohol_consumption = EXCLUDED.alcohol_consumption,
            tracks_menstrual_cycle = EXCLUDED.tracks_menstrual_cycle,
            menstrual_cycle_data = EXCLUDED.menstrual_cycle_data,
            updated_at = EXCLUDED.updated_at
        "#,
        lifestyle_id,
        user_id,
        data.activity_level,
        bedtime,
        wake_time,
        data.is_smoker,
        data.alcohol_consumption,
        data.tracks_menstrual_cycle,
        data.menstrual_cycle_data,
        now
    )
    .execute(pool.get_ref())
    .await;

    if let Err(e) = lifestyle_result {
        tracing::error!("Failed to insert lifestyle info: {:?}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: Some("Failed to save lifestyle information".to_string()),
            data: None::<()>,
        });
    }

    // Clear existing medical conditions
    let result = sqlx::query!(
        r#"
        DELETE FROM user_medical_conditions
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(pool.get_ref())
    .await;
    
    match result {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to clear existing medical conditions: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    }

    // Process medical conditions
    for condition_name in &data.medical_conditions {
        // Get condition_id for the condition name
        let result = sqlx::query_as!(
            MedicalConditionType,
            r#"
            SELECT id, name, description, created_at FROM medical_condition_types
            WHERE name = $1
            "#,
            condition_name
        )
        .fetch_optional(pool.get_ref())
        .await;

        let condition = match result {
            Ok(Some(record)) => record,
            Ok(None) => {
                tracing::warn!("Medical condition type not found: {}", condition_name);
                continue; // Skip this condition
            },
            Err(e) => {
                tracing::error!("Failed to query medical condition type: {:?}", e);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: "error".to_string(),
                    message: Some("Database error".to_string()),
                    data: None::<()>,
                });
            }
        };

        // Insert user medical condition
        let result = sqlx::query!(
            r#"
            INSERT INTO user_medical_conditions (
                id, user_id, condition_id, diagnosed_at, notes, created_at, updated_at
            )
            VALUES ($1, $2, $3, NULL, NULL, $4, $4)
            "#,
            Uuid::new_v4(),
            user_id,
            condition.id,
            now
        )
        .execute(pool.get_ref())
        .await;

        match result {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to insert user medical condition: {:?}", e);
            }
        }
    }

    // Update onboarding progress
    let result = sqlx::query!(
        r#"
        UPDATE onboarding_progress
        SET 
            lifestyle_health_completed = true,
            current_step = CASE WHEN current_step = 'lifestyle_health' THEN 'permissions_setup' ELSE current_step END,
            updated_at = $1
        WHERE user_id = $2
        "#,
        now,
        user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to update onboarding progress: {:?}", e);
        }
    }

    // Return success response
    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: Some("Lifestyle and health info saved successfully".to_string()),
        data: None::<()>,
    })
}

// Handler for getting lifestyle & health info
#[tracing::instrument(
    name = "Get lifestyle health info",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_lifestyle_health(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Parse user ID from claims
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
                data: None::<()>,
            });
        }
    };

    // Get lifestyle info
    let result = sqlx::query_as!(
        LifestyleInfo,
        r#"
        SELECT
            id, user_id, activity_level, bedtime, wake_time,
            is_smoker, alcohol_consumption, tracks_menstrual_cycle, 
            created_at, menstrual_cycle_data, updated_at
        FROM lifestyle_info
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    let lifestyle = match result {
        Ok(Some(record)) => record,
        Ok(None) => {
            return HttpResponse::NotFound().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Lifestyle info not found".to_string()),
                data: None::<()>,
            });
        },
        Err(e) => {
            tracing::error!("Failed to get lifestyle info: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    let result = sqlx::query_as!(
        MedicalConditionType,
        r#"
        SELECT mct.name, mct.id, mct.created_at, mct.description
        FROM user_medical_conditions umc
        JOIN medical_condition_types mct ON umc.condition_id = mct.id
        WHERE umc.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    let conditions = match result {
        Ok(records) => records,
        Err(e) => {
            tracing::error!("Failed to get medical conditions: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Format time fields if present
    let bedtime_string = lifestyle.bedtime.map(|t| t.format("%H:%M").to_string());
    let wake_time_string = lifestyle.wake_time.map(|t| t.format("%H:%M").to_string());

    // Prepare response
    let response = LifestyleHealthResponse {
        activity_level: lifestyle.activity_level,
        bedtime: bedtime_string,
        wake_time: wake_time_string,
        is_smoker: lifestyle.is_smoker,
        alcohol_consumption: lifestyle.alcohol_consumption,
        tracks_menstrual_cycle: lifestyle.tracks_menstrual_cycle,
        medical_conditions: conditions.into_iter().map(|c| c.name).collect(),
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}