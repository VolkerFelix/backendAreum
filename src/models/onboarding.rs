use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Data Transfer Objects (DTOs) for API requests

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicInfoRequest {
    pub display_name: Option<String>,
    pub date_of_birth: Option<String>, // Format: YYYY-MM-DD
    pub biological_sex: Option<String>, // 'male', 'female', 'other', 'prefer_not_to_say'
    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub goals: Vec<String>, // Array of goal type names
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LifestyleHealthRequest {
    pub activity_level: String, // 'sedentary', 'lightly_active', 'active', 'very_active'
    pub bedtime: Option<String>, // Format: HH:MM (24-hour)
    pub wake_time: Option<String>, // Format: HH:MM (24-hour)
    pub is_smoker: Option<bool>,
    pub alcohol_consumption: Option<String>, // 'none', 'occasional', 'moderate', 'frequent'
    pub tracks_menstrual_cycle: Option<bool>,
    pub menstrual_cycle_data: Option<serde_json::Value>,
    pub medical_conditions: Vec<String>, // Array of condition names
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionsSetupRequest {
    pub heart_rate_enabled: bool,
    pub temperature_enabled: bool,
    pub spo2_enabled: bool,
    pub accelerometer_enabled: bool,
    pub notifications_enabled: bool,
    pub background_usage_enabled: bool,
    pub third_party_connections: Vec<ThirdPartyConnectionRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThirdPartyConnectionRequest {
    pub connection_type: String, // 'apple_health', 'google_fit', 'fitbit', 'garmin'
    pub connection_data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalizationRequest {
    pub stress_triggers: Option<Vec<String>>,
    pub work_type: Option<String>, // 'office', 'remote', 'shift_based', 'student'
    pub daily_routine: Option<serde_json::Value>,
    pub timezone: Option<String>,
    pub location_data: Option<serde_json::Value>,
}

// Database Models

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub biological_sex: Option<String>,
    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoalType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub goal_type_id: Uuid,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LifestyleInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_level: String,
    pub bedtime: Option<NaiveTime>,
    pub wake_time: Option<NaiveTime>,
    pub is_smoker: Option<bool>,
    pub alcohol_consumption: Option<String>,
    pub tracks_menstrual_cycle: Option<bool>,
    pub menstrual_cycle_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedicalConditionType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMedicalCondition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub condition_id: Uuid,
    pub diagnosed_at: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionsSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub heart_rate_enabled: bool,
    pub temperature_enabled: bool,
    pub spo2_enabled: bool,
    pub accelerometer_enabled: bool,
    pub notifications_enabled: bool,
    pub background_usage_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThirdPartyConnectionType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserThirdPartyConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub connection_type_id: Uuid,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub connection_status: String, // 'connected', 'pending', 'disconnected', 'failed'
    pub last_sync_at: Option<DateTime<Utc>>,
    pub connection_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalizationInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stress_triggers: Option<Vec<String>>,
    pub work_type: Option<String>,
    pub daily_routine: Option<serde_json::Value>,
    pub timezone: Option<String>,
    pub location_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnboardingProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub basic_info_completed: bool,
    pub lifestyle_health_completed: bool,
    pub permissions_setup_completed: bool,
    pub personalization_completed: bool,
    pub onboarding_completed: bool,
    pub current_step: String, // 'basic_info', 'lifestyle_health', 'permissions_setup', 'personalization', 'completed'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Response DTOs for API endpoints

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnboardingStatusResponse {
    pub user_id: String,
    pub basic_info_completed: bool,
    pub lifestyle_health_completed: bool,
    pub permissions_setup_completed: bool,
    pub personalization_completed: bool,
    pub onboarding_completed: bool,
    pub current_step: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicInfoResponse {
    pub display_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub biological_sex: Option<String>,
    pub height_cm: Option<f64>,
    pub weight_kg: Option<f64>,
    pub goals: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LifestyleHealthResponse {
    pub activity_level: String,
    pub bedtime: Option<String>,
    pub wake_time: Option<String>,
    pub is_smoker: Option<bool>,
    pub alcohol_consumption: Option<String>,
    pub tracks_menstrual_cycle: Option<bool>,
    pub medical_conditions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionsSetupResponse {
    pub heart_rate_enabled: bool,
    pub temperature_enabled: bool,
    pub spo2_enabled: bool,
    pub accelerometer_enabled: bool,
    pub notifications_enabled: bool,
    pub background_usage_enabled: bool,
    pub third_party_connections: Vec<ThirdPartyConnectionResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThirdPartyConnectionResponse {
    pub connection_type: String,
    pub connection_status: String,
    pub last_sync_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalizationResponse {
    pub stress_triggers: Option<Vec<String>>,
    pub work_type: Option<String>,
    pub timezone: Option<String>,
}

// Generic API response wrapper
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: Option<String>,
    pub data: Option<T>,
}