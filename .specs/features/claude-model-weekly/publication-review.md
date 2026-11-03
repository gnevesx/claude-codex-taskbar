# Published branch independent review

**Handoff: PASS. Feature/native acceptance: PENDING.** The full feature result
remains FAIL until native Windows tests and visual QA provide the missing evidence.
**Date:** 2026-11-03. **Verifier:** fresh independent reviewer, not source author.
**Range:** `f5636ec7d9e5dfcaef6fac63ae83655c43184b56..8eda83f3404454ca3f1b128138aa8121c5899c25`.

## Published source and scope

All 13 source/test/product-documentation files match the frozen, checked draft
byte for byte (`cct-contribution-notes/publish/final-source.json`). The orchestrator
confirmed the fork's remote branch at the same HEAD and no open PR for this head.
The source review found no implementation blocker or remaining style defect.
Changes follow the existing parser/state/view/i18n boundaries and project guidance
in AGENTS.md, CONTRIBUTING.md and `.specs/project-style.md`. No optional strip item,
dependency, endpoint, credential change, or animation was added.

## Spec-anchored evidence

PASS means fresh execution in the isolated parser/state harness. PENDING means
the assertion was reviewed and previously cross-compiled, without native execution.

| AC | Spec outcome and exact assertion evidence | Status |
| --- | --- | --- |
| 1 | `src/limits/claude.rs:221` expects Opus 72.0/reset 1793948399 and Sonnet 0.0/None; `:266` expects clamped 100.0/0.0 | PASS |
| 2 | `src/limits/claude.rs:253` asserts empty model vector for absent/null/invalid fields; `:221` retains real Sonnet 0.0 | PASS |
| 3 | `src/limits/claude.rs:296` asserts false for non-true flags; `:321` asserts true; `src/ui/view/claude_tests.rs:60` and `:73` assert localized presence/absence | Parser PASS; view PENDING |
| 4 | `src/ui/view/claude_tests.rs:43` covers En/PtBr; `:55` expects Opus 72% with localized reset; `:59` expects Sonnet 0% without reset; `:71` asserts absent Opus omitted | PENDING |
| 5 | `src/limits/state.rs:271` expects only Sonnet 0.0/None exactly at Opus reset; `:281` preserves enabled extra usage; `:282` preserves 92.0 overall usage | PASS |
| 6 | `src/limits/state.rs:109` expects exact Stale status; `src/ui/view/claude_tests.rs:94` and `:96` retain model/stale text; `:107`–`:109` omit details for loading/disabled/failed states | State PASS; view PENDING |
| 7 | `src/limits/codex.rs:260` and `:267` retain 12%/40% and original resets; `src/ui/view/claude_tests.rs:131` expects only ClaudeFive 18.0 and ClaudeWeek 3.0 | PENDING |

No spec-precision gaps found. Each of the 13 added tests maps to these criteria.
Inventory independently checked: **123 tests/6 ignored → 136 tests/same 6 ignored**.
No existing test was removed, weakened, or newly ignored.

## Fresh discrimination sensor

Control: `cargo test --offline --locked --lib`, **29 passed, 0 failed, 1 existing
live endpoint test ignored**. The harness uses exact published parser/state files,
verbatim shared types and pure timestamp helpers, plus an HTTP panic fake.
View, Codex and Windows APIs remain outside this harness.

One scratch-only fault changed `src/limits/claude.rs:56` from Opus to Sonnet for
`seven_day_opus`. The mutant compiled and was **killed by 3 assertion failures**:
identity/reset, clamping, and model-only response tests. **1 injected, 1 killed,
0 survived.** Test code stayed identical. Scratch copies were deleted.
Real-tree pre/post porcelain was empty; tracked content hashes and source diff
hash matched: `f56c859ec44ef826a9407165549363910510273c93cc0fc994dee11289e276c0`.
Logs and pre/post snapshots: `/home/gnevesx/code/cct-contribution-notes/publication-sensor/`.

## Remaining acceptance and release boundary

The prior fmt, Windows GNU clippy, all-test compilation and release-build evidence
applies to the identical published source. Broad builds and Wine diagnosis were
not repeated. Wine bootstrap failure establishes no product defect.
Native MSVC tests/build, opt-in live checks with prerequisites, and tooltip/DPI
screenshots remain pending under `docs/windows-validation.md`.
The completion validator still rejects feature completion, as expected.
No new grounded source failure or surviving mutant requires another lesson.
**Do not open a PR before the user reboots into Windows and completes native testing.**
