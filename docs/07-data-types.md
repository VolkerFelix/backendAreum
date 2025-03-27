# Data Types and Formats

## Overview

Areum Health API uses standardized data types and formats to ensure consistency and interoperability.

## Timestamp Format

### ISO 8601 Standard
- Format: `YYYY-MM-DDTHH:mm:ss.sssZ`
- Always in UTC timezone
- Example: `"2025-03-10T14:27:31.850Z"`

### Precision
- Millisecond-level accuracy
- Consistent across all timestamps

## Sensor Data Types

### Device Information
```json
{
  "device_type": "smartphone" | "smartwatch" | "fitness_tracker",
  "model": "string",
  "os_version": "string",
  "device_id": "optional-unique-identifier"
}
```

### Acceleration Data
```json
{
  "timestamp": "ISO 8601 datetime",
  "x": number,  // g-force (-2 to +2)
  "y": number,  // g-force (-2 to +2)
  "z": number   // g-force (-2 to +2)
}
```
- Represents 3-axis acceleration
- Units: g-force
- Range: Typically -2g to +2g

### Heart Rate Data
```json
{
  "timestamp": "ISO 8601 datetime",
  "heart_rate": number,     // Beats per minute
  "confidence": number      // Optional (0-1)
}
```
- Measured in beats per minute (BPM)
- Optional confidence score

### Blood Oxygen Data
```json
{
  "timestamp": "ISO 8601 datetime",
  "spo2": number,           // Oxygen saturation percentage
  "confidence": number      // Optional (0-1)
}
```
- Measured as SpO2 percentage
- Typical range: 95-100%

### Skin Temperature Data
```json
{
  "timestamp": "ISO 8601 datetime",
  "temperature": number,    // Celsius
  "confidence": number,     // Optional (0-1)
  "body_location": "string" // Optional
}
```
- Measured in Celsius
- Optional body location indicator

### GPS Location Data
```json
{
  "timestamp": "ISO 8601 datetime",
  "latitude": number,
  "longitude": number,
  "altitude": number,       // Optional (meters)
  "accuracy": number,       // Optional (meters)
  "speed": number,          // Optional (meters/second)
  "bearing": number         // Optional (degrees)
}
```
- Required: Latitude and Longitude
- Optional: Altitude, Accuracy, Speed, Bearing

## Sleep Stage Data

### Sleep Stage Enum
```
Awake
Light
Deep
REM
Unknown
```

### Sleep Stage Sample
```json
{
  "timestamp": "ISO 8601 datetime",
  "stage": "Sleep Stage Enum",
  "confidence": number,     // Optional (0-1)
  "duration_seconds": number
}
```

## Metadata

### Purpose
- Provides contextual information
- Optional additional details
- Supports advanced analysis

### Example Metadata
```json
{
  "activity": "walking" | "resting" | "sleeping",
  "location": "indoor" | "outdoor",
  "environment": "home" | "office" | "gym"
}
```

## Units and Conventions

### Standard Units
- Time: Seconds
- Distance: Meters
- Temperature: Celsius
- Acceleration: g-force
- Heart Rate: Beats per minute
- Oxygen Saturation: Percentage

### Numeric Conventions
- Floating-point numbers
- Consistent decimal precision
- Range-specific validation

## Data Validation

### Recommended Validation
- Check timestamp format
- Validate numeric ranges
- Verify required fields
- Sanitize input data

### Recommended Ranges
- Acceleration: -2 to +2 g
- Heart Rate: 30-220 BPM
- SpO2: 90-100%
- Temperature: 35-42°C

---

Previous: [System Health Endpoints](05-system-health-endpoints.md)
Next: [Error Handling](07-error-handling.md)