import {
  BoxRenderable, InputRenderable, InputRenderableEvents,
  TextRenderable, createCliRenderer,
} from "@opentui/core";

// Intentionally external to root Bun workspaces: OpenTUI native packages
// are only installed by teams that choose a TUI implementation.
const api = process.env.API_URL ?? "http://127.0.0.1:3000";
const renderer = await createCliRenderer({ exitOnCtrlC: true, backgroundColor: "#0c121d" });

const panel = new BoxRenderable(renderer, {
  width: 76,
  height: 21,
  flexDirection: "column",
  padding: 1,
  gap: 1,
  backgroundColor: "#121c2b",
});
const title = new TextRenderable(renderer, {
  content: "GODSPEED / NOTES  —  OpenTUI optional reference",
  fg: "#86e2b3",
});
const legend = new TextRenderable(renderer, {
  content: "Enter a title and press Return to save | Ctrl+R reload | Esc quit",
  fg: "#b2c5dc",
});
const input = new InputRenderable(renderer, {
  id: "note-title",
  width: 65,
  maxLength: 120,
  placeholder: "New note title",
  backgroundColor: "#26354a",
  focusedBackgroundColor: "#314660",
});
const status = new TextRenderable(renderer, {
  content: "Connecting to the Bun API...",
  fg: "#e1ebf4",
});
const list = new TextRenderable(renderer, {
  content: "No records loaded",
  fg: "#c5d2df",
  wrapMode: "word",
  width: 70,
});

panel.add(title);
panel.add(legend);
panel.add(input);
panel.add(status);
panel.add(list);
renderer.root.add(panel);
input.focus();

async function refresh() {
  try {
    const response = await fetch(api + "/api/notes");
    if (!response.ok) throw new Error("HTTP " + response.status);
    const rows = (await response.json()) as Array<{ title: string; createdAt: string }>;
    status.content = "Connected — " + rows.length + " notes";
    list.content = rows.slice(0, 10).map((row, n) => (n + 1) + ". " + row.title).join("\n") || "No notes yet";
  } catch (error) {
    status.content = "API unavailable: " + String(error);
  }
}

input.on(InputRenderableEvents.ENTER, (value: string) => {
  const text = value.trim();
  if (!text) return;
  void (async () => {
    try {
      const response = await fetch(api + "/api/notes", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title: text, body: "" }),
      });
      if (!response.ok) throw new Error("HTTP " + response.status);
      input.value = "";
      await refresh();
    } catch (error) {
      status.content = "Save failed: " + String(error);
    }
  })();
});

const onKey = (key: { name: string; ctrl?: boolean }) => {
  if (key.name === "escape") renderer.destroy();
  else if (key.ctrl && key.name === "r") void refresh();
};
renderer.keyInput.on("keypress", onKey);
renderer.once("destroy", () => renderer.keyInput.off("keypress", onKey));

void refresh();
