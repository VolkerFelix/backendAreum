# Areum Health Backend API Documentation

## Base URL

The base URL for all API endpoints depends on your deployment environment:

- **Production**: `https://backendareum.fly.dev`
- **Development/Testing**: `http://localhost:8080`

## Authentication

Most endpoints require authentication using JWT (JSON Web Tokens). 

### Authentication Flow
1. Register a new user
2. Login to obtain a JWT token
3. Include the token in the Authorization header for protected endpoints:
   ```
   Authorization: Bearer <your_jwt_token>
   ```

## Authentication Endpoints

### Register User
- **Endpoint**: `POST /register_user`
- **Authentication**: None
- **Request Body**:
  ```json
  {
    "username": "string",
    "password": "string",
    "email": "string"
  }
  ```
- **Response**: 
  - 200 OK: Empty response body
  - 400 Bad Request: Error details

### Login
- **Endpoint**: `POST /login`
- **Authentication**: None
- **Request Body**:
  ```json
  {
    "username": "string",
    "password": "string"
  }
  ```
- **Response**:
  ```json
  {
    "token": "string"
  }
  ```

## Health Data Endpoints

### Acceleration Data

#### Upload Acceleration Data
- **Endpoint**: `POST /health/upload_acceleration`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "acceleration",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 100,
    "start_time": "2025-03-10T14:27:31.850Z",
    "end_time": "2025-03-10T14:27:32.100Z",
    "samples": [
      {
        "timestamp": "2025-03-10T14:27:31.850Z", 
        "x": 0.012, 
        "y": -0.043, 
        "z": 0.971
      }
    ],
    "metadata": { // optional
      "activity": "walking",
      "location": "pocket"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Acceleration data uploaded successfully"
  }
  ```

#### Get Acceleration Data
- **Endpoint**: `GET /health/acceleration_data`
- **Authentication**: Required
- **Response**:
  ```json
  {
    "status": "success",
    "count": 1,
    "data": [
      {
        "id": "string",
        "user_id": "string",
        "data_type": "acceleration",
        "device_info": {...},
        "sampling_rate_hz": 100,
        "start_time": "2025-03-10T14:27:31.850Z",
        "end_time": "2025-03-10T14:27:32.100Z",
        "data": {
          "samples": [...],
          "metadata": {...}
        },
        "created_at": "2025-03-10T14:28:00.000Z"
      }
    ]
  }
  ```

### Heart Rate Data

#### Upload Heart Rate Data
- **Endpoint**: `POST /health/upload_heart_rate`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "heart_rate",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "heart_rate": 72, 
        "confidence": 0.95 // optional
      }
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Heart rate data uploaded successfully"
  }
  ```

#### Get Heart Rate Data
- **Endpoint**: `GET /health/heart_rate_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

### Blood Oxygen Data

#### Upload Blood Oxygen Data
- **Endpoint**: `POST /health/upload_blood_oxygen`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "blood_oxygen",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "spo2": 98.5, 
        "confidence": 0.95 // optional
      }
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Blood oxygen data uploaded successfully"
  }
  ```

#### Get Blood Oxygen Data
- **Endpoint**: `GET /health/blood_oxygen_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

### Skin Temperature Data

#### Upload Skin Temperature Data
- **Endpoint**: `POST /health/upload_skin_temperature`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "skin_temperature",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "temperature": 36.2, 
        "confidence": 0.95, // optional
        "body_location": "wrist" // optional
      }
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Skin temperature data uploaded successfully"
  }
  ```

#### Get Skin Temperature Data
- **Endpoint**: `GET /health/skin_temperature_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

### GPS Location Data

#### Upload GPS Location Data
- **Endpoint**: `POST /health/upload_gps_location`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "gps_location",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:05:00Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "latitude": 40.7128, 
        "longitude": -74.0060, 
        "altitude": 10.5, // optional
        "accuracy": 5.0, // optional
        "speed": 0.0, // optional
        "bearing": 0.0 // optional
      }
    ],
    "metadata": { // optional
      "activity": "walking"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "GPS location data uploaded successfully"
  }
  ```

