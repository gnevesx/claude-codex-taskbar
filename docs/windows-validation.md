# Native Windows validation for issue #13

Run this checklist on Windows 10/11 x64 before opening the contribution PR.
The change adds no animation or interaction flow; existing taskbar behavior remains.

## Setup

Use the MSVC toolchain and Visual Studio Build Tools with **Desktop development
with C++** and the Windows SDK, as described in [Contributing](../CONTRIBUTING.md).
The current CI pins Rust 1.99.1. From the repository root in PowerShell:

```powershell
rustup toolchain install 1.99.1-x86_64-pc-windows-msvc --component rustfmt,clippy
cargo +1.99.1-x86_64-pc-windows-msvc fmt --all -- --check
cargo +1.99.1-x86_64-pc-windows-msvc clippy --locked --all-targets -- -D warnings
cargo +1.99.1-x86_64-pc-windows-msvc test --locked
cargo +1.99.1-x86_64-pc-windows-msvc build --release --locked
```

Expected automated inventory: 136 tests in total, with the same six existing
live tests ignored by default. The 13 new tests cover model parsing, missing and
invalid values, real zero usage, clamping, partial responses, expiration, both
languages, stale/error states, and unchanged strip items.

## Live checks

The repository also has six opt-in tests for real endpoints, the installed CLIs,
hardware counters, and taskbar queries. On Windows with the corresponding
logged-in CLIs and hardware available:

```powershell
cargo +1.99.1-x86_64-pc-windows-msvc test --locked -- --ignored
```

Record any environmental prerequisite that is missing; preserve the tests.

## Visual check

Run `target\release\claude-codex-taskbar.exe` with the normal Windows taskbar.
Use the existing CLI login and the account's existing usage settings.

- Hover: overall 5-hour and weekly readings still appear as before.
- If the endpoint reports Opus/Sonnet, each reported model has its own percentage
  and a local weekday/time when its reset timestamp is available.
- Unreported models have no line. Real `0%` is still a valid reading.
- Extra usage is mentioned only when the endpoint reports it as enabled.
- The tooltip follows the Windows UI language. Unit tests cover English and pt-BR.
- Failed/loading states do not invent model readings; retained old data keeps
  its existing stale notice, and expired model windows disappear.
- The strip and Settings continue to work at the actual display scaling.

The fake model fixture covers populated-model cases even when the real account
has no per-model reading. Live account state determines which visual cases can
be observed; record unavailable cases honestly.

Capture the tooltip and record Windows version/build, display scaling, monitor
setup, taskbar alignment and auto-hide. These are the visual PR evidence requested
by the contribution guide. Keep the PR unopened until native validation is complete.
