import { NoteService } from "@template/core";
import { createDb, PostgresNoteRepository } from "./index";

const db = createDb();
const service = new NoteService(new PostgresNoteRepository(db));
let id: string | undefined;
try {
  const note = await service.create({ title: "SQL smoke test", body: "Postgres.js via Kysely" });
  id = note.id;
  const found = (await service.list()).find((n) => n.id === note.id);
  if (!found || found.title !== note.title) throw new Error("SQL roundtrip failed");
  console.log("PASS: Postgres.js -> PostgreSQL insert and select");

  const rolledBackId = crypto.randomUUID();
  const rollback = new Error("intentional rollback");
  try {
    await db.transaction().execute(async (trx) => {
      await trx.insertInto("notes").values({
        id: rolledBackId, title: "must not persist", body: "",
        status: "open", created_at: new Date(),
      }).execute();
      throw rollback;
    });
    throw new Error("Expected transaction to abort");
  } catch (error) {
    if (error !== rollback) throw error;
  }
  const persisted = await db.selectFrom("notes")
    .select("id").where("id", "=", rolledBackId).executeTakeFirst();
  if (persisted) throw new Error("Rollback did not preserve transactional isolation");
  console.log("PASS: transaction rollback");
} finally {
  if (id) await db.deleteFrom("notes").where("id", "=", id).execute();
  await db.destroy();
}
