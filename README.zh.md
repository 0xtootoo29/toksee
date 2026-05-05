# TokSee

[English](README.md) · **中文**

> 看见你的 Token。一款 macOS 菜单栏应用，实时追踪你在 Claude Code、Codex 等 AI 编码工具上的花费。

<p align="center">
  <img src="src/assets/png/toksee-256.png" width="128" alt="TokSee logo">
</p>

<p align="center">
  <strong>菜单栏里的 token 用量与花费仪表盘 — 一眼可见，永不离开你的电脑。</strong>
</p>

---

## 这是什么

- **常驻菜单栏** — 一眼看到当日花费
- **覆盖所有 AI 编码工具** — Claude Code、Codex（CLI + 桌面端）、GPT-5.5 等
- **3 套主题** — macOS 原生、Vivid 彩色、Linear/Vercel Pro 风（每套含浅色和深色）
- **4 个时间窗口** — 今日 / 7 天 / 14 天 / 30 天
- **按模型拆分** — 看清哪个模型最烧钱
- **每小时自动刷新** — 不用手动点
- **100% 本地** — 数据从不离开你的电脑

## 为什么需要它

Anthropic Console 和 OpenAI 后台只能告诉你"昨天的账单"，前提是你记得登进去看。它们不会告诉你 **5 分钟前那条命令刚烧了 $40 的 Opus token**。

TokSee 把这些信息放在你随时能看见的地方，让你**在账单送达之前**就能调整。

## v0.1 适用范围

> **本版本只统计 Claude / Codex 官方订阅账号的用量。**
>
> 如果你的 Claude Code / Codex 是用 Anthropic / OpenAI 官方 API key 登录（或者用官方桌面端），TokSee 能看到完整的真实用量，并按官方价格算成本。
>
> **v0.1 暂不支持**：三方 API 聚合服务（比如 Claude Code 配置了自定义 `base_url` 指向 OpenRouter / DeepInfra / AnyKey 之类的代理）。这种场景下 token 数可能不准（某些代理不暴露相同字段），成本一定不准（你的折扣价 ≠ 官方价）。
>
> **v0.2 会加**：自定义 base URL 检测 + 各聚合服务的定价覆盖。

## 系统要求

- macOS 11+（Big Sur 及以上）
- Apple Silicon (M1 / M2 / M3 / M4) — Intel 版本在路线图里
- 已安装 [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) — TokSee 把底层 token 扫描交给这个 CLI

## 安装

### 1. 装 tokkit（数据后端）

```bash
pip install "git+https://github.com/yaojingang/yao-cli-tools.git#subdirectory=tools/tokkit"
tok scan all
```

`tok scan all` 会初次扫描你电脑上所有能识别的 AI 工具。装完这一步，TokSee 才能读到数据。

### 2. 下载 TokSee

