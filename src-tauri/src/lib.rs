use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, Manager, PhysicalPosition, WindowEvent,
};

const POPOVER_WIDTH: f64 = 380.0;
const MENUBAR_HEIGHT_LOGICAL: f64 = 24.0;

// ============================================================
// `tok` CLI path discovery
// ============================================================
//
// `tok` is the tokkit Python CLI. It can live in many places depending
// on how the user installed Python: anaconda, miniconda, pyenv, brew,
// pipx, or a project venv. We probe common locations first, then fall
// back to a login-shell `which tok` to inherit the user's PATH.
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

/// Pricing for Anthropic models (as of 2026-05).
/// tokkit's built-in pricing is correct for OpenAI (where cached_input is a
/// subset of input), but undercounts Anthropic by ~6x because Anthropic's
/// `cache_read_input_tokens` is a SEPARATE field from `input_tokens`, not a
/// subset. We override Anthropic costs ourselves.
struct ModelPrice {
    input: f64,
    cached: f64,
    output: f64,
}

fn anthropic_price(model_label: &str) -> Option<ModelPrice> {
    let l = model_label.to_lowercase();
    if l.contains("opus") {
        Some(ModelPrice { input: 5.0, cached: 0.50, output: 25.0 })
    } else if l.contains("sonnet") {
        Some(ModelPrice { input: 3.0, cached: 0.30, output: 15.0 })
    } else if l.contains("haiku") {
        Some(ModelPrice { input: 1.0, cached: 0.10, output: 5.0 })
    } else {
        None
    }
}

fn cost_anthropic(input: u64, cached: u64, output: u64, model_label: &str) -> Option<f64> {
    let p = anthropic_price(model_label)?;
    Some(
        (input as f64 / 1_000_000.0) * p.input
            + (cached as f64 / 1_000_000.0) * p.cached
            + (output as f64 / 1_000_000.0) * p.output,
    )
}

/// Patch tokkit's per-model + total cost figures using correct Anthropic math.
/// Mutates `data` in place.
///
/// Why we need this:
/// tokkit assumes `cached_input_tokens` is a SUBSET of `input_tokens` (the
/// OpenAI model). For Anthropic, `cache_read_input_tokens` is a SEPARATE
/// field, so tokkit silently undercounts Anthropic costs by ~6x. We override
/// the cost for any model whose label contains "opus" / "sonnet" / "haiku".
///
/// Strategy:
/// 1. Recompute each `by_model[].estimated_cost_usd` for Anthropic models
/// 2. Compute the SUM ratio (corrected / original) from by_model
/// 3. Apply that ratio to every other rollup view (by_date, by_hour,
///    by_terminal, by_source) so chart proportions stay sane and the
///    grand totals match
/// 4. If a `totals` field exists (only present for `tok json today`),
///    overwrite it with the corrected sum
fn correct_anthropic_costs(data: &mut serde_json::Value) {
    // Step 1: recompute by_model costs, track sum before & after
    let mut sum_before: f64 = 0.0;
    let mut sum_after: f64 = 0.0;

    if let Some(by_model) = data.get_mut("by_model").and_then(|v| v.as_array_mut()) {
        for entry in by_model.iter_mut() {
            let old = entry
                .get("estimated_cost_usd")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            sum_before += old;

            let label = entry
                .get("model_label")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let input = entry
                .get("input_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let cached = entry
                .get("cached_input_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let output = entry
                .get("output_tokens")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            if let Some(corrected) = cost_anthropic(input, cached, output, &label) {
                entry["estimated_cost_usd"] =
                    serde_json::json!((corrected * 1e6).round() / 1e6);
                sum_after += corrected;
            } else {
                sum_after += old;
            }
        }
    }

    // Step 2: derive scaling ratio
    let ratio = if sum_before > 0.001 {
        sum_after / sum_before
    } else {
        return;
    };

    // Step 3: scale every other rollup view by the ratio
    for key in ["by_date", "by_hour", "by_terminal", "by_source"] {
        if let Some(arr) = data.get_mut(key).and_then(|v| v.as_array_mut()) {
            for entry in arr.iter_mut() {
                if let Some(cost) = entry.get_mut("estimated_cost_usd") {
                    let old = cost.as_f64().unwrap_or(0.0);
                    *cost = serde_json::json!(((old * ratio) * 1e6).round() / 1e6);
                }
            }
        }
    }

    // Step 4: if `totals` exists (only for `tok json today`), overwrite it
    if let Some(totals) = data.get_mut("totals") {
        if let Some(cost) = totals.get_mut("estimated_cost_usd") {
            *cost = serde_json::json!((sum_after * 1e6).round() / 1e6);
        }
    }
}

/// Run `tok json <args>` and return JSON with corrected Anthropic costs.
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

    let output = Command::new(&tok)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run tok: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "tok exited with status {}: {}",
            output.status, stderr
        ));
    }

    let mut data: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse tok json output: {}", e))?;

    correct_anthropic_costs(&mut data);
    Ok(data)
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
        .invoke_handler(tauri::generate_handler![get_usage, get_menubar_summary])
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
                                "window.dispatchEvent(new Event('toksee:refresh'))",
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
                        // Trigger data refresh in WebView (no full reload)
                        let _ = window.eval(
                            "window.dispatchEvent(new Event('toksee:refresh'))",
                        );
                    }
                })
                .build(app)?;

            // Initial menubar title refresh + hourly auto-refresh
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                refresh_menubar_title(&app_handle).await;
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                    refresh_menubar_title(&app_handle).await;
                    // Also nudge the popover to refresh
                    if let Some(window) = app_handle.get_webview_window("popover") {
                        let _ = window.eval(
                            "window.dispatchEvent(new Event('toksee:refresh'))",
                        );
                    }
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
