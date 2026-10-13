# ADR 0002 — Tauri desktop without a Bun sidecar

Status: accepted as **implementation route**, pending full cross-platform release gates.
Date: 2026-10-13

## Context

The previous Tauri WebView shell required a separately running Bun HTTP API and PostgreSQL server. That is not an installable stand-alone user experience.

## Decision

- Preserve TypeScript/Bun/Kysely/**Postgres.js** for the browser/service path.
- For Tauri, invoke narrowly scoped Rust functions over IPC, with Rust **SQLx** accessing PostgreSQL directly.
- Package a pinned SHA256-verified **pg0** binary for the target OS/architecture; use it to start and stop an app-specific localhost PostgreSQL instance automatically.
- Store DB data in the application's user data directory; do not remove it on normal exit.
- Reject arbitrary SQL/process/shell permission exposure to the renderer; keep calls typed and narrow.

## Consequences and evidence

The application does not need a Bun sidecar on desktop. It still uses the OS WebView's JavaScript engine; Rust SQLx does not remove that dependency.

A GitHub Actions Linux workflow compiles and packages the app, installs the .deb, and invokes an actual persistence/restart smoke against bundled pg0. Separate checks exercise launch and shutdown via GUI under Xvfb. Windows/macOS runtime, credentials, upgrades, backup/restore, Rust/TypeScript contract parity, pg0 ABI and packaged RuVector remain open gates. Tests are evidence only if the respective job reports success.
