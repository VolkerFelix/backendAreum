use actix_web::{web, HttpResponse};
use chrono::{NaiveTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, LifestyleHealthRequest, LifestyleHealthResponse
};
use super::status::get_or_create_onboarding_progress;

// Continuation of the submit_lifestyle_health function
pub async fn submit_lifestyle_health(
    data: web::Json<LifestyleHealthRequest>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Previous code for parsing and validating input goes here...
    // (The code from the previous artifact continues here)
    
    // Continue with medical conditions processing
    // Clear existing medical conditions for the user
    match sqlx::query!(
        r#"
        DELETE FROM user_medical_conditions
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&mut *tx)
    .await {
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
        let condition = match sqlx::query!(
            r#"
            SELECT id FROM medical_condition_types
            WHERE name = $1
            "#,
            condition_name
        )
        .fetch_optional(&mut *tx)
        .await {
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
        match sqlx::query!(
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
        .execute(&mut *tx)
        .await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to insert user medical condition: {:?}", e);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: "error".to_string(),
                    message: Some("Database error".to_string()),
                    data: None::<()>,
                });
            }
        }
    }

    // Update onboarding progress
    match sqlx::query!(
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
    .execute(&mut *tx)
    .await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to update onboarding progress: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    }

    // Commit transaction
    match tx.commit().await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to commit transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
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
    let lifestyle = match sqlx::query!(
        r#"
        SELECT 
            activity_level, bedtime, wake_time,
            is_smoker, alcohol_consumption, tracks_menstrual_cycle
        FROM lifestyle_info
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(&**pool)
    .await {
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

    // Get medical conditions
    let conditions = match sqlx::query!(
        r#"
        SELECT mct.name
        FROM user_medical_conditions umc
        JOIN medical_condition_types mct ON umc.condition_id = mct.id
        WHERE umc.user_id = $1
        "#,
        user_id
    )
    .fetch_all(&**pool)
    .await {
        Ok(records) => records.into_iter().map(|r| r.name).collect::<Vec<String>>(),
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
        medical_conditions: conditions,
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}