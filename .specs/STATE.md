# STATE

## Decisions

## Handoff

- **Feature**: Claude model weekly usage, issue #13.
- **Phase / Task**: source checkpoint T3/T5; native acceptance remains pending.
- **Fork**: https://github.com/gnevesx/claude-codex-taskbar (verified fork).
- **Branch**: feat/claude-model-weekly-limits.
- **Base**: f5636ec7d9e5dfcaef6fac63ae83655c43184b56.
- **Next step**: finish publishing checkpoints, then test on the user's Windows.
- **User constraint**: do not open a PR before native Windows testing.
- **Evidence**: fmt, Windows GNU clippy, full test compilation and release build passed
  on the complete draft; 29 offline parser/state tests passed; five mutations killed.
- **Pending**: native MSVC suite, six existing live tests, tooltip and DPI screenshots.
- **Guide**: docs/windows-validation.md.
- **Tracking**: .specs is included in this fork's work branch for a portable handoff;
  omit these internal notes from the eventual contribution PR.