到 [Releases 页面](https://github.com/0xtootoo29/toksee/releases) 下载最新的 `.dmg` 文件，双击挂载。

### 3. 拖到 Applications

.dmg 弹出窗口里，把 `TokSee.app` 拖到 `Applications` 文件夹快捷方式上。

### 4. 首次启动 — 处理 Gatekeeper 警告

在 `/Applications` 里双击 `TokSee.app`，会看到这个弹窗：

> **"TokSee" 无法打开，因为它来自身份不明的开发者。**
> macOS 无法验证此 App 是否包含恶意软件。

这是没有付费 Apple Developer 证书（$99/年）的开源 app 的标准提示。TokSee 用了 **ad-hoc 自签**，macOS 知道这个二进制没被篡改 — 只是没法验证作者身份。

你只需要**手动放行一次**。有两种方法：

#### 方式 A — 右键 → 打开（最快，3 步）

1. 在 `/Applications` 里**右键**点 `TokSee.app`（或按住 Control 键点击）
2. 在弹出菜单里选 **打开**
3. 会出现一个带 **打开** 按钮的新对话框 — 点它
4. TokSee 启动。之后再开就静默了。

#### 方式 B — 系统设置放行（如果方式 A 的"打开"按钮是灰的，用这个）

1. 正常双击 TokSee，看到警告，点 **取消** 或 **完成**
2. 打开 **系统设置**（左上角苹果菜单 → 系统设置）
3. 在侧边栏点 **隐私与安全性**
4. 滚到 **安全性** 区域
5. 你会看到：**已阻止 "TokSee" 的使用，因为它不是来自已识别的开发者。**
6. 点该信息右侧的 **仍要打开**
7. 如果系统要求，用 Touch ID 或密码验证
8. 再试一次启动 TokSee — 这次会弹一个带 **打开** 按钮的确认对话框，点 **打开**

### 5. 完成

看屏幕右上角菜单栏，应该能看到 **7 柱图标 + token 数**。点击展开 popover。点 popover 右上角的滑块图标可以切换主题。

> 如果你装了 **Hidden Bar**、**Bartender** 或 **iStat Menus**，TokSee 图标可能默认被藏到隐藏区。从隐藏区拖出来就好。

## 成本计算

TokSee 用 tokkit 做 token 扫描，但**重写了 Anthropic 模型的 cost 计算**。tokkit 上游有个 bug：把 `cached_input_tokens` 当作 `input_tokens` 的子集（这是 OpenAI 的模型）。Anthropic 的 `cache_read_input_tokens` 是**独立字段**，所以 tokkit 默认把 Claude 的费用低估约 **6 倍**。

TokSee 用的官方 Anthropic 价格（每 1M tokens，2026-05）：

| 模型 | Input | Cached | Output |
|---|---|---|---|
| Claude Opus 4.7 / 4.6 | $5.00 | $0.50 | $25.00 |
| Claude Sonnet 4.6 / 4.5 | $3.00 | $0.30 | $15.00 |
| Claude Haiku 4.5 | $1.00 | $0.10 | $5.00 |

OpenAI / GPT 模型用 tokkit 内置定价（对 OpenAI 是对的）。

> **注意**：
> - Opus 4.7 的 1M context 溢价（$10/$1/$37.50）还没特殊处理，>200k token 的请求会略微低估。
> - 成本是按 **官方订阅价** 算的。如果你用三方 API 聚合服务（折扣价），数字会不对（见上面"v0.1 适用范围"）。

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

发布构建会自动 ad-hoc 签名，并同时产出 `.app` 和 `.dmg`，路径在 `src-tauri/target/release/bundle/`。

## 技术栈

- **[Tauri 2](https://tauri.app)** — Rust 后端 + WKWebView 前端
- **Rust** — 系统托盘、popover 定位、IPC、cost 重算
- **HTML/CSS/JS** — 主题、图表、切换器（无框架，无构建步骤）
- **[tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit)** — 本地 token 数据扫描

## 路线图

### v0.2.0
- [ ] 三方 API / 聚合服务支持（自定义 base URL，OpenRouter、DeepInfra 等）
- [ ] 聚合服务自定义定价覆盖
- [ ] Universal binary（Intel + ARM64）— 等绕过 Homebrew Rust 工具链限制
- [ ] 期间对比（今日 vs 昨日 delta）
- [ ] Opus 4.7 1M-context 溢价

### v0.3.0+
- [ ] **把 tokkit 打包进 .app 内部** — 不再要求用户 pip install Python 包（当前需要先 `pip install tokkit`；这一步会把 tokkit + 最小 Python runtime 一起冻进 `TokSee.app`，普通用户**下载即用**）
- [ ] 自定义刷新间隔
- [ ] 超过日预算时桌面通知
- [ ] CSV 导出
- [ ] 代码签名（等到有赞助再做）

## 致谢

- [tokkit](https://github.com/yaojingang/yao-cli-tools/tree/main/tools/tokkit) by [@yaojingang](https://github.com/yaojingang) — token 扫描的核心。TokSee 只是它上面的一层好看 UI。

## 更新日志

见 [CHANGELOG.md](CHANGELOG.md)（英文）查看版本历史。

## 贡献

见 [CONTRIBUTING.md](CONTRIBUTING.md)（英文）了解如何提 issue 和 PR。

## License

MIT — 见 [LICENSE](LICENSE)。
