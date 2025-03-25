use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{ApiResponse, OnboardingStatusResponse};
use super::common::get_or_create_onboarding_progress;

/// Handler for getting a user's onboarding status
#[tracing::instrument(
    name = "Get onboarding status",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_onboarding_status(
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

    // Get or create onboarding progress record
    let _ = match get_or_create_onboarding_progress(user_id, &pool).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Database error getting/creating onboarding progress: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Failed to retrieve onboarding status".to_string()),
                data: None::<()>,
            });
        }
    };

    // Get onboarding progress
    let progress = match sqlx::query!(
        r#"
        SELECT 
            basic_info_completed, lifestyle_health_completed, 
            permissions_setup_completed, personalization_completed, 
            onboarding_completed, current_step
        FROM onboarding_progress
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(&**pool)
    .await {
        Ok(record) => record,
        Err(e) => {
            tracing::error!("Database error retrieving onboarding progress: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Failed to retrieve onboarding status".to_string()),
                data: None::<()>,
            });
        }
    };

    // Prepare response
    let response = OnboardingStatusResponse {
        user_id: user_id.to_string(),
        basic_info_completed: progress.basic_info_completed,
        lifestyle_health_completed: progress.lifestyle_health_completed,
        permissions_setup_completed: progress.permissions_setup_completed,
        personalization_completed: progress.personalization_completed,
        onboarding_completed: progress.onboarding_completed,
        current_step: progress.current_step,
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}