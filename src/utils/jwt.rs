use jsonwebtoken::{encode, Header, EncodingKey};

use crate::models::user::User;
use chrono::{Utc, Duration};

const SECRET_KEY: &str = "mysecretkey";

pub fn create_jwt(user: &User) -> String {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = serde_json::json!({
        "sub": user.id.to_string(),
        "exp": expiration.timestamp(),
    });

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY.as_ref())).unwrap()
}