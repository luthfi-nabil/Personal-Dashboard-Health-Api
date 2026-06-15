# Health API

Insulin-tracking backend for the **personal-dashboard** suite. Built with
[Actix-Web](https://actix.rs/) (Rust) and MySQL/MariaDB, it manages a user's
insulin item catalog, batch/pen assignments, and usage (dose) logs. It is
consumed by the [Flutter dashboard's](../personal_dashboard_flutter) Insulin
screen.

## Purpose

- Maintain a catalog of **insulin items** (e.g. NovoRapid, Lantus) per user.
- Track **insulin assignments** — linking a physical batch/pen to an insulin
  item, with remaining units.
- Record **insulin usage** events (doses administered), deducting from the
  assigned batch.
- Expose a **Flutter sync** endpoint (`/api/flutter/health-sync*`) used by the
  mobile app's offline-cache sync.

Every user-scoped endpoint lives under `/api/user/...` and requires an
`Authorization: Bearer <jwt>` header. The API validates the token and uses its
signed `sub` claim as `created_by`.

## Tech stack

- **Actix-Web 4** — HTTP server/routing
- **MySQL/MariaDB** (via the `mysql` crate) — primary datastore, via `establish_connection_v2`
- **rusqlite** — present as a dependency (legacy `transaction.db`), unused by the active code path
- **jsonwebtoken** — used by the Flutter sync flow

## Project structure

```
src/
  main.rs               # entrypoint: table init, routing setup
  routes/main_route.rs  # route table
  handlers/             # request handlers (insulin items/assign/usage, sync, swagger)
  repository/           # SQL queries + table creation (init_create_table_v2)
  models/               # request/response structs
  route_middleware/      # CreatedBy extraction, JSON error wrapping
  helper/                # DB connection, response codes
swagger.yaml             # OpenAPI 3 spec, served at /docs in development
```

## Prerequisites

- Rust toolchain (edition 2024 — see `Cargo.toml`)
- A running MySQL/MariaDB server
- The `health` schema, imported from [`../health_db.sql`](../health_db.sql)

```bash
mysql -u root -p < ../health_db.sql
```

On startup, `init_create_table_v2()` also ensures the required tables exist.

## Configuration

Configuration is read from a `.env` file in this directory (via `dotenv`):

| Variable    | Default        | Description |
|-------------|----------------|-------------|
| `HOST`      | `127.0.0.1`    | Bind address. Use `0.0.0.0` to accept connections from other devices (e.g. a phone running the Flutter app on the same LAN). |
| `PORT`      | `8080`         | Bind port (project default is `4000`). |
| `APP_ENV`   | `production`   | Set to `development` to enable Swagger UI at `/docs` and `/docs/openapi.yaml`. |
| `DB_HOST`   | `127.0.0.1`    | MySQL host. |
| `DB_PORT`   | `3306`         | MySQL port. |
| `DB_USER`   | `root`         | MySQL user. |
| `DB_PASS`   | `123456`       | MySQL password. |
| `DB_NAME`   | `health`       | MySQL database name (matches `health_db.sql`). |
| `RUST_BACKTRACE` | -         | Set to `1` for full panic backtraces during development. |

Example `.env`:

```env
RUST_BACKTRACE=1
HOST=0.0.0.0
PORT=4000
APP_ENV=development
DB_HOST=127.0.0.1
DB_PORT=3306
DB_USER=root
DB_PASS=123456
DB_NAME=health
```

## Running locally

```bash
cargo run
```

The server prints its bound address on startup:

```
Created tables
Server running at http://0.0.0.0:4000
Swagger UI  →  http://0.0.0.0:4000/docs
OpenAPI spec →  http://0.0.0.0:4000/docs/openapi.yaml
```

## API documentation

With `APP_ENV=development`, browse to `/docs` for the Swagger UI, backed by
[`swagger.yaml`](swagger.yaml). Key endpoint groups:

- `GET/POST /api/user/insulin-item` — insulin item catalog
- `GET /api/user/insulin-assign-usage` — combined assignment + usage view
- `POST /api/user/insulin-assign`, `DELETE .../insulin-assign/{insulin_assign_id}` — batch/pen assignments
- `POST/DELETE /api/user/insulin-usage` — dose logging
- `GET /api/flutter/health-sync`, `POST /api/flutter/health-sync/push` — offline-cache sync for the Flutter app

## CORS and logging

Unlike `transaction-api`, this service currently has **no CORS middleware**
and **no request logging** configured — only `JsonErrorMiddleware` (wraps
errors as JSON) is applied. If this API is called from a browser/Flutter-web
context, add `actix-cors` (`Cors::permissive()`) the same way it was added to
`transaction-api`.

## Deployment

A [`Dockerfile`](Dockerfile) is provided for the build stage (`cargo build`);
the runtime stage is left as a template (commented out) for you to complete —
e.g. copy `target/debug/health-api` (or `target/release/...` with
`cargo build --release`) into a slim runtime image along with a `.env` file.

For local network access (e.g. testing from a phone), set `HOST=0.0.0.0` and
point the Flutter app's "Health API base URL" (Settings screen) at this
machine's LAN IP, e.g. `http://192.168.1.x:4000`. Ensure your firewall allows
inbound connections to the chosen port.
