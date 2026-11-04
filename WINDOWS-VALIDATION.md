# Reset countdown: Windows validation handoff

This guide is published only on `handoff/reset-countdown-windows` for access
from Windows. Do not merge this guide branch or include this file in the feature PR.

## Repositories and scope

- Upstream: https://github.com/chiarorosa/claude-codex-taskbar
- Fork: https://github.com/gnevesx/claude-codex-taskbar
- Implementation branch: `feat/reset-countdown`
- Expected implementation commit: `d3373ef021c4c027b54d7066ccb480b1e52d178f`
- Existing PR: https://github.com/chiarorosa/claude-codex-taskbar/pull/30
- Issue: https://github.com/chiarorosa/claude-codex-taskbar/issues/14
- Maintainer assignment: https://github.com/chiarorosa/claude-codex-taskbar/issues/14#issuecomment-6631692866

Only slice 1, the reset countdown, is implemented. Pace, projections and bar
markers remain out of scope. The future PR must use `Refs #14`, not `Closes #14`.

## Instructions for the Windows coding agent

Validate the implementation on native Windows using the existing checkout and
the project's MSVC setup in CONTRIBUTING.md. Preserve any local work before
switching branches. Work on `feat/reset-countdown`, not this guide branch.
Read AGENTS.md and follow the existing code style. Do not remove, ignore or weaken
tests. Push any necessary fixes to the user's fork, updating the existing PR #30.
Do not open another PR or merge it; the maintainer handles acceptance. Do not include
this guide in the contribution; keep validation results separate from product code.

## Update the checkout

Run in the existing project's folder:

```powershell
git fetch origin
git switch feat/reset-countdown
git pull --ff-only origin feat/reset-countdown
git rev-parse HEAD
```

Confirm the expected implementation commit above, or account explicitly for any
later fixes. If the repository is not cloned yet, clone the user's fork first.

## Required checks

```powershell
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
(Get-Item target\release\claude-codex-taskbar.exe).Length
```

Record actual output, test counts and release size. The binary budget is
1,000,000 bytes. There are 171 source test functions, including the same six
existing live tests ignored by default; this is not a claim they all executed.

## Actual taskbar hover inspection

- Record Windows build, UI language, display scaling and taskbar alignment.
- Keep percentages and absolute local reset times; append the countdown to
  known future reset times for Claude/Codex, 5h/weekly and model-specific windows.
- Expected text: `resets at 14:20 (in 1h 12m)` / `reseta às 14:20 (em 1h 12min)`.
- Durations: EN `45m`, `1h 12m`, `2d 3h`; pt-BR `45min`, `1h 12min`, `2d 3h`.
  Omit a zero second unit: `1h`, `7d`; do not repeat the reset verb.
- Positive seconds round up to minutes; durations of at least 24 rounded hours
  use days/hours and omit residual minutes.
- Missing, elapsed and unrepresentable reset intervals produce no countdown.
- Reopen the hover after a minute and verify the remaining time changes with
  display time rather than the fetch timestamp. No new timer was introduced.
- Confirm actual popup wrapping and readability; capture the native popup and
  record the display scaling. Keep weekday formatting for weekly absolute times.
- Preserve error/loading behavior, genuine zero/100 percent readings and the
  original timestamped stale/session notices. Unit tests cover fixture cases;
  distinguish those from observations obtained using a real account.

## Evidence already collected on Linux

- 35 actual-source view/config/model/i18n tests passed with a documented fake
  Windows local-time boundary; nine tests were added without removing old tests.
- Windows GNU formatting, all-target Clippy, test-executable compilation and
  release build passed; that release executable is 913,408 bytes.
- Independent local TLC review passed 7/7 acceptance criteria and detected all
  seven deliberately injected behavior faults in isolated scratch copies.
- These Linux checks do not establish native Windows execution or actual hover.

## Current native Windows CI

The review fix d3373ef passed the PR's Windows/MSVC CI: 165 tests (148 + 17),
the same six existing ignores, formatting, Clippy and release all passed.
MSVC executable: 803,840 bytes, below 1,000,000.
Run: https://github.com/chiarorosa/claude-codex-taskbar/actions/runs/41852124622.
Actual hover was validated on prior head 5c8bef5; no new interactive hover result
is claimed for d3373ef. Recheck its shorter parenthetical text when inspecting it.

