# Architecture and constraints

## The default vertical slice

~~~text
.sea/notes.sea (illustrative canonical domain representation)
      | manual contract translation until projection is validated
      v
@template/contracts (Valibot, Standard Schema V1)
      |
      v
@template/core (NoteService + NoteRepository port)
      |
      +--- @template/db (Kysely -> kysely-postgres-js -> Postgres.js)
      |                         |
      |                       pg0 / PostgreSQL
      |
      +--- apps/api (Bun.serve HTTP boundary)
                |
         +------+--------+
         |               |
      apps/web         apps/cli
      React/Vite       headless CLI
         |
    Tauri 2 WebView (optional shell)
~~~

Core types and behavior must remain independent of database server placement and the UI. Postgres.js is a client. pg0 is a distribution/launcher of PostgreSQL. No database access belongs in frontend code.

## Boundary contracts

- Contract: request validation is strict and performed before invoking domain operations. TypeScript types alone are not runtime evidence.
- Core: receives an already validated CreateNote and depends on the NoteRepository port, not SQL.
- Persistence: repository adapter maps domain Note to SQL columns. Migrations are versioned through Kysely's Migrator.
- API: localhost-only development server. Request errors do not disclose database internals.
- Web: browser client calling a separate API, not a database client.
- Desktop: Tauri embeds web assets. Currently relies on a separately running localhost Bun API; packaging is NOT self-contained.
- Native layer: add narrow Tauri commands only after determining permissions and implementing tests.
- DomainForge: .sea can model the domain, but a passing parser alone does not prove the TS translation matches the source.

## Architectural decisions

- Kysely rather than a heavy ORM: query types without losing PostgreSQL-native features.
- Postgres.js rather than Bun.SQL: keeps client portable to Node and Bun.
- Valibot with Standard Schema V1: validation abstraction independent of callers.
- Bun for web tooling and HTTP runtime; Rust for native facilities. Do not assume Bun executes inside the Tauri WebView.
- UI is a projection. CLI, web and Tauri can share behavior without sharing rendering technology.

## Platform-specific runtime realities

On Windows, Tauri defaults to WebView2 (Chromium/V8); on macOS and Linux it uses WebKit. Bun uses JavaScriptCore. Servo uses SpiderMonkey; Boa is an alternative Rust JS interpreter. Merely writing Rust does not remove the JS engine used to run a React web frontend.

On packaged Tauri, web origins differ by OS. This template's API development CORS allowlist is configured for the Vite origin. The production/offline path must be separately implemented and verified before a real desktop distribution.

## Unresolved gates

1. Generate and commit bun.lock; prove clean installs.
2. Tauri build prerequisites, desktop app icons, release CSP and platform packaging.
3. Bundled Bun backend or native Rust persistence and secure desktop IPC.
4. RuVector PostgreSQL extension ABI/build compatibility with pg0 PostgreSQL 18.
5. DomainForge projection generation into schema/contracts.
6. Optional OpenTUI UI and Servo/Boa experiments.
7. Production auth, tenancy, observability, backup/recovery and deployment.

See docs/validation.md for measurable checks.
