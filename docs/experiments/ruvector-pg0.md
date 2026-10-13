# RuVector PostgreSQL extension: genuine, integration outstanding

RuVector **does have a PostgreSQL extension**. Upstream source is https://github.com/ruvnet/RuVector/tree/main/crates/ruvector-postgres. It documents CREATE EXTENSION ruvector, the ruvector dimensional type, and the ruhnsw access method.

Our pg0 v0.15.2 bundle runs PostgreSQL 18, containing pgvector 0.8.5. The earlier CI query confirmed no installed ruvector extension.

The engineering gap is extension ABI and distribution, not the existence of the extension. RuVector upstream's older documented pgrx 0.12.9 build targets PG16 and its extension CI has checked PG17. Newer pgrx >=0.16 adds PG18 support, but migrating the extension and passing its tests against pg0's particular PostgreSQL build requires evidence.

## Acceptance protocol

1. Pin RuVector commit, pg0 tag, and PostgreSQL 18 server build.
2. Confirm pg18 support in extension Cargo features; update pgrx bindings if required (fork only when necessary).
3. Extract bundled pg0 on disposable instance, locate pg_config, include/library and extension target paths.
4. Build extension library/control/SQL files for exact PostgreSQL ABI on Linux, Windows and macOS.
5. CREATE EXTENSION ruvector; create ruvector(3) column, build ruhnsw index, insert test vectors and prove nearest neighbors.
6. Restart PostgreSQL and repeat query; run dump/restore and extension upgrade/rollback.
7. Package vetted extension files in Tauri build; **never** install native shared libraries dynamically from arbitrary URLs on desktop launch.
8. Distinguish an optional RuVector gate from default PostgreSQL availability; do not silently use pgvector as RuVector.

Fail closed on PostgreSQL-ABI mismatch or data-loss risk.
