import type { CreateNote, Note } from "@template/contracts";

export interface NoteRepository {
  list(): Promise<Note[]>;
  insert(note: Note): Promise<void>;
}

export class NoteService {
  constructor(private readonly repository: NoteRepository) {}

  list(): Promise<Note[]> {
    return this.repository.list();
  }

  async create(input: CreateNote): Promise<Note> {
    const note: Note = {
      id: crypto.randomUUID(),
      title: input.title,
      body: input.body,
      status: "open",
      createdAt: new Date().toISOString(),
    };
    await this.repository.insert(note);
    return note;
  }
}
