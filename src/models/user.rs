use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use secrecy::SecretString;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}
impl std::fmt::Display for RegistrationRequest{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.username, self.password, self.email)
    }
}