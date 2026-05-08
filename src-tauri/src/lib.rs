use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, Manager, PhysicalPosition, WindowEvent,
};

const POPOVER_WIDTH: f64 = 400.0;
const MENUBAR_HEIGHT_LOGICAL: f64 = 24.0;

// ============================================================
// `tok` CLI path discovery
// ============================================================
//
// `tok` is the tokkit Python CLI. It can live in many places depending
// on how the user installed Python: anaconda, miniconda, pyenv, brew,
// pipx, or a project venv. We probe common locations first, then fall
// back to a login-shell `which tok` to inherit the user's PATH.
/// Resolve the system's IANA timezone name (e.g. "Asia/Shanghai") so we can
/// pass it to tokkit via `TOKKIT_TIMEZONE`.
///
/// Why this exists: tokkit's `utils.get_timezone()` falls back to UTC on
/// macOS because it tries `ZoneInfo(tzname())` and `tzname()` returns
/// abbreviations like "CST" / "PDT" that aren't valid IANA names. Reading
/// `/etc/localtime` (always a symlink into `…/zoneinfo/<Region>/<City>`)
/// is the cheapest stable way to recover the actual IANA name on macOS.
/// Pending an upstream tokkit fix, we forward this to every `tok` invocation
/// so hour labels and `local_date` boundaries land in the user's wall clock.
fn find_local_timezone() -> Option<String> {
    static CACHE: OnceLock<Option<String>> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            let target = std::fs::read_link("/etc/localtime").ok()?;
            let s = target.to_string_lossy();
            let marker = "zoneinfo/";
            let idx = s.find(marker)?;
            let name = s[idx + marker.len()..].trim_matches('/').to_string();
            if name.is_empty() { None } else { Some(name) }
        })
        .clone()
}

fn find_tok() -> Option<PathBuf> {
    static CACHE: OnceLock<Option<PathBuf>> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            let candidates = [
                "/opt/anaconda3/bin/tok",
                "/opt/homebrew/anaconda3/bin/tok",
                "/opt/homebrew/bin/tok",
                "/usr/local/bin/tok",
                "/Users/Shared/anaconda3/bin/tok",
            ];
            for path in candidates {
                if std::path::Path::new(path).exists() {
                    return Some(PathBuf::from(path));
                }
            }
            // Fallback: ask zsh login shell to resolve tok in user's PATH
            Command::new("/bin/zsh")
                .args(["-l", "-c", "which tok"])
                .output()
                .ok()
                .and_then(|o| {
                    let s = String::from_utf8(o.stdout).ok()?;
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(trimmed))
                    }
                })
        })
        .clone()
}

// ============================================================
// IPC commands
// ============================================================

/// Run `tok json <args>` and return tokkit's JSON unchanged.
/// `period` accepts: "today", "7", "14", "30", "yesterday", "month", "week"
#[tauri::command]
async fn get_usage(period: String) -> Result<serde_json::Value, String> {
    let tok = find_tok().ok_or_else(|| {
        "Could not find `tok`. Install tokkit: \
         `pip install \"git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit\"`"
            .to_string()
    })?;

    let args: Vec<&str> = match period.as_str() {
        "today" => vec!["json", "today"],
        "yesterday" => vec!["json", "yesterday"],
        "week" => vec!["json", "week"],
        "month" => vec!["json", "month"],
        n if n.parse::<u32>().is_ok() => vec!["json", "last", n],
        _ => return Err(format!("Unknown period: {}", period)),
    };

    let mut cmd = Command::new(&tok);
    cmd.args(&args);
    if let Some(tz) = find_local_timezone() {
        cmd.env("TOKKIT_TIMEZONE", tz);
    }
    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run tok: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "tok exited with status {}: {}",
            output.status, stderr
        ));
    }

    let data: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse tok json output: {}", e))?;

    Ok(data)
}

