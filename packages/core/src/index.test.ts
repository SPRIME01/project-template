import { expect, test } from "bun:test";
import type { Note } from "@template/contracts";
import { NoteService, type NoteRepository } from "./index";

test("creates and retrieves a note through the domain port", async () => {
  const stored: Note[] = [];
  const repo: NoteRepository = {
    list: async () => [...stored],
    insert: async (note) => { stored.push(note); },
  };
  const service = new NoteService(repo);
  const note = await service.create({ title: "First", body: "Testing" });
  expect(note.status).toBe("open");
  expect((await service.list())[0]?.id).toBe(note.id);
});
