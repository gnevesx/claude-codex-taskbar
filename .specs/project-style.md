# Baseline style review for issue #13

Evidence uses tracked commit `f5636ec` (`git show HEAD:<file>`), not our working
changes. Citations below are **baseline file:line** references. The diff review
describes the uncommitted snapshot inspected on 2026-11-03; it is not a test result.
These are repository conventions, not judgments about the author's credentials.

## Repeated conventions and how to adapt

- **Direct Rust domain types, little ceremony.** Shared structs/enums express
  optional windows and status variants (`src/limits/mod.rs:10`, `:19`, `:25`, `:34`).
  `Provider` is the small external-provider boundary (`src/limits/mod.rs:57`). Add
  the model enum and optional usage fields here; avoid a new service hierarchy.
- **Parser → state → view → native shell.** JSON interpretation is in the Claude
  provider (`src/limits/claude.rs:43`, `:54`); aging is in `src/limits/state.rs:34`;
  tooltip construction is in `src/ui/view.rs:125`, `:138`; wiring is in
  `src/ui/app.rs:1`, `:17`. Extend those existing paths, without moving Win32 code
  into parsers or introducing another request.
- **Free functions and small local helpers.** Examples include `parse_window`
  (`src/limits/claude.rs:43`), `map_status` (`src/limits/http.rs:34`), and the local
  `keep` closure (`src/limits/state.rs:36`). Names are practical snake_case; local
  names such as `v`, `u`, `w`, and `pct` are common (`src/limits/claude.rs:44`,
  `src/ui/view.rs:93`). The baseline does not impose a universal 20-line limit:
  `build_items` spans `src/ui/view.rs:39`–`:106`.
- **Absence, freshness, and security are explicit.** Parsers return `Option` or
  `Result` and reuse clamping (`src/limits/claude.rs:43`, `:59`). States preserve
  timestamps and stale status (`src/limits/state.rs:18`, `:49`), while missing
  readings render `—` (`src/ui/view.rs:16`, `:22`). Token `Debug` is manually
  redacted (`src/limits/claude.rs:20`); HTTP errors omit headers
  (`src/limits/http.rs:42`). Preserve these invariants for each new model window.
- **Comments explain boundaries and nonobvious reasons.** Brief English `//!`
  module summaries (`src/lib.rs:1`, `src/limits/state.rs:1`), units in field docs
  (`src/limits/mod.rs:13`), redirect rationale (`src/limits/http.rs:11`), and
  `SAFETY:` comments (`src/ui/app.rs:66`) recur. Public helpers often have one short
  behavior sentence, not mandatory usage examples (`src/config.rs:49`, `:60`).
- **Tests sit beside the implementation.** Inline `#[cfg(test)]` modules occur in
  `src/limits/claude.rs:113`, `src/limits/state.rs:66`, and `src/ui/view.rs:227`;
  additional concerns get another inline module (`src/ui/view.rs:386`, `:406`).
  Parsers use JSON fixtures and targeted `json!` cases (`src/limits/claude.rs:119`,
  `:179`); state/view tests build typed values (`src/limits/state.rs:71`,
  `src/ui/view.rs:249`). Assertions describe observable behavior, including
  thresholds and missing data (`src/model.rs:50`, `src/ui/view.rs:323`). Real
  endpoint tests are ignored explicitly (`src/limits/claude.rs:230`).
- **Localization is centralized.** Add each key to `Key`, `Key::ALL`, and the
  English/pt-BR tuple match (`src/i18n.rs:30`, `:67`, `:105`). A generic completeness
  test exists (`src/i18n.rs:191`); view tests also assert visible localized text
  (`src/ui/view.rs:273`, `:276`). Reuse existing reset-time formatting.
- **Documentation is user-oriented; changes are focused.** Feature bullets and
  troubleshooting explain what users see (`README.md:35`, `:95`), while contribution
  rules stay in `CONTRIBUTING.md:52`. English is required (`CONTRIBUTING.md:96`).
  Recent commits use imperative Conventional Commits: `fb11763` fixes HTTP with
  tests in one module; `3c2f2b5` fixes stale/reset behavior across the necessary
  modules. The latter shows that a focused fix can legitimately touch several files.
  Their bodies explain the trigger and new behavior; the HTTP commit closes its issue.
- **Use tooling, preserve footprint.** UTF-8, LF, four-space Rust indentation are
  declared (`.editorconfig:4`); fmt/clippy/test are required (`AGENTS.md:21`). Keep
  dependencies unchanged where possible (`Cargo.toml:18`, `AGENTS.md:43`).

## Visible differences in our current draft

- `src/ui/view.rs` adds `#[path = "view/claude_tests.rs"]`, unlike the repeated
  inline test modules above; no tracked baseline Rust file uses `#[path]`. Prefer
  an inline `model_usage_tests` module if it fits the user's file-size constraint;
  otherwise document why the separate test file is needed.
- New state/view tests parse the shared Claude JSON fixture, while baseline tests
  at these layers construct `ProviderUsage` directly. Prefer typed helpers for unit
  coverage; retain parser-to-tooltip coverage when it tests a distinct regression.
- Several added structs and assertions are compressed onto single lines, unlike
  the corresponding multiline baseline cases (`src/limits/claude.rs:162`,
  `src/ui/view.rs:264`). Run rustfmt before assessing the final diff.
- The new parser docstring appends `Example: parse_claude_usage(&json)`, which is
  more formulaic than the short baseline docs. The user's general instructions
  request public-function examples, so keep a useful example without expanding
  unrelated documentation merely to imitate a universal template.
- The new standalone Markdown file is a new docs shape: baseline `docs/` contains
  images. It is justified by the user's documentation instruction; keep it short,
  explain behavior, and omit discussion of deferred contributions from product docs.
  Say "Windows UI language", since detection is automatic (`src/i18n.rs:19`,
  `src/ui/app.rs:127`), rather than suggesting a language selector.
- `..Default::default()` is already common in baseline Win32 setup
  (`src/ui/app.rs:96`); deriving `Default` for usage is an extension, not by itself a
  style violation. The shared `ModelKind`, localized keys, parser helper, and
  existing aging/tooltip paths otherwise match the established design.

## Adaptation recorded after the review

Typed test setup now replaces parser-dependent view/state setup. The redundant
path attribute is gone; the child module uses normal Rust discovery. A separate
test file preserves existing tests and keeps view.rs below 500 lines. Rustfmt passes.
The docs describe Windows UI language detection and omit contribution plans.
The parser doc remains short, with the example required by the user's guidance.
