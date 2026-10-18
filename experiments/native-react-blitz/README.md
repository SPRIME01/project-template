# Native React: Boa + Blitz + Astryx

**Status: experimental; no React DOM/Astryx pass demonstrated.** This adds opt-in probes to project-template without changing its default Tauri/Wry desktop renderer, pg0 packaging or API.

## Why these projects

- [jhead/bevy-react](https://github.com/jhead/bevy-react) (MIT) demonstrates React 19 in Boa, with React scheduling, a custom reconciler, native events, batched mutation messages and typed native calls. Bevy UI/ECS is only one host implementation.
- [DioxusLabs/blitz](https://github.com/DioxusLabs/blitz) already has `blitz-vibey-script` (Boa plus a DOM/CSS/native event shell). Its `examples/preact_script.rs` and TodoMVC example demonstrate Preact. Its `examples/preact/react_dom_apis.md` inventories React DOM requirements, but **does not prove React DOM compatibility**.
- [Astryx](https://github.com/facebook/astryx) requires React 19 and react-dom, supplies compiled CSS and theme variables, and may depend on DOM measurement, portals, focus and event behavior.

Therefore: try extending Blitz's existing DOM bindings to support standard React DOM *before* porting a custom reconciler. If that proves more expensive, reuse the reconciler/protocol/Boa scheduling work from bevy-react while swapping its Bevy host for a Blitz document host. In either route, the requirement is unmodified Astryx components, not a hand-drawn approximation.

## Try the independent baselines

From project-template root:

- `just native-render-preact`: pin and launch upstream Blitz/Boa with its known Preact example.
- `just native-render-astryx`: build the local React 19/Astryx sample with Vite, then attempt to launch its HTML in Blitz/Boa. This is a **discovery probe**, expected to reveal compatibility failures. A browser build or Preact pass is *not* proof of React/Astryx support.

Requires Git, Cargo/Rust >= 1.91 and Blitz's OS GPU/windowing prerequisites; Astryx additionally needs Bun >= 1.4. The script downloads and executes upstream code. Inspect it first. Upstream checkout is pinned under ignored `.cache/`. No platform smoke has yet been executed for this experiment.

## Executable native JS/DOM evidence

[CI](../../.github/workflows/native-react-blitz.yml) now independently bundles React 19/Astryx, then invokes Blitz's actual Boa-based ScriptDocument using a Rust headless harness. It checks JS exceptions, verifies that the application populated the DOM, and dispatches a real native click into React. This is not a screenshot/paint/layout or accessibility test.

The upstream Preact TodoMVC is the hard-gated control. React/Astryx is deliberately **non-blocking and recorded as an explicit exploratory failure** until it works. Green CI by itself is therefore not evidence that React/Astryx works in Boa.

Locally, run `just native-render-headless-preact` and `just native-render-headless-astryx`. Exact dependency commits and JS errors appear in output. No renderer is promoted to production by this work.

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
