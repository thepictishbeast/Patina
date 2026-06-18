# Tempered Studio — Improvement & Completion Backlog

> Living execution queue for the "finish + perfect + audit all platforms" loop.
> Each loop tick: pick the highest-priority unblocked item, land it (build + test),
> commit, tick the box, save state. Blocked items are marked 🔒 with the gate.
> Priorities: **P0** = correctness/finish, **P1** = core UX/pedagogy, **P2** = polish,
> **P3** = reach/packaging.

Surfaces: **Web** (`rpro-serve`, shipped + live), **CLI** (`rpro`), **TUI** (`rpro` dashboard),
**Android** (Termux embed — planned), **Desktop** (Tauri wrap — planned).

---

## A. Web GUI (rpro-serve)
- [x] **P1 — In-browser code editing + run-the-edit** (`15c3bee`) — editable `<textarea>` + per-exercise localStorage + ↺reset; Run/Check POST the edited buffer via `/api/run` `code` (clamped 256KiB).
- [x] **P1 — Functional nav tabs** (`6d67b6a`) — Dashboard/Exercise/Book/Roadmap switch views; Roadmap renders `/api/roadmap`.
- [x] **P1 — Hint ladder UI** (web, `85a3b17`) — 💡Hint button climbs concept → expected-error → solution-outline (last), server-clamped, no early leak.
- [x] **P1 — Predict-first capture** (`56ed101`) — "lock your prediction" bar (compiles? which Exxxx?) → predicted-vs-actual after Run.
- [x] **P1 — Pass → advance** (`c5b8076`) — passing Run/Test marks Done + promotes next (server-resolved inside `/api/run`, returns `advanced_to`; no separate endpoint needed); list/gauge refresh.
- [ ] **P2 — Exercise click-to-select.** Clicking a list item sets it current (`POST /api/select`, validated against discovered ids only).
- [ ] **P2 — Syntax highlighting** in the code pane (escape-then-tokenize; safe).
- [x] **P2 — Phone-first responsive reflow** (`05c0777`) — 3-col grid → single column under 760px via `@media`; `.pane{min-width:0}` fix; verified at 390px.
- [ ] **P2 — Explain-from-diagnostic.** Click a diagnostic row → runs `explain` for that code.
- [x] **P2 — Security headers** (`9e7c12c`) — CSP/X-Content-Type-Options/Referrer-Policy/X-Frame-Options via a `map_response` layer; loopback-only already done.
- [x] **P3 — a11y pass** (`599b89f`) — keyboard nav (r/c/h/b/t/d, typing-guarded) + tablist/tab roles + aria-live + aria-labels. (Lighthouse audit = G65, still open.)

## B. CLI (rpro)
- [x] **P0 — `init` seeds bundled exercises** + sets first current (`43b2947`) — `copy_tree` mirrors the server's seeding; fresh HOME → exercises seeded, first set Current.
- [x] **P1 — `hint` command** backed by the shared hint ladder (this tick) — `rpro exercise hint [--level N] [--solution]` prints book refs + the laddered `ExerciseMetadata::hint` text (identical to web/TUI); `--solution` jumps to the top rung. Verified L1/L2/L3 output by hand.
- [x] **P1 — `exercise skip` / `reset`** flows + `progress` summary view (this tick + pre-existing) — `rpro exercise skip` marks current Skipped and advances; `rpro exercise reset` clears done/attempts for a fresh attempt; `rpro progress` summary already existed. Backed by new pure `Progress::set_skipped`/`reset` (2 tests). Verified by hand on a seeded store.
- [ ] **P2 — `explain` offline fallback** when `rustc --explain` is unavailable.

## C. TUI
- [ ] **P1 — Editable exercise pane** + run-the-edit (parity with web).
- [ ] **P1 — Book reader content** (render real chapters, not placeholder).
- [x] **P1 — Hint ladder** parity (`2318acc`→this) — `h` climbs the same shared `ExerciseMetadata::hint` ladder; revealed rung shown in a conditional panel (last resort flagged); resets on advance. 2 TestBackend tests.
- [x] **P2 — Roadmap tab** live from ROADMAP.md — `render_roadmap` renders the baked-in `docs/ROADMAP.md` with status-glyph styling.

