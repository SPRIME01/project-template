# Optional OpenTUI client

Run from this directory after the main API is started:

~~~sh
bun install
bun run dev
~~~

This is intentionally not a root workspace dependency. It uses the API contract rather than duplicating SQL or domain logic. The first projection supports creating a title and reading saved notes. It is not a full reusable component library.

Acceptance: press Return to create, Ctrl+R to reload, Escape to exit and verify terminal restoration, then repeat on Windows Terminal, WSL and macOS.
