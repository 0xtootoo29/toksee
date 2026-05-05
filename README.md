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

### 1. Install tokkit (the data backend)

```bash
pip install "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
tok scan all
```

### 2. Download TokSee

Grab the latest `.dmg` from [Releases](https://github.com/0xtootoo29/toksee/releases). Double-click it and drag `TokSee.app` to the `Applications` folder shortcut inside.

### 3. First launch (one-time Gatekeeper warning)

TokSee is **ad-hoc signed** but not signed by an Apple Developer ID (a $99/yr membership). On first launch macOS will block it once with this dialog:

> **"TokSee" cannot be opened because it is from an unidentified developer.**

This is normal for open-source macOS apps. To allow it:

**Option A — Right-click → Open** (easiest):
1. Right-click `TokSee.app` in `/Applications` → **Open**
2. Click **Open** in the warning dialog
3. macOS remembers this. Next launches are silent.

**Option B — System Settings**:
1. Try to open TokSee normally (you'll see the warning, click Cancel)
2. Open **System Settings → Privacy & Security**
3. Scroll down — you'll see **"TokSee was blocked from use because it is not from an identified developer"**
4. Click **Open Anyway** → confirm
5. TokSee opens.

After this one-time approval, TokSee launches like any other app.

### 4. Done

Look for the 7-bar icon + token count in your menu bar (top right of screen). Click it.

> If you have **Hidden Bar** or **Bartender**, the icon may start in the hidden zone. Drag it out to make it always visible.

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

### 1. 装 tokkit（数据后端）

```bash
pip install "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
tok scan all
```

### 2. 下载 TokSee

到 [Releases](https://github.com/0xtootoo29/toksee/releases) 下载最新的 `.dmg` 文件。双击打开，把 `TokSee.app` 拖到里面的 `Applications` 文件夹快捷方式上。

### 3. 首次启动（一次性放行）

TokSee 用了 **ad-hoc 自签**（免费），但不是 Apple Developer 证书签的（要 $99/年）。所以首次启动 macOS 会拦一次：

> **"TokSee"无法打开，因为它来自身份不明的开发者。**

这是开源 macOS 应用的常见情况。两种方式放行：

**方式 A — 右键 → 打开**（最快）：
1. 在 `/Applications` 里**右键**点 `TokSee.app` → **打开**
2. 在警告对话框里点**打开**
3. macOS 会记住，之后再启动就静默了

**方式 B — 系统设置放行**：
1. 正常双击 TokSee（会看到警告，点取消）
2. 打开 **系统设置 → 隐私与安全性**
3. 滚到下面 — 会看到 **"已阻止"TokSee"的使用，因为它不是来自被认证的开发者"**
4. 点 **仍要打开** → 确认
5. TokSee 启动

完成一次性放行后，TokSee 跟其他 app 一样直接双击就开。

### 4. 完成

看屏幕右上角菜单栏，应该能看到 **7 柱图标 + token 数**。点击展开 popover。

> 如果你装了 **Hidden Bar** 或 **Bartender**，图标可能被藏起来了。从隐藏区拖出来就好。

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
