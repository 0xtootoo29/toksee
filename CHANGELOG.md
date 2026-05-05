# Changelog

All notable changes to TokSee are documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
