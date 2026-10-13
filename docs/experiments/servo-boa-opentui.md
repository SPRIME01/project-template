# Deferred native and terminal experiments

## Servo

Hypothesis: an embedded Rust-native rendering engine could reduce selected packaging or integration constraints. Servo itself uses SpiderMonkey and remains a major browser engine with extensive dependencies. It is not a small Tauri plug-in. Benchmark actual startup, RSS, component compatibility, cross-platform surface and developer burden before replacing a system WebView.

## Boa

Boa executes ECMAScript in Rust without V8. It is a JS engine, not an HTML/CSS/DOM implementation. A feature-gated Boa Rust capability can be prototyped for bounded pure-script execution, but never let untrusted scripts gain unrestricted filesystem or process access. Replacing Servo's SpiderMonkey with Boa would be a separate, high-cost browser-engine project.

## OpenTUI

Use an additional projection of the same core/API contract. Do not fork a second business-logic implementation into a terminal package. First implement list and create operations, then verify keyboard interaction, terminal resizing and Windows/macOS/Linux behavior. Keep a simple headless CLI until the richer terminal UI earns its cost.

## Decision criterion

Promote any candidate only when it demonstrates a useful new spendable capability or a measured decrease in burden while meeting correctness, portability, governance and recovery requirements. Avoid runtime dependencies from aspiration alone.
