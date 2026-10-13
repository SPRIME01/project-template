# GodSpeed Project Template

Reference starter for Bun + TypeScript + Valibot + Standard Schema V1 + Kysely + Postgres.js + pg0 + PostgreSQL, with React/Vite, an optional Tauri 2 desktop shell, and isolated OpenTUI and Boa experiments.

## Status

Implemented reference path: contracts -> core service -> Postgres.js persistence -> Bun HTTP API -> React UI. This is an intentionally small Notes example; replace the domain, not the architecture.

Experimental/not production-certified: RuVector's compatibility with pg0; Servo/Boa browser integration; OpenTUI terminal deployment; packaged offline Tauri backend; DomainForge-to-TypeScript generated projections. See docs/architecture.md and docs/validation.md. Source committed does not imply tests or CI passed.

## Quick start

Prerequisites: Bun 1.4+, pg0 (installed separately), and Just (recommended). Rust and Tauri system prerequisites are needed only for desktop.

1. bun install
2. Copy .env.example to .env (PowerShell: Copy-Item .env.example .env)
3. pg0 start --name project-template --port 5432
4. bun run db:migrate
5. bun run db:smoke
6. bun run dev
7. Open http://127.0.0.1:5173

The development API runs at http://127.0.0.1:3000. If pg0 chose a different URI, update DATABASE_URL in .env before migration and running the API. On Windows, pg0 uses its native executable; on WSL install the Linux release. Default credentials in this example are for local development only.

## Verification commands

- just setup: install workspace packages
- just db-start / db-stop: manage local pg0
- just migrate / db-smoke / db-ruvector: migration, PostgreSQL roundtrip, extension detection
- just dev: concurrently run API and web
- just check: TypeScript checks, Bun tests, web build
- just desktop-dev: run Tauri shell with API already running
- just doctor: inspect toolchain commands

Without Just, package.json offers Bun script equivalents.

## Optional experiments (not default dependencies)

- OpenTUI: cd experiments/opentui, then bun install and bun run dev (start the API separately).
- Boa: cd experiments/boa, then cargo run and cargo test.
- Servo: research path only; no plug-and-play integration is claimed.
- Tauri icons: cd apps/web, then bun run desktop:icons before release packaging.

## Stack decisions

See docs/decisions/0001-development-stack.md and stack.toml. Postgres.js, not Bun.SQL, is the standardized SQL driver. pg0 is the database distribution, not a driver. Kysely generates SQL and Postgres.js executes it. Runtime validation stays at boundaries. Native Tauri capabilities must use narrow, authorized IPC commands.

## Repository map

- apps/api: Bun HTTP server with Valibot request validation
- apps/web: React/Vite browser UI and Tauri desktop shell
- apps/cli: headless API client
- packages/contracts: runtime schemas and Standard Schema adapter
- packages/core: domain logic behind repository port
- packages/db: Kysely models, Postgres.js adapter, migrations
- experiments: isolated optional OpenTUI and Boa adapters
- .sea: illustrative DomainForge model
- stack.toml: machine-readable stack selections and evidence status
- docs: architecture, decisions and experimental verification
- .github: initial CI workflow

## Security boundaries

This is a **local-development reference**, not an internet-facing service. It deliberately has no authentication, tenancy, CSRF protections or production secrets management. Do not expose the API publicly. Do not package the desktop application as standalone/offline until the backend/database lifecycle, production origin/CORS and native IPC are designed and tested.

## Use as a template

In GitHub Settings > General, enable Template repository if you want the "Use this template" action. On a copied project, rename the @template/* package namespace, project name and Tauri application identifier, generate and commit bun.lock, replace the Notes example with an actual domain, and validate .sea translations before using them as authority.

Keep adapters optional. A useful reference should make the next project cheaper to start without imposing dependencies that project cannot use.
