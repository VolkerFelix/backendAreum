# Areum Backend API Documentation

This document provides an overview of the available API endpoints in the Areum Backend application.

## Base URL

The base URL for all API endpoints is determined by your environment configuration. The application supports both local and production environments.

## Authentication

Most endpoints require authentication using JWT (JSON Web Tokens). To authenticate, include the JWT token in the Authorization header:

```
Authorization: Bearer <your_jwt_token>
```

## Available Endpoints

### Authentication

#### Register User
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
- **Response**: 200 OK on success

#### Login
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

### Health Data

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
      "device_id": "string" (optional)
    },
    "sampling_rate_hz": number,
    "start_time": "ISO 8601 datetime",
    "samples": [
      {
        "timestamp": "ISO 8601 datetime",
        "x": number,
        "y": number,
        "z": number
      }
    ],
    "metadata": {} (optional)
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
    "count": number,
    "data": [
      {
        "id": "string",
        "data_type": "acceleration",
        "device_info": {
          "device_type": "string",
          "model": "string",
          "os_version": "string",
          "device_id": "string"
        },
        "sampling_rate_hz": number,
        "start_time": "ISO 8601 datetime",
        "data": {
          "samples": [...]
        },
        "created_at": "ISO 8601 datetime"
      }
    ]
  }
  ```

### System Health

#### Backend Health Check
- **Endpoint**: `GET /backend_health`
- **Authentication**: None
- **Response**:
  ```json
  {
    "status": "UP"
  }
  ```

### Protected Resources

#### Protected Resource
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

## Error Responses

The API uses standard HTTP status codes:
- 200: Success
- 400: Bad Request
- 401: Unauthorized (invalid or missing token)
- 500: Internal Server Error

Error responses typically include a message explaining the error:
```json
{
  "status": "error",
  "message": "Error description"
}
```

## Data Types

### DateTime Format
All datetime fields use ISO 8601 format with UTC timezone.

### Device Info
Device information includes:
- device_type: Type of device (e.g., "smartphone", "tablet")
- model: Device model name
- os_version: Operating system version
- device_id: Optional unique identifier for the device

### Acceleration Data
Acceleration data is stored with:
- Three-axis measurements (x, y, z)
- Timestamps for each measurement
- Sampling rate in Hz
- Optional metadata for additional context 