import { Migrator } from "kysely";
import { createDb } from "./index";
import { initialMigration } from "./migration";

const db = createDb();
try {
  const migrator = new Migrator({
    db,
    provider: {
      async getMigrations() {
        return { "001_initial_notes": initialMigration };
      },
    },
  });
  const { error, results } = await migrator.migrateToLatest();
  for (const result of results ?? []) console.log(result.migrationName, result.status);
  if (error) throw error;
  console.log("Migration check complete");
} finally {
  await db.destroy();
}
