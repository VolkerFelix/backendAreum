use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::Claims;
use crate::models::sleep::{
    ProcessedSleepData, SleepSummary, SleepDateQuery, SleepRangeQuery
};

#[tracing::instrument(
    name = "Get processed sleep data by date",
    skip(pool, claims),
    fields(
        username = %claims.username,
    )
)]
pub async fn get_sleep_data_by_date(
    query: web::Query<SleepDateQuery>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("Sleep data retrieval handler called for date: {}", query.date);
    
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
    
    // Validate date format
    if let Err(_) = NaiveDate::parse_from_str(&query.date, "%Y-%m-%d") {
        tracing::warn!("Invalid date format provided: {}", query.date);
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid date format. Expected YYYY-MM-DD."
        }));
    }
    
    tracing::info!("Beginning database query for processed sleep data");
    
    // Query to get processed sleep data for the specific date
    match sqlx::query!(
        r#"
        SELECT 
            id, 
            user_id, 
            data_type, 
            data as "data: serde_json::Value", 
            created_at
        FROM processed_sleep_data 
        WHERE user_id = $1 AND night_date = $2 AND data_type = 'sleep_stages'
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        user_id,
        query.date
    )
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(record)) => {
            tracing::info!("Successfully retrieved sleep data for date: {}", query.date);
            
            // Parse the data into our ProcessedSleepData struct
            let processed_sleep_data = match serde_json::from_value::<ProcessedSleepData>(record.data.clone()) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to parse processed sleep data: {:?}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": "Failed to parse processed sleep data"
                    }));
                }
            };
            
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": processed_sleep_data
            }))
        },
        Ok(None) => {
            tracing::info!("No sleep data found for date: {}", query.date);
            HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "No sleep data found for the specified date"
            }))
        },
        Err(e) => {
            tracing::error!("Database error when fetching sleep data: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve sleep data"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Get sleep data for date range",
    skip(pool, claims),
    fields(
        username = %claims.username,
    )
)]
pub async fn get_sleep_data_range(
    query: web::Query<SleepRangeQuery>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("Sleep data range retrieval handler called for period: {} to {}", 
                 query.start_date, query.end_date);
    
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
    
    // Validate date formats
    let start_date = match NaiveDate::parse_from_str(&query.start_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            tracing::warn!("Invalid start date format: {}", query.start_date);
            return HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Invalid start date format. Expected YYYY-MM-DD."
            }));
        }
    };
    
    let end_date = match NaiveDate::parse_from_str(&query.end_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            tracing::warn!("Invalid end date format: {}", query.end_date);
            return HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Invalid end date format. Expected YYYY-MM-DD."
            }));
        }
    };
    
    // Check if date range is valid
    if end_date < start_date {
        tracing::warn!("Invalid date range: end date before start date");
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "End date must be equal to or after start date"
        }));
    }
    
    tracing::info!("Beginning database query for sleep data in range");
    
    // Query to get processed sleep data for the date range
    match sqlx::query!(
        r#"
        SELECT 
            id, 
            user_id, 
            night_date,
            data as "data: serde_json::Value", 
            created_at
        FROM processed_sleep_data 
        WHERE user_id = $1 
          AND night_date >= $2 
          AND night_date <= $3
          AND data_type = 'sleep_stages'
        ORDER BY night_date ASC
        "#,
        user_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(records) => {
            tracing::info!("Successfully retrieved {} sleep records for date range", records.len());
            
            if records.is_empty() {
                return HttpResponse::Ok().json(json!({
                    "status": "success",
                    "count": 0,
                    "data": []
                }));
            }
            
            // Parse each record into our ProcessedSleepData struct
            let mut processed_data = Vec::new();
            
            for record in records {
                match serde_json::from_value::<ProcessedSleepData>(record.data.clone()) {
                    Ok(data) => {
                        processed_data.push(data);
                    },
                    Err(e) => {
                        tracing::warn!("Failed to parse sleep data for {}: {:?}", record.night_date, e);
                        // Continue processing other records
                    }
                }
            }
            
            HttpResponse::Ok().json(json!({
                "status": "success",
                "count": processed_data.len(),
                "data": processed_data
            }))
        },
        Err(e) => {
            tracing::error!("Database error when fetching sleep data range: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve sleep data"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Get sleep summary by date",
    skip(pool, claims),
    fields(
        username = %claims.username,
    )
)]
pub async fn get_sleep_summary_by_date(
    query: web::Query<SleepDateQuery>,
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("Sleep summary retrieval handler called for date: {}", query.date);
    
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
    
    // Validate date format
    if let Err(_) = NaiveDate::parse_from_str(&query.date, "%Y-%m-%d") {
        tracing::warn!("Invalid date format provided: {}", query.date);
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid date format. Expected YYYY-MM-DD."
        }));
    }
    
    tracing::info!("Beginning database query for sleep summary");
    
    // Query to get sleep summary for the specific date
    match sqlx::query!(
        r#"
        SELECT 
            id, 
            user_id, 
            night_date,
            data as "data: serde_json::Value", 
            created_at
        FROM processed_sleep_data 
        WHERE user_id = $1 AND night_date = $2 AND data_type = 'sleep_summary'
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        user_id,
        query.date
    )
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(record)) => {
            tracing::info!("Successfully retrieved sleep summary for date: {}", query.date);
            
            // Parse the data into our SleepSummary struct
            let sleep_summary = match serde_json::from_value::<SleepSummary>(record.data.clone()) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to parse sleep summary data: {:?}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": "Failed to parse sleep summary data"
                    }));
                }
            };
            
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": sleep_summary
            }))
        },
        Ok(None) => {
            tracing::info!("No sleep summary found for date: {}", query.date);
            HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "No sleep summary found for the specified date"
            }))
        },
        Err(e) => {
            tracing::error!("Database error when fetching sleep summary: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve sleep summary"
            }))
        }
    }
}

