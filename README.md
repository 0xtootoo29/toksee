# TokSee

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

## Requirements

- macOS 11+ (Big Sur or later)
- Apple Silicon (M1 / M2 / M3 / M4) — Intel build is on the roadmap
- [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) installed locally — TokSee delegates raw token scanning to this excellent CLI

## Install

### 1. Install tokkit

```bash
pip install "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
```

Confirm it works:

```bash
tok scan claude-code
tok scan codex
tok today
```

### 2. Download TokSee

Grab the latest `.app.zip` from [Releases](https://github.com/0xtootoo29/toksee/releases), unzip, drag `TokSee.app` to `/Applications`.

### 3. First launch (Gatekeeper bypass)

TokSee is not code-signed (Apple Developer membership costs $99/yr — not worth it for an open-source tool). On first launch macOS will refuse to open the app:

> **"TokSee can't be opened because Apple cannot check it for malicious software"**

Two ways to bypass:

**Option A — Right-click → Open (one-time):**
1. Right-click `TokSee.app` in Finder → `Open`
2. Click `Open` in the warning dialog
3. Done. macOS remembers this for future launches.

**Option B — Terminal (also one-time):**
```bash
xattr -d com.apple.quarantine /Applications/TokSee.app
```

After this, TokSee launches normally like any other app.

## Cost calculation

TokSee uses tokkit for token scanning, but **overrides the cost calculation for Anthropic models**. tokkit upstream has a bug: it treats `cached_input_tokens` as a subset of `input_tokens` (the OpenAI model). Anthropic's `cache_read_input_tokens` is a *separate* field, so tokkit silently undercounts Claude costs by ~6x.

TokSee's prices (per 1M tokens, as of 2026-05):

| Model | Input | Cached | Output |
|---|---|---|---|
| Claude Opus 4.7 / 4.6 | $5.00 | $0.50 | $25.00 |
| Claude Sonnet 4.6 / 4.5 | $3.00 | $0.30 | $15.00 |
| Claude Haiku 4.5 | $1.00 | $0.10 | $5.00 |

OpenAI / GPT models use tokkit's built-in pricing (which is correct for OpenAI's API).

**Note**: Opus 4.7 1M-context premium ($10/$1/$37.50) is not yet special-cased — costs for >200k token requests will be slightly underestimated.

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

## Tech stack

- **[Tauri 2](https://tauri.app)** — Rust backend + WKWebView frontend
- **Rust** — system tray, popover positioning, IPC, cost recomputation
- **HTML/CSS/JS** — themes, charts, theme switcher (no framework, no build step)
- **[tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit)** — local token data scanning

## Roadmap

- [ ] Universal binary (Intel + ARM64) once we work around the Homebrew Rust toolchain
- [ ] Real-time refresh (currently hourly)
- [ ] Configurable refresh interval
- [ ] Notification when daily budget exceeded
- [ ] Export usage as CSV
- [ ] Compare period-over-period (today vs yesterday delta)
- [ ] Opus 4.7 1M-context premium pricing
- [ ] Custom pricing override for new models
- [ ] Code signing once we have a sponsor

## Acknowledgments

- [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) by [@yaojingang](https://github.com/yaojingang) — the heavy lifting on token scanning. TokSee is just a pretty UI on top.

## License

MIT — see [LICENSE](LICENSE).
