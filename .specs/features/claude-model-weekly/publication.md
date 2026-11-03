# Fork publication

Fork and parent were verified through the GitHub connector. Remote refs matched
local source checkpoint `8eda83f3404454ca3f1b128138aa8121c5899c25`. The fork's main branch remains at the
original upstream base. No PR has been opened.

## Atomic source checkpoints

```text
d099bd2 feat(limits): add model-specific usage types
a78e999 feat(claude): parse per-model weekly limits and extra usage
7c0fbd9 fix(limits): drop expired model usage windows
4d9049e feat(tooltip): show localized Claude model usage
8eda83f docs(claude): describe usage details and Windows validation
```

Each commit was pushed immediately. Candidate trees passed fmt and Windows GNU
clippy independently. Offline candidate parser/state execution progressed from
22 passing tests at T1, to 28 at T2, and 29 at T3. The existing live Claude test
remained ignored. T4's localized view tests were cross-compiled, with native
execution deferred explicitly by the user. All 13 final source/test/doc files
match the complete draft previously cross-checked and built.

## Resume

Read `.specs/README.md` and `docs/windows-validation.md` on Windows. The feature's
native acceptance gate is pending; this publication is a work-in-progress handoff.
Use the independent publication review as the source handoff record. Keep these
internal notes outside the eventual PR to the professor.
