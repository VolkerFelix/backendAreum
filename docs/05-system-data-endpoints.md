# System Health Endpoints

## Backend Health Check

### Endpoint
- `GET /backend_health`
- **Authentication**: None required

### Purpose
Provides a simple health check to verify the API is operational.

### Response
```json
{
  "status": "UP"
}
```

### Use Cases
- Monitoring service availability
- Automated health checks
- Verifying API connectivity

## Protected Resource Endpoint

### Endpoint
- `GET /protected/resource`
- **Authentication**: Required (JWT Token)

### Purpose
Demonstrates protected endpoint access and token validation.

### Response
```json
{
  "status": "success",
  "message": "You have access to protected resource",
  "user_id": "unique-user-identifier",
  "username": "example_user"
}
```

### Validation Checks
- Token existence
- Token validity
- User authorization

## Monitoring and Diagnostics

### Health Check Best Practices
- Perform periodic health checks
- Implement retry mechanisms
- Log connection attempts
- Handle temporary service interruptions

### Typical Health Check Intervals
- Frontend: Every 5-10 minutes
- Monitoring systems: Every 1-2 minutes
- Critical services: Real-time monitoring

## Error Handling

### Common Health Check Scenarios
- **Service Unavailable**: 503 status
- **Unauthorized**: 401 status
- **Internal Error**: 500 status

### Recommended Client Actions
- Cache last known good state
- Implement exponential backoff
- Provide user-friendly error messages
- Log diagnostic information

## Security Considerations

- Health endpoints should minimize information exposure
- Avoid revealing sensitive system details
- Use minimal authentication for public health checks
- Implement rate limiting

---

Previous: [Sleep Data Endpoints](04-sleep-data-endpoints.md)
Next: [Data Types](06-data-types.md)