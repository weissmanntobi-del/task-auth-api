# Task Auth API — Production-Oriented Rust Backend

Companion repository for **Rust Mini Projects — From Zero to Production-Grade Services**.

This project turns the book's final production project into a real Rust repository: an Axum + SQLx + Postgres backend with JWT authentication, migrations, tracing, Docker, health checks, and integration tests.

## What this repo demonstrates

- Axum HTTP routing and typed extractors
- SQLx Postgres pool and migrations
- Argon2 password hashing
- JWT access tokens
- Hashed refresh-token storage and revocation
- Protected routes with a real `CurrentUser` extractor
- Structured JSON error responses
- Liveness and readiness endpoints
- Docker and Docker Compose
- CI with format, clippy, and tests
- Register → login → `/me` integration test

## API surface

| Method | Path             | Purpose                                    |
|--------|------------------|--------------------------------------------|
| GET    | `/health/live`   | Liveness probe                             |
| GET    | `/health/ready`  | Readiness probe backed by Postgres         |
| POST   | `/auth/register` | Create user and return tokens              |
| POST   | `/auth/login`    | Verify credentials and return tokens       |
| POST   | `/auth/refresh`  | Rotate refresh token and return new tokens |
| POST   | `/auth/logout`   | Revoke a refresh token                     |
| GET    | `/me`            | Return authenticated user profile          |
| GET    | `/tasks`         | List authenticated user's tasks            |
| POST   | `/tasks`         | Create authenticated user's task           |
| PATCH  | `/tasks/{id}`    | Update authenticated user's task           |

## Quick start with Docker Compose

```bash
docker compose up --build
```

Then check:

```bash
curl http://localhost:3000/health/live
curl http://localhost:3000/health/ready
```

Register a user:

```bash
curl -s -X POST http://localhost:3000/auth/register \
  -H 'content-type: application/json' \
  -d '{"email":"demo@example.com","password":"correct-horse-battery-staple"}'
```

Login:

```bash
curl -s -X POST http://localhost:3000/auth/login \
  -H 'content-type: application/json' \
  -d '{"email":"demo@example.com","password":"correct-horse-battery-staple"}'
```

Use the returned access token:

```bash
curl http://localhost:3000/me \
  -H "authorization: Bearer <ACCESS_TOKEN>"
```

## Local development

Start Postgres:

```bash
docker compose up -d db
```

Set environment variables:

```bash
cp .env.example .env
export APP__DATABASE__URL=postgres://app:app@localhost:5432/task_api
export APP__AUTH__JWT_SECRET=local-dev-secret-change-before-production
```

Run:

```bash
cargo run
```

## Quality gate

```bash
make check
```

This runs:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## Integration test

The auth flow test uses a real database. Start Postgres and set `TEST_DATABASE_URL`:

```bash
docker compose up -d db
export TEST_DATABASE_URL=postgres://app:app@localhost:5432/task_api
cargo test register_login_and_me_flow_works -- --nocapture
```

## Production-grade meaning

This repository is production-oriented rather than a complete enterprise platform. It includes the baseline engineering habits expected from a serious backend project: explicit config, migrations, secure password hashing, token-based auth, protected routes, structured errors, tracing, health checks, Docker packaging, and CI.

Areas intentionally left for future hardening:

- Rate limiting
- Metrics endpoint
- Password reset and email verification
- OpenAPI generation
- Role-based authorization
- Dedicated migration job for zero-downtime deployments
- Secret manager integration

## License

MIT
