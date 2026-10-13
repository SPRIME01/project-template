# Validation and evidence

## Acceptance hierarchy

- Core: .github/workflows/ci.yml checks TypeScript, unit tests, web build, pg0 migrations and PostgreSQL roundtrip.
- Optional adapters: .github/workflows/optional-validation.yml checks Boa, OpenTUI under a real pseudo-terminal, and Tauri compile plus Linux .deb bundle.
- DomainForge: .github/workflows/domainforge-validation.yml compiles and parses the example .sea. Passing grammar/IR checks does not yet prove TypeScript contracts are generated or semantically identical.
- RuVector in pg0: pg0 includes pgvector, not RuVector. Extension availability is observed by bun run db:ruvector. This is not a verified RuVector integration.
- Servo: research-only. No executable or Tauri WebView replacement is integrated; do not claim validation.

Check each workflow's actual status before saying it passed.

## Manual exercises

bun install
bun run check
pg0 start --name project-template --port 5432
bun run db:migrate
bun run db:smoke
bun run db:ruvector

OpenTUI: cd experiments/opentui && bun install && bun run typecheck; in a PTY run bun run probe, then bun run dev while the API runs. Verify input and terminal restoration on each target OS.

Boa: cargo test --manifest-path experiments/boa/Cargo.toml and cargo run --manifest-path experiments/boa/Cargo.toml.

Tauri: generate app icons; cargo check --manifest-path apps/web/src-tauri/Cargo.toml; bundle Linux .deb. A successful bundle is not a functional desktop release until local API lifecycle, origin/permissions, installation, launch, restart and recovery are verified.

RuVector: verify PostgreSQL 18 ABI compatibility, build/install extension into pg0 on a disposable database, CREATE EXTENSION, index/query roundtrip, recovery and safe uninstall. Do not report it as installed just because pgvector is available.

Servo: measured embed, web compatibility, memory/startup data, multi-OS distribution and failure recovery. Not implemented.

DomainForge: validate .sea; inspect resulting IR; to close the projection gate also regenerate TS schemas, policy and tests from the same IR and compare semantics.

## Production gates

No release until reproducible lockfiles, CI evidence, governed auth and authorization, platform lifecycle, secure permissions and trusted origins, signing, backup/rollback and recovery are proven. Experimental adapters not selected by a project do not block the default reference.
