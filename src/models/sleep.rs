use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SleepStage {
    #[serde(rename = "Awake")]
    Awake,
    #[serde(rename = "Light")]
    Light,
    #[serde(rename = "Deep")]
    Deep,
    #[serde(rename = "REM")]
    REM,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for SleepStage {
    fn default() -> Self {
        SleepStage::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SleepStageSample {
    pub timestamp: DateTime<Utc>,
    pub stage: SleepStage,
    #[serde(default)]
    pub confidence: Option<f64>,
    #[serde(default)]
    pub duration_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SleepMetrics {
    #[serde(default)]
    pub sleep_efficiency: Option<f64>,  // Percentage of time in bed spent sleeping
    #[serde(default)]
    pub sleep_latency_seconds: Option<i32>,  // Time taken to fall asleep
    #[serde(default)]
    pub awakenings: Option<i32>,  // Number of times woken up
    #[serde(default)]
    pub time_in_bed_seconds: Option<i32>,  // Total time in bed
    #[serde(default)]
    pub total_sleep_seconds: Option<i32>,  // Total time spent sleeping
    #[serde(default)]
    pub light_sleep_seconds: Option<i32>,  // Time spent in light sleep
    #[serde(default)]
    pub deep_sleep_seconds: Option<i32>,  // Time spent in deep sleep
    #[serde(default)]
    pub rem_sleep_seconds: Option<i32>,  // Time spent in REM sleep
    #[serde(default)]
    pub awake_seconds: Option<i32>,  // Time spent awake after falling asleep
}

// This represents processed sleep data as stored by the Python microservice
#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessedSleepData {
    pub id: String,
    pub user_id: String,
    pub night_date: String,  // YYYY-MM-DD format
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub samples: Vec<SleepStageSample>,
    #[serde(rename = "sleep_metrics")]
    pub sleep_metrics: SleepMetrics,
    pub sleep_score: i32,  // 0-100 score
    pub created_at: DateTime<Utc>,
}

// Implement custom Debug for better error tracing
impl fmt::Display for ProcessedSleepData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProcessedSleepData {{ 
            id: {}, 
            user_id: {}, 
            night_date: {}, 
            start_time: {}, 
            end_time: {}, 
            samples: {} items, 
            sleep_metrics: {:?}, 
            sleep_score: {}, 
            created_at: {} 
        }}", 
            self.id, 
            self.user_id, 
            self.night_date, 
            self.start_time, 
            self.end_time, 
            self.samples.len(), 
            self.sleep_metrics, 
            self.sleep_score, 
            self.created_at
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SleepSummary {
    pub id: String,
    pub user_id: String,
    pub night_date: String,  // YYYY-MM-DD format
    pub sleep_metrics: SleepMetrics,
    pub sleep_score: i32,  // 0-100 score
    pub overall_quality: String,  // "Poor", "Fair", "Good", "Excellent"
    pub highlights: Vec<String>,  // Positive aspects
    pub issues: Vec<String>,      // Identified issues
    pub stage_distribution: StageDistribution,
    pub recommendations: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StageDistribution {
    pub awake_percentage: f64,
    pub light_percentage: f64,
    pub deep_percentage: f64,
    pub rem_percentage: f64,
}

// Query parameters for retrieving sleep data
#[derive(Deserialize, Debug)]
pub struct SleepDateQuery {
    pub date: String,  // Expected format: YYYY-MM-DD
}

#[derive(Deserialize, Debug)]
pub struct SleepRangeQuery {
    pub start_date: String,  // YYYY-MM-DD
    pub end_date: String,    // YYYY-MM-DD
}