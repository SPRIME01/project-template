import { Kysely } from "kysely";
import { PostgresJSDialect } from "kysely-postgres-js";
import postgres from "postgres";
import type { Note } from "@template/contracts";
import type { NoteRepository } from "@template/core";

export interface Database {
  notes: {
    id: string;
    title: string;
    body: string;
    status: "open" | "done";
    created_at: Date;
  };
}

export function createDb(url = process.env.DATABASE_URL): Kysely<Database> {
  if (!url) throw new Error("DATABASE_URL must be set");
  const client = postgres(url, {
    max: 5,
    prepare: true,
    connect_timeout: 5,
  });
  return new Kysely<Database>({
    dialect: new PostgresJSDialect({
      postgres: client,
    }),
  });
}

export class PostgresNoteRepository implements NoteRepository {
  constructor(private readonly db: Kysely<Database>) {}

  async list(): Promise<Note[]> {
    const rows = await this.db.selectFrom("notes").selectAll()
      .orderBy("created_at", "desc").execute();
    return rows.map((row) => ({
      id: row.id,
      title: row.title,
      body: row.body,
      status: row.status,
      createdAt: new Date(row.created_at).toISOString(),
    }));
  }

  async insert(note: Note): Promise<void> {
    await this.db.insertInto("notes").values({
      id: note.id,
      title: note.title,
      body: note.body,
      status: note.status,
      created_at: new Date(note.createdAt),
    }).execute();
  }
}
