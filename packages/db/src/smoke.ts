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
} finally {
  if (id) await db.deleteFrom("notes").where("id", "=", id).execute();
  await db.destroy();
}
