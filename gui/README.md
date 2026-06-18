# Tempered Studio — GUI shell (frontend)

The **magnificent host shell**. The bottom pane is the **embedded terminal** —
the real `rpro` TUI + `cargo`/`rustc` running *inside* the GUI (Termux's terminal
on Android; a pty on desktop; a backend pty over WebSocket on web). That is the
GUI⊕TUI coupling: the graphical shell wraps a live terminal, it doesn't replace
it. Implements the visual system in `../docs/UX.md`.

## Test it now (zero install)

It's a self-contained static file — no build step, no CDN, works offline.

```sh
cd gui && python3 -m http.server 8799      # python3 is already on the box
# then browse to http://127.0.0.1:8799/  (over SSH: ssh -L 8799:127.0.0.1:8799 …)
```

Or just open `gui/index.html` in any browser.

> Note: emoji render only if your system has an emoji font (most do; a headless
> server may show boxes). The layout/colors/animation don't depend on emoji.

## How this becomes all three surfaces

One frontend, wrapped by **Tauri v2** (Rust core + webview), consuming the
language-seam crates (`rpro-lang`, `rpro-core`, …):

| Surface | How | Priority (per Paul) |
|---|---|---|
| **Android** | Tauri v2 mobile; embedded terminal = Termux `rust` package (no proot) | most **useful** |
| **Linux desktop** | Tauri v2 desktop (webkit2gtk); pty-backed terminal; rust-analyzer | most **capable** |
| **Web** | same frontend, remote-toolchain backend; this static shell | easiest to **test/use** |

Next wiring steps: real `xterm.js` terminal ↔ a pty/WebSocket backend that runs
`rpro`; Monaco/CodeMirror editor; then the Tauri shell.
