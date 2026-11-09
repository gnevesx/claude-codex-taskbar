# Issue #15 slice 2 — Windows validation handoff

Feature: `feat/limit-notifications`, commit `0a3d5f7`.
No upstream PR until native visual UAT is complete. Native Windows/MSVC tests,
banner visibility and DND remain pending. [Fork CI run 42678397352](https://github.com/gnevesx/claude-codex-taskbar/actions/runs/42678397352)
was blocked before any job steps because the account has a billing issue; it did
not execute checks. Local evidence: 82 portable tests passed, GNU test binaries
cross-compiled, and GNU release size was 959,488 bytes. These are not native UAT.

Use Windows 10/11 x64, Visual Studio Build Tools C++ and Rust 1.99.1/MSVC.
Record Windows version, DPI, taskbar alignment, locale and exact feature SHA.

## Build the actual feature

Run in PowerShell, under a working directory such as `$HOME\code`:

```powershell
git clone --branch feat/limit-notifications https://github.com/gnevesx/claude-codex-taskbar.git cct-notification-uat
cd cct-notification-uat
git rev-parse HEAD
cargo +1.99.1 fmt --check
cargo +1.99.1 clippy --locked --all-targets -- -D warnings
cargo +1.99.1 test --locked
cargo +1.99.1 build --release --locked
$size = (Get-Item target/release/claude-codex-taskbar.exe).Length
Write-Output "Release: $size bytes; limit: 1000000"
if ($size -gt 1000000) { throw 'Binary exceeds the size budget' }
```

Check the SHA against the final handoff before testing. The current source
inventory is 267 test functions: expected 259 passed and 8 existing live tests
ignored across all test binaries. This count is an expectation, not an execution
result. Keep the ordinary ignored tests unchanged; they require live dependencies.

## Optional standalone transport preview

The helper belongs only to `handoff/limit-notifications-windows`, at
`examples/limit-notification-preview.rs`. It must stay outside the feature PR.
Download into the feature clone:

```powershell
New-Item -ItemType Directory -Force examples | Out-Null
Invoke-WebRequest 'https://raw.githubusercontent.com/gnevesx/claude-codex-taskbar/handoff/limit-notifications-windows/examples/limit-notification-preview.rs' -OutFile examples/limit-notification-preview.rs
cargo +1.99.1 run --locked --example limit-notification-preview
```

This visible GUI uses the feature's real crossing engine, localized balloon text,
Win32 transport, version-4 callbacks and icon/timer cleanup. Its quota readings are
synthetic 70% → 97% for five-hour and weekly windows. It never creates providers,
reads credentials, requests network access, runs a CLI, or changes CCT settings.
Preview alerts start on in a separate in-memory Config; CCT defaults remain off.

- Enter/New both starts fresh Claude and Codex synthetic crossing histories.
  Both 80/95 crossings are aggregated to the highest threshold per window (95%).
- Repeat 97% (`R`) keeps history: it must produce no additional notification.
- EN/pt-BR (`L`) selects balloon language; a subsequent new crossing uses it.
- Delay 5 s (`D`) schedules a crossing; switch apps or enable DND during the delay.
  Repeated delay clicks replace the pending timer. Alerts on/off (`O`) cancels it.
- The title reports submitted/suppressed; only a visible banner confirms display.
  Close the window to cancel its timers and temporary icon. No sound is expected.

## Native visual UAT

Record pass/fail with screenshots of nonsensitive UI and exact reproduction steps.
Never include credential files, tokens, account identifiers, or personal paths.

- Actual app, fresh nonsensitive config profile: notification checkbox defaults
  off; Tab/Space/Enter work; Cancel preserves the prior choice; OK and restart
  persist the choice. Keep the existing profile backed up when testing defaults.
- Actual app: enable/disable alerts and each provider, then re-enable; no old
  crossing is replayed. Disabling while a balloon is active removes its tray icon.
  Reject optional login keep-alive when keeping CLI activity outside this UAT.
- Preview: English and pt-BR text is readable, accented correctly and identifies
  the provider/window/95% threshold; batched notices remain within native buffers.
  Repeat 97% does not repeat; click, timeout and close clean up the temporary icon.
- Preview delay: exercise DND/Focus Assist, lock and shell notification states.
  Return to normal: no suppressed alert is replayed; test a new crossing again.
  Quiet time differs from modern DND; native behavior must be observed. The helper
  checks Shell acceptance only. F11/fullscreen may not be rejected by Shell; the
  actual App has an additional foreground-fullscreen guard that this helper does
  not run. Verify fullscreen/presentation suppression separately in the real App.
- Explorer restart: no orphan icon or replay; the next new crossing can display.
  Exit during a delay/active banner, and repeat rapid actions: no lingering icon,
  crash, sound, or late banner. Check normal and high-DPI layouts where available.

The preview establishes real transport behavior with fake readings. It does not
establish real `App` result draining, provider freshness, Settings wiring, or real
quota crossings; validate those separately with the actual feature application.
Do not create artificial usage charges merely to force a threshold. Report any
integration case that could not be exercised, alongside native checks and build
results, before deciding whether the feature is ready for an upstream PR.
