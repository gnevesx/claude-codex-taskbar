# Checks completed on Linux

Date: 2026-11-03. Native Windows validation is pending by explicit user request.
No PR may be opened before that validation.

- Rust 1.99.1 matches upstream CI.
- cargo fmt --all -- --check: PASS.
- cargo clippy --locked --all-targets --target x86_64-pc-windows-gnu -- -D warnings: PASS.
- cargo test --locked --target x86_64-pc-windows-gnu --no-run: PASS (all executables).
- cargo build --release --locked --target x86_64-pc-windows-gnu: PASS.
- Release executable: 951296 bytes.
- Offline exact-source parser/state harness: 29 PASS, 1 existing live test ignored.
- Independent isolated behavior mutations: 5/5 killed, real source unchanged.
- Full Windows GNU suite execution: NOT RUN successfully; Wine exited53 in bootstrap.
- Native Windows/MSVC full suite, live tests, and visual/DPI validation: PENDING.
- Baseline inventory123tests/6ignored; current136tests/same6ignored.

The offline harness copies pure timestamp parsing and replaces external HTTP with
a panic fake. It does not execute view/Codex or native Windows APIs. Its result
is deliberately not presented as the full project test suite.

Fork verified at https://github.com/gnevesx/claude-codex-taskbar.
Source checkpoints are published to feat/claude-model-weekly-limits.
Native acceptance is pending by the user's explicit instruction.
No PR has been opened. Internal TLC notes belong only in this work branch.
