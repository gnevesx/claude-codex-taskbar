//! Windows UAT helper for the handoff branch; keep outside the feature PR.
//! Uses CCT's real notification transport with synthetic usage; no provider is instantiated.

use claude_codex_taskbar::config::Config;
use claude_codex_taskbar::i18n::{Key, Lang, t};
use claude_codex_taskbar::limits::{ProviderStatus, ProviderUsage, WindowUsage};
use claude_codex_taskbar::model::Kind;
use claude_codex_taskbar::timeutil::now_unix;
use claude_codex_taskbar::ui::notifications::shell::{
    WindowsNotificationSink, cleanup_timer, decode_callback, timer_icon,
};
use claude_codex_taskbar::ui::notifications::{
    BalloonCommand, BalloonLife, Notice, NotificationSink, Notifications, balloon_text,
    send_balloon,
};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{
    NIN_BALLOONHIDE, NIN_BALLOONSHOW, NIN_BALLOONTIMEOUT, NIN_BALLOONUSERCLICK,
};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::{PCWSTR, w};

const CALLBACK: u32 = WM_APP + 10;
const DELAY_TIMER: usize = 1;
const BOTH: usize = 10;
const DELAY: usize = 13;
const LOCALE: usize = 14;
const ENABLE: usize = 15;
const REPEAT: usize = 16;
static TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);

struct Preview {
    cfg: Config,
    lang: Lang,
    engine: Notifications,
    life: BalloonLife,
    reset_at: i64,
}

thread_local! {
    static PREVIEW: RefCell<Option<Preview>> = const { RefCell::new(None) };
}

fn with_preview<R>(f: impl FnOnce(&mut Preview) -> R) -> Option<R> {
    PREVIEW.with(|cell| cell.try_borrow_mut().ok()?.as_mut().map(f))
}

fn shown(id: u16) -> bool {
    with_preview(|p| p.life.is_shown(id)).unwrap_or(false)
}

fn sink(hwnd: HWND) -> WindowsNotificationSink {
    WindowsNotificationSink {
        hwnd,
        callback: CALLBACK,
        shown,
    }
}