## D. Educational engine (#2) — the shared core
- [x] **P1 — Hint ladder model** (this tick) — `ExerciseMetadata::hint(requested) -> (level, max, text)` in rpro-state: pure, wasm-safe, seam-clean, surface-agnostic wording; max_level 3 only with an outline (L1/L2 can never leak it). 2 tests. Web + TUI both call it.
- [x] **P1 — Spaced repetition** keyed on error code — *model done* (`6bbdc39`): `rpro-state::review::ReviewState`, clock-free Leitner-box, pure+wasm-safe, in `Progress.reviews`, 3 tests.
  - [x] **P1 — wire it into the web surface** — rpro-serve `update_progress` folds each run via `fold_review` (guard: the exercise's *own* expected error is the lesson, not a miss — only learner-introduced errors record/reset); `GET /api/review` exposes the weakest-first queue + mastery counts; dashboard shows a "↻ Recall" widget. 5 unit tests + smoke assertion + live round-trip verified.
  - [x] **P1 — mirror Recall in rpro-tui** — dashboard "↻ Recall" panel reads shared `Progress.reviews` (due weakest-first + N/M mastered), hidden until a concept is tracked, ASCII-safe. 3 TestBackend tests.
  - [x] **P1 — share progress-update + record on TUI runs** (refactor) — `ReviewState::fold_run` → rpro-state (pure, with the guard); `primary_error_code` + `record_run` → rpro-runner (the one helper web/TUI/CLI share). rpro-serve delegates; rpro-tui now records on every run (attempt + spaced-rep + advance) and refreshes the dashboard, so TUI runs fill the Recall panel + gauge. 2 `record_run` integration tests + relocated fold tests.
- [x] **P1 — Tutor (guide-not-solve)** templates (this tick) — `rpro-state::tutor`: `LoopStep` (9 steps, per-step tutor directive + actor + `is_tutor_turn`), `HintRung` (4 rungs), `GUARDRAILS` (7 enforcement rules), `guide_turn`/`guide_rung` over a `TutorContext` that takes NO solution input. Templates are meta-instructions to the LLM (the top rung *generates* a throwaway example — no stored fix), seam-clean (no toolchain/error-code literals), wasm-safe; 5 structural tests. **Scaffolding-level enforcement only** — guarantees the tutor's *instructions* forbid the fix, NOT that a backend obeys (the [D46] seam + behaviour are unverifiable here).
- [ ] **P2 — Pluggable AI tutor seam** (Claude/Gemini) behind a trait; must refuse to type the fix.

## E. Content (#8) + corpus
- [ ] **P1 — Exercises for every phase.** *Phases 1–6 DONE (23 exercises across basics/control-flow/collections/ownership/types/modules, each rustc-verified — `8b25d97` and the t12–16 cluster commits).* Remaining: **phases 7–9** (generics/traits/lifetimes → concurrency → advanced) — gated on the corpus matrix for those phases (E/Corpus 7–9 below).
- [ ] **P1 — Embedded Book chapters.** `book/` ships READMEs only; bundle the real chapter markdown the exercises reference (ch04-01 etc.).
- [ ] **P2 — Corpus Phases 7–9** catalog+matrix (generics/traits/lifetimes → concurrency → advanced), proven extract→map→verify workflow.
- [ ] 🔒 **Error-handling matrix #37 section** — confirm scope with Paul (deferred Phase-5 follow-up).

## F. Packaging / distribution (#6) — 🔒 environment-gated here
- [ ] 🔒 **P3 — AppImage / .deb / .rpm / APK** (release.yml exists; needs CI / a machine with native toolchains — crates.io is blocked in this sandbox). Verify release.yml is correct; document local build.
- [ ] 🔒 **P3 — Android** (Termux `rust` embed) + **Desktop** (Tauri wrap of the same `gui/` frontend). Needs webkit2gtk/node/android-sdk — not buildable here.
- [x] **P2 — Verify release.yml** (this tick) — all 3 workflows parse as valid YAML; deb/rpm asset paths match the built `rpro` binary; dispatch/tag fallback + `contents: write` correct; consistent with DISTRIBUTION.md (self-hosted `plausiden` runner is the documented intent). Added a `SHA256SUMS` artifact (collect + upload) for download integrity, and a "Local build & verify" how-to in DISTRIBUTION.md. Open hardening noted: artifact GPG signing, runner exec timeout (SECURITY.md F1).

## G. Audit & test (run periodically + before "done")
- [x] **P1 — Full workspace test + build** green — verified every tick; `rpro-serve` endpoint tests added (`bedab94`: op-whitelist, no-leak of solution/expected_error, hint gating) + `record_run` integration tests. Standing gate (re-run before "done").
- [x] **P1 — seam-grep gate + wasm32 gate** pass — verified every tick via `$TC` toolchain (`RUSTC=$TC/rustc $TC/cargo`, `RUSTDOC=$TC/rustdoc` for doctests); seam CLEAN, wasm32 pure-core builds. CI: `.github/workflows/seam-gates.yml`.
- [x] **P1 — clippy** named fix — the rpro-cli pedantic `similar_names` cleared in the workspace clippy pass (`4bba688`/`8f091bb`). NOTE: `cargo clippy` isn't installed in this sandbox toolchain; remaining pedantic/nursery lints are WARNINGS (non-gate).
- [ ] **P2 — E2E browser test** (Playwright): seed → render real exercise → Run → assert E0382 in terminal + Diagnostics; assert no solution/expected_error in page source.
- [x] **P2 — Security review** of rpro-serve + dependency audit → `docs/SECURITY.md` (this tick). Threat model (loopback/single-user/no privilege boundary), all controls cited by symbol, honest findings. Fixed F3 (added 1 MiB `DefaultBodyLimit` + propagate the extractor's real status → over-limit body now 413, was a blanket 400; smoke asserts it). Residual: F1 below, F2 nonce-CSP (informational).
- [ ] **P3 — Run execution timeout** (SECURITY.md F1) — `LocalProcess::exec` has no deadline, so `loop{}` hangs the worker until kill. *Availability, not a vuln* (loopback/self-DoS). Add a deadline+kill executor with threaded stdout/stderr capture (avoid pipe-buffer deadlock); env-configurable, `0` = off so CLI/TUI keep current behaviour.
- [ ] **P2 — Lighthouse / a11y audit** of the web GUI.

## H. Lessons (#7) — 🔒 gated on Paul
- [ ] 🔒 Author Lessons 2–8 — blocked on **Paul's L1 calibration + Phase-5 matrix review** (hard rule). Cannot proceed without his review.

---

### Loop discipline
ONE heavy workflow at a time (rate limits). Small items: do directly per tick. Big
items (content phases, edu-engine, exercise authoring): one workflow, monitor across
ticks, then next. Commit every landed item; refresh `project-loop-state` memory each tick.
