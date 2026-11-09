# Issue #15 slice 1: Windows validation handoff

Read this in a new Codex chat on Windows. This handoff branch contains only this
guide on top of the feature. Build/test `feat/limit-crossings`, not the handoff branch.

## Scope and repositories

- Upstream: https://github.com/chiarorosa/claude-codex-taskbar.
- Fork: https://github.com/gnevesx/claude-codex-taskbar.
- Issue: https://github.com/chiarorosa/claude-codex-taskbar/issues/15.
- Pablo assigned slice 1, accepted by gnevesx. Pure threshold logic and Config only.
- Feature branch: `feat/limit-crossings`, head `291aaf9` (base `9f49b33`, v0.3.0).
- Commits: `fd9a629` configuration; `d1c95c8` pure crossings and regression tests.
- Review fix: `291aaf9` tolerates estimated reset timestamp jitter.
- Existing PR: https://github.com/chiarorosa/claude-codex-taskbar/pull/73.
- Keep this guide outside the feature PR.

## Behavior under test

- Notifications default off; thresholds default to [80,95]. Config clamps to
  1..100, sorts/deduplicates; an empty list restores defaults. Old configs still load.
- `limits::notify::crossings` returns new caller-owned history plus typed events.
- Only ProviderStatus::Ok can update history or emit events. Other states preserve it.
- First reading seeds a quiet baseline; 70 -> 97 emits both 80 and 95 afterward.
- Each threshold emits once per window despite percentage dips or missing data.
- A known resets_at must advance by more than 60 seconds to rearm that window;
  ±1s jitter, backward shifts and absent metadata never rearm.
- Five-hour/weekly windows and caller-owned provider histories are independent.
- There is no new notification UI in this slice; balloon/Settings/DND are slice 2.

## Completed Linux checks

Actual-source portable tests: 43 passed, including all new Config/engine tests.
Full format, Clippy -D warnings, Windows GNU test compilation and release passed.
GNU release: 951,296 bytes. Eight existing live tests remain ignored; none added.
Native Windows/MSVC CI also passed for 291aaf9: 234 tests, zero failures, the same
eight live ignores; fmt/Clippy/release/size passed. MSVC release: 842,240 bytes.
https://github.com/chiarorosa/claude-codex-taskbar/actions/runs/42670947025.
Earlier manual Windows validation at d1c95c8 passed231 tests and release841216B;
that earlier result applies only to the old head. Current native evidence is CI.

## Get an isolated copy on Windows

Use PowerShell with Git, Rust stable (CI uses 1.99.1) and Visual Studio C++ Build
Tools/Windows SDK available. Use a fresh directory to preserve other local work:

```powershell
git clone --branch feat/limit-crossings https://github.com/gnevesx/claude-codex-taskbar.git cct-limit-crossings
cd cct-limit-crossings
git rev-parse HEAD
rustc -Vv
```

Confirm HEAD starts with 291aaf9 and Rust's host is x86_64-pc-windows-msvc.

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

Expected test inventory: 242 test functions, 234 passing and eight existing live
ignores (217 unit + 17 workflow tests). Record actual output rather than assuming it.
No new visual behavior needs a hover or popup check for this logic-only slice.

## Record and continue

Record exact HEAD, Windows build, Rust host/version, each command's result,
passed/ignored counts and native MSVC executable size. Investigate any failure;
preserve existing tests and assertions. Source changes must be committed and pushed
to the feature branch. Keep guide/process notes out of its diff.
Record any additional native checks on existing PR #73. Merge stays with Pablo.
The PR uses `Refs #15` because notification UI remains a separate slice.
