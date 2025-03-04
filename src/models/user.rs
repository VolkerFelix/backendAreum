use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use secrecy::SecretString;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(serialize_with = "serialize_secret_string", deserialize_with = "deserialize_secret_string")]
    pub password_hash: SecretString,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    #[serde(serialize_with = "serialize_secret_string", deserialize_with = "deserialize_secret_string")]
    pub password: SecretString,
    pub email: String,
}
impl std::fmt::Display for RegistrationRequest{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Username: {}, Email: {}", self.username, self.email)
    }
}

fn serialize_secret_string<S>(_: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str("[REDACTED]")
}

fn deserialize_secret_string<'de, D>(deserializer: D) -> Result<SecretString, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(SecretString::new(s.into_boxed_str()))
}