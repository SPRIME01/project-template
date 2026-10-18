# ADR 0003: optional Boa + Blitz desktop rendering experiment

Date: 2026-10-17
Status: **proposed; not a shipping backend**

We retain the existing Tauri 2/Wry UI and Rust SQLx/pg0 desktop reference. Bun remains the Cognate execution runtime. In a separate experiment we test React 19/Astryx without V8, Chromium, or any full browser engine.

Blitz already implements Boa-driven DOM script execution; begin by measuring the missing React DOM surface and extending it. Use jhead/bevy-react as the implementation reference for React's reconciler, JS scheduler shims, event priorities and compact mutation protocol. If a custom host wins, map operations to Blitz DocumentMutator and preserve an adapter boundary rather than importing Bevy ECS.

A React component executing in Boa is **not** proof that Astryx works. Astryx currently relies on react-dom and compiled CSS. Portal/selection, event delegation, accessibility and CSS must be verified, not substituted with placeholders. A Blitz window isn't automatically a Tauri WebView; desktop integration and cross-platform packaging remain separate gates.

Success: all checks in experiments/native-react-blitz/README.md pass and are recorded. Failure or regression: retain the existing Wry desktop route. No production runtime dependencies, no new global process privileges, no changes to the default Tauri commands are authorized by this ADR.
