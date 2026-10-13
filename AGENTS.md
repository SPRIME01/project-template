# Repository execution contract

This repository is a reference implementation. Treat its architectural boundaries as invariants until an owner explicitly changes them.

## Governing rules

- The current source domain is .sea/notes.sea, but the TypeScript contracts remain manually maintained until DomainForge generation has been proven. Never describe them as generated.
- Preserve clean direction of dependency: contracts -> core ports -> infrastructure adapters -> API and UI. Core must not import Kysely, Postgres.js, Bun or Tauri.
- Postgres.js is the selected SQL wire client. pg0 runs PostgreSQL. Do not replace Postgres.js with Bun.SQL or confuse pg0 with a client.
- Validate untrusted input with Valibot at the API boundary; use Standard Schema V1 for library interoperability.
- Give application code typed ports and avoid putting SQL in React, CLI or core.
- Authentication, rate-limiting, tenancy, production secrets and packaged-offline desktop behavior are not implemented. No production-readiness claims.
- RuVector is intentionally not silently created: verify availability, ABI compatibility, migration and fallback first.
- Servo, Boa and OpenTUI are experiments, not hard dependencies. Do not add runtime dependencies for architectural hypotheticals.
- Tauri native operations require explicitly scoped commands and permissions; avoid exposing arbitrary Rust execution over IPC.
- Add a test or reproducible evidence command for every consequential change.
- Commit lockfiles after first successful package resolution; avoid unpinned installs in production CI.
- Treat any representation/plan as fallible: compare expected with observed behavior and record mismatches.

## Development

Run bun install; pg0 start --name project-template --port 5432; bun run db:migrate; bun run db:smoke; bun run check. Use bun run dev for API+UI. Do not claim these passed unless they were actually executed.

When adding an optional adapter, provide a contract, a minimal capability test, a portability matrix, and a clean failure/rollback path. Prefer small composable extensions over embedding unnecessary large toolkits.
