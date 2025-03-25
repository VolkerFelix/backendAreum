# Authentication

## Overview

Areum Health API uses JSON Web Tokens (JWT) for secure authentication.

## Authentication Flow

1. **User Registration**
   - Endpoint: `POST /register_user`
   - Request Body:
     ```json
     {
       "username": "string",
       "password": "string",
       "email": "string"
     }
     ```
   - Response: 200 OK (Empty body)

2. **User Login**
   - Endpoint: `POST /login`
   - Request Body:
     ```json
     {
       "username": "string",
       "password": "string"
     }
     ```
   - Response:
     ```json
     {
       "token": "string"
     }
     ```

## Using the JWT Token

- Include the token in the `Authorization` header for protected endpoints
- Format: `Authorization: Bearer <your_jwt_token>`

## Token Lifecycle

- Token Expiration: 24 hours
- Refresh mechanism implemented
- Secure storage recommended

## Security Best Practices

1. Store token securely
2. Implement token refresh logic
3. Handle token expiration gracefully
4. Never share tokens

## Error Handling

- 401 Unauthorized: Invalid or expired token
- 403 Forbidden: Insufficient permissions

## Token Validation

- Cryptographically signed
- Includes user claims
- Verified on each request

## Secure Transmission

- Always use HTTPS
- Tokens transmitted over encrypted channels
- Protect against man-in-the-middle attacks

## Logout and Token Invalidation

- Client-side token removal
- Server-side token blacklisting (optional)

---

Previous: [Introduction](01-introduction.md)
Next: [Health Data Endpoints](03-health-data-endpoints.md)