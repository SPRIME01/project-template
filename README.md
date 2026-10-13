# GodSpeed Project Template

Reference starter for Bun + TypeScript + Valibot + Standard Schema V1 + Kysely + Postgres.js + pg0 + PostgreSQL, with React/Vite and an optional Tauri 2 desktop shell.

## Status

Implemented reference path: contracts -> core service -> Postgres.js persistence -> Bun HTTP API -> React UI. This is an intentionally small Notes example; replace the domain, not the architecture.

Experimental/not production-certified: RuVector's compatibility with pg0; Servo/Boa embedding; OpenTUI; packaged offline Tauri backend; DomainForge-to-TypeScript automated projections. See docs/architecture.md and docs/validation.md. No CI or deployment success should be inferred before running the checks.

## Quick start

Prerequisites: Bun 1.4+, pg0 (installed separately), and Just (recommended). Rust and Tauri system prerequisites are needed only for desktop.

1. bun install
2. cp .env.example .env  (or copy it in PowerShell)
3. pg0 start --name project-template --port 5432
4. bun run db:migrate
5. bun run db:smoke
6. bun run dev
7. Open http://127.0.0.1:5173

The development API runs at http://127.0.0.1:3000. If pg0 chose a different URI, update DATABASE_URL in .env before migration and running the API. On Windows, pg0 uses its native executable; on WSL install the Linux release. Default pg0 credentials in this example are for local development only.

## Commands

- just setup: install workspace packages
- just db-start / db-stop: manage the local pg0 instance
- just migrate / db-smoke: create schema and verify an actual SQL roundtrip
- just dev: concurrently run API and web
- just check: TypeScript checks, Bun tests and web build
- just desktop-dev: run Tauri desktop shell (requires running API)
- just doctor: inspect toolchain commands

Without Just, all commands have Bun script equivalents in package.json.

## Stack decisions

See docs/decisions/0001-development-stack.md. Postgres.js, not Bun.SQL, is the standardized SQL driver. pg0 is the database distribution, not a driver. Kysely generates SQL and Postgres.js executes it. Runtime validation stays at boundaries. Native Tauri capabilities must use narrow, authorized IPC commands.

## Repository map

- apps/api: Bun HTTP server, Valibot request validation
- apps/web: React/Vite browser UI and optional Tauri desktop shell
- apps/cli: headless API CLI example (OpenTUI not required)
- packages/contracts: runtime schemas and TypeScript types
- packages/core: domain logic behind a repository port
- packages/db: Kysely models, Postgres.js adapter, migrations
- .sea: illustrative DomainForge source model
- docs: architecture, open experiments, verification criteria
- .github: CI checks

## Security boundaries

This is a **local-development reference**, not an internet-facing service. It deliberately has no authentication, tenancy, CSRF protections or production secrets management. Do not expose the API publicly. Do not package the desktop application as standalone/offline until the backend/database lifecycle is designed and tested.

## Forking this reference

Rename @template/* packages and the Tauri identifier, replace Notes with a real domain model, and add domain settlement tests before introducing additional services. Preserve the domain/repository boundary and prove integrations independently.