#### Get GPS Location Data
- **Endpoint**: `GET /health/gps_location_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

#### Get Health Data With GPS
- **Endpoint**: `GET /health/health_data_with_gps`
- **Authentication**: Required
- **Query Parameters**:
  - `data_type`: Type of health data (e.g., "heart_rate")
  - `start_time`: ISO 8601 datetime
  - `end_time`: ISO 8601 datetime
- **Response**:
  ```json
  {
    "status": "success",
    "count": 1,
    "data": [
      {
        "id": "string",
        "data_type": "heart_rate",
        "device_info": { ... },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-15T10:05:00Z",
        "end_time": "2025-03-15T10:20:00Z",
        "data": { ... },
        "created_at": "2025-03-15T10:21:00Z",
        "gps_data": [
          {
            "timestamp": "2025-03-15T10:05:00Z",
            "latitude": 40.7129,
            "longitude": -74.0061
          }
        ]
      }
    ]
  }
  ```

## Sleep Data Endpoints

### Get Sleep Data by Date

Retrieve detailed sleep stage data for a specific date.

- **Endpoint**: `GET /health/sleep_data`
- **Authentication**: Required
- **Query Parameters**:
  - `date`: Date in YYYY-MM-DD format
- **Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "string",
      "user_id": "string",
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
- **Errors**:
  - 404: No sleep data found for the specified date
  - 400: Invalid date format

### Get Sleep Data Range

Retrieve sleep stage data for a specific date range.

- **Endpoint**: `GET /health/sleep_data_range`
- **Authentication**: Required
- **Query Parameters**:
  - `start_date`: Start date in YYYY-MM-DD format
  - `end_date`: End date in YYYY-MM-DD format
- **Response**:
  ```json
  {
    "status": "success",
    "count": 3,
    "data": [
      {
        "id": "string",
        "user_id": "string",
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
- **Errors**:
  - 400: Invalid date range (end date before start date)

### Get Sleep Summary by Date

Retrieve a summary of sleep metrics for a specific date.

- **Endpoint**: `GET /health/sleep_summary`
- **Authentication**: Required
- **Query Parameters**:
  - `date`: Date in YYYY-MM-DD format
- **Response**:
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
- **Errors**:
  - 404: No sleep summary found for the specified date
  - 400: Invalid date format

### Get Weekly Sleep Trends

Retrieve sleep trends for the past week.

- **Endpoint**: `GET /health/sleep_trends`
- **Authentication**: Required
- **Response**:
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
- **Response Conditions**:
  - Returns success even with no data
  - Provides trend information when sleep data is available

## System Health

### Backend Health Check
- **Endpoint**: `GET /backend_health`
- **Authentication**: None
- **Response**:
  ```json
  {
    "status": "UP"
  }
  ```

### Protected Resource
- **Endpoint**: `GET /protected/resource`
- **Authentication**: Required
- **Response**:
  ```json
  {
    "status": "success",
    "message": "You have access to protected resource",
    "user_id": "string",
    "username": "string"
  }
  ```

## Data Types and Formats

### Device Information
Device information provides context about the data source:
- `device_type`: Type of device (e.g., "smartphone", "smartwatch")
- `model`: Specific device model
- `os_version`: Operating system version
- `device_id`: Optional unique device identifier

### Timestamp Format
- All timestamps use ISO 8601 format with UTC timezone
- Example: `"2025-03-10T14:27:31.850Z"`

### Sampling Rates
- Measured in Hertz (Hz)
- Indicates number of measurements per second
- Typical ranges:
  - Acceleration: 50-100 Hz
  - Heart Rate: 1 Hz
  - GPS: Varies by activity (0.1-1 Hz)

### Sensor Data Types

#### Acceleration Data
- Measures 3-axis acceleration (x, y, z)
- Units: g-force (-2g to +2g)
- Captures movement and orientation

#### Heart Rate Data
- Measured in beats per minute (BPM)
- Optional confidence score (0-1)
- Tracks cardiovascular activity

#### Blood Oxygen Data
- Measured as SpO2 percentage (95-100%)
- Optional confidence score (0-1)
- Indicates blood oxygen saturation

#### Skin Temperature Data
- Measured in Celsius
- Optional confidence score (0-1)
- Optional body location indicator

#### GPS Location Data
- Latitude and Longitude (required)
- Optional: Altitude, Accuracy, Speed, Bearing

## Error Handling

### Standard HTTP Status Codes
- `200 OK`: Successful request
- `400 Bad Request`: Invalid input or parameters
- `401 Unauthorized`: Authentication required or token invalid
- `404 Not Found`: Requested resource doesn't exist
- `500 Internal Server Error`: Server-side error

### Error Response Format
```json
{
  "status": "error",
  "message": "Detailed error description"
}
```

### Common Error Scenarios
- Missing or invalid authentication token
- Incorrect data format
- Date range validation failures
- Resource not found
- Server-side processing errors

## Best Practices

### Authentication
1. Securely store JWT token
2. Refresh token before expiration
3. Handle token invalidation

### Data Collection
1. Batch data uploads
2. Ensure device time is synchronized
3. Handle offline data collection
4. Respect battery optimization

### Privacy and Security
1. Obtain user consent for data collection
2. Implement secure data transmission
3. Allow user control over data sharing
4. Anonymize data where possible

### Performance Considerations
1. Minimize data transfer size
2. Use efficient data compression
3. Implement intelligent sync strategies
4. Handle intermittent connectivity

## Versioning and Compatibility

- Current API Version: 1.0
- Backward compatibility maintained
- Version changes communicated via documentation
- Deprecation notices provided for significant changes

## Support and Documentation

For additional support:
- API Documentation: [Project README](README.md)
- Contact Support: support@areum.health
- Report Issues: GitHub Issues

---

Last Updated: March 25, 2025# Areum Health Backend API Documentation

## Base URL

The base URL for all API endpoints depends on your deployment environment:

- **Production**: `https://backendareum.fly.dev`
- **Development/Testing**: `http://localhost:8080`

## Authentication

Most endpoints require authentication using JWT (JSON Web Tokens). 

### Authentication Flow
1. Register a new user
2. Login to obtain a JWT token
3. Include the token in the Authorization header for protected endpoints:
   ```
   Authorization: Bearer <your_jwt_token>
   ```

## Authentication Endpoints

### Register User
- **Endpoint**: `POST /register_user`
- **Authentication**: None
- **Request Body**:
  ```json
  {
    "username": "string",
    "password": "string",
    "email": "string"
  }
  ```
- **Response**: 
  - 200 OK: Empty response body
  - 400 Bad Request: Error details

### Login
- **Endpoint**: `POST /login`
- **Authentication**: None
- **Request Body**:
  ```json
  {
    "username": "string",
    "password": "string"
  }
  ```
- **Response**:
  ```json
  {
    "token": "string"
  }
  ```

## Health Data Endpoints

### Acceleration Data

#### Upload Acceleration Data
- **Endpoint**: `POST /health/upload_acceleration`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "acceleration",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 100,
    "start_time": "2025-03-10T14:27:31.850Z",
    "end_time": "2025-03-10T14:27:32.100Z",
    "samples": [
      {
        "timestamp": "2025-03-10T14:27:31.850Z", 
        "x": 0.012, 
        "y": -0.043, 
        "z": 0.971
      }
    ],
    "metadata": { // optional
      "activity": "walking",
      "location": "pocket"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Acceleration data uploaded successfully"
  }
  ```

#### Get Acceleration Data
- **Endpoint**: `GET /health/acceleration_data`
- **Authentication**: Required
- **Response**:
  ```json
  {
    "status": "success",
    "count": 1,
    "data": [
      {
        "id": "string",
        "user_id": "string",
        "data_type": "acceleration",
        "device_info": {...},
        "sampling_rate_hz": 100,
        "start_time": "2025-03-10T14:27:31.850Z",
        "end_time": "2025-03-10T14:27:32.100Z",
        "data": {
          "samples": [...],
          "metadata": {...}
        },
        "created_at": "2025-03-10T14:28:00.000Z"
      }
    ]
  }
  ```

### Heart Rate Data

#### Upload Heart Rate Data
- **Endpoint**: `POST /health/upload_heart_rate`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "heart_rate",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "heart_rate": 72, 
        "confidence": 0.95 // optional
      }
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Heart rate data uploaded successfully"
  }
  ```

#### Get Heart Rate Data
- **Endpoint**: `GET /health/heart_rate_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

### Blood Oxygen Data

#### Upload Blood Oxygen Data
- **Endpoint**: `POST /health/upload_blood_oxygen`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "blood_oxygen",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "spo2": 98.5, 
        "confidence": 0.95 // optional
      }
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Blood oxygen data uploaded successfully"
  }
  ```

#### Get Blood Oxygen Data
- **Endpoint**: `GET /health/blood_oxygen_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

### Skin Temperature Data

#### Upload Skin Temperature Data
- **Endpoint**: `POST /health/upload_skin_temperature`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "skin_temperature",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "temperature": 36.2, 
        "confidence": 0.95, // optional
        "body_location": "wrist" // optional
      }
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "Skin temperature data uploaded successfully"
  }
  ```

#### Get Skin Temperature Data
- **Endpoint**: `GET /health/skin_temperature_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

### GPS Location Data

#### Upload GPS Location Data
- **Endpoint**: `POST /health/upload_gps_location`
- **Authentication**: Required
- **Request Body**:
  ```json
  {
    "data_type": "gps_location",
    "device_info": {
      "device_type": "string",
      "model": "string",
      "os_version": "string",
      "device_id": "string" // optional
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:05:00Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "latitude": 40.7128, 
        "longitude": -74.0060, 
        "altitude": 10.5, // optional
        "accuracy": 5.0, // optional
        "speed": 0.0, // optional
        "bearing": 0.0 // optional
      }
    ],
    "metadata": { // optional
      "activity": "walking"
    }
  }
  ```
- **Response**:
  ```json
  {
    "id": "string",
    "status": "success",
    "message": "GPS location data uploaded successfully"
  }
  ```

#### Get GPS Location Data
- **Endpoint**: `GET /health/gps_location_data`
- **Authentication**: Required
- **Response**: Similar structure to get acceleration data

#### Get Health Data With GPS
- **Endpoint**: `GET /health/health_data_with_gps`
- **Authentication**: Required
- **Query Parameters**:
  - `data_type`: Type of health data (e.g., "heart_rate")
  - `start_time`: ISO 8601 datetime
  - `end_time`: ISO 8601 datetime
- **Response**:
  ```json
  {
    "status": "success",
    "count": 1,
    "data": [
      {
        "id": "string",
        "data_type": "heart_rate",
        "device_info": { ... },
        "sampling_rate_hz": 1,
        "start_time": "2025-03-15T10:05:00Z",
        "end_time": "2025-03-15T10:20:00Z",
        "data": { ... },
        "created_at": "2025-03-15T10:21:00Z",
        "gps_data": [
          {
            "timestamp": "2025-03-15T10:05:00Z",
            "latitude": 40.7129,
            "longitude": -74.0061
          }
        ]
      }
    ]
  }
  ```

## Sleep Data Endpoints

### Get Sleep Data by Date

Retrieve detailed sleep stage data for a specific date.

- **Endpoint**: `GET /health/sleep_data`
- **Authentication**: Required
- **Query Parameters**:
  - `date`: Date in YYYY-MM-DD format
- **Response**:
  ```json
  {
    "status": "success",
    "data": {
      "id": "string",
      "user_id": "string",
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
- **Errors**:
  - 404: No sleep data found for the specified date
  - 400: Invalid date format

### Get Sleep Data Range

Retrieve sleep stage data for a specific date range.

- **Endpoint**: `GET /health/sleep_data_range`
- **Authentication**: Required
- **Query Parameters**:
  - `start_date`: Start date in YYYY-MM-DD format
  - `end_date`: End date in YYYY-MM-DD format
- **Response**:
  ```json
  {
    "status": "success",
    "count": 3,
    "data": [
      {
        "id": "string",
        "user_id": "string",
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
- **Errors**:
  - 400: Invalid date range (end date before start date)

### Get Sleep Summary by Date

Retrieve a summary of sleep metrics for a specific date.

- **Endpoint**: `GET /health/sleep_summary`
- **Authentication**: Required
- **Query Parameters**:
  - `date`: Date in YYYY-MM-DD format
- **Response**:
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
- **Errors**:
  - 404: No sleep summary found for the specified date
  - 400: Invalid date format

### Get Weekly Sleep Trends

Retrieve sleep trends for the past week.

- **Endpoint**: `GET /health/sleep_trends`
- **Authentication**: Required
- **Response**:
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
- **Response Conditions**:
  - Returns success even with no data
  - Provides trend information when sleep data is available

## System Health

### Backend Health Check
- **Endpoint**: `GET /backend_health`
- **Authentication**: None
- **Response**:
  ```json
  {
    "status": "UP"
  }
  ```

### Protected Resource
- **Endpoint**: `GET /protected/resource`
- **Authentication**: Required
- **Response**:
  ```json
  {
    "status": "success",
    "message": "You have access to protected resource",
    "user_id": "string",
    "username": "string"
  }
  ```

## Data Types and Formats

### Device Information
Device information provides context about the data source:
- `device_type`: Type of device (e.g., "smartphone", "smartwatch")
- `model`: Specific device model
- `os_version`: Operating system version
- `device_id`: Optional unique device identifier

### Timestamp Format
- All timestamps use ISO 8601 format with UTC timezone
- Example: `"2025-03-10T14:27:31.850Z"`

### Sampling Rates
- Measured in Hertz (Hz)
- Indicates number of measurements per second
- Typical ranges:
  - Acceleration: 50-100 Hz
  - Heart Rate: 1 Hz
  - GPS: Varies by activity (0.1-1 Hz)

### Sensor Data Types

#### Acceleration Data
- Measures 3-axis acceleration (x, y, z)
- Units: g-force (-2g to +2g)
- Captures movement and orientation

#### Heart Rate Data
- Measured in beats per minute (BPM)
- Optional confidence score (0-1)
- Tracks cardiovascular activity

#### Blood Oxygen Data
- Measured as SpO2 percentage (95-100%)
- Optional confidence score (0-1)
- Indicates blood oxygen saturation

#### Skin Temperature Data
- Measured in Celsius
- Optional confidence score (0-1)
- Optional body location indicator

#### GPS Location Data
- Latitude and Longitude (required)
- Optional: Altitude, Accuracy, Speed, Bearing

## Error Handling

### Standard HTTP Status Codes
- `200 OK`: Successful request
- `400 Bad Request`: Invalid input or parameters
- `401 Unauthorized`: Authentication required or token invalid
- `404 Not Found`: Requested resource doesn't exist
- `500 Internal Server Error`: Server-side error

### Error Response Format
```json
{
  "status": "error",
  "message": "Detailed error description"
}
```

### Common Error Scenarios
- Missing or invalid authentication token
- Incorrect data format
- Date range validation failures
- Resource not found
- Server-side processing errors

## Best Practices

### Authentication
1. Securely store JWT token
2. Refresh token before expiration
3. Handle token invalidation

### Data Collection
1. Batch data uploads
2. Ensure device time is synchronized
3. Handle offline data collection
4. Respect battery optimization

### Privacy and Security
1. Obtain user consent for data collection
2. Implement secure data transmission
3. Allow user control over data sharing
4. Anonymize data where possible

### Performance Considerations
1. Minimize data transfer size
2. Use efficient data compression
3. Implement intelligent sync strategies
4. Handle intermittent connectivity

## Versioning and Compatibility

- Current API Version: 1.0
- Backward compatibility maintained
- Version changes communicated via documentation
- Deprecation notices provided for significant changes

## Support and Documentation

For additional support:
- Repo Documentation: [Project README](README.md)
- Contact Support: support@areum.health
- Report Issues: GitHub Issues

---

Last Updated: March 25, 2025# Areum Health Backend API Documentation