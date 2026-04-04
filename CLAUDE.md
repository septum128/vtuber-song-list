# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
# Start the server
cargo loco start

# Run all tests (requires PostgreSQL and Redis)
DATABASE_URL=postgres://loco:loco@localhost:5432/vtuber-song-list_development REDIS_URL=redis://localhost:6379 cargo test --all-features --all

# Run a single test
DATABASE_URL=postgres://... cargo test <test_name>

# Lint
cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2018-idioms

# Format check
cargo fmt --all -- --check

# Format
cargo fmt --all

# Generate a new migration
cargo loco generate migration <name>
```

## Architecture

This is a [Loco](https://loco.rs) web framework application (Rust, version 0.16) built on Axum + SeaORM. It uses the SaaS starter template.

**Entry point:** `src/app.rs` — implements `Hooks` trait for `App`, wiring together routes, workers, initializers, migrations, and seed data.

**Key layers:**

- `src/controllers/` — Axum route handlers. Currently only `auth.rs` (registration, login, JWT, magic links, password reset). Add new controllers here and register them in `app.rs`.
- `src/models/` — Two-level model structure:
  - `src/models/_entities/` — auto-generated SeaORM entities (do not edit manually)
  - `src/models/users.rs` — business logic layer on top of the entity (validations, finders, password hashing, JWT generation)
- `src/workers/downloader.rs` — Background worker skeleton using `BackgroundWorker` trait; registered in `app.rs`. Runs in `BackgroundAsync` mode.
- `src/initializers/view_engine.rs` — Tera view engine initializer, registered at startup.
- `src/mailers/auth.rs` — Transactional email templates (welcome, forgot password, magic link) in `src/mailers/auth/*/`.
- `migration/` — SeaORM migration crate. New migrations go here and must be registered in `migration/src/lib.rs`.

**Database:** PostgreSQL (default) or SQLite. Connection URL via `DATABASE_URL` env var; falls back to `postgres://loco:loco@localhost:5432/vtuber-song-list_development`. Auto-migrates on startup in development.

**Auth:** JWT-based (`loco_rs::auth::jwt`). Tokens encode user `pid` (UUID). Also supports API key auth and magic link (5-minute expiry, 32-char token).

**Testing:** Uses `loco-rs` test helpers with `insta` for snapshot testing. Tests require a real PostgreSQL database — no mocking. Snapshots live alongside test files in `snapshots/` directories.

**Code style:** `rustfmt` with `max_width = 100`. Clippy is configured pedantic + nursery + rust-2018-idioms.

## Adding New Features

When adding a new model:
1. Run `cargo loco generate model <name>` to create the migration and entity
2. Add business logic in `src/models/<name>.rs`
3. Register the model in `src/models/mod.rs`
4. Add truncation in `App::truncate` and seeding in `App::seed` in `app.rs` if needed

When adding a new controller:
1. Create `src/controllers/<name>.rs`
2. Export it from `src/controllers/mod.rs`
3. Register its routes in `App::routes` in `app.rs`

## Branch Naming Convention

Format: `<type>/<issue-id>-<short-description>`

| type | 用途 |
|------|------|
| `fix` | バグ修正 |
| `feat` | 新機能 |
| `hotfix` | 緊急修正 |
| `refactor` | リファクタリング |
| `chore` | 雑務・依存更新 |
| `docs` | ドキュメント |

Examples:
- `fix/123-login-error`
- `feat/101-user-profile`
- `hotfix/789-payment-failure`

Rules:
- 英数字・ハイフンのみ（スペース・アンダースコア禁止）
- 小文字統一
- 短く端的に（3〜5単語程度）
- Issue番号があれば含める
