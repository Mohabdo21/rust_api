# rust_api

Small Axum + SeaORM service for users and API keys.

No auth. No caching. No magic.

## What It Does

- Creates and lists users.
- Creates, lists, revokes, and deletes API keys.
- Uses SQLite.
- Runs migrations automatically on startup.

## Requirements

- Rust toolchain (stable)
- SQLite (through the bundled SQLite driver used by SeaORM)

## Run

```bash
cargo run
```

Server defaults:

- Host: 127.0.0.1
- Port: 3000
- Database URL: sqlite://app.db?mode=rwc

Startup runs pending migrations before serving requests.

## Configuration

Environment variables:

- DATABASE_URL (default: sqlite://app.db?mode=rwc)
- APP_HOST (default: 127.0.0.1)
- APP_PORT (default: 3000)

A .env file is loaded if present. Real environment variables still win.

## Quick Smoke Test

```bash
# 1) create user
curl -sS -X POST http://127.0.0.1:3000/users \
  -H 'content-type: application/json' \
  -d '{"name":"Alice","email":"alice@example.com"}'

# 2) list users
curl -sS http://127.0.0.1:3000/users

# 3) create key (replace USER_ID)
curl -sS -X POST http://127.0.0.1:3000/api-keys \
  -H 'content-type: application/json' \
  -d '{"user_id":"USER_ID","label":"local-dev"}'
```
