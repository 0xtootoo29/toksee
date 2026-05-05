# Contributing to TokSee

Thanks for your interest! TokSee is a small open-source project — issues,
PRs, and feedback all welcome.

## Reporting issues

Before opening an issue, please:

1. Check existing [issues](https://github.com/0xtootoo29/toksee/issues) for duplicates.
2. Make sure your tokkit install works on its own:
   ```bash
   tok today
   tok json today | head
   ```
   If `tok` itself is broken, the issue is upstream — file it at
   [yao-cli-tools](https://github.com/yaojingang/yao-cli-tools/issues), not here.
3. Include your macOS version, TokSee version, and (if a cost calculation issue)
   a redacted excerpt of `tok json today` showing the raw token counts you expect.

## Contributing code

```bash
git clone https://github.com/0xtootoo29/toksee.git
cd toksee
bun install
bun run tauri dev
```

A few conventions:

- **Bilingual docs**: every change to `README.md` should also be reflected in
  `README.zh.md`, and vice-versa. The two files mirror each other.
- **No new dependencies without discussion**: TokSee aims to stay tiny
  (~3 MB binary). Open an issue first if you want to add a Rust crate or
  embed a JS framework.
- **Cost calculations**: any pricing change must cite an official source
  (Anthropic / OpenAI pricing page link) in the commit message.
- **Run `cargo clippy` and `cargo fmt`** on Rust changes.
- **Manual smoke test** before opening a PR:
  - Tray icon appears on menu bar
  - Click → popover opens flush against menu bar
  - All 4 tabs (today / 7d / 14d / 30d) load without errors
  - All 3 themes × 2 modes render correctly
  - Quitting via tray menu works

## Roadmap & priorities

Current priorities are listed in the [Roadmap](README.md#roadmap) section of
the README. PRs that move those items forward get the fastest review.

If you want to work on something not on the roadmap, please open an issue
first to discuss whether it fits the project's direction — this avoids
wasted effort.

## License

By contributing, you agree your contributions will be licensed under the
[MIT License](LICENSE), the same license as the project.
