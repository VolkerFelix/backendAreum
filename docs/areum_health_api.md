# Areum Health API Documentation

This document provides comprehensive documentation for the Areum Health API, which serves as a guideline for mobile app implementation.

## Base URL

The base URL for all API endpoints depends on your deployment environment:

- **Production**: `https://backendareum.fly.dev`
- **Development/Testing**: `http://localhost:8080`

## Authentication

Most endpoints require authentication using JWT (JSON Web Tokens). To authenticate:

1. Register or login to obtain a JWT token
2. Include the token in the Authorization header of your requests:

```
Authorization: Bearer <your_jwt_token>
```

## Error Handling

The API uses standard HTTP status codes:
- `200 OK`: Success
- `400 Bad Request`: Invalid request data
- `401 Unauthorized`: Missing or invalid authentication
- `500 Internal Server Error`: Server-side error

Error responses include a JSON object with error details:

```json
{
  "status": "error",
  "message": "Error description"
}
```

## API Endpoints

### Authentication

#### Register User

Creates a new user account.

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
  - Success (200 OK): Empty response body
  - Error (400): `{"status": "error", "message": "Error message"}`

#### Login

Authenticates a user and returns a JWT token.

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
  - Success (200 OK): 
    ```json
    {
      "token": "string"
    }
    ```
  - Error (401): No content or error message

### Health Data

All health data endpoints require authentication and are prefixed with `/health`.

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
      },
      // Additional samples...
    ],
    "metadata": { // optional
      "activity": "walking",
      "location": "pocket",
      "environment": "indoors"
    }
  }
  ```
- **Response**:
  - Success (200 OK): 
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
  - Success (200 OK): 
    ```json
    {
      "status": "success",
      "count": 1,
      "data": [
        {
          "id": "string",
          "user_id": "string",
          "data_type": "acceleration",
          "device_info": {
            "device_type": "string",
            "model": "string",
            "os_version": "string",
            "device_id": "string"
          },
          "sampling_rate_hz": 100,
          "start_time": "2025-03-10T14:27:31.850Z",
          "end_time": "2025-03-10T14:27:32.100Z",
          "data": {
            "samples": [
              // Array of acceleration samples
            ],
            "metadata": { }
          },
          "created_at": "2025-03-10T14:28:00.000Z"
        }
      ]
    }
    ```

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
      },
      // Additional samples...
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  - Success (200 OK): 
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
      },
      // Additional samples...
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  - Success (200 OK): 
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
- **Response**: Similar structure to other get data endpoints

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
      },
      // Additional samples...
    ],
    "metadata": { // optional
      "activity": "resting"
    }
  }
  ```
- **Response**:
  - Success (200 OK): 
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
- **Response**: Similar structure to other get data endpoints

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
      },
      // Additional samples...
    ],
    "metadata": { // optional
      "activity": "walking"
    }
  }
  ```
- **Response**:
  - Success (200 OK): 
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
- **Response**: Similar structure to other get data endpoints

#### Get Health Data With GPS

Gets health data of a specific type with associated GPS data.

- **Endpoint**: `GET /health/health_data_with_gps`
- **Authentication**: Required
- **Query Parameters**:
  - `data_type`: Type of health data (e.g., "heart_rate")
  - `start_time`: ISO 8601 datetime
  - `end_time`: ISO 8601 datetime
- **Example**: `/health/health_data_with_gps?data_type=heart_rate&start_time=2025-03-15T10:00:00Z&end_time=2025-03-15T11:00:00Z`
- **Response**:
  - Success (200 OK): 
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
              "longitude": -74.0061,
              "altitude": 10.5,
              "accuracy": 5.0,
              "speed": 1.2,
              "bearing": 45.0
            },
            // Additional GPS points corresponding to health data timestamps
          ]
        }
      ]
    }
    ```

### System Health

#### Backend Health Check

- **Endpoint**: `GET /backend_health`
- **Authentication**: None
- **Response**:
  - Success (200 OK): 
    ```json
    {
      "status": "UP"
    }
    ```

### Protected Resources

#### Protected Resource

Example endpoint demonstrating protection with JWT.

- **Endpoint**: `GET /protected/resource`
- **Authentication**: Required
- **Response**:
  - Success (200 OK): 
    ```json
    {
      "status": "success",
      "message": "You have access to protected resource",
      "user_id": "string",
      "username": "string"
    }
    ```

## Data Types

### DateTime Format
All datetime fields use ISO 8601 format with UTC timezone (e.g., `2025-03-10T14:27:31.850Z`).

### Device Info
Device information includes:
- `device_type`: Type of device (e.g., "smartphone", "smartwatch", "tablet")
- `model`: Device model name (e.g., "iPhone 14", "Samsung Galaxy Watch")
- `os_version`: Operating system version (e.g., "iOS 16.5", "watchOS 10.1")
- `device_id`: Optional unique identifier for the device

### Health Data Types

#### Acceleration Data
- Three-axis measurements (x, y, z) in g-force units (typically -2g to +2g)
- High sampling rate (50-100Hz or higher)
- Optional metadata for context

#### Heart Rate Data
- Heart rate measurements in beats per minute (BPM)
- Optional confidence score (0-1)
- Typically sampled at 1Hz or lower

#### Blood Oxygen Data
- SpO2 percentage (typically 95-100%)
- Optional confidence score (0-1)
- Typically sampled occasionally or on-demand

#### Skin Temperature Data
- Temperature in Celsius
- Optional confidence score (0-1)
- Optional body location (e.g., "wrist")
- Typically sampled occasionally (e.g., every few minutes)

#### GPS Location Data
- Latitude and longitude (required)
- Optional altitude (meters)
- Optional accuracy (meters)
- Optional speed (meters/second)
- Optional bearing (degrees)
- Typically sampled at variable rates depending on activity

## Implementation Guidelines

### Mobile App Implementation

1. **Authentication Flow**:
   - Implement registration and login screens
   - Store JWT token securely (e.g., in Keychain for iOS, EncryptedSharedPreferences for Android)
   - Add interceptor/middleware to include token in all authenticated requests
   - Handle token expiration and refresh

2. **Data Collection**:
   - Use device-specific APIs to collect health data (HealthKit for iOS, Health Connect for Android)
   - Batch data and upload periodically or when conditions are favorable (e.g., device charging, on WiFi)
   - Consider local storage for offline operation
   - Implement retry logic for failed uploads

3. **Data Visualization**:
   - Fetch data using the GET endpoints
   - Implement charts and graphs to visualize health metrics
   - Consider caching strategies for improved performance

### Best Practices

1. **Error Handling**:
   - Implement comprehensive error handling for all API calls
   - Provide meaningful error messages to users
   - Log errors for debugging purposes

2. **Battery Optimization**:
   - Adjust data collection frequency based on activity
   - Batch uploads to reduce network usage
   - Consider device battery level when scheduling background tasks

3. **Network Considerations**:
   - Implement offline mode
   - Handle intermittent connectivity
   - Consider data compression for large datasets

4. **Privacy and Security**:
   - Clearly communicate data collection practices to users
   - Implement proper authentication flows
   - Secure local storage of sensitive data
   - Consider data anonymization where appropriate

## Change Management

Any changes to the API will be communicated through:
1. Updated API documentation
2. Semantic versioning
3. Release notes

---

Last Updated: March 24, 2025