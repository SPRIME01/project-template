import { checkCreateNote } from "@template/contracts";
import { NoteService } from "@template/core";
import { createDb, PostgresNoteRepository } from "@template/db";

const db = createDb();
const service = new NoteService(new PostgresNoteRepository(db));
const port = Number(process.env.API_PORT ?? "3000");
const permittedOrigin = process.env.WEB_ORIGIN ?? "http://127.0.0.1:5173";

function cors(origin: string | null): HeadersInit {
  if (origin !== permittedOrigin) return {};
  return {
    "Access-Control-Allow-Origin": permittedOrigin,
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
    Vary: "Origin",
  };
}

function json(data: unknown, status = 200, headers: HeadersInit = {}): Response {
  return Response.json(data, { status, headers });
}

Bun.serve({
  hostname: "127.0.0.1",
  port,
  async fetch(request) {
    const origin = request.headers.get("Origin");
    const headers = cors(origin);
    if (request.method === "OPTIONS") {
      return new Response(null, { status: origin === permittedOrigin ? 204 : 403, headers });
    }
    const path = new URL(request.url).pathname;
    if (path === "/health" && request.method === "GET")
      return json({ status: "up" }, 200, headers);
    if (path !== "/api/notes") return json({ error: "Not found" }, 404, headers);
    try {
      if (request.method === "GET")
        return json(await service.list(), 200, headers);
      if (request.method === "POST") {
        let payload: unknown;
        try { payload = await request.json(); }
        catch { return json({ error: "Invalid JSON" }, 400, headers); }
        const result = checkCreateNote(payload);
        if (!result.success)
          return json({ error: "Validation failed", issues: result.issues }, 422, headers);
        return json(await service.create(result.output), 201, headers);
      }
      return json({ error: "Method not allowed" }, 405, { ...headers, Allow: "GET, POST, OPTIONS" });
    } catch (error) {
      console.error("Request failed", error);
      return json({ error: "Internal server error" }, 500, headers);
    }
  },
});

console.log("API listening at http://127.0.0.1:" + port);
