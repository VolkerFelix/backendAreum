# Sleep Data Endpoints

## Overview

Areum Health provides comprehensive sleep tracking and analysis endpoints.

## Get Sleep Data by Date

### Endpoint
- `GET /health/sleep_data`
- **Authentication**: Required

### Query Parameters
- `date`: Date in YYYY-MM-DD format

### Response Example
```json
{
  "status": "success",
  "data": {
    "id": "unique-identifier",
    "user_id": "user-uuid",
    "night_date": "2025-03-24",
    "start_time": "2025-03-24T22:30:00Z",
    "end_time": "2025-03-25T06:30:00Z",
    "samples": [
      {
        "timestamp": "2025-03-24T22:30:00Z",
        "stage": "Awake" | "Light" | "Deep" | "REM" | "Unknown",
        "confidence": 0.95,
        "duration_seconds": 600
      }
    ],
    "sleep_metrics": {
      "sleep_efficiency": 92.5,
      "sleep_latency_seconds": 600,
      "awakenings": 3,
      "time_in_bed_seconds": 29700,
      "total_sleep_seconds": 27500,
      "light_sleep_seconds": 14400,
      "deep_sleep_seconds": 7200,
      "rem_sleep_seconds": 5900,
      "awake_seconds": 2200
    },
    "sleep_score": 85
  }
}
```

## Get Sleep Data Range

### Endpoint
- `GET /health/sleep_data_range`
- **Authentication**: Required

### Query Parameters
- `start_date`: Start date in YYYY-MM-DD format
- `end_date`: End date in YYYY-MM-DD format

### Response Example
```json
{
  "status": "success",
  "count": 3,
  "data": [
    {
      "id": "unique-identifier",
      "user_id": "user-uuid",
      "night_date": "2025-03-22",
      "start_time": "2025-03-22T22:30:00Z",
      "end_time": "2025-03-23T06:30:00Z",
      "samples": [...],
      "sleep_metrics": {...},
      "sleep_score": 82
    }
  ]
}
```

## Get Sleep Summary by Date

### Endpoint
- `GET /health/sleep_summary`
- **Authentication**: Required

### Query Parameters
- `date`: Date in YYYY-MM-DD format

### Response Example
```json
{
  "status": "success",
  "data": {
    "night_date": "2025-03-24",
    "sleep_metrics": {
      "sleep_efficiency": 92.5,
      "sleep_latency_seconds": 600,
      "time_in_bed_seconds": 29700,
      "total_sleep_seconds": 27500
    },
    "sleep_score": 85,
    "overall_quality": "Good",
    "highlights": [
      "Excellent deep sleep duration",
      "Consistent sleep schedule"
    ],
    "issues": [
      "Slightly long time to fall asleep"
    ],
    "stage_distribution": {
      "awake_percentage": 7.5,
      "light_percentage": 48.5,
      "deep_percentage": 24.2,
      "rem_percentage": 19.8
    },
    "recommendations": [
      "Consider relaxation techniques before bedtime",
      "Maintain your consistent sleep schedule"
    ]
  }
}
```

## Get Weekly Sleep Trends

### Endpoint
- `GET /health/sleep_trends`
- **Authentication**: Required

### Response Example
```json
{
  "status": "success",
  "data": {
    "days_with_data": 5,
    "average_sleep_score": 83.6,
    "average_sleep_time_hours": 7.2,
    "average_deep_sleep_percentage": 24.5,
    "daily_summaries": [
      {
        "date": "2025-03-21",
        "sleep_score": 82,
        "total_sleep_hours": 7.1,
        "deep_sleep_percentage": 24.0,
        "overall_quality": "Good"
      }
    ]
  }
}
```

## Sleep Stage Definitions

### Sleep Stages
- **Awake**: Time spent not sleeping
- **Light Sleep**: Initial sleep stage, transitional
- **Deep Sleep**: Most restorative sleep stage
- **REM (Rapid Eye Movement)**: Dream state, crucial for cognitive functions

## Sleep Metrics Explained

### Sleep Efficiency
- Percentage of time in bed actually spent sleeping
- Higher percentage indicates better sleep quality

### Sleep Latency
- Time taken to fall asleep
- Shorter latency typically indicates better sleep readiness

### Awakenings
- Number of times woken up during the night
- Fewer awakenings suggest more continuous sleep

### Stage Distribution
- Breakdown of sleep stages
- Ideal distribution varies by age and individual factors

## Sleep Score Interpretation

### Score Range: 0-100
- **80-100**: Excellent Sleep
- **60-79**: Good Sleep
- **40-59**: Fair Sleep
- **0-39**: Poor Sleep

### Factors Affecting Sleep Score
- Total sleep time
- Sleep stage distribution
- Number of awakenings
- Sleep consistency
- Sleep efficiency

---

Previous: [Health Data Endpoints](04-health-data-endpoints.md)
Next: [System Health Endpoints](06-system-health-endpoints.md)