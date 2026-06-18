# Tempered Studio — Roadmap

The build order and where we are. The in-app **Roadmap** tab renders this file,
so the learner (and Paul) can see what's done and what's next. Status:
`[x]` done · `[>]` in progress · `[ ]` planned.

## Phase 0 — Language seam & engine
- [x] Pure language seam (rpro-lang): traits + neutral types, wasm-safe
- [x] Rust as the one language crate (languages/rust) behind the seam
- [x] Async orchestrator (rpro-core): Language + Toolchain + Storage
- [x] Local toolchain + filesystem storage impls
- [x] CI gates: wasm32 pure-core build + language-seam grep guard
- [x] rpro run / check / test / explain — real cargo/rustc, by hand

## Phase 1 — Terminal UI (TUI)
- [x] Theme + status vocabulary (color, glyphs, throbber, NO_COLOR)
- [x] App model + tabbed event loop (dashboard / exercise / book / roadmap)
- [x] Dashboard: progress gauge, current exercise, up-next
- [x] Exercise view: raw output + additive diagnostics + Free/Learning badge
- [x] Live run: r/c compiles on a background thread, fills the raw pane
- [x] Book reader: chapter list + markdown
- [>] Roadmap screen (this view)
- [ ] Nicer markdown rendering + content scroll
- [ ] Snapshot tests at wide + narrow widths

## Phase 2 — GUI shell (web → Tauri v2 → Android/desktop)
- [x] Magnificent web shell (gui/): layout + visual system, render-verified
- [ ] Live embedded terminal (xterm.js ↔ pty backend) running rpro
- [ ] Editor pane (Monaco/CodeMirror)
- [ ] Wrap in Tauri v2 → desktop + Android (Termux rust), web frontend reuse

## Phase 3 — Learning content
- [x] Merged corpus: concept matrix Phases 1–5 (compile-verified)
- [x] Education engine spec: interactive loop + hint ladder (docs/EDUCATION.md)
- [ ] Lesson 1 calibration (needs Paul) → unlocks Lessons 2–8
- [ ] Content Phases 6–9 (generics/traits/lifetimes → concurrency → advanced)

## Phase 4 — Distribution & sync
- [x] Packaging plan + .deb/.rpm + release CI (docs/DISTRIBUTION.md)
- [ ] AppImage + auto-update (zsync); signed APT/dnf repos
- [ ] APK + F-Droid (mobile repo)
- [ ] Cross-device sync/backup of progress + code
