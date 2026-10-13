import * as v from "valibot";

export const CreateNoteSchema = v.object({
  title: v.pipe(v.string(), v.trim(), v.minLength(1), v.maxLength(120)),
  body: v.pipe(v.string(), v.maxLength(4000)),
});

export const NoteSchema = v.object({
  id: v.pipe(v.string(), v.uuid()),
  title: v.string(),
  body: v.string(),
  status: v.picklist(["open", "done"]),
  createdAt: v.string(),
});

export type CreateNote = v.InferOutput<typeof CreateNoteSchema>;
export type Note = v.InferOutput<typeof NoteSchema>;

// Valibot implements the Standard Schema V1 contract through the
// ~standard property on schemas. Consumers should target that interface,
// rather than depend on Valibot-specific internals.
export function parseCreateNote(value: unknown): CreateNote {
  return v.parse(CreateNoteSchema, value);
}

export function checkCreateNote(value: unknown) {
  return v.safeParse(CreateNoteSchema, value);
}
