use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, PersonalizationRequest, PersonalizationResponse
};
use super::status::get_or_create_onboarding_progress;

// Continuation of submit_personalization function
pub async fn submit_personalization(
    data: web::Json<PersonalizationRequest>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Previous code for parsing, transaction setup, and database operations goes here...

    // The commit transaction code was truncated in the previous artifact
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
        message: Some("Personalization info saved successfully. Onboarding completed.".to_string()),
        data: None::<()>,
    })
}

// Handler for getting personalization info
#[tracing::instrument(
    name = "Get personalization info",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_personalization(
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

    // Get personalization info
    let personalization = match sqlx::query!(
        r#"
        SELECT 
            stress_triggers, work_type, timezone
        FROM personalization_info
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
                message: Some("Personalization info not found".to_string()),
                data: None::<()>,
            });
        },
        Err(e) => {
            tracing::error!("Failed to get personalization info: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Prepare response
    let response = PersonalizationResponse {
        stress_triggers: personalization.stress_triggers.map(|triggers| triggers.to_vec()),
        work_type: personalization.work_type,
        timezone: personalization.timezone,
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}