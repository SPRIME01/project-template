import { sql } from "kysely";
import { createDb } from "./index";

// Probe only. pg0 bundles pgvector, not RuVector. This does NOT install
// an extension or imply its ABI is compatible with the bundled PostgreSQL.
const db = createDb();
try {
  const result = await sql<{name: string; default_version: string | null; installed_version: string | null}>`select name, default_version, installed_version
  from pg_available_extensions
  where name in ('ruvector', 'vector')
  order by name`.execute(db);
  console.table(result.rows);
  if (!result.rows.some((row) => row.name === "ruvector")) {
    console.log("RuVector is unavailable in this pg0 PostgreSQL installation.");
    console.log("See docs/experiments/ruvector-pg0.md before attempting to install it.");
  }
} finally {
  await db.destroy();
}