fn wide_z(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

fn status(hwnd: HWND, text: &str) {
    let title = wide_z(&format!("CCT notification preview — {text}"));
    // SAFETY: the title buffer outlives this call on our UI-thread window.
    unsafe {
        let _ = SetWindowTextW(hwnd, PCWSTR(title.as_ptr()));
    }
}

fn status_choice(hwnd: HWND, condition: bool, yes: &str, no: &str) {
    status(hwnd, if condition { yes } else { no });
}

fn synthetic(percent: f32, now: i64, reset_at: i64) -> ProviderStatus {
    let window = |reset: i64| {
        Some(WindowUsage {
            used_pct: percent,
            resets_at: Some(reset),
        })
    };
    ProviderStatus::Ok {
        usage: ProviderUsage {
            five_hour: window(reset_at),
            weekly: window(reset_at + 500_000),
            ..Default::default()
        },
        at: now,
    }
}

fn notices(p: &mut Preview, new_crossing: bool) -> Vec<Notice> {
    let now = now_unix();
    if new_crossing {
        p.engine = Notifications::default();
    }
    let mut batch = Vec::new();
    for kind in [Kind::Claude, Kind::Codex] {
        if new_crossing {
            p.engine
                .observe(&p.cfg, kind, &synthetic(70.0, now, p.reset_at), now);
        }
        batch.extend(
            p.engine
                .observe(&p.cfg, kind, &synthetic(97.0, now, p.reset_at), now),
        );
    }
    batch
}

fn finish(hwnd: HWND, id: u16) {
    with_preview(|p| p.life.finish(id));
    sink(hwnd).remove(id);
}

fn cancel(hwnd: HWND) {
    let id = with_preview(|p| p.life.cancel()).flatten();
    // SAFETY: this timer belongs to the preview window.
    unsafe {
        let _ = KillTimer(Some(hwnd), DELAY_TIMER);
    }
    if let Some(id) = id {
        sink(hwnd).remove(id);
    }
}

fn generate(hwnd: HWND, new_crossing: bool) {
    let prepared = with_preview(|p| {
        let batch = notices(p, new_crossing);
        let body = balloon_text(p.lang, &p.cfg, &batch)?;
        let (previous, id) = p.life.begin();
        Some((previous, id, t(p.lang, Key::NotifyTitle), body))
    })
    .flatten();
    let Some((previous, id, title, body)) = prepared else {
        status(hwnd, "no new crossing, or preview alerts off");
        return;
    };
    if let Some(previous) = previous {
        sink(hwnd).remove(previous);
    }
    let sent = send_balloon(&sink(hwnd), id, title, &body, || {
        with_preview(|p| p.cfg.notify_near_limit && p.life.is_current(id)).unwrap_or(false)
    });
    if !sent {
        finish(hwnd, id);
    }
    status_choice(
        hwnd,
        sent,
        "submitted; wait for native banner",
        "suppressed or unavailable",
    );
}

fn action(hwnd: HWND, command: usize) {
    match command {
        BOTH => generate(hwnd, true),
        REPEAT => generate(hwnd, false),
        DELAY => {
            // SAFETY: replacing this timer prevents repeated clicks accumulating work.
            unsafe {
                SetTimer(Some(hwnd), DELAY_TIMER, 5_000, None);
            }
            status(hwnd, "5 seconds: enable DND / switch apps");
        }
        LOCALE => {
            let lang = with_preview(|p| {
                p.lang = if p.lang == Lang::En {
                    Lang::PtBr
                } else {
                    Lang::En
                };
                p.lang
            });
            status_choice(
                hwnd,
                lang == Some(Lang::PtBr),
                "pt-BR selected",
                "English selected",
            );
        }
        ENABLE => {
            let enabled = with_preview(|p| {
                p.cfg.notify_near_limit = !p.cfg.notify_near_limit;
                p.cfg.notify_near_limit
            })
            .unwrap_or(false);
            if !enabled {
                cancel(hwnd);
            }
            status_choice(hwnd, enabled, "preview alerts on", "preview alerts off");
        }
        _ => {}
    }
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            action(hwnd, wp.0 & 0xffff);
            LRESULT(0)
        }
        CALLBACK => {
            let (id, event) = decode_callback(lp);
            if event == NIN_BALLOONSHOW {
                if with_preview(|p| p.life.mark_shown(id)) == Some(true)
                    && !sink(hwnd).send(BalloonCommand::ArmCleanup, id, "", "")
                {
                    finish(hwnd, id);
                }
            } else if [NIN_BALLOONHIDE, NIN_BALLOONTIMEOUT, NIN_BALLOONUSERCLICK].contains(&event) {
                finish(hwnd, id);
            }
            LRESULT(0)
        }
        WM_TIMER => {
            if wp.0 == DELAY_TIMER {
                // SAFETY: this is a one-shot preview action.
                unsafe {
                    let _ = KillTimer(Some(hwnd), DELAY_TIMER);
                }
                action(hwnd, BOTH);
            } else if let Some(id) = timer_icon(wp.0) {
                // SAFETY: stop the expired timer identified by its owning icon ID.
                unsafe {
                    let _ = KillTimer(Some(hwnd), cleanup_timer(id));
                }
                finish(hwnd, id);
            }
            LRESULT(0)
        }
        WM_CLOSE => {
            // SAFETY: destroying our own UI-thread window.
            unsafe {
                let _ = DestroyWindow(hwnd);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            cancel(hwnd);
            PREVIEW.with(|cell| {
                cell.borrow_mut().take();
            });
            // SAFETY: the preview owns this message loop.
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ if msg == TASKBAR_CREATED.load(Ordering::Relaxed) && msg != 0 => {
            cancel(hwnd);
            status(hwnd, "Explorer restarted; create a new crossing");
            LRESULT(0)
        }
        // SAFETY: standard default processing for our native window.
        _ => unsafe { DefWindowProcW(hwnd, msg, wp, lp) },
    }
}

fn shortcut(key: usize) -> Option<usize> {
    match key {
        13 => Some(BOTH),
        68 => Some(DELAY),
        76 => Some(LOCALE),
        79 => Some(ENABLE),
        82 => Some(REPEAT),
        _ => None,
    }
}

fn main() -> windows::core::Result<()> {
    PREVIEW.with(|cell| {
        *cell.borrow_mut() = Some(Preview {
            cfg: Config {
                notify_near_limit: true,
                claude_enabled: true,
                codex_enabled: true,
                ..Config::default()
            },
            lang: Lang::En,
            engine: Notifications::default(),
            life: BalloonLife::default(),
            reset_at: now_unix() + 604_800,
        })
    });
    // SAFETY: standard native window bootstrap and UI-thread message loop.
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(wndproc),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: windows::Win32::Graphics::Gdi::HBRUSH(6usize as _),
            lpszClassName: w!("CctNotificationPreview"),
            ..Default::default()
        };
        RegisterClassExW(&class);
        TASKBAR_CREATED.store(
            RegisterWindowMessageW(w!("TaskbarCreated")),
            Ordering::Relaxed,
        );
        let hwnd = CreateWindowExW(
            Default::default(),
            class.lpszClassName,
            w!("CCT preview — synthetic 70% → 97%; no credentials/network/CLI"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            660,
            180,
            None,
            None,
            Some(instance.into()),
            None,
        )?;
        let buttons = [
            (BOTH, "New both (Enter)"),
            (DELAY, "Delay 5 s (D)"),
            (LOCALE, "EN / pt-BR (L)"),
            (ENABLE, "Alerts on/off (O)"),
            (REPEAT, "Repeat 97% (R)"),
        ];
        for (index, (id, label)) in buttons.into_iter().enumerate() {
            let label = wide_z(label);
            CreateWindowExW(
                Default::default(),
                w!("BUTTON"),
                PCWSTR(label.as_ptr()),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                16 + (index as i32 % 3) * 210,
                16 + (index as i32 / 3) * 50,
                200,
                34,
                Some(hwnd),
                Some(HMENU(id as _)),
                None,
                None,
            )?;
        }
        let _ = ShowWindow(hwnd, SW_SHOW);
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_KEYDOWN
                && let Some(command) = shortcut(msg.wParam.0)
            {
                if msg.lParam.0 & (1 << 30) == 0 {
                    action(hwnd, command);
                }
                continue;
            }
            if !IsDialogMessageW(hwnd, &msg).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
    Ok(())
}
