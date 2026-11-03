# Claude model weekly usage tasks

## Execution Protocol
Use tlc-spec-driven. The existing implementation is uncommitted; these records
make its remaining checks and publication traceable. Source checkpoints are published in order; native acceptance remains pending.
Tools: GitHub connector, local Git/Rust tooling, independent Verifier.
User authorized autonomous execution, tests, and the TLC workflow.

## Test Coverage Matrix
Guidelines: AGENTS.md, CONTRIBUTING.md, .github/workflows/ci.yml; sampled parser,
state, view, i18n, config, model, and workflow tests at upstream f5636ec.

| Code Layer | Required Test Type | Coverage Expectation | Location Pattern | Run Command |
| --- | --- | --- | --- | --- |
| Domain types | unit/build | Constructor compatibility; no Codex behavior change | Existing limits tests | cargo test --locked |
| Claude parser | unit | CCT-01/02/03; missing/null/invalid, 0, clamping, partial payload | Inline tests plus fake fixture | cargo test --locked limits::claude |
| Aging | unit | CCT-05/06; independent reset, active windows and flag retained | Inline state tests | cargo test --locked limits::state |
| Tooltip/i18n | unit | CCT-03/04/06/07; both languages, absence, stale, strip regression | View child test module and existing i18n tests | cargo test --locked |
| Workflow regressions | integration | Existing supply-chain invariants unchanged | tests/workflows.rs | cargo test --locked --test workflows |
| Native presentation | manual | Windows tooltip/reset text, DPI, taskbar setup | Native Windows 10/11 | Native app plus screenshot |

## Gate Check Commands
Authoritative native gate is unchanged. Linux checks use the Windows GNU target
and isolated Wine runner; they do not replace native MSVC/visual verification.

| Gate Level | When to Use | Command |
| --- | --- | --- |
| Quick | A logic task | cargo test --locked |
| Full | Integration | cargo test --locked |
| Build | Final change | cargo fmt --all -- --check; cargo clippy --locked --all-targets -- -D warnings; cargo test --locked; cargo build --release --locked |

## Execution Plan
### Phase 1: Required feature
```text
T1 -> T2 -> T3 -> T4 -> T5
```
Tasks are implementation checkpoints, with pending atomic commits. Multiple files
in T1/T4 are required to keep the shared type and localized renderer compilable.

## Task Breakdown
### T1: Extend the shared usage model
**What**: Add model identity, reported model windows, and enabled extra-usage flag.
**Where**: src/limits/mod.rs; mechanical constructors in src/limits/codex.rs, src/limits/keepalive.rs, src/limits/worker.rs, src/limits/state.rs, src/ui/view.rs.
**Depends on**: None
**Requirement**: CCT-01, CCT-03, CCT-07
**Tests**: unit/build — existing constructor and Codex/strip regression tests.
**Gate**: build

- [x] Source checkpoint verified on Linux and prepared for commit/push.
- [ ] Native acceptance on Windows is pending.




### T2: Parse reported Claude details
**What**: Parse named model windows and extra-usage flag without fabricating data.
**Where**: src/limits/claude.rs; tests/fixtures/claude_usage_models.json.
**Depends on**: T1
**Requirement**: CCT-01, CCT-02, CCT-03
**Tests**: unit — fixture and missing/null/invalid/clamp/partial-payload cases.
**Gate**: quick

- [x] Source checkpoint verified on Linux and prepared for commit/push.
- [ ] Native acceptance on Windows is pending.



### T3: Age model windows independently
**What**: Drop expired model windows while retaining unrelated readings and flag.
**Where**: src/limits/state.rs
**Depends on**: T2
**Requirement**: CCT-05, CCT-06
**Tests**: unit — boundary reset and existing freshness behavior.
**Gate**: quick

- [x] Source checkpoint verified on Linux and prepared for commit/push.
- [ ] Native acceptance on Windows is pending.


### T4: Render localized usage details
**What**: Add optional model and extra-usage tooltip lines in both languages.
**Where**: src/ui/view.rs; src/ui/view/claude_tests.rs; src/i18n.rs.
**Depends on**: T3
**Requirement**: CCT-03, CCT-04, CCT-06, CCT-07
**Tests**: unit — reset text, both languages, absence, stale/error states, strip.
**Gate**: build

- [x] Source checkpoint verified on Linux and prepared for commit/push.
- [ ] Native acceptance on Windows is pending.

### T5: Document and package the contribution
**What**: Explain the new visible behavior and prepare the focused PR.
**Where**: docs/claude-usage.md; README.md.
**Depends on**: T4
**Requirement**: CCT-01 through CCT-07
**Tests**: documentation review plus full existing automated suite.
**Gate**: build
- [ ] Independent final report, native QA, fork, atomic commits/push, PR recorded.

## Diagram-Definition Cross-Check
| Task | Depends On | Diagram Shows | Result |
| --- | --- | --- | --- |
| T1 | None | Start | Match |
| T2 | T1 | T1 to T2 | Match |
| T3 | T2 | T2 to T3 | Match |
| T4 | T3 | T3 to T4 | Match |
| T5 | T4 | T4 to T5 | Match |

## Test Co-location Validation
T1 includes compatibility changes only; T2/T3/T4 each include their behavior tests.
T5 is documentation and release evidence. No test has been removed or weakened.
The multi-file warnings reflect the shared type/localization compile boundaries.
