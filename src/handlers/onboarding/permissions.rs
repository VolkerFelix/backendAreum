use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, PermissionsSetupRequest, PermissionsSetupResponse, 
    ThirdPartyConnectionResponse
};
use super::status::get_or_create_onboarding_progress;

// Continuation of submit_permissions_setup function
pub async fn submit_permissions_setup(
    data: web::Json<PermissionsSetupRequest>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Previous code for user ID parsing and transaction setup goes here...

    // Continue with third-party connection processing
    match existing_connection {
        Ok(Some(existing)) => {
            // Update existing connection
            match sqlx::query!(
                r#"
                UPDATE user_third_party_connections
                SET 
                    connection_status = 'pending',
                    connection_data = $1,
                    updated_at = $2
                WHERE id = $3
                "#,
                connection.connection_data,
                now,
                existing.id
            )
            .execute(&mut *tx)
            .await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to update third-party connection: {:?}", e);
                    return HttpResponse::InternalServerError().json(ApiResponse {
                        status: "error".to_string(),
                        message: Some("Database error".to_string()),
                        data: None::<()>,
                    });
                }
            }
        },
        Ok(None) => {
            // Insert new connection
            match sqlx::query!(
                r#"
                INSERT INTO user_third_party_connections (
                    id, user_id, connection_type_id, access_token,
                    refresh_token, token_expires_at, connection_status,
                    last_sync_at, connection_data, created_at, updated_at
                )
                VALUES ($1, $2, $3, NULL, NULL, NULL, 'pending', NULL, $4, $5, $5)
                "#,
                Uuid::new_v4(),
                user_id,
                connection_type.id,
                connection.connection_data,
                now
            )
            .execute(&mut *tx)
            .await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to insert third-party connection: {:?}", e);
                    return HttpResponse::InternalServerError().json(ApiResponse {
                        status: "error".to_string(),
                        message: Some("Database error".to_string()),
                        data: None::<()>,
                    });
                }
            }
        },
        Err(e) => {
            tracing::error!("Failed to check for existing connection: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    }

    // Update onboarding progress
    match sqlx::query!(
        r#"
        UPDATE onboarding_progress
        SET 
            permissions_setup_completed = true,
            current_step = CASE WHEN current_step = 'permissions_setup' THEN 'personalization' ELSE current_step END,
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
        message: Some("Permissions and setup saved successfully".to_string()),
        data: None::<()>,
    })
}

// Handler for getting permissions & setup info
#[tracing::instrument(
    name = "Get permissions setup info",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_permissions_setup(
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

    // Get permissions settings
    let permissions = match sqlx::query!(
        r#"
        SELECT 
            heart_rate_enabled, temperature_enabled, spo2_enabled,
            accelerometer_enabled, notifications_enabled, background_usage_enabled
        FROM permissions_settings
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
                message: Some("Permissions settings not found".to_string()),
                data: None::<()>,
            });
        },
        Err(e) => {
            tracing::error!("Failed to get permissions settings: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Get third-party connections
    let connections = match sqlx::query!(
        r#"
        SELECT 
            tpct.name as connection_type,
            utpc.connection_status,
            utpc.last_sync_at
        FROM user_third_party_connections utpc
        JOIN third_party_connection_types tpct ON utpc.connection_type_id = tpct.id
        WHERE utpc.user_id = $1
        "#,
        user_id
    )
    .fetch_all(&**pool)
    .await {
        Ok(records) => {
            records.into_iter().map(|r| {
                ThirdPartyConnectionResponse {
                    connection_type: r.connection_type,
                    connection_status: r.connection_status,
                    last_sync_at: r.last_sync_at.map(|dt| dt.to_rfc3339()),
                }
            }).collect::<Vec<ThirdPartyConnectionResponse>>()
        },
        Err(e) => {
            tracing::error!("Failed to get third-party connections: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Prepare response
    let response = PermissionsSetupResponse {
        heart_rate_enabled: permissions.heart_rate_enabled,
        temperature_enabled: permissions.temperature_enabled,
        spo2_enabled: permissions.spo2_enabled,
        accelerometer_enabled: permissions.accelerometer_enabled,
        notifications_enabled: permissions.notifications_enabled,
        background_usage_enabled: permissions.background_usage_enabled,
        third_party_connections: connections,
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}