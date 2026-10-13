// A headless CLI projection of the API. The richer OpenTUI adapter remains optional.
// Explicit ESM marker is required for top-level await when TypeScript detects no imports.
export {};
const [command = "help", ...args] = process.argv.slice(2);
const base = process.env.API_URL ?? "http://127.0.0.1:3000";

if (command === "help") {
  console.log("Commands: health | list | create <title> [body]");
} else if (command === "health" || command === "list") {
  const response = await fetch(base + (command === "health" ? "/health" : "/api/notes"));
  if (!response.ok) throw new Error("HTTP " + response.status);
  console.log(JSON.stringify(await response.json(), null, 2));
} else if (command === "create") {
  const [title, body = ""] = args;
  if (!title) throw new Error("Title is required");
  const response = await fetch(base + "/api/notes", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ title, body }),
  });
  if (!response.ok) throw new Error("HTTP " + response.status);
  console.log(JSON.stringify(await response.json(), null, 2));
} else {
  console.error("Unknown command: " + command);
  process.exitCode = 2;
}