#[tracing::instrument(
    name = "Get weekly sleep trends",
    skip(pool, claims),
    fields(
        username = %claims.username,
    )
)]
pub async fn get_weekly_sleep_trends(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>
) -> HttpResponse {
    tracing::info!("Weekly sleep trends handler called");
    
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
    
    // Calculate date for 7 days ago
    let today = Utc::now().date_naive();
    let seven_days_ago = today.pred_opt().and_then(|d| d.pred_opt()).and_then(|d| d.pred_opt())
        .and_then(|d| d.pred_opt()).and_then(|d| d.pred_opt()).and_then(|d| d.pred_opt())
        .and_then(|d| d.pred_opt()).unwrap_or(today);
    
    let today_str = today.format("%Y-%m-%d").to_string();
    let seven_days_ago_str = seven_days_ago.format("%Y-%m-%d").to_string();
    
    tracing::info!("Fetching sleep trends from {} to {}", seven_days_ago_str, today_str);
    
    // Query to get sleep trends for the past week
    match sqlx::query!(
        r#"
        SELECT 
            id, 
            user_id, 
            night_date,
            data as "data: serde_json::Value", 
            created_at
        FROM processed_sleep_data 
        WHERE user_id = $1 
          AND night_date >= $2 
          AND night_date <= $3
          AND data_type = 'sleep_summary'
        ORDER BY night_date ASC
        "#,
        user_id,
        seven_days_ago_str,
        today_str
    )
    .fetch_all(pool.get_ref())
    .await {
        Ok(records) => {
            tracing::info!("Successfully retrieved {} sleep summaries for weekly trends", records.len());
            
            if records.is_empty() {
                return HttpResponse::Ok().json(json!({
                    "status": "success",
                    "message": "No sleep data available for the past week",
                    "data": {
                        "days_with_data": 0,
                        "trends": []
                    }
                }));
            }
            
            // Calculate weekly trends
            let mut sleep_scores = Vec::new();
            let mut total_sleep_times = Vec::new();
            let mut deep_sleep_percentages = Vec::new();
            let mut daily_summaries = Vec::new();
            
            for record in records {
                match serde_json::from_value::<SleepSummary>(record.data.clone()) {
                    Ok(summary) => {
                        sleep_scores.push(summary.sleep_score);
                        
                        if let Some(total_sleep) = summary.sleep_metrics.total_sleep_seconds {
                            total_sleep_times.push(total_sleep as f64 / 3600.0); // Convert to hours
                        }
                        
                        if let (Some(deep_sleep), Some(total_sleep)) = (
                            summary.sleep_metrics.deep_sleep_seconds, 
                            summary.sleep_metrics.total_sleep_seconds
                        ) {
                            if total_sleep > 0 {
                                deep_sleep_percentages.push((deep_sleep as f64 / total_sleep as f64) * 100.0);
                            }
                        }
                        
                        daily_summaries.push(json!({
                            "date": record.night_date,
                            "sleep_score": summary.sleep_score,
                            "total_sleep_hours": summary.sleep_metrics.total_sleep_seconds.map(|s| s as f64 / 3600.0),
                            "deep_sleep_percentage": summary.stage_distribution.deep_percentage,
                            "overall_quality": summary.overall_quality
                        }));
                    },
                    Err(e) => {
                        tracing::warn!("Failed to parse sleep summary for {}: {:?}", record.night_date, e);
                        // Continue processing other records
                    }
                }
            }
            
            // Calculate averages
            let avg_sleep_score = if !sleep_scores.is_empty() {
                sleep_scores.iter().sum::<i32>() as f64 / sleep_scores.len() as f64
            } else {
                0.0
            };
            
            let avg_sleep_time = if !total_sleep_times.is_empty() {
                total_sleep_times.iter().sum::<f64>() / total_sleep_times.len() as f64
            } else {
                0.0
            };
            
            let avg_deep_sleep_percentage = if !deep_sleep_percentages.is_empty() {
                deep_sleep_percentages.iter().sum::<f64>() / deep_sleep_percentages.len() as f64
            } else {
                0.0
            };
            
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": {
                    "days_with_data": records.len(),
                    "average_sleep_score": avg_sleep_score,
                    "average_sleep_time_hours": avg_sleep_time,
                    "average_deep_sleep_percentage": avg_deep_sleep_percentage,
                    "daily_summaries": daily_summaries
                }
            }))
        },
        Err(e) => {
            tracing::error!("Database error when fetching weekly sleep trends: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve sleep trends"
            }))
        }
    }
}