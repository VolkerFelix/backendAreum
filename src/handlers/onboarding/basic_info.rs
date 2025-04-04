use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use num_traits::cast::ToPrimitive;

use crate::middleware::auth::Claims;
use crate::models::onboarding::{
    ApiResponse, BasicInfoRequest, BasicInfoResponse
};

#[tracing::instrument(
    name = "Submit basic info",
    skip(data, pool, claims),
    fields(
        user_id = %claims.sub
    )
)]
pub async fn submit_basic_info(
    data: web::Json<BasicInfoRequest>,
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
    
    // Update or insert user profile
    let now = Utc::now();
    
    // Parse date of birth if provided
    let date_of_birth = match &data.date_of_birth {
        Some(dob_str) => match NaiveDate::parse_from_str(dob_str, "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(e) => {
                tracing::error!("Invalid date format: {:?}", e);
                return HttpResponse::BadRequest().json(ApiResponse {
                    status: "error".to_string(),
                    message: Some("Invalid date format. Expected YYYY-MM-DD.".to_string()),
                    data: None::<()>,
                });
            }
        },
        None => None,
    };

    // Insert or update user profile
    match sqlx::query!(
        r#"
        INSERT INTO user_profiles (
            id, user_id, display_name, date_of_birth, biological_sex,
            height_cm, weight_kg, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
        ON CONFLICT (user_id) DO UPDATE
        SET
            display_name = EXCLUDED.display_name,
            date_of_birth = EXCLUDED.date_of_birth,
            biological_sex = EXCLUDED.biological_sex,
            height_cm = EXCLUDED.height_cm,
            weight_kg = EXCLUDED.weight_kg,
            updated_at = EXCLUDED.updated_at
        "#,
        Uuid::new_v4(),
        user_id,
        data.display_name,
        date_of_birth,
        data.biological_sex,
        data.height_cm,
        data.weight_kg,
        now
    )
    .execute(pool.get_ref())
    .await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to update user profile: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: Some("Database error".to_string()),
                data: None::<()>,
            });
        }
    }
    // Clear existing goals for the user
    match sqlx::query!(
        r#"
        DELETE FROM user_goals
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(pool.get_ref())
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
        .fetch_optional(pool.get_ref())
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
        .execute(pool.get_ref())
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
    .execute(pool.get_ref())
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
            tracing::error!("Invalid user ID format");
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
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(record)) => record,
        Ok(None) => {
            tracing::warn!("User profile not found");
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
    .fetch_all(pool.get_ref())
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
        height_cm: profile.height_cm.map(|h| h.to_f64().unwrap_or(0.0)),
        weight_kg: profile.weight_kg.map(|w| w.to_f64().unwrap_or(0.0)),
        goals,
    };

    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: None,
        data: Some(response),
    })
}