# Health Data Endpoints

## General Upload Pattern

All health data endpoints follow a similar structure:
- Endpoint: `POST /health/upload_{data_type}`
- Authentication: Required
- Request Body: Consistent format across sensor types

## Acceleration Data

### Upload Acceleration Data
- **Endpoint**: `POST /health/upload_acceleration`
- **Request Body Example**:
  ```json
  {
    "data_type": "acceleration",
    "device_info": {
      "device_type": "smartphone",
      "model": "iPhone 14",
      "os_version": "iOS 16.5"
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
    "metadata": {
      "activity": "walking",
      "location": "pocket"
    }
  }
  ```

### Get Acceleration Data
- **Endpoint**: `GET /health/acceleration_data`
- **Response**: List of acceleration data records

## Heart Rate Data

### Upload Heart Rate Data
- **Endpoint**: `POST /health/upload_heart_rate`
- **Request Body Example**:
  ```json
  {
    "data_type": "heart_rate",
    "device_info": {
      "device_type": "smartwatch",
      "model": "Apple Watch",
      "os_version": "watchOS 10.1"
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "heart_rate": 72, 
        "confidence": 0.95
      }
    ],
    "metadata": {
      "activity": "resting"
    }
  }
  ```

### Get Heart Rate Data
- **Endpoint**: `GET /health/heart_rate_data`
- **Response**: List of heart rate data records

## Blood Oxygen Data

### Upload Blood Oxygen Data
- **Endpoint**: `POST /health/upload_blood_oxygen`
- **Request Body Example**:
  ```json
  {
    "data_type": "blood_oxygen",
    "device_info": {
      "device_type": "smartwatch",
      "model": "Fitbit Sense",
      "os_version": "Fitbit OS 5.3"
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "spo2": 98.5, 
        "confidence": 0.95
      }
    ],
    "metadata": {
      "activity": "resting"
    }
  }
  ```

### Get Blood Oxygen Data
- **Endpoint**: `GET /health/blood_oxygen_data`
- **Response**: List of blood oxygen data records

## Skin Temperature Data

### Upload Skin Temperature Data
- **Endpoint**: `POST /health/upload_skin_temperature`
- **Request Body Example**:
  ```json
  {
    "data_type": "skin_temperature",
    "device_info": {
      "device_type": "smartwatch",
      "model": "Apple Watch",
      "os_version": "watchOS 10.1"
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:00:10Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "temperature": 36.2, 
        "confidence": 0.95,
        "body_location": "wrist"
      }
    ],
    "metadata": {
      "activity": "resting"
    }
  }
  ```

### Get Skin Temperature Data
- **Endpoint**: `GET /health/skin_temperature_data`
- **Response**: List of skin temperature data records

## GPS Location Data

### Upload GPS Location Data
- **Endpoint**: `POST /health/upload_gps_location`
- **Request Body Example**:
  ```json
  {
    "data_type": "gps_location",
    "device_info": {
      "device_type": "smartphone",
      "model": "iPhone 14",
      "os_version": "iOS 16.5"
    },
    "sampling_rate_hz": 1,
    "start_time": "2025-03-10T12:00:00Z",
    "end_time": "2025-03-10T12:05:00Z",
    "samples": [
      {
        "timestamp": "2025-03-10T12:00:00Z", 
        "latitude": 40.7128, 
        "longitude": -74.0060, 
        "altitude": 10.5,
        "accuracy": 5.0,
        "speed": 0.0,
        "bearing": 0.0
      }
    ],
    "metadata": {
      "activity": "walking"
    }
  }
  ```

### Get GPS Location Data
- **Endpoint**: `GET /health/gps_location_data`
- **Response**: List of GPS location data records

## Cross-Data Correlation

### Get Health Data with GPS
- **Endpoint**: `GET /health/health_data_with_gps`
- **Query Parameters**:
  - `data_type`: Type of health data
  - `start_time`: ISO 8601 datetime
  - `end_time`: ISO 8601 datetime
- **Purpose**: Correlate health metrics with GPS location

---

Previous: [Authentication](02-authentication.md)
Next: [Sleep Data Endpoints](04-sleep-data-endpoints.md)