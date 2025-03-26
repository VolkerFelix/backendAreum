use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, PersonalizationRequest, PersonalizationResponse
};

#[tracing::instrument(
    name = "Submit personalization",
    skip(data, pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn submit_personalization(
    data: web::Json<PersonalizationRequest>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Parse user ID from claims
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid user ID format");
            return HttpResponse::BadRequest().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
                data: None::<()>,   
            });
        }
    };

    // Insert personalization info into database
    let result = sqlx::query!(
        r#"
        INSERT INTO personalization_info (user_id, stress_triggers, work_type, timezone)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        data.stress_triggers.as_deref(),
        data.work_type,
        data.timezone
    )
    .execute(pool.get_ref())    
    .await;

    if let Err(e) = result {
        tracing::error!("Failed to insert personalization info: {:?}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to insert personalization info: {}", e)),
            data: None::<()>,
        });
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
            tracing::error!("Invalid user ID format");
            return HttpResponse::BadRequest().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
                data: None::<()>,
            });
        }
    };

    // Get personalization info
    let personalization = match sqlx::query_as!(
        PersonalizationResponse,
        r#"
        SELECT 
            stress_triggers,
            work_type,
            timezone
        FROM personalization_info
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await{
        Ok(Some(record)) => record,
        Ok(None) => {
            tracing::warn!("Personalization info not found");
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

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(personalization),
    })
}