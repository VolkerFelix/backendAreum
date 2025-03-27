use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

/// Gets or creates an onboarding progress record for a user
/// 
/// This function checks if a user already has an onboarding_progress record
/// and creates one if it doesn't exist.
#[tracing::instrument(
    name = "Get or create onboarding progress",
    skip(user_id, pool)
)]
pub async fn get_or_create_onboarding_progress(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Uuid, sqlx::Error> {
    // Try to get existing progress
    let progress = sqlx::query!(
        r#"
        SELECT id FROM onboarding_progress
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = progress {
        return Ok(record.id);
    }

    // Create a new progress record
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO onboarding_progress (
            id, user_id, basic_info_completed, lifestyle_health_completed,
            permissions_setup_completed, personalization_completed, 
            onboarding_completed, current_step, created_at, updated_at
        )
        VALUES ($1, $2, false, false, false, false, false, 'basic_info', $3, $3)
        "#,
        id,
        user_id,
        Utc::now(),
    )
    .execute(pool)
    .await?;

    Ok(id)
}