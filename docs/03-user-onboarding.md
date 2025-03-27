# User Onboarding

## Overview

The user onboarding process consists of several steps that users need to complete to fully set up their account. This section documents all the endpoints related to the onboarding process.

## Onboarding Status

### Get Onboarding Status
- **Endpoint**: `GET /onboarding/status`
- **Authentication**: Required
- **Description**: Retrieves the current status of the user's onboarding process
- **Response**:
  ```json
  {
    "status": "success",
    "data": {
      "basic_info_completed": boolean,
      "lifestyle_health_completed": boolean,
      "permissions_setup_completed": boolean,
      "personalization_completed": boolean,
      "onboarding_completed": boolean,
      "current_step": string
    }
  }
  ```
- **Possible current_step values**:
  - "basic_info"
  - "lifestyle_health"
  - "permissions_setup"
  - "personalization"

## Basic Information

### Submit Basic Information
- **Endpoint**: `POST /onboarding/basic_info`
- **Authentication**: Required
- **Description**: Submit basic user information
- **Request Body**:
  ```json
  {
    "name": "string",
    "date_of_birth": "YYYY-MM-DD",
    "gender": "string",
    "height_cm": number,
    "weight_kg": number
  }
  ```
- **Response**:
  ```json
  {
    "status": "success",
    "message": "Basic information updated successfully"
  }
  ```

## Lifestyle and Health Information

### Submit Lifestyle and Health Information
- **Endpoint**: `POST /onboarding/lifestyle_health`
- **Authentication**: Required
- **Description**: Submit lifestyle and health-related information
- **Request Body**:
  ```json
  {
    "activity_level": "string",
    "bedtime": "HH:mm",
    "wake_time": "HH:mm",
    "is_smoker": boolean,
    "alcohol_consumption": "string",
    "tracks_menstrual_cycle": boolean,
    "medical_conditions": ["string"]
  }
  ```
- **Response**:
  ```json
  {
    "status": "success",
    "message": "Lifestyle and health information updated successfully"
  }
  ```

## Permissions Setup

### Submit Permissions Setup
- **Endpoint**: `POST /onboarding/permissions_setup`
- **Authentication**: Required
- **Description**: Configure data collection permissions and third-party connections
- **Request Body**:
  ```json
  {
    "heart_rate_enabled": boolean,
    "temperature_enabled": boolean,
    "spo2_enabled": boolean,
    "accelerometer_enabled": boolean,
    "notifications_enabled": boolean,
    "background_usage_enabled": boolean,
    "third_party_connections": [
      {
        "connection_type": "string",
        "connection_data": {
          "permissions": ["string"]
        }
      }
    ]
  }
  ```
- **Response**:
  ```json
  {
    "status": "success",
    "message": "Permissions setup completed successfully"
  }
  ```

## Personalization

### Submit Personalization
- **Endpoint**: `POST /onboarding/personalization`
- **Authentication**: Required
- **Description**: Set up user preferences and personalization settings
- **Request Body**:
  ```json
  {
    "preferred_units": {
      "temperature": "celsius|fahrenheit",
      "distance": "metric|imperial",
      "weight": "metric|imperial"
    },
    "notification_preferences": {
      "health_alerts": boolean,
      "daily_summary": boolean,
      "weekly_report": boolean
    },
    "theme_preference": "light|dark|system"
  }
  ```
- **Response**:
  ```json
  {
    "status": "success",
    "message": "Personalization settings updated successfully"
  }
  ```

## Error Responses

All onboarding endpoints may return the following error responses:

- **401 Unauthorized**: When the authentication token is missing or invalid
- **400 Bad Request**: When the request body is invalid or missing required fields
- **500 Internal Server Error**: When there's a server-side error

## Best Practices

1. Complete the onboarding steps in order
2. Validate all input data before submission
3. Handle errors gracefully and provide appropriate feedback to users
4. Store sensitive information securely
5. Implement proper validation for all user inputs

---

Previous: [Authentification](02-authentification.md)
Next: [Health Data Endpoints](04-health-data-endpoints.md)