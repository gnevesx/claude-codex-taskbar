# Issue #15 slice 1: Windows validation handoff

Read this in a new Codex chat on Windows. This handoff branch contains only this
guide on top of the feature. Build/test `feat/limit-crossings`, not the handoff branch.

## Scope and repositories

- Upstream: https://github.com/chiarorosa/claude-codex-taskbar.
- Fork: https://github.com/gnevesx/claude-codex-taskbar.
- Issue: https://github.com/chiarorosa/claude-codex-taskbar/issues/15.
- Pablo assigned slice 1, accepted by gnevesx. Pure threshold logic and Config only.
- Feature branch: `feat/limit-crossings`, head `d1c95c8` (base `9f49b33`, v0.3.0).
- Commits: `fd9a629` configuration; `d1c95c8` pure crossings and regression tests.
- No upstream PR has been opened. Keep this guide outside any future PR.

## Behavior under test

- Notifications default off; thresholds default to [80,95]. Config clamps to
  1..100, sorts/deduplicates; an empty list restores defaults. Old configs still load.
- `limits::notify::crossings` returns new caller-owned history plus typed events.
- Only ProviderStatus::Ok can update history or emit events. Other states preserve it.
- First reading seeds a quiet baseline; 70 -> 97 emits both 80 and 95 afterward.
- Each threshold emits once per window despite percentage dips or missing data.
- A changed known resets_at rearms only that window; absent metadata never rearms.
- Five-hour/weekly windows and caller-owned provider histories are independent.
- There is no new notification UI in this slice; balloon/Settings/DND are slice 2.

## Completed Linux checks

Actual-source portable tests: 40 passed, including all new Config/engine tests.
Full format, Clippy -D warnings, Windows GNU test compilation and release passed.
GNU release: 951,296 bytes. Eight existing live tests remain ignored; none added.
This does not establish native Windows/MSVC success.
Fork Actions run 42641727703 could not start due to an account billing lock:
https://github.com/gnevesx/claude-codex-taskbar/actions/runs/42641727703.
No test steps ran there; do not report that run as passed or as a code regression.

## Get an isolated copy on Windows

Use PowerShell with Git, Rust stable (CI uses 1.99.1) and Visual Studio C++ Build
Tools/Windows SDK available. Use a fresh directory to preserve other local work:

```powershell
git clone --branch feat/limit-crossings https://github.com/gnevesx/claude-codex-taskbar.git cct-limit-crossings
cd cct-limit-crossings
git rev-parse HEAD
rustc -Vv
```

Confirm HEAD starts with d1c95c8 and Rust's host is x86_64-pc-windows-msvc.

## Run the native checks

```powershell
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
$size = (Get-Item target/release/claude-codex-taskbar.exe).Length
Write-Output "Release size: $size bytes"
if ($size -gt 1000000) { throw "Release exceeds the 1 MB budget" }
git status --short
```

Expected test inventory: 239 test functions, 231 passing and eight existing live
ignores (214 unit + 17 workflow tests). Record actual output rather than assuming it.
No new visual behavior needs a hover or popup check for this logic-only slice.

## Record and continue

Record exact HEAD, Windows build, Rust host/version, each command's result,
passed/ignored counts and native MSVC executable size. Investigate any failure;
preserve existing tests and assertions. Source changes must be committed and pushed
to the feature branch. Keep guide/process notes out of its diff.
After native checks pass, report to Guilherme before opening an upstream PR.
A future PR references `Refs #15` because notification UI remains a separate slice.
