# Validation and evidence

## Core baseline

- .github/workflows/ci.yml: TypeScript, Bun tests, React build, pg0 and Postgres.js persistence/rollback.
- .github/workflows/domainforge-validation.yml: real DomainForge .sea validation + JSON IR parse.
- .github/workflows/optional-validation.yml: Boa Rust execution, OpenTUI native PTY renderer, Tauri Linux installer/persistence.

## Standalone desktop acceptance

1. Stage pg0 v0.15.2 from official GitHub release, verify exact SHA-256.
2. Run cargo check and cargo test for Rust/SQLx desktop.
3. Build Linux .deb with pg0 bundled.
4. Install .deb and check pg0 executable exists inside installed package.
5. Run installed application executable with --desktop-smoke, using installed pg0, a disposable app-data directory and unique test instance. The test must write a note, stop database, restart, read note, and emit DESKTOP_PERSISTENCE_RESTART_OK.
6. The smoke must succeed with no Bun API or separately launched PostgreSQL server.

No pass should be claimed until the corresponding GitHub Actions run is completed/success.

## Still unvalidated

- Clicking and entering data through the installed GUI after reboot and system upgrades.
- Windows/macOS packaged app and device-level persistence.
- Failure injection, multiuser permissions, secret protection, process management on crashes, data backup and restore.
- RuVector native PG18 binary and index/query semantics inside pg0.
- DomainForge generation of Rust/TypeScript/Valibot from canonical .sea; no drift test yet.
- Servo browser-engine integration and cross-platform OpenTUI keyboard flows.

## Linux local reproduction

Install Bun, Rust/Cargo and Tauri 2 dependencies.

1. bun install
2. bun run --filter @template/web desktop:prepare
3. bun run --filter @template/web desktop:icons
4. bun run --filter @template/web desktop:build -- --bundles deb
5. Install the generated .deb, inspect packaged binaries, run binary --desktop-smoke with pg0 executable path and a fresh temp directory.

Smoke explicitly drops only a UUID-named disposable PostgreSQL instance; never test using application user data.
