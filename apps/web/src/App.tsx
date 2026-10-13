import { useCallback, useEffect, useState, type FormEvent } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import type { Note } from "@template/contracts";

const endpoint = import.meta.env.VITE_API_URL ?? "http://127.0.0.1:3000";
const desktop = isTauri();

async function readNotes(): Promise<Note[]> {
  if (desktop) return invoke<Note[]>("list_notes");
  const response = await fetch(endpoint + "/api/notes");
  if (!response.ok) throw new Error("API returned HTTP " + response.status);
  return (await response.json()) as Note[];
}

async function createNote(title: string, body: string): Promise<Note> {
  if (desktop) return invoke<Note>("create_note", { input: { title, body } });
  const response = await fetch(endpoint + "/api/notes", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ title, body }),
  });
  if (!response.ok) throw new Error("Could not create note (HTTP " + response.status + ")");
  return (await response.json()) as Note;
}

export function App() {
  const [notes, setNotes] = useState<Note[]>([]);
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try { setNotes(await readNotes()); setError(null); }
    catch (err) { setError(err instanceof Error ? err.message : "Could not load notes"); }
  }, []);

  useEffect(() => { void refresh(); }, [refresh]);

  async function addNote(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setBusy(true);
    setError(null);
    try {
      await createNote(title, body);
      setTitle("");
      setBody("");
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Could not create note");
    } finally {
      setBusy(false);
    }
  }

  return (
    <main className="shell">
      <header className="hero">
        <span className="eyebrow">GODSPEED / REFERENCE PROJECT</span>
        <h1>Build on a <em>proven path.</em></h1>
        <p>A small inspectable application: typed intent, governed native execution and durable PostgreSQL settlement.</p>
        <div className="tags"><span>Bun + TypeScript (web)</span><span>Valibot</span><span>Kysely + Postgres.js (API)</span><span>Rust SQLx (desktop)</span><span>Bundled pg0</span></div>
      </header>
      <div className="grid">
        <section className="panel">
          <div className="panel-top"><span className="step">01 / CREATE</span><h2>New note</h2></div>
          <form onSubmit={(event) => void addNote(event)}>
            <label htmlFor="title">Title</label>
            <input id="title" maxLength={120} required value={title} onChange={(event) => setTitle(event.target.value)} placeholder="Name the next move" />
            <label htmlFor="body">Detail</label>
            <textarea id="body" maxLength={4000} value={body} onChange={(event) => setBody(event.target.value)} rows={5} placeholder="What will we test or deliver?" />
            <button type="submit" disabled={busy || !title.trim()}>{busy ? "Saving…" : "Save to PostgreSQL →"}</button>
          </form>
        </section>
        <section className="panel">
          <div className="panel-top line"><div><span className="step">02 / OBSERVE</span><h2>Saved notes</h2></div><button className="secondary" onClick={() => void refresh()}>Refresh</button></div>
          {error && <p className="error" role="alert">{String(error)}. {desktop ? "Check the bundled pg0 runtime and app logs." : "Check that pg0, the migration, and the API are running."}</p>}
          {notes.length === 0 && !error && <p className="empty">Nothing here yet. Create a note to verify an end-to-end database write.</p>}
          <div className="notes">
            {notes.map((note) => <article className="note" key={note.id}>
              <h3>{note.title}</h3><p>{note.body || "No detail supplied"}</p>
              <small>{new Date(note.createdAt).toLocaleString()}</small>
            </article>)}
          </div>
        </section>
      </div>
      <footer>Domain → Contract → Service → SQL → Evidence. {desktop ? "Desktop mode: no Bun API process required." : "Web mode: Bun API."}</footer>
    </main>
  );
}
