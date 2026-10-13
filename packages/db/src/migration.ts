import type { Migration } from "kysely";

// Migration history is managed by Kysely. Do not silently mutate old migrations
// after databases have applied them; add a new migration instead.
export const initialMigration: Migration = {
  async up(db) {
    await db.schema.createTable("notes")
      .addColumn("id", "uuid", (col) => col.primaryKey())
      .addColumn("title", "varchar(120)", (col) => col.notNull())
      .addColumn("body", "text", (col) => col.notNull())
      .addColumn("status", "varchar(16)", (col) => col.notNull())
      .addColumn("created_at", "timestamptz", (col) => col.notNull())
      .execute();
  },
  async down(db) {
    await db.schema.dropTable("notes").execute();
  },
};
