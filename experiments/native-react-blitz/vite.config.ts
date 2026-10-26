import { defineConfig } from "vite";
import { fileURLToPath } from "node:url";
export default defineConfig({
  base: "./",
  build: {
    outDir: "dist",
    modulePreload: { polyfill: false },
    rollupOptions: {
      input: {
        astryx: fileURLToPath(new URL("./index.html", import.meta.url)),
        react: fileURLToPath(new URL("./basic.html", import.meta.url)),
        stress: fileURLToPath(new URL("./stress.html", import.meta.url)),
        stressMemo: fileURLToPath(new URL("./stress-memo.html", import.meta.url)),
      },
    },
  },
});
