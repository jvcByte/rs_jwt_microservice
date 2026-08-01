# Backend

Actix-Web REST API providing JWT authentication and user management. Handles registration, login, token refresh, and protected user endpoints.

## Prerequisites

- Rust 1.70+ (edition 2021)
- PostgreSQL 12+
- A running instance of the [database](../database/README.md) migrations

## Setup

1. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```

   Configure your `.env` file:
   ```env
   DATABASE_URL=postgresql://username:password@localhost:5432/your_database
   JWT_SECRET=your_super_secret_jwt_key_here
   JWT_ACCESS_TOKEN_EXPIRATION_MINUTES=15
   JWT_REFRESH_TOKEN_EXPIRATION_DAYS=30
   ADDRESS=127.0.0.1
   PORT=8080
   RUST_LOG=debug
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run database migrations** — see [database/README.md](../database/README.md)

4. **Start the server**
   ```bash
   cargo run
   ```

The server starts on `http://127.0.0.1:8080` by default.

## API Endpoints

### Authentication
- `POST /auth/register` - User registration
- `POST /auth/login` - User login

### Users
- `GET /users/me` - Get current user profile (requires auth)
- `PUT /users/me` - Update current user profile (requires auth)

### Tokens
- `POST /refresh-tokens` - Refresh access token

### Health Check
- `GET /` - Service health check

## API Usage Examples

### User Registration
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "johndoe",
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

### User Login
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

Response:
```json
{
  "access_token": "<JWT_ACCESS_TOKEN>",
  "token_type": "Bearer",
  "expires_in": 900,
  "refresh_token": "<REFRESH_TOKEN>",
  "user": {
    "id": "uuid",
    "name": "Alice",
    "email": "alice@example.com"
  }
}
```

### Access Protected Endpoint
```bash
curl -X GET http://localhost:8080/api/auth/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

## Testing

```bash
cargo test
```

## Logging

Configure log level via `RUST_LOG`:
- `error` - Only errors
- `warn` - Warnings and errors
- `info` - Info, warnings, and errors
- `debug` - Debug info and above
- `trace` - All logs

## Configuration Reference

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret key for JWT signing | Required |
| `JWT_ACCESS_TOKEN_EXPIRATION_MINUTES` | Access token lifetime | 15 |
| `JWT_REFRESH_TOKEN_EXPIRATION_DAYS` | Refresh token lifetime | 30 |
| `ADDRESS` | Server bind address | 127.0.0.1 |
| `PORT` | Server port | 8080 |
| `RUST_LOG` | Log level | debug |

## Security Features

- **Password Hashing**: Argon2 algorithm for secure password storage
- **JWT Tokens**: Stateless authentication with configurable expiration
- **Input Validation**: Proper validation of user inputs
- **CORS**: Configurable Cross-Origin Resource Sharing (if needed)
- **Rate Limiting**: Can be added via middleware

## Tech Stack

- **Actix Web 4.13.0**: High-performance web framework
- **SeaORM 1.1.19**: Type-safe ORM for Rust
- **JWT 10.3.0**: JSON Web Token authentication
- **Argon2 0.5.3**: Modern password hashing
- **Serde**: Serialization framework
- **Chrono**: Date/time handling
- **UUID**: Unique identifier generation
