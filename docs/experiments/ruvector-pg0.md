# RuVector + pg0 compatibility experiment

Status: unresolved. pg0 currently bundles PostgreSQL 18 and pgvector. It does not thereby bundle RuVector.

Run bun run db:ruvector to read pg_available_extensions using the accepted Postgres.js driver.

## Experiment protocol

1. Record pg0 version, PostgreSQL server_version, pg_config path and OS/architecture.
2. Inspect RuVector's PostgreSQL extension build matrix for a PostgreSQL 18 compatible pgrx build.
3. Determine where pg0 places extension control files, SQL files and native libraries; do not modify live data directories without backups.
4. Build a disposable instance and verify a native module's PostgreSQL major ABI and dynamic dependencies.
5. Test CREATE EXTENSION ruvector in isolation, then a small ruvector(3) vector table, index and deterministic nearest-neighbor queries.
6. Compare against pgvector for correctness, failure behavior, memory and latency.
7. Repeat on Windows, WSL/Linux and macOS before calling the combination cross-platform.
8. Document restore and clean uninstall of extension artifacts.

Hard stop: unavailable PostgreSQL 18 target, incompatible shared libraries, or no reliable rollback. Never silently substitute pgvector and call it RuVector.

Alternatives are a separately managed PostgreSQL installation with RuVector, or an explicitly independent embedded RuVector storage adapter. They have different semantics, durability and maintenance costs.
