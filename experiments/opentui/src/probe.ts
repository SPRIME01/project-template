import { TextRenderable, createCliRenderer } from "@opentui/core";

// Requires a PTY (see CI). Verifies renderer initialization, native draw,
// teardown and process restoration. Does not replace interactive usability tests.
const renderer = await createCliRenderer({
  exitOnCtrlC: false,
  backgroundColor: "#0c121d",
  targetFps: 15,
});
try {
  renderer.root.add(new TextRenderable(renderer, {
    id: "smoke",
    content: "OpenTUI native renderer smoke",
    fg: "#86e2b3",
  }));
  await Bun.sleep(100);
} finally {
  renderer.destroy();
}
console.log("OPENTUI_RENDERER_OK");
