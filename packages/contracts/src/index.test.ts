import { expect, test } from "bun:test";
import { checkCreateNote, CreateNoteSchema, validateStandard } from "./index";

test("trims titles and rejects blank titles", () => {
  const result = checkCreateNote({ title: "  Hello ", body: "" });
  expect(result.success).toBe(true);
  if (result.success) expect(result.output.title).toBe("Hello");
  expect(checkCreateNote({ title: "   ", body: "" }).success).toBe(false);
});

test("offers the Standard Schema interface", () => {
  expect("~standard" in CreateNoteSchema).toBe(true);
});

test("validates through Standard Schema without Valibot-specific consumer calls", async () => {
  const note = await validateStandard(CreateNoteSchema, { title: "  test ", body: "" });
  expect(note.title).toBe("test");
  await expect(validateStandard(CreateNoteSchema, { title: "", body: "" })).rejects.toThrow();
});
