# ADR 0001 — Reference development stack

Status: Accepted for the default stack; experimental integrations remain unaccepted.
Date: 2026-10-13
Scope: reusable local-first TypeScript and Rust application template.

## Accepted

- Bun 1.4+ for TypeScript application execution, package management and scripts.
- TypeScript for primary application implementation.
- Rust for native implementation, platform bindings and security-sensitive modules.
- Kysely for composable type-safe SQL queries.
- Postgres.js (npm: postgres) as the PostgreSQL client.
- kysely-postgres-js as the Kysely dialect.
- pg0 as local PostgreSQL runtime/distribution, not a driver.
- PostgreSQL for relational and transactional state.
- Valibot for runtime validation, using Standard Schema V1 for interoperability.
- React + Vite for browser UI.
- Tauri 2 as the preferred desktop shell.
- Just as a single operator-facing task interface; Devbox as optional toolchain environment.

## Deferred

- RuVector extension in pg0: preferred, but packaging and PostgreSQL ABI compatibility remain unproven. pg0 includes pgvector; do not claim RuVector is present.
- Servo/Boa: candidates for rendering/embedded JavaScript in Rust. Neither is a drop-in Tauri backend, and Boa is not a DOM/browser implementation.
- OpenTUI, TanStack graph/chart/canvas tooling, Astryx, RVF, DuckDB, NATS/PGMQ: add only for concrete product requirements.
- Cognate/SEA-Forge/DomainForge production wiring: this template supplies boundaries, not a claim that those integrations are connected.

## Drivers considered

pg is a reasonable fallback with Kysely's built-in PostgresDialect. Bun.SQL provides fewer package dependencies, but ties the adapter to Bun. Rust SQLx is a candidate for native backend implementations; it is not an SQL client for Kysely.

## Consequences

Build small portable domain modules. Reject infrastructure imports in core. Use functional verification instead of naming all intended technologies as working dependencies.
