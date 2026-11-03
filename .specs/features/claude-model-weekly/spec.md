# Claude model weekly usage

## Problem Statement
Issue #13: Claude returns per-model weekly limits that users cannot see today.

## Out of Scope
Optional strip item/settings, generic limits[] parsing, new endpoints, dependencies,
notifications, credential changes, and animations.

## Assumptions & Open Questions
| Assumption | Chosen default | Rationale |
| --- | --- | --- |
| Extra usage detail | Display enabled status only | Issue specifies no monetary schema |
| Model order | Opus then Sonnet | Matches named fields and predictable output |
| Stale data | Keep last data with existing stale text | Preserve existing error semantics |
| Expired windows | Remove model entries at reset, keep unrelated windows | Match existing display-time aging |
Open questions: none.

## User Stories
As a Claude subscriber, I can inspect model-specific quotas without opening the CLI.

Acceptance Criteria:
1. WHEN named model windows have utilization, the parser SHALL preserve model identity, reset timestamp, and percentage clamped to 0–100.
2. WHEN named fields are null, missing, or lack numeric utilization, the parser SHALL omit them, while retaining a real 0% reading.
3. WHEN extra_usage.is_enabled is true, the tooltip SHALL show a localized enabled line; otherwise it SHALL omit that line.
4. WHEN model readings exist, the tooltip SHALL show Opus/Sonnet weekly lines in English and pt-BR and reset text only when reset timestamps exist.
5. WHEN a model window reaches its reset timestamp, age_status SHALL remove it independently and preserve active model windows and extra-usage state.
6. WHILE data is stale, model lines SHALL retain the existing stale notice; failed, loading, and disabled states SHALL not invent model readings.
7. WHEN Codex data is parsed or existing strip items are rendered, their previous behavior SHALL remain unchanged.

## Requirement Traceability
| ID | Criterion | Component | Status |
| --- | --- | --- | --- |
| CCT-01 | AC1 model identity, reset and clamp | Claude parser | Logic tests passed |
| CCT-02 | AC2 absent values and real zero | Claude parser | Logic tests passed |
| CCT-03 | AC3 enabled extra usage | Parser and tooltip | Full tests pending |
| CCT-04 | AC4 localized model lines and reset | Tooltip and i18n | Full tests pending |
| CCT-05 | AC5 independent expiration | Provider aging | Logic tests passed |
| CCT-06 | AC6 stale/failed/loading/disabled | Provider status and tooltip | Full tests pending |
| CCT-07 | AC7 Codex and strip regression | Existing parsers and strip | Full tests pending |

## Execution Plan
Five atomic source checkpoints are defined in tasks.md. Each is checked in an
isolated staged tree and pushed immediately after its commit. Native acceptance
is deferred to the user's Windows by explicit request. Internal TLC notes travel
in this fork work branch; omit them from the eventual upstream PR.

## Status
Draft implementation exists. Formatting and Windows GNU clippy pass. Offline
parser/state checks: 29 passed, 1 upstream live test ignored; five mutations killed
by an independent reviewer. Windows GNU release build passed; full Wine execution failed during bootstrap.
Native MSVC/Windows visual QA remains pending; source checkpoints are published to the verified fork. No PR is open.

## User-directed native handoff
On 2026-11-03 the user explicitly requested publication to their own fork only,
then native Windows testing on this same PC. Opening a PR is deferred until those
checks pass. Cross compilation is supporting evidence, not native QA.
The fork is confirmed. Verify remote refs after publishing. Open no upstream PR before native acceptance.
Windows instructions: docs/windows-validation.md. Internal TLC/style notes must
remain available for the Windows handoff without being added to the eventual PR.
