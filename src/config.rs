use dotenv::dotenv;
use std::env::{self, VarError};
use secrecy::{SecretBox, ExposeSecret};

#[derive(serde::Deserialize, Debug)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings{
    pub user: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password.expose_secret(), self.host, self.port, self.db_name
        )))
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password.expose_secret(), self.host, self.port
        )
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings{
    pub user: String,
    pub password: String,
    pub port: u16,
}

pub fn get_config() -> Result<Settings, VarError> {
    dotenv().ok();

    let db_settings = DatabaseSettings {
        user: env::var("POSTGRES_USER")?,
        password: SecretBox::new(Box::new(env::var("POSTGRES_PASSWORD")?)),
        port: env::var("POSTGRES_PORT")
            .ok().and_then(|s| s.parse().ok()).unwrap(),
        host: env::var("POSTGRES_HOST")?,
        db_name: env::var("POSTGRES_DB_NAME")?,
    };

    let app_settings = ApplicationSettings {
        user: env::var("APP_USER")?,
        password: env::var("APP_PASSWORD")?,
        port: env::var("APP_PORT")
            .ok().and_then(|s| s.parse().ok()).unwrap(),
    };

    let settings = Settings {
        database: db_settings,
        application: app_settings,
    };

    Ok(settings)
}