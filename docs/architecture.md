# Architecture and constraints

## Two executable paths, one PostgreSQL engine

Browser/CLI: React -> Bun API -> Valibot -> domain service -> Kysely -> Postgres.js -> PostgreSQL (pg0)

Desktop: React -> Tauri IPC -> narrow Rust adapter -> SQLx -> PostgreSQL (pg0 included in installer)

The JavaScript default remains Postgres.js. SQLx is a separately governed Rust-only adapter for desktop. There is **no HTTP or Bun API sidecar** inside the desktop app. React still runs in the system WebView.

## Desktop persistence and recovery

1. At build time, apps/web/scripts/prepare-desktop.ts retrieves the official pg0 v0.15.2 binary for the build OS/CPU and verifies its release SHA-256. Tauri packages that binary. It does not download PostgreSQL at application runtime.
2. On first desktop command, the Rust backend creates app-local data, a random local database credential, and starts pg0 bound to 127.0.0.1. Pg0 extracts its embedded PostgreSQL engine into the user profile; the cluster data lives in the app-local data directory.
3. The backend reads pg0 info in JSON to determine the real port and URI. It validates loopback host, expected database and expected user before opening a SQLx pool.
4. CREATE TABLE IF NOT EXISTS initializes the Notes schema, and narrow IPC commands provide list/create operations.
5. On normal application exit, the backend closes the pool and stops a database instance **only if the app started it**. Database files are never deleted on normal exit.
6. The desktop smoke command writes a record, fully stops PostgreSQL, recreates the client, restarts PostgreSQL and verifies that the record persists.

The smoke uses a UUID-named **test-only instance** and deletes only that instance after verification. It must never run against real user data.

## Security and portability caveats

- Only list_notes and create_note are invokable. The renderer does not receive SQL or filesystem/process APIs.
- pg0 listens on loopback with a random credential, not the public example password.
- Local credential is created owner-only on Unix, but the instance metadata and Windows ACLs need a separate audit.
- Initial password is visible briefly as a command-line argument to the local operating system: replace with a protected provisioning mechanism before handling sensitive data.
- UI data comes back as the same Note shape as the web API. Rust and Valibot contracts are currently duplicated; semantic parity needs explicit testing.
- pg0 system shared libraries still apply on supported Linux distributions.
- Single-instance plugin discourages concurrent desktop database starts; crash and contention tests are still required.
- Packaged installer success alone does not prove live GUI interaction on all OSes.

## RuVector is a genuine PostgreSQL extension

Upstream RuVector contains crates/ruvector-postgres and implements CREATE EXTENSION ruvector. The unresolved task is producing an ABI-compatible binary for pg0's PostgreSQL 18 and packaging the DLL/shared object plus SQL/control files. Historical docs use cargo-pgrx 0.12.9 and PG16, and the upstream CI has tested PG17, even though the project changelog claims PG18 support. Do not assume it can be copied directly into pg0 without rebuilding and validating.

The existing pg0 instance includes pgvector, not RuVector. See docs/experiments/ruvector-pg0.md and GitHub issue #1.

## Deferred

Servo browser embedding; DomainForge-generated code/contracts; RuVector PostgreSQL 18 integration; full multi-OS installed-GUI verification; production credentials, backup/restore, security review and signing.
