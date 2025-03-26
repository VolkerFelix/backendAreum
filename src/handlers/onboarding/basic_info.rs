use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, BasicInfoRequest, BasicInfoResponse
};

// Continuation of the submit_basic_info handler for goal processing
pub async fn submit_basic_info(
    data: web::Json<BasicInfoRequest>,
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

    // Start a transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Previous code for parsing DOB and user profile handling goes here...

    // Clear existing goals for the user
    match sqlx::query!(
        r#"
        DELETE FROM user_goals
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(&mut *tx)
    .await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to clear existing user goals: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    }

    let now = Utc::now();

    // Process goals if provided
    for (index, goal_name) in data.goals.iter().enumerate() {
        // Get goal_type_id for the goal name
        let goal_type = match sqlx::query!(
            r#"
            SELECT id FROM goal_types
            WHERE name = $1
            "#,
            goal_name
        )
        .fetch_optional(&mut *tx)
        .await {
            Ok(Some(record)) => record,
            Ok(None) => {
                tracing::warn!("Goal type not found: {}", goal_name);
                continue; // Skip this goal
            },
            Err(e) => {
                tracing::error!("Failed to query goal type: {:?}", e);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    status: "error".to_string(),
                    message: Some("Database error".to_string()),
                    data: None::<()>,
                });
            }
        };

        // Insert user goal
        match sqlx::query!(
            r#"
            INSERT INTO user_goals (
                id, user_id, goal_type_id, priority, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $5)
            "#,
            Uuid::new_v4(),
            user_id,
            goal_type.id,
            (index + 1) as i32, // Priority based on order
            now
        )
        .execute(&mut *tx)
        .await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to insert user goal: {:?}", e);
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
            basic_info_completed = true,
            current_step = CASE WHEN current_step = 'basic_info' THEN 'lifestyle_health' ELSE current_step END,
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
        message: Some("Basic info saved successfully".to_string()),
        data: None::<()>,
    })
}

// Handler for getting basic info
#[tracing::instrument(
    name = "Get basic info",
    skip(pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn get_basic_info(
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

    // Get user profile
    let profile = match sqlx::query!(
        r#"
        SELECT 
            display_name, date_of_birth, biological_sex,
            height_cm, weight_kg
        FROM user_profiles
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
                message: Some("User profile not found".to_string()),
                data: None::<()>,
            });
        },
        Err(e) => {
            tracing::error!("Failed to get user profile: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Get user goals
    let goals = match sqlx::query!(
        r#"
        SELECT gt.name
        FROM user_goals ug
        JOIN goal_types gt ON ug.goal_type_id = gt.id
        WHERE ug.user_id = $1
        ORDER BY ug.priority
        "#,
        user_id
    )
    .fetch_all(&**pool)
    .await {
        Ok(records) => records.into_iter().map(|r| r.name).collect::<Vec<String>>(),
        Err(e) => {
            tracing::error!("Failed to get user goals: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    };

    // Format the date_of_birth if present
    let dob_string = profile.date_of_birth.map(|d| d.to_string());

    // Prepare response
    let response = BasicInfoResponse {
        display_name: profile.display_name,
        date_of_birth: dob_string,
        biological_sex: profile.biological_sex,
        height_cm: profile.height_cm,
        weight_kg: profile.weight_kg,
        goals,
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}