# TokSee

**English** · [中文](README.zh.md)

> See your tokens. A macOS menu bar app that tracks how much you spend on Claude Code, Codex, and other AI coding tools.

<p align="center">
  <img src="src/assets/png/toksee-256.png" width="128" alt="TokSee logo">
</p>

<p align="center">
  <strong>Real-time token usage and cost tracking — right in your menu bar.</strong>
</p>

---

## What it does

- **Lives in your menu bar** — click the icon to see today's spend
- **Tracks every AI coding tool** — Claude Code, Codex (CLI + Desktop), GPT-5.5, more
- **3 themes** — macOS Native, Vivid colorful, Linear/Vercel Pro (each with light & dark)
- **4 time windows** — today / 7d / 14d / 30d
- **Per-model breakdown** — see which model burns the most budget
- **Auto-refresh** — once an hour, no clicks needed
- **100% local** — all data stays on your machine, no telemetry

## Why

The official Anthropic Console / OpenAI dashboard show you yesterday's bill *if* you remember to check. They don't tell you that the script you ran 5 minutes ago just cost $40 in Opus tokens.

TokSee surfaces this glanceably so you can adjust **before** the bill arrives.

## v0.1 scope

> **This release tracks usage from official Claude / Codex subscription accounts only.**
>
> If you log into Claude Code or Codex with the standard Anthropic / OpenAI API keys (or use the official desktop apps), TokSee will see your actual usage and apply the correct official pricing.
>
> **Not yet supported in v0.1**: third-party API providers / aggregators (e.g., using Claude Code via a custom `base_url` pointing to an OpenRouter / DeepInfra / AnyKey-style proxy). The token counts will be off (some proxies don't surface the same fields) and the cost will be wrong (your discounted rate ≠ official rate).
>
> **v0.2 will add**: custom base URL detection + per-aggregator pricing overrides.

## Requirements

- macOS 11+ (Big Sur or later)
- Apple Silicon (M1 / M2 / M3 / M4) — Intel build is on the roadmap
- [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) installed locally — TokSee delegates raw token scanning to this excellent CLI

## Install

### 1. Install tokkit (the data backend)

```bash
pip install "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
tok scan all
```

`tok scan all` does an initial scan of every detectable AI tool on your machine. After this, TokSee can read the data.

### 2. Download TokSee

Grab the latest `.dmg` from the [Releases page](https://github.com/0xtootoo29/toksee/releases). Double-click it to mount.

### 3. Drag to Applications

In the .dmg window, drag `TokSee.app` onto the `Applications` folder shortcut.

### 4. First launch — Gatekeeper warning

Double-click `TokSee.app` in `/Applications`. You will see this dialog:

> **"TokSee" cannot be opened because it is from an unidentified developer.**
> macOS cannot verify that this app is free of malware.

This is normal for open-source apps without a paid Apple Developer certificate ($99/yr). TokSee is **ad-hoc signed** so macOS knows the binary hasn't been tampered with — it just can't verify the author identity.

You only need to allow it **once**. Two ways:

#### Option A — Right-click → Open (fastest, ~3 clicks)

1. In `/Applications`, **right-click** `TokSee.app` (or Control-click)
2. Select **Open** from the context menu
3. A new dialog appears with an **Open** button — click it
4. TokSee launches. Future launches are silent.

#### Option B — System Settings (works if Option A is greyed out)

1. Try to open TokSee normally. The warning appears. Click **Cancel** or **Done**.
2. Open **System Settings** (Apple menu → System Settings)
3. Click **Privacy & Security** in the sidebar
4. Scroll down to the **Security** section
5. You'll see: **"TokSee" was blocked from use because it is not from an identified developer.**
6. Click **Open Anyway** to the right of that message
7. Authenticate with Touch ID or your password if prompted
8. Try opening TokSee again — now you'll see a confirmation dialog with an **Open** button. Click it.

### 5. Done

Look for the **7-bar icon + token count** in your menu bar (top right of screen). Click it to see today's usage. Click the sliders icon (top right of the popover) to switch themes.

> If you have **Hidden Bar**, **Bartender**, or **iStat Menus**, the TokSee icon may default to the hidden zone. Drag it out of hiding to make it always visible.

## Cost calculation

TokSee uses tokkit for both token scanning and cost computation. As of TokSee v0.1.1 (and tokkit `main` after [yao-cli-tools#3](https://github.com/yaojingang/yao-cli-tools/pull/3)), tokkit's `pricing.py` correctly accounts for Anthropic's disjoint `cache_read_input_tokens` field, so no client-side override is needed.

> **Make sure your tokkit is up to date** — older tokkit (≤ commit `c9ed365`) silently undercounts Claude costs ~6× because it treats `cached_input_tokens` as a subset of `input_tokens`. Reinstall with:
> ```
> pip install --force-reinstall "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
> ```

> **Caveats**:
> - Opus 4.7's 1M-context premium pricing ($10/$1/$37.50) is not yet special-cased — costs for >200k token requests will be slightly underestimated.
> - Costs assume **official subscription rates**. If you use a third-party API aggregator with custom rates, the numbers will be wrong (see "v0.1 scope" above).

## Themes

Click the sliders icon (top-right of the popover) to switch:

- **Native** — macOS system look (vibrancy + SF Pro + system blue)
- **Vivid** — pink/purple gradient hero card, rounded
- **Pro** — Linear / Vercel SaaS aesthetic, monochrome

Each theme has light, dark, and "follow system" modes. Choice persists across launches.

## Build from source

```bash
git clone https://github.com/0xtootoo29/toksee.git
cd toksee
bun install
bun run tauri dev      # development mode (hot reload)
bun run tauri build    # release build → src-tauri/target/release/bundle/macos/TokSee.app
```

The release build ad-hoc signs the .app and produces both a `.app` and a `.dmg` in `src-tauri/target/release/bundle/`.

## Tech stack

- **[Tauri 2](https://tauri.app)** — Rust backend + WKWebView frontend
- **Rust** — system tray, popover positioning, IPC, cost recomputation
- **HTML/CSS/JS** — themes, charts, theme switcher (no framework, no build step)
- **[tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit)** — local token data scanning

## Roadmap

### v0.2.0
- [ ] Third-party API / aggregator support (custom base URLs, OpenRouter, DeepInfra, etc.)
- [ ] Custom pricing override for aggregator rates
- [ ] Universal binary (Intel + ARM64) once we work around the Homebrew Rust toolchain
- [ ] Compare period-over-period (today vs yesterday delta)
- [ ] Opus 4.7 1M-context premium pricing

### v0.3.0+
- [ ] **Bundle tokkit into the .app** — no Python install required (currently users must `pip install tokkit`; this would freeze tokkit + a minimal Python runtime inside `TokSee.app` so end-users can just download and run)
- [ ] Configurable refresh interval
- [ ] Notification when daily budget exceeded
- [ ] Export usage as CSV
- [ ] Code signing once we have a sponsor

## Acknowledgments

- [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) by [@yaojingang](https://github.com/yaojingang) — the heavy lifting on token scanning. TokSee is just a pretty UI on top.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to file issues and submit PRs.

## License

MIT — see [LICENSE](LICENSE).
