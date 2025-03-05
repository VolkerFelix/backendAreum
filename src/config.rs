use std::env;
use config::{Config, File, ConfigError};
use dotenv::dotenv;
use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize, Debug)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings{
    pub user: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub db_name: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password.expose_secret(), self.host, self.port, self.db_name
        ).into_boxed_str())
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
    pub password: SecretString, 
    pub port: u16,
    pub host: String,
    pub log_level: String
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    
    dotenv().ok();

    let environment: Environment = env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let env_filename = format!("{}.yml", environment.as_str());
    let settings = Config::builder()
        .add_source(File::from(configuration_directory.join("base.yml")))
        .add_source(File::from(configuration_directory.join(env_filename)))
        .add_source(
            config::Environment::default()
                .prefix("POSTGRES")
                .prefix_separator("__")
                .separator("__")
        )
        .add_source(
            config::Environment::default()
                .prefix("APP")
                .prefix_separator("__")
                .separator("__")
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local` or `production`.",
                other
            )),
        }
    }
}