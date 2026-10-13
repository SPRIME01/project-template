# Native and terminal experiments

## Servo

Hypothesis: an embedded Rust-native rendering engine could reduce selected packaging or integration constraints. Servo itself uses SpiderMonkey and remains a major browser engine with extensive dependencies. It is not a small Tauri plug-in. Benchmark actual startup, RSS, compatibility, cross-platform rendering and developer burden before replacing a system WebView. No Servo dependency is installed by default.

## Boa

The optional code at experiments/boa demonstrates ECMAScript evaluation using Rust-native boa_engine 0.22. Boa is not an HTML/CSS/DOM implementation. This fixed-program proof is not a sandbox for arbitrary code. A future feature-gated Boa native capability must have bounded execution and authority controls before any untrusted scripts are accepted.

## OpenTUI

experiments/opentui supplies a small client that uses the same Bun HTTP API. Its native Zig renderer is deliberately isolated from the root workspace so projects without terminal UI do not pay dependency cost. Validate terminal behavior and packaging before promoting into a default app.

## Decision criterion

Promote any candidate only when it demonstrates a newly spendable capability or measured payment reduction while meeting correctness, portability, governance and recovery requirements. Do not add runtime dependencies from aspiration alone.
