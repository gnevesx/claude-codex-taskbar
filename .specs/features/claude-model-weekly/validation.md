# Claude model weekly usage — partial independent validation

**Result: FAIL** — complete native test execution and Windows visual QA pending.
**Date:** 2026-11-03. **Verifier:** independent reviewer, not implementation author.
**Spec:** `.specs/features/claude-model-weekly/spec.md`.
**Diff range:** upstream `f5636ec7d9e5dfcaef6fac63ae83655c43184b56` through the prepared feature draft,
including new fixture, view test module, and user documentation.
This is a progress report, not final completion. No tasks are marked done here.

## Source result and corrected gap

**Source/style review: PASS.** Existing provider/parser/state/view boundaries,
optional readings, i18n, credential restrictions, and dependency footprint hold.
The extra view test module respects the user's 500-line limit without moving old
tests. No remaining implementation blocker was found by source review.

**AC gap resolved:** the original response-validity guard ignored new model/extra
readings and rejected model-only or enabled-extra-only responses. It now includes
both (`src/limits/claude.rs:72`). Regression assertions at `:304` and `:321` pass;
restoring the old guard fails both new regression tests. This was an independently
identified source-review defect, corrected before this report.

## Acceptance evidence

PASS below means executed in the isolated parser/state harness; PENDING means the
assertions were source-reviewed and cross-compiled but have not run successfully.

| AC | Spec-defined outcome | File:line and assertion | Status |
| --- | --- | --- | --- |
| 1 | Preserve Opus/Sonnet identity, resets, clamp to 0–100 | `src/limits/claude.rs:221` `assert_eq!(u.weekly_by_model, vec![...Opus 72.0/reset 1793948399, Sonnet 0.0/None])`; `:266` expects 100.0 and 0.0 | PASS |
| 2 | Missing/null/non-numeric omitted; real zero retained | `src/limits/claude.rs:253` `assert_eq!(u.weekly_by_model, vec![])`; `:234` expects Sonnet 0.0 in complete vector assertion | PASS |
| 3 | Enabled line only for boolean true, localized | `src/limits/claude.rs:296` asserts flag false for null/missing/false/string; `:321` asserts true; `src/ui/view/claude_tests.rs:60` asserts localized extra line and `:73` its absence | Parser PASS; view PENDING |
| 4 | Only reported model lines; both languages; optional reset | `src/ui/view/claude_tests.rs:55` asserts exact Opus 72%/localized reset line; `:59` Sonnet 0% without reset; `:71` absent Opus; loop `:43` tests En/PtBr | PENDING |
| 5 | Drop reset model independently, retain active data/flag | `src/limits/state.rs:271` expects only Sonnet 0.0/None at Opus reset; `:281` asserts extra enabled; `:282` expects overall 92.0 | PASS |
| 6 | Stale notice retained; failed/loading/disabled invent no data | `src/limits/state.rs:109` asserts exact Stale status; `src/ui/view/claude_tests.rs:94` and `:96` assert model/stale text; `:107`–`:109` assert details absent in error/loading/disabled | State PASS; view PENDING |
| 7 | Codex and existing strip unchanged | `src/limits/codex.rs:260`, `:267` expect original 12%/40% and resets; `src/ui/view/claude_tests.rs:131` expects only ClaudeFive 18% and ClaudeWeek 3% | PENDING |

No spec-precision gaps found. Missing full execution evidence prevents feature PASS.

## Offline execution and discrimination sensor

Control: **29 passed, 0 failed, 1 ignored** (existing live Claude endpoint test).
Exact Claude/state modules, unchanged tests/fixtures, verbatim shared types and
pure time helpers; HTTP is a panic fake. View/Codex/native UI are outside harness.

| Scratch fault | Relevant original assertion | Result |
| --- | --- | --- |
| Omit parsed models (`src/limits/claude.rs:69`) | Model identity/clamp/model-only cases | Killed, 3 failures |
| Fabricate absent windows as 0% (`src/limits/claude.rs:60`) | Missing/null/invalid case plus regressions | Killed, 4 failures |
| Accept any non-null extra flag (`src/limits/claude.rs:70`) | Explicit boolean case | Killed, 2 failures |
| Preserve expired model (`src/limits/state.rs:46`) | Independent model reset case | Killed, 1 failure |
| Restore old guard (`src/limits/claude.rs:72`) | Model-only and extra-only regressions | Killed, 2 failures |

**5 injected, 5 compiled and killed, 0 survived.** Tests remained byte-identical.
Pre/post-sensor porcelain, tracked diff hash, and tracked/untracked content hashes
matched. Logs/results: `/home/gnevesx/code/cct-contribution-notes/mutation-sensor/`.
Sensor covered the passing offline subset, not the still-pending full native gate.

## Build and test integrity

Cross-checks below were observed by the orchestrator; reviewer did not rerun them.
Rust 1.99.1 matches `.github/workflows/ci.yml:18`.

| Check | Evidence/result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --locked --all-targets --target x86_64-pc-windows-gnu -- -D warnings` | PASS |
| `cargo test --locked --target x86_64-pc-windows-gnu --no-run` | PASS; all test executables compiled |
| `cargo build --release --locked --target x86_64-pc-windows-gnu` | PASS; reviewer confirmed executable size 951,296 bytes |
| Full test execution under Wine | Bootstrap failed exit 53 before tests; no product failure established |
| Native MSVC tests/build, visual/DPI verification | PENDING until user boots Windows |

Reviewer counted baseline **123 tests / 6 ignored**, current **136 / same 6 ignored**
(+13; expected normal execution 130 runnable, 6 ignored). No tests were removed,
skipped, or weakened. Existing ignores require live hardware counters, taskbar,
Claude/Codex endpoints, or installed CLIs. Wine output is preserved in
`/home/gnevesx/code/cct-contribution-notes/full-test.log`; no further Wine diagnosis.

## Remaining work and publication constraint

1. Run native MSVC format/clippy/tests/release gate and verify all expected tests.
2. User tests tooltip languages, omission/reset/stale behavior, taskbar and DPI,
   recording screenshot and display scale as required by `CONTRIBUTING.md:83`.
3. Fork is verified; source checkpoints are being published only to that fork.
4. **Do not open a PR until the user reboots into Windows and performs native tests.**

Native acceptance of T1–T5 remains pending. The completion gate was run by the orchestrator
and returned FAIL as expected; native evidence and final verification remain.
Corrected parser-guard lesson recorded by `lessons.py` as L-001, candidate;
environment/native-pending evidence does not establish another codebase defect.
