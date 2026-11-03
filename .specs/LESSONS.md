# LESSONS - auto-maintained by scripts/lessons.py

> Machine-owned. Do NOT hand-edit. Changes are overwritten on the next `lessons.py` write.
> Canonical state lives in `.specs/lessons.json`. Edit lessons only via the script.
> promote_threshold=2 distinct features · window_days=45 · quarantine_threshold=2

## Confirmed (load these at Specify/Design)

Corroborated across multiple features. Safe to apply as guidance.

_none_

## Candidates (under observation - do NOT load as guidance yet)

Seen once or not yet corroborated. Tracked, not trusted.

### L-001 - When extending Claude usage parsing, update response-validity checks and test each newly supported reading without legacy window keys.
- signal: `ac_gap` · recurrence: 1 feature(s) · scope: `src/limits/claude.rs` · harmful: 0
- features: claude-model-weekly
- evidence: .specs/features/claude-model-weekly/validation.md:17 (src/limits/claude.rs)
- last seen: 2026-11-03T21:43:36Z

## Quarantined (failed when applied - ignore)

A confirmed lesson that recurred alongside failure. Kept for the maintainer to review.

_none_
