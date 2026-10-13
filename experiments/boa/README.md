# Optional Rust-native JavaScript engine proof

From this folder:

~~~sh
cargo run
cargo test
~~~

This uses Boa only and carries no browser/DOM code, Tauri integration or claim of a secure untrusted-script sandbox. Keep it as an independent experiment until an application capability requires controlled JavaScript evaluation.

Measured comparison must address V8/JavaScriptCore/Boa correctness, startup, memory, security, execution limits and script compatibility. Adding Boa to Servo in place of SpiderMonkey is not part of this experiment.
