use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use tracing::Instrument;

use crate::models::user::AuthRequest;
use crate::utils::password::hash_password;

pub async fn register_user(
    user_form: web::Json<AuthRequest>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let hashed_password = hash_password(&user_form.password);
    let user_id = uuid::Uuid::new_v4();

    let request_id = uuid::Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new user.",
        %request_id,
        user_name = %user_form.username
    );
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!(
        "Saving a new user to database."
    );
    match sqlx::query!(
        r#"
        INSERT INTO users (id, username, password, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        &user_form.username,
        &hashed_password,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool.get_ref())
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!(
                "Failed to execute query: {:?}", e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}