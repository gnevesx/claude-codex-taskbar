# STATE

## Decisions

## Handoff

- **Feature**: Claude model weekly usage, issue #13.
- **Phase / Task**: source checkpoints T1–T5 committed and pushed; native acceptance pending.
- **Fork**: https://github.com/gnevesx/claude-codex-taskbar.
- **Branch**: feat/claude-model-weekly-limits.
- **Source checkpoint**: 8eda83f3404454ca3f1b128138aa8121c5899c25.
- **Completed**: author-style audit; parser, independent reset handling, localized
  tooltip, 13 new tests and user docs. Isolated staged trees passed formatting and
  Windows GNU clippy. Offline parser/state suite: 29 passed, 1 upstream live ignore.
  Complete draft also compiled all test executables and a 951296-byte release.
- **Review**: see publication-review.md; native acceptance remains outstanding.
- **Next step**: clone this branch on the user's Windows, use tlc-spec-driven,
  reconcile Git with these notes, then follow docs/windows-validation.md.
- **Native checks**: Rust 1.99.1 MSVC fmt/clippy/test/release; expected 136 tests,
  130 runnable by default, same six existing live ignores. Run live tests with
  their CLI/taskbar/hardware prerequisites; collect tooltip/DPI screenshots.
- **User constraint**: no PR before native Windows validation. No PR is open.
- **Tracking**: internal .specs notes are portable in this fork work branch.
  Exclude them from the eventual upstream PR diff. Keep source tests intact.
- **Remotes**: origin is the user's fork; upstream is the professor's repository.
- **Local changes**: final handoff metadata only; source matches the checked draft.
