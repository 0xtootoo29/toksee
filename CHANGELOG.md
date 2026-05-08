# Changelog

All notable changes to TokSee are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.3] — 2026-05-08

Major visual redesign. The popover frontend is rewritten end-to-end on a
new design system co-developed with Claude Design.

### Added

- **Three color schemes**: 商务 / Business (macOS Liquid Glass, system blue,
  default), 活泼 / Vibrant (Sunset Aurora — magenta-to-indigo gradient hero
  strip, white luminous numerals), 极简 / Minimal (Stripe / Mercury soft
  cloud, royal-blue accent). Switch via the gear icon in the popover header.
- **Light / Dark / Auto** mode picker. Auto follows the system color scheme
  via `prefers-color-scheme`.
- **English ↔ 中文** language toggle (i18n on every label, period name,
  metric key, footer text).
- Multi-metric **hero strip** replaces the single-number hero — token total,
  delta, COST · RECORDS · AVG/CALL all in one glance.
- **Chart side rail** shows NOW / PEAK price flag with token count.
- All preferences (scheme, mode, lang) persist in `localStorage`.

### Changed

- Popover surface uses `backdrop-filter: blur(40px)` for real macOS-style
  vibrancy in the Business scheme; Vibrant gets a saturated pink → magenta
  → indigo gradient *inside* the hero strip while the rest of the popover
  stays clean white; Minimal goes pure Stripe-style monochrome.
- Bar chart re-styled: rounded micro-radius bars, today's bar shifts to
  scheme-specific accent color (indigo for Vibrant, blue for Business),
  peak bar highlighted, side flag tag pinned to the now/peak position.