/// Generate and open a tokkit HTML report covering the last `last_days` days.
/// Wraps `tok html last <N> open`, which writes the report to
/// `~/.tokkit/reports/` and hands the path to the OS default browser.
/// Intended for the popover "完整报告" link, scoped to whichever tab is active.
#[tauri::command]
async fn open_html_report(last_days: u32) -> Result<(), String> {
    let tok = find_tok().ok_or_else(|| {
        "Could not find `tok`. Install tokkit: \
         `pip install \"git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit\"`"
            .to_string()
    })?;

    let days = last_days.max(1).to_string();

    // Generate the HTML without `open` so we can patch the file before
    // handing it to the browser (otherwise we'd race the OS opener).
    let mut cmd = Command::new(&tok);
    cmd.args(["html", "last", &days]);
    if let Some(tz) = find_local_timezone() {
        cmd.env("TOKKIT_TIMEZONE", tz);
    }
    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run tok html: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "tok html exited with status {}: {}",
            output.status, stderr
        ));
    }

    // tok prints `wrote HTML report to <path>` on success.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let report_path = stdout
        .lines()
        .find_map(|line| line.trim().strip_prefix("wrote HTML report to ").map(str::trim))
        .ok_or_else(|| format!("could not parse HTML report path from tok output: {}", stdout.trim()))?;

    // Hide tokkit's 7/14/30 range switcher in the topbar. It does subset
    // filtering on already-embedded data, so when the report's RAW window
    // is < 30 days the buttons silently do nothing — more confusing than
    // helpful. We patch the rendered file rather than tokkit itself so the
    // upstream report stays as-is for non-toksee users.
    let html = std::fs::read_to_string(report_path)
        .map_err(|e| format!("Failed to read HTML report at {}: {}", report_path, e))?;
    let patched = html.replacen(
        "</head>",
        "<style>.range-group{display:none!important}</style></head>",
        1,
    );
    std::fs::write(report_path, patched)
        .map_err(|e| format!("Failed to write patched HTML report: {}", e))?;

    Command::new("open")
        .arg(report_path)
        .output()
        .map_err(|e| format!("Failed to open HTML report: {}", e))?;

    Ok(())
}

/// Lightweight call for the menubar title — just returns today's
/// total tokens + estimated cost so the tray title can be refreshed
/// without re-fetching the whole popover dataset.
#[tauri::command]
async fn get_menubar_summary() -> Result<serde_json::Value, String> {
    let json = get_usage("today".to_string()).await?;
    let totals = json
        .get("totals")
        .ok_or_else(|| "Missing `totals` in tok json today".to_string())?;
    Ok(serde_json::json!({
        "total_tokens": totals.get("total_tokens").cloned().unwrap_or(serde_json::json!(0)),
        "estimated_cost_usd": totals.get("estimated_cost_usd").cloned().unwrap_or(serde_json::json!(0.0)),
        "records": totals.get("records").cloned().unwrap_or(serde_json::json!(0)),
    }))
}

