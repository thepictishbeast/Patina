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
- [x] Audit baseline: 122 tests + 18-assertion end-to-end smoke + curriculum-integrity, book-anchor & GUI-transform gates (all CI jobs) + cited security review (docs/AUDIT.md, docs/SECURITY.md)

## Phase 1 — Terminal UI (TUI)
- [x] Theme + status vocabulary (color, glyphs, throbber, NO_COLOR)
- [x] App model + tabbed event loop (dashboard / exercise / book / roadmap)
- [x] Dashboard: progress gauge, current exercise, up-next
- [x] Exercise view: raw output + additive diagnostics + Free/Learning badge
- [x] Live run: r/c compiles on a background thread, fills the raw pane
- [x] Book reader: chapter list + cleaned markdown (mdBook directives stripped, code listings linked out, blockquote rails, inline markup styled — code/bold/italic/links)
- [x] Roadmap screen (this view)
- [x] Book + roadmap content scroll (PgUp/PgDn)
- [x] Hint ladder on `h` + "↻ Recall" spaced-repetition panel (shared state)
- [x] Records each run into shared progress (attempt + spaced-rep + advance)
- [x] TestBackend render tests (dashboard/exercise/book/roadmap, incl. narrow reflow)

## Phase 2 — Web GUI (live; → Tauri v2 → Android/desktop)
- [x] Web shell (gui/): layout + visual system, vendored xterm.js (no CDN)
- [x] Local web server (rpro-serve): loopback axum, Core front-end, op-whitelist, security headers, body limit
- [x] Live terminal: xterm.js streams the REAL run output via the server (no pty needed)
- [x] Editable code pane + run-the-edit (textarea + per-exercise localStorage)
- [x] Full educational loop in-browser: predict-first → run → diagnose → hint ladder → pass→advance → ↻ Recall; click-a-diagnostic-to-explain; phone reflow; keyboard + a11y
- [x] Embedded Book reader (web): /api/book TOC + cleaned chapters with syntax-highlighted code, rich markdown (tables, emphasis, h1–h6, callouts); full-text search box (/api/book?q=, ranked hits → jump-to-match); exercise book-refs jump to the cited section (heading-slug anchors); tap a list item to switch exercise (/api/select)
- [ ] Wrap in Tauri v2 → desktop + Android (Termux rust), reusing this frontend (env-gated here)

## Phase 3 — Learning content
- [x] Merged corpus: concept matrix Phases 1–6 (compile-verified)
- [x] 32 rustc-verified exercises across 9 phases (basics → advanced), each with book refs + expected error + solution outline
- [x] 23 embedded Book chapters (bundled + seeded into the store; code listings link out to the live Book — see the vendor-vs-link decision below)
- [x] Education engine — spec (docs/EDUCATION.md) AND implementation: shared hint ladder, spaced-repetition, tutor guide-not-solve scaffolding
- [ ] Lesson 1 calibration (needs Paul) → unlocks Lessons 2–8
- [ ] Corpus concept-matrix for Phases 7–9; book code-listings: vendor ~200 files vs keep link-out (needs Paul)

## Phase 4 — Distribution & sync
- [x] Packaging plan + .deb/.rpm + release CI (docs/DISTRIBUTION.md)
- [x] AppImage of the `rpro` CLI/TUI + zsync auto-update (`scripts/build-appimage.sh`, release.yml `appimage` job; built + run-verified locally)
- [ ] GUI AppImage (needs `rpro-serve` relocatable — see BACKLOG §F); signed APT/dnf repos
- [ ] APK + F-Droid (mobile repo)
- [ ] Cross-device sync/backup of progress + code
