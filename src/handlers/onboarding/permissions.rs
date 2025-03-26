use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, PermissionsSetupRequest, PermissionsSetupResponse, 
    ThirdPartyConnectionResponse, PermissionsSettings
};

#[tracing::instrument(
    name = "Get permissions setup",
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
            tracing::error!("Invalid user ID format");
            return HttpResponse::BadRequest().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Invalid user ID format".to_string()),
                data: None::<()>,
            });
        }
    };

    // Get permissions settings from database
    let result = sqlx::query_as!(
        PermissionsSettings,
        r#"
        SELECT
            id,
            user_id,
            heart_rate_enabled,
            temperature_enabled,
            spo2_enabled,
            accelerometer_enabled,
            notifications_enabled,
            background_usage_enabled,
            created_at,
            updated_at
        FROM permissions_settings
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool.get_ref())
    .await;

    let permissions = match result {
        Ok(permissions) => permissions,
        Err(e) => {
            tracing::error!("Failed to fetch permissions settings: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to fetch permissions settings: {}", e)),
                data: None,
            });
        }
    };

    // Get third-party connections from database
    let result = sqlx::query_as!(
        ThirdPartyConnectionResponse,
        r#"
        SELECT
            id,
            user_id,
            connection_type_id,
            access_token,
            refresh_token,
            token_expires_at,
            connection_status, 
            last_sync_at,
            connection_data,
            created_at,
            updated_at
        FROM user_third_party_connections
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    let third_party_connections = match result {
        Ok(connections) => connections,
        Err(e) => {
            tracing::error!("Failed to fetch third-party connections: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to fetch third-party connections: {}", e)),
                data: None,
            });
        }
    };

    // Return the permissions and third-party connections
    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: Some("Permissions setup fetched successfully".to_string()),
        data: Some(PermissionsSetupResponse {
            heart_rate_enabled: permissions.heart_rate_enabled,
            temperature_enabled: permissions.temperature_enabled,
            spo2_enabled: permissions.spo2_enabled,
            accelerometer_enabled: permissions.accelerometer_enabled,
            notifications_enabled: permissions.notifications_enabled,
            background_usage_enabled: permissions.background_usage_enabled,
            third_party_connections,
        }),
    })
}

#[tracing::instrument(
    name = "Submit permissions setup",
    skip(data,pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn submit_permissions_setup(
    data: web::Json<PermissionsSetupRequest>,
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
    let now = Utc::now();

    // Update or insert permissions settings
    let result = sqlx::query!(
        r#"
        INSERT INTO permissions_settings (
            id, user_id, heart_rate_enabled, temperature_enabled,
            spo2_enabled, accelerometer_enabled, notifications_enabled,
            background_usage_enabled, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9)
        ON CONFLICT (user_id) DO UPDATE
        SET
            heart_rate_enabled = EXCLUDED.heart_rate_enabled,
            temperature_enabled = EXCLUDED.temperature_enabled,
            spo2_enabled = EXCLUDED.spo2_enabled,
            accelerometer_enabled = EXCLUDED.accelerometer_enabled,
            notifications_enabled = EXCLUDED.notifications_enabled,
            background_usage_enabled = EXCLUDED.background_usage_enabled,
            updated_at = EXCLUDED.updated_at
        "#,
        Uuid::new_v4(),
        user_id,
        data.heart_rate_enabled,
        data.temperature_enabled,
        data.spo2_enabled,
        data.accelerometer_enabled,
        data.notifications_enabled,
        data.background_usage_enabled,
        now
    )
    .execute(pool.get_ref())
    .await;

    if let Err(e) = result {
        tracing::error!("Failed to update permissions settings: {:?}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to update permissions settings: {}", e)),
            data: None,
        });
    }

    // Handle third-party connections
    for connection in &data.third_party_connections {
        // Get connection type ID
        let connection_type_id = match sqlx::query!(
            "SELECT id FROM third_party_connection_types WHERE name = $1",
            connection.connection_type
        )
        .fetch_optional(pool.get_ref())
        .await
        {
            Ok(Some(record)) => record.id,
            Ok(None) => {
                tracing::error!("Invalid connection type: {}", connection.connection_type);
                return HttpResponse::BadRequest().json(ApiResponse {
                    status: "error".to_string(),
                    message: Some(format!("Invalid connection type: {}", connection.connection_type)),
                    data: None,
                });
            }
            Err(e) => {
                tracing::error!("Failed to get connection type: {:?}", e);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: "error".to_string(),
                    message: Some(format!("Failed to get connection type: {}", e)),
                    data: None,
                });
            }
        };

        // Update or insert third-party connection
        let result = sqlx::query!(
            r#"
            INSERT INTO user_third_party_connections (
                id, user_id, connection_type_id, access_token,
                refresh_token, token_expires_at, connection_status,
                last_sync_at, connection_data, created_at, updated_at
            )
            VALUES ($1, $2, $3, NULL, NULL, NULL, 'pending', NULL, $4, $5, $5)
            ON CONFLICT (user_id, connection_type_id) DO UPDATE
            SET
                connection_status = 'pending',
                connection_data = EXCLUDED.connection_data,
                updated_at = EXCLUDED.updated_at
            "#,
            Uuid::new_v4(),
            user_id,
            connection_type_id,
            connection.connection_data,
            now
        )
        .execute(pool.get_ref())
        .await;

        if let Err(e) = result {
            tracing::error!("Failed to update third-party connection: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to update third-party connection: {}", e)),
                data: None,
            });
        }
    }

    // Update onboarding progress
    let result = sqlx::query!(
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
    .execute(pool.get_ref())
    .await;

    if let Err(e) = result {
        tracing::error!("Failed to update onboarding progress: {:?}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            status: "error".to_string(),
            message: Some(format!("Failed to update onboarding progress: {}", e)),
            data: None,
        });
    }

    // Get the updated permissions settings and third-party connections for the response
    let permissions = match sqlx::query_as!(
        PermissionsSettings,
        r#"
        SELECT 
            id,
            user_id,
            heart_rate_enabled,
            temperature_enabled,
            spo2_enabled,
            accelerometer_enabled,
            notifications_enabled,
            background_usage_enabled, 
            created_at,
            updated_at
        FROM permissions_settings
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(record) => record,
        Err(e) => {
            tracing::error!("Failed to fetch updated permissions: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to fetch updated permissions: {}", e)),
                data: None,
            });
        }
    };

    let third_party_connections = match sqlx::query_as!(
        ThirdPartyConnectionResponse,
        r#"
        SELECT
            utpc.id,
            utpc.user_id,
            utpc.connection_type_id,
            utpc.access_token,
            utpc.refresh_token,
            utpc.token_expires_at,
            utpc.connection_status,
            utpc.last_sync_at,
            utpc.connection_data,
            utpc.created_at, utpc.updated_at
        FROM user_third_party_connections utpc
        JOIN third_party_connection_types tpct ON utpc.connection_type_id = tpct.id
        WHERE utpc.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(records) => records,
        Err(e) => {
            tracing::error!("Failed to fetch third-party connections: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to fetch third-party connections: {}", e)),
                data: None,
            });
        }
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: Some("Permissions setup completed successfully".to_string()),
        data: Some(PermissionsSetupResponse {
            heart_rate_enabled: permissions.heart_rate_enabled,
            temperature_enabled: permissions.temperature_enabled,
            spo2_enabled: permissions.spo2_enabled,
            accelerometer_enabled: permissions.accelerometer_enabled,
            notifications_enabled: permissions.notifications_enabled,
            background_usage_enabled: permissions.background_usage_enabled,
            third_party_connections,
        }),
    })
}