// ============================================================
// App entrypoint
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_usage,
            get_menubar_summary,
            open_html_report
        ])
        .setup(|app| {
            // macOS: accessory app — no Dock icon, no main menu
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            // Wire blur-to-hide
            if let Some(popover) = app.get_webview_window("popover") {
                let popover_for_blur = popover.clone();
                popover.on_window_event(move |event| {
                    if let WindowEvent::Focused(false) = event {
                        let _ = popover_for_blur.hide();
                    }
                });
            }

            // Tray menu (right-click)
            let show_item = MenuItem::with_id(app, "show", "显示 TokSee", true, None::<&str>)?;
            let refresh_item = MenuItem::with_id(app, "refresh", "立即刷新", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let about_item = MenuItem::with_id(app, "about", "关于 TokSee", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrCtrl+Q"))?;
            let menu = Menu::with_items(
                app,
                &[&show_item, &refresh_item, &separator, &about_item, &quit_item],
            )?;

            // Embedded menubar template image
            let tray_icon_bytes = include_bytes!("../icons/menubar-template.png");
            let tray_icon = tauri::image::Image::from_bytes(tray_icon_bytes)?;

            let _tray = TrayIconBuilder::with_id("main")
                .icon(tray_icon)
                .icon_as_template(true)
                .title("…")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("popover") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "refresh" => {
                        if let Some(window) = app.get_webview_window("popover") {
                            let _ = window.eval(
                                "window.dispatchEvent(new CustomEvent('toksee:refresh', { detail: { source: 'manual' } }))",
                            );
                        }
                    }
                    "about" => {
                        if let Some(window) = app.get_webview_window("popover") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        rect,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let Some(window) = app.get_webview_window("popover") else {
                            return;
                        };

                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                            return;
                        }

                        let scale = window
                            .current_monitor()
                            .ok()
                            .flatten()
                            .map(|m| m.scale_factor())
                            .unwrap_or(2.0);

                        let tray_pos = rect.position.to_physical::<f64>(scale);
                        let tray_size = rect.size.to_physical::<f64>(scale);

                        // Center popover on tray icon, flush with menubar bottom
                        let popover_w_px = POPOVER_WIDTH * scale;
                        let x = tray_pos.x + (tray_size.width / 2.0) - (popover_w_px / 2.0);
                        let y = tray_pos.y + MENUBAR_HEIGHT_LOGICAL * scale;

                        let _ = window.set_position(PhysicalPosition::new(x, y));
                        let _ = window.show();
                        let _ = window.set_focus();
                        // Reopening the popover is equivalent to a fresh load,
                        // so treat it like an "auto" refresh (re-seed the footer
                        // "下次更新" time alongside the data fetch).
                        let _ = window.eval(
                            "window.dispatchEvent(new CustomEvent('toksee:refresh', { detail: { source: 'auto' } }))",
                        );
                    }
                })
                .build(app)?;

            // Initial menubar title refresh, then auto-refresh aligned to the
            // next clock hour and every hour after that. Aligning to clock hours
            // (rather than 3600s after launch) lets the footer "下次更新 HH:MM"
            // predict the next refresh in human terms.
            //
            // Implementation note: Unix-epoch seconds are computed from UTC,
            // and `epoch % 3600` matches the local clock for any integer-hour
            // timezone (covers virtually every user including UTC+8). Users in
            // half-hour zones (IN, NP, NL) will see refreshes ~30/45min off
            // their wall clock — acceptable v0.1 trade-off.
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                refresh_menubar_title(&app_handle).await;

                let now_secs = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let secs_until_next_hour = 3600 - (now_secs % 3600);
                tokio::time::sleep(std::time::Duration::from_secs(secs_until_next_hour)).await;

                loop {
                    refresh_menubar_title(&app_handle).await;
                    if let Some(window) = app_handle.get_webview_window("popover") {
                        let _ = window.eval(
                            "window.dispatchEvent(new CustomEvent('toksee:refresh', { detail: { source: 'auto' } }))",
                        );
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Update the tray title with today's token count (abbreviated).
async fn refresh_menubar_title(app: &tauri::AppHandle) {
    let summary = match get_menubar_summary().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[toksee] menubar refresh failed: {}", e);
            return;
        }
    };
    let tokens = summary
        .get("total_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let title = format_tokens_compact(tokens);
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_title(Some(&title));
    }
}

/// 1,770,401 → "1.77M"; 222,571,338 → "223M"; 8,000 → "8.0K"
fn format_tokens_compact(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 10_000_000 {
        format!("{}M", (n as f64 / 1_000_000.0).round() as u64)
    } else if n >= 1_000_000 {
        format!("{:.2}M", n as f64 / 1_000_000.0)
    } else if n >= 10_000 {
        format!("{}K", (n as f64 / 1_000.0).round() as u64)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
