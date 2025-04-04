use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub device_type: String,
    pub model: String,
    pub os_version: String,
    #[serde(default)]
    pub device_id: Option<String>,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            device_type: String::from("unknown"),
            model: String::from("unknown"),
            os_version: String::from("unknown"),
            device_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccelerationSample {
    pub timestamp: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartRateSample {
    pub timestamp: DateTime<Utc>,
    pub heart_rate: i32,  // beats per minute
    #[serde(default)]
    pub confidence: Option<f64>,  // confidence score between 0 and 1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccelerationDataUpload {
    pub data_type: String,
    pub device_info: DeviceInfo,
    pub sampling_rate_hz: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub samples: Vec<AccelerationSample>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartRateDataUpload {
    pub data_type: String,
    pub device_info: DeviceInfo,
    pub sampling_rate_hz: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub samples: Vec<HeartRateSample>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BloodOxygenSample {
    pub timestamp: DateTime<Utc>,
    pub spo2: f32,  // SpO2 percentage (typically 95-100%)
    #[serde(default)]
    pub confidence: Option<f64>,  // confidence score between 0 and 1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BloodOxygenDataUpload {
    pub data_type: String,
    pub device_info: DeviceInfo,
    pub sampling_rate_hz: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub samples: Vec<BloodOxygenSample>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkinTemperatureSample {
    pub timestamp: DateTime<Utc>,
    pub temperature: f32,  // Temperature in Celsius
    #[serde(default)]
    pub confidence: Option<f64>,  // confidence score between 0 and 1
    #[serde(default)]
    pub body_location: Option<String>,  // Optional body location (e.g., "wrist", "forehead")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkinTemperatureDataUpload {
    pub data_type: String,
    pub device_info: DeviceInfo,
    pub sampling_rate_hz: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub samples: Vec<SkinTemperatureSample>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GpsLocationSample {
    pub timestamp: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>,
    pub speed: Option<f64>,
    pub bearing: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GpsLocationDataUpload {
    pub data_type: String,
    pub device_info: DeviceInfo,
    pub sampling_rate_hz: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub samples: Vec<GpsLocationSample>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthDataRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data_type: String,
    pub device_info: serde_json::Value,
    pub sampling_rate_hz: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthDataResponse {
    pub id: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthDataTimeQuery {
    pub data_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
