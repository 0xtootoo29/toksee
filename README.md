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

### 3. First launch (Gatekeeper bypass) — IMPORTANT

TokSee is not code-signed (Apple Developer membership costs $99/yr — not worth it for an open-source tool). On first launch macOS will refuse to open the app with one of these messages:

> **"TokSee is damaged and can't be opened. You should move it to the Trash."**
> (macOS Sequoia 15+)

> **"TokSee can't be opened because Apple cannot check it for malicious software."**
> (older macOS versions)

**Do NOT move it to the Trash.** The app isn't damaged — macOS just adds a quarantine flag to anything downloaded from a browser. Run this one command to clear the flag:

```bash
xattr -cr /Applications/TokSee.app
```

After that, double-click TokSee.app like any other app.

> **Why doesn't right-click → Open work?** On macOS Sequoia 15+, Apple removed the right-click bypass for apps marked as "damaged". The `xattr -cr` command is now the only easy fix for unsigned open-source apps.

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

---

# 中文说明

> See your tokens. macOS 菜单栏小应用，实时追踪 Claude Code、Codex 等 AI 编码工具的 token 用量和花费。

## 这是什么

- **常驻菜单栏** — 一眼看到当日花费
- **覆盖所有 AI 编码工具** — Claude Code、Codex (CLI + Desktop)、GPT-5.5 等
- **3 套主题** — macOS Native / Vivid 彩色 / Linear & Vercel Pro 风（每套都有浅色和深色）
- **4 个时间窗口** — 今日 / 7 天 / 14 天 / 30 天
- **按模型拆分** — 看清哪个模型最烧钱
- **每小时自动刷新** — 不用手动点
- **100% 本地** — 数据从不离开你的电脑

## 为什么需要它

Anthropic Console 和 OpenAI 后台只能告诉你"昨天的账单"，前提是你记得登进去看。它们不会告诉你**5 分钟前那条命令刚烧了 $40 的 Opus token**。

TokSee 把这个信息放在你随时能看见的地方，让你**在账单送达之前**就能调整。

## 系统要求

- macOS 11+ (Big Sur 及以上)
- Apple Silicon (M1/M2/M3/M4) — Intel 版本在路线图里
- 已安装 [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) — TokSee 把底层 token 扫描交给这个 CLI

## 安装

### 1. 安装 tokkit

```bash
pip install "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
```

确认能跑：

```bash
tok scan claude-code
tok scan codex
tok today
```

### 2. 下载 TokSee

到 [Releases](https://github.com/0xtootoo29/toksee/releases) 下载最新的 `.app.zip`，解压，把 `TokSee.app` 拖到 `/Applications`。

### 3. 首次启动 — 重要

TokSee 没有代码签名（Apple Developer 账号要 $99/年，开源工具不值得），首次启动 macOS 会拒绝打开，弹出：

> **"TokSee 已损坏，无法打开。您应该将它移到废纸篓。"**（macOS Sequoia 15+）
>
> **"无法打开"TokSee"，因为 Apple 无法检查其是否包含恶意软件。"**（更老版本）

**不要把它扔进废纸篓。** 应用没坏 — macOS 只是给从浏览器下载的文件加了 quarantine 标记。终端里跑这一条命令清除：

```bash
xattr -cr /Applications/TokSee.app
```

之后双击 TokSee.app 就跟其他应用一样能开。

> **为什么右键 → 打开不管用了？** macOS Sequoia 15+ Apple 取消了 "damaged" 标记应用的右键绕过。对未签名的开源应用，`xattr -cr` 现在是唯一简单方法。

## 成本计算

TokSee 用 tokkit 做 token 扫描，但**重写了 Anthropic 模型的 cost 计算**。tokkit 上游有个 bug：把 `cached_input_tokens` 当作 `input_tokens` 的子集（这是 OpenAI 的模型）。Anthropic 的 `cache_read_input_tokens` 是**独立字段**，所以 tokkit 默认把 Claude 的费用低估约 **6 倍**。

TokSee 用的官方 Anthropic 价格（每 1M tokens，2026-05）：

| 模型 | Input | Cached | Output |
|---|---|---|---|
| Claude Opus 4.7 / 4.6 | $5.00 | $0.50 | $25.00 |
| Claude Sonnet 4.6 / 4.5 | $3.00 | $0.30 | $15.00 |
| Claude Haiku 4.5 | $1.00 | $0.10 | $5.00 |

OpenAI / GPT 模型用 tokkit 内置定价（对 OpenAI 是对的）。

> **注意**：Opus 4.7 的 1M context 溢价 ($10/$1/$37.50) 还没特殊处理，>200k token 的请求会略微低估。

## 主题切换

点 popover 右上角的滑块图标切换：

- **Native** — macOS 原生质感（vibrancy + SF Pro + 系统蓝）
- **Vivid** — 粉紫渐变 hero 卡，圆角，Notion 风
- **Pro** — Linear / Vercel SaaS 仪表盘风，黑白点缀

每套都有 **浅色 / 深色 / 跟随系统** 三种模式。选择会保存。

## 从源码构建

```bash
git clone https://github.com/0xtootoo29/toksee.git
cd toksee
bun install
bun run tauri dev      # 开发模式（热重载）
bun run tauri build    # 发布构建 → src-tauri/target/release/bundle/macos/TokSee.app
```

## 技术栈

- **[Tauri 2](https://tauri.app)** — Rust 后端 + WKWebView 前端
- **Rust** — 系统托盘、popover 定位、IPC、cost 重算
- **HTML/CSS/JS** — 主题、图表、切换器（无框架，无构建步骤）
- **[tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit)** — 本地 token 数据扫描

## 路线图

- [ ] Universal binary（Intel + ARM64）— 等绕过 Homebrew Rust 限制
- [ ] 实时刷新（目前每小时）
- [ ] 自定义刷新间隔
- [ ] 超过日预算时桌面通知
- [ ] CSV 导出
- [ ] 期间对比（今日 vs 昨日 delta）
- [ ] Opus 4.7 1M-context 溢价
- [ ] 新模型自定义定价
- [ ] 等到有赞助再做代码签名

## 致谢

- [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) by [@yaojingang](https://github.com/yaojingang) — token 扫描的核心。TokSee 只是它上面的一层好看 UI。

## License

MIT — 见 [LICENSE](LICENSE)。
