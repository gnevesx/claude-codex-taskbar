# CCT contribution work branch

This fork branch implements issue #13 and carries TLC handoff notes. Native
Windows acceptance is pending. The user requested no PR until that validation.

## Resume on Windows

```powershell
git clone --branch feat/claude-model-weekly-limits https://github.com/gnevesx/claude-codex-taskbar.git "$env:USERPROFILE\code\claude-codex-taskbar"
cd "$env:USERPROFILE\code\claude-codex-taskbar"
```

Use `tlc-spec-driven` and read `.specs/STATE.md`, the feature spec/tasks/validation,
and `.specs/project-style.md` before editing. Reconcile the notes with Git first.
Then follow `docs/windows-validation.md` for the pinned MSVC toolchain, automated
checks, six opt-in live tests, and taskbar/tooltip screenshots.

The Linux cross checks and offline tests are supporting evidence. They do not
establish successful native test execution or visual behavior on Windows.

## Eventual contribution PR

Keep the professor's code style and the issue's required tooltip scope. The model
strip item is optional and outside this contribution. Preserve tests and report
environmental prerequisites accurately.

These internal `.specs` files support the user's work branch. Prepare the eventual
PR branch with the source, tests, fixtures, and user documentation only; leave
internal TLC and author-style notes outside the upstream PR diff.
