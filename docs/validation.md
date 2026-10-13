# Validation and evidence

This repository was written through the GitHub connector. At the time of creation, dependencies were not installed and neither Bun nor Cargo execution was available in the authoring environment. The available connector exposed no completed CI check result. The commands below are acceptance criteria, not claimed historical passes.

## Tier 0 — package and compile

1. bun install
2. Commit generated bun.lock for reproducible installs.
3. bun run typecheck
4. bun run test
5. bun run build

Expected: strict TypeScript checks and all contract/domain unit tests pass; Vite emits apps/web/dist. A package-resolution failure is a real blocker, not a reason to silently downgrade the stack.

## Tier 1 — actual PostgreSQL settlement

1. pg0 start --name project-template --port 5432
2. Verify pg0 info --name project-template and match DATABASE_URL.
3. bun run db:migrate
4. bun run db:smoke
5. bun run db:ruvector

Expected: a real PostgreSQL insert/read succeeds; a deliberately failed transaction leaves no row; cleanup removes the smoke test note. The RuVector probe may report unavailable. That is an unresolved integration, not a failure of the relational core.

## Tier 2 — user journey

1. bun run dev
2. GET http://127.0.0.1:3000/health
3. Open http://127.0.0.1:5173.
4. Create a note and reload the browser.
5. Invoke bun run cli list.

Expected: persisted note reappears after reload and is visible through CLI. An API health response alone proves only liveness, not database readiness.

## Tier 3 — desktop

1. Install Tauri 2 OS prerequisites and Rust.
2. In apps/web run bun run desktop:icons to generate icons.
3. With API running, bun run desktop:dev.
4. Verify app loads and writes a note via the Vite development origin.
5. Verify bundled production origin, API lifecycle, origin allowlist, signing and recovery separately.

Expected: development WebView shell works. A successful GUI launch does NOT establish deployable standalone desktop software.

## Tier 4 — optional experiments

- OpenTUI: install inside experiments/opentui, run Bun client against the API, create a record and confirm clean terminal teardown.
- Boa: inside experiments/boa run cargo test and cargo run; fixed ECMAScript executes without V8.
- RuVector: pg_available_extensions inspection, ABI match, CREATE EXTENSION, index construction, exact query correctness, rollback.
- Servo: feature parity, WebView embed integration, memory/startup measurements, native packaging, recovery.
- DomainForge: validate .sea and prove generated contracts semantically equivalent, including negative tests.

## Release gates

No release until: clean install with committed lockfile, CI green, migrations verified, security model written, trusted origins configured, integration proof, and rollback tested. Agentic workloads require separate SEA-Forge authorization and RealityTrace evidence if connected.
