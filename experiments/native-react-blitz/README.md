# Native React: Boa + Blitz + Astryx

**Status: experimental native UI backend; basic React/Astryx execution and native interaction now verified in Linux headless CI.** The opt-in experiments do not change the default Tauri/Wry desktop renderer, pg0 packaging or API.

## Why these projects

- [jhead/bevy-react](https://github.com/jhead/bevy-react) (MIT) demonstrates React 19 in Boa, with React scheduling, a custom reconciler, native events, batched mutation messages and typed native calls. Bevy UI/ECS is only one host implementation.
- [DioxusLabs/blitz](https://github.com/DioxusLabs/blitz) already has `blitz-vibey-script` (Boa plus a DOM/CSS/native event shell). Its `examples/preact_script.rs` and TodoMVC example demonstrate Preact. Its `examples/preact/react_dom_apis.md` inventories React DOM requirements. Our pinned Linux headless smoke now independently proves basic React 19 and Astryx DOM mount, button click and controlled text input in Boa/Blitz. It **does not prove complete React DOM or Astryx compatibility**.
- [Astryx](https://github.com/facebook/astryx) requires React 19 and react-dom, supplies compiled CSS and theme variables, and may depend on DOM measurement, portals, focus and event behavior.

Therefore: try extending Blitz's existing DOM bindings to support standard React DOM *before* porting a custom reconciler. If that proves more expensive, reuse the reconciler/protocol/Boa scheduling work from bevy-react while swapping its Bevy host for a Blitz document host. In either route, the requirement is unmodified Astryx components, not a hand-drawn approximation.

## Try the independent baselines

From project-template root:

- `just native-render-preact`: pin and launch upstream Blitz/Boa with its known Preact example.
- `just native-render-astryx`: build the local React 19/Astryx sample with Vite, then attempt to launch its HTML in Blitz/Boa. This is a live Blitz/Boa native GUI probe; basic DOM interaction is independently proven by the headless regression tests, but CSS painting/window launch remain separate gates.

Requires Git, Cargo/Rust >= 1.91 and Blitz's OS GPU/windowing prerequisites; Astryx additionally needs Bun >= 1.4. The script downloads and executes upstream code. Inspect it first. Upstream checkout is pinned under ignored `.cache/`. Linux headless native runtime smoke has been executed in GitHub Actions; other platforms and graphical display are not yet verified.

## Executable native JS/DOM evidence

[CI](../../.github/workflows/native-react-blitz.yml) now independently bundles React 19/Astryx, then invokes Blitz's actual Boa-based ScriptDocument using a Rust headless harness. It checks JS exceptions, verifies that the application populated the DOM, and dispatches a real native click into React. This is not a screenshot/paint/layout or accessibility test.

The upstream Preact TodoMVC, minimal React counter and Astryx Button/Theme counter have now passed real Boa execution and click tests on Linux CI. They are hard gates. A **separate vanilla React DOM counter** distinguishes React browser-API failures from Astryx component or CSS failures. Native controlled-input behavior **also passed** for both plain React (with explicit `type="text"`) and unmodified Astryx `TextInput`, after resolving the Blitz layout tree. These are now required CI gates. Green CI proves these specific event/DOM paths, **not** full Astryx CSS/paint, general IME compositions, accessibility, dialogs or production readiness.

Locally, run `just native-render-headless-preact`, `just native-render-headless-react`, and `just native-render-headless-astryx`. Run `just native-render-headless-react-input` and `just native-render-headless-astryx-input` to exercise IME input. Exact dependency commits and JS errors appear in output. The headless harness can also render the full CSS/layout tree using **Vello CPU** into an evidence PNG, independently of GPU-backed window initialization. An independent **experimental native GUI job** also attempts to compile Blitz's real window host and capture a screenshot of the Astryx app on a software-rendered Xvfb display. This probes window/paint rather than just DOM mutation. It is intentionally non-blocking until platform prerequisites and visual output are validated. No renderer is promoted to production by this work.

## Port architecture (after profiling both strategies)

`React + Astryx -> react-dom or react-reconciler -> Boa JS host -> Blitz DOM -> Stylo CSS -> Taffy layout -> Vello paint -> Winit window`.

Cognate's Bun runtime and pg0 remain independent, using the already defined typed RuntimeService contract. Protect agent capabilities from untrusted renderer code with narrow authenticated IPC. Do not assume Blitz's Winit shell is directly substitutable for Tauri's WebView runtime; that requires a separate Tauri custom-runtime/packaging investigation.

If porting from bevy-react, consider these as donors: `packages/bevy-react/src/reconciler.ts`, `src/protocol.ts`, `plugin/src/react/shim.rs`, `plugin/src/js/*`. Replace `plugin/src/react/systems/render.rs` and Bevy-specific UI/widget/style plumbing with a Blitz `DocumentMutator` adapter. Preserve original MIT notices. Avoid vendoring all of Bevy.

## Acceptance gates

1. Preact control runs and reacts to mouse/keyboard in Boa/Blitz.
2. React 19 mounts, updates state from a click and typed input, then unmounts without exceptions.
3. Unmodified Astryx Theme, Card, Button and TextInput display the compiled CSS, theme variables and correct state.
4. Focus/keyboard, IME, selection, scroll, portal/dialog, SVG/icons, clipboard and accessible semantics work.
5. Live Cognate event streaming, continuation approval and restart recovery work via governed transport.
6. Linux, Windows and macOS installed packages are exercised; no Chromium/V8 or unadvertised browser sidecars.
7. Cold start, idle/peak RSS, app size and frame/input latency are measured against Tauri/Wry.
8. Security review covers imported scripts, native commands, file access, shutdown and job/microtask starvation.

Log exact commit hashes, environment, test receipts, screenshots, failures and benchmarks. No promotion to the default desktop route until gates pass.

Reference decision: [ADR 0003](../../docs/decisions/0003-boa-blitz-native-react.md).