- Model rows now use SF Pro Display tabular figures, share progress bars
  with scheme-aware fills, version chip next to model name (e.g. "Claude
  Opus 4.5").
- `src/index.html` rewritten — was 1438 lines, now 1528 lines including
  the full template + IPC bridge. The 3 v0.1.2 themes (Native / Vivid / Pro)
  are removed; replaced by Business / Vibrant / Minimal.

### Preserved

- All v0.1.2 wiring: hourly clock-aligned auto refresh from Rust, manual
  refresh that does NOT shift the schedule, `TOKKIT_TIMEZONE` env injection
  for hour labels in local time, "完整报告" link generates a tokkit HTML
  report scoped to the active tab with the upstream range switcher hidden,
  next-update HH:MM footer.
- All Tauri IPC commands (`get_usage`, `open_html_report`,
  `get_menubar_summary`).

[0.1.3]: https://github.com/0xtootoo29/toksee/releases/tag/v0.1.3

---

## [0.1.2] — 2026-05-06

Wired up four interactive elements that were inert in v0.1.0/v0.1.1.

### Added

- **Refresh button** in the popover header now actually refetches `tok json`
  (previously a decorative icon with no `onclick`).
- **完整报告 / Full report** link now opens a tokkit HTML report scoped to the
  active tab (今日 → 1 day, 7d/14d/30d → matching window). Generates via
  `tok html last <N>` to `~/.tokkit/reports/`, patches the rendered HTML to
  hide the upstream 7/14/30-day range switcher (it does subset filtering on
  already-embedded data, so on a 1-/7-/14-day report the inactive buttons
  silently do nothing — clearer to remove the dead control than to leave it
  inert), then hands the file to the OS default browser. Tokkit itself is
  untouched; only TokSee-rendered opens are patched.
- **下次更新 HH:MM** footer now shows the next clock-aligned hour (e.g. open
  the popover at 14:32 → "下次更新 15:00"). Re-seeded on popover open and on
  every hourly auto-refresh; unaffected by manual refresh and tab switches —
  manual refresh is for "I want fresh data NOW", not for shifting the
  scheduled cadence.

### Changed

- Hourly auto-refresh now aligns to the local clock hour instead of running
  3600s after launch (`src-tauri/src/lib.rs`). Open at 14:32 → first auto
  refresh fires at 15:00, then 16:00, 17:00 … matching what the footer
  predicts. Integer-hour timezones only; half-hour zones (IN/NP/NL) will be
  ~30/45min off — acceptable for v0.1.
- Today's hourly chart X-axis labels now show full `HH:MM` (e.g. "08:00") in
  the user's wall clock — see the timezone fix below for why this matters.
- Rust → WebView refresh dispatches now use `CustomEvent` with
  `detail.source ∈ {"auto", "manual"}` so the WebView can tell hourly
  refreshes apart from user-initiated ones.

### Fixed

- **Hour labels were UTC instead of local time.** tokkit's
  `utils.get_timezone()` falls back to UTC on macOS because `tzname()`
  returns abbreviations ("CST" / "PDT") that aren't valid IANA names —
  `ZoneInfo("CST")` raises and tokkit silently uses UTC. As a result a
  09:00 Beijing token spike showed up on the chart at "01:00".
  TokSee now reads `/etc/localtime` and forwards `TOKKIT_TIMEZONE` to
  every `tok` invocation (`get_usage`, `open_html_report`), so chart
  labels and `local_date` boundaries match the wall clock.
  Upstream tokkit fix tracked separately.
- Removed the dead "⌘ R" keybind hint next to "完整报告" — there is no
  command-R shortcut wired up; the link is click-only for now.

[0.1.2]: https://github.com/0xtootoo29/toksee/releases/tag/v0.1.2

---

## [0.1.1] — 2026-05-05

### Removed

- `correct_anthropic_costs()` Rust workaround (`src-tauri/src/lib.rs`,
  ~115 lines). Upstream tokkit fixed the `cached_input_tokens` accounting
  for Anthropic models in
  [yao-cli-tools#3](https://github.com/yaojingang/yao-cli-tools/pull/3)
  (closing [yao-cli-tools#2](https://github.com/yaojingang/yao-cli-tools/issues/2)).
  TokSee now passes tokkit's JSON through unchanged — costs are computed
  upstream with the correct disjoint cache-read accounting.

### Required

- Reinstall tokkit from latest `main` to pick up the fix:
  `pip install --force-reinstall "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"`.
  Older tokkit installs will still undercount Claude costs ~6× — the
  workaround that previously masked this is gone, so the wrong number
  will be visible until tokkit is upgraded.

[0.1.1]: https://github.com/0xtootoo29/toksee/releases/tag/v0.1.1

---

## [0.1.0] — 2026-05-04

First public release. macOS menu bar app for tracking token usage and cost
across Claude Code, Codex, and other AI coding tools.

### Added

- Menu bar tray icon with today's token count (auto-updates hourly).
- Popover dashboard with 4 time windows: today / 7d / 14d / 30d.
- Per-model breakdown sorted by cost.
- 3 themes (Native / Vivid / Pro), each with light + dark + auto modes.
  Theme persists across launches via `localStorage`.
- 7-day trend chart with peak-day highlight and today indicator.
- Live data via [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) — TokSee shells out to `tok json today/last N`,
  no Python embedded.
- macOS `.dmg` and `.zip` distributions, both ad-hoc signed with hardened runtime.
- Bilingual README ([English](README.md) + [中文](README.zh.md)).

### Fixed

- **Anthropic cost undercount**: tokkit's `pricing.py` treats `cached_input_tokens`
  as a subset of `input_tokens` (the OpenAI model). Anthropic's
  `cache_read_input_tokens` is a *separate* field, so tokkit silently undercounts
  Claude costs by ~6×. TokSee recomputes Anthropic costs in Rust using official
  per-model rates. Reported upstream as
  [yao-cli-tools#2](https://github.com/yaojingang/yao-cli-tools/issues/2).

### Known limitations

- **Apple Silicon only** for now. Intel users can build from source.
  Cross-compile blocked on Homebrew Rust toolchain — universal binary planned for v0.2.
- **Official subscription accounts only**. Third-party API aggregators
  (OpenRouter, DeepInfra, etc.) not yet supported — costs will be wrong if
  you route through a custom `base_url`. Planned for v0.2.
- **Opus 4.7 1M-context premium pricing** not special-cased — costs for
  >200k token requests slightly underestimated.
- **First launch requires Gatekeeper bypass** (right-click → Open, or System
  Settings → Privacy & Security → Open Anyway) because we're not signed with a
  paid Apple Developer certificate.

[0.1.0]: https://github.com/0xtootoo29/toksee/releases/tag/v0.1.0
