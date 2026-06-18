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
- [ ] **P1 — In-browser code editing + run-the-edit.** The learner must *write*, not just read. Replace the read-only `<pre>` with an editable editor (vendored CodeMirror 6 or a styled `<textarea>` fallback); Run/Check POST the edited buffer via the existing `code` field on `/api/run`. Persist edits per-exercise (localStorage + optional server scratch).
- [ ] **P1 — Functional nav tabs.** Dashboard / Exercise / Book / Roadmap are decorative. Wire view-switching; Book renders embedded chapters, Roadmap renders ROADMAP.md.
- [ ] **P1 — Hint ladder UI.** Keybar shows `h hint` but nothing is wired. Surface progressive hints (book-refs → conceptual nudge → narrowed location → near-solution), gated by attempt count. Never auto-reveal the fix.
- [ ] **P1 — Predict-first capture.** A small "lock your prediction" input (will it compile? which Exxxx?) before Run; compare to the real result in Compare step.
- [ ] **P1 — Pass → advance.** On a passing Run: mark done, unlock + set next current, refresh the list/gauge. Needs `POST /api/advance` (server-resolved).
- [ ] **P2 — Exercise click-to-select.** Clicking a list item sets it current (`POST /api/select`, validated against discovered ids only).
- [ ] **P2 — Syntax highlighting** in the code pane (escape-then-tokenize; safe).
- [ ] **P2 — Phone-first responsive reflow.** 3-col grid → single column under ~760px (the locked learner profile is phone-first).
- [ ] **P2 — Explain-from-diagnostic.** Click a diagnostic row → runs `explain` for that code.
- [ ] **P2 — Security headers.** CSP, X-Content-Type-Options, Referrer-Policy on served responses (secure-by-default; loopback-only already done).
- [ ] **P3 — a11y pass.** Keyboard nav, ARIA roles, focus rings, contrast (light + dark).

## B. CLI (rpro)
- [ ] **P0 — `init` seeds bundled exercises** + sets first current (today it only scaffolds empty dirs; the server auto-seeds but the CLI doesn't — inconsistent).
- [ ] **P1 — `hint` command** backed by the shared hint ladder (`hint`, `hint --solution`).
- [ ] **P1 — `exercise skip` / `reset`** flows; `progress` summary view.
- [ ] **P2 — `explain` offline fallback** when `rustc --explain` is unavailable.

## C. TUI
- [ ] **P1 — Editable exercise pane** + run-the-edit (parity with web).
- [ ] **P1 — Book reader content** (render real chapters, not placeholder).
- [ ] **P1 — Hint ladder** parity.
- [ ] **P2 — Roadmap tab** live from ROADMAP.md (already partially done).

## D. Educational engine (#2) — the shared core
- [ ] **P1 — Hint ladder model** (pure, wasm-safe, seam-clean crate `rpro-edu` or in `rpro-state`): levels keyed on (exercise, attempt count, observed error code). Unit-tested.
- [x] **P1 — Spaced repetition** keyed on error code — *model done* (`6bbdc39`): `rpro-state::review::ReviewState`, clock-free Leitner-box, pure+wasm-safe, in `Progress.reviews`, 3 tests.
  - [x] **P1 — wire it into the web surface** — rpro-serve `update_progress` folds each run via `fold_review` (guard: the exercise's *own* expected error is the lesson, not a miss — only learner-introduced errors record/reset); `GET /api/review` exposes the weakest-first queue + mastery counts; dashboard shows a "↻ Recall" widget. 5 unit tests + smoke assertion + live round-trip verified.
  - [ ] **P1 — mirror Recall in rpro-tui** (show the due queue + mastery on a key). *(web resurfacing is now live; TUI parity remains)*
- [ ] **P1 — Tutor (guide-not-solve)** prompt templates per step of PREDICT→RUN→COMPARE→READ-RAW→DIAGNOSE→GUIDE→EXPLAIN→RETRY→RECALL (spec: docs/EDUCATION.md).
- [ ] **P2 — Pluggable AI tutor seam** (Claude/Gemini) behind a trait; must refuse to type the fix.

## E. Content (#8) + corpus
- [ ] **P1 — Exercises for every phase.** Only `04-ownership` (3) exist. Author exercise sets for phases 1–3, 5–9 (each: `name.rs` failing + `name.toml` with book_refs, expected_error_code, solution_outline).
- [ ] **P1 — Embedded Book chapters.** `book/` ships READMEs only; bundle the real chapter markdown the exercises reference (ch04-01 etc.).
- [ ] **P2 — Corpus Phases 7–9** catalog+matrix (generics/traits/lifetimes → concurrency → advanced), proven extract→map→verify workflow.
- [ ] 🔒 **Error-handling matrix #37 section** — confirm scope with Paul (deferred Phase-5 follow-up).

## F. Packaging / distribution (#6) — 🔒 environment-gated here
- [ ] 🔒 **P3 — AppImage / .deb / .rpm / APK** (release.yml exists; needs CI / a machine with native toolchains — crates.io is blocked in this sandbox). Verify release.yml is correct; document local build.
- [ ] 🔒 **P3 — Android** (Termux `rust` embed) + **Desktop** (Tauri wrap of the same `gui/` frontend). Needs webkit2gtk/node/android-sdk — not buildable here.
- [ ] **P2 — Verify release.yml** by inspection + a dry-run lint (doable here).

## G. Audit & test (run periodically + before "done")
- [ ] **P1 — Full workspace test + build** green (currently 62 tests). Add tests for `rpro-serve` (endpoint unit/integration tests: op whitelist, no-leak of solution/expected_error, loopback bind).
- [ ] **P1 — seam-grep gate + wasm32 gate** pass (wasm needs the matched rustup toolchain — invoke `$TC/bin/cargo` + `$TC/bin/rustc`).
- [ ] **P1 — clippy** via the rustup stable toolchain (system cargo lacks it); fix the 2 known pedantic `similar_names` in rpro-cli.
- [ ] **P2 — E2E browser test** (Playwright): seed → render real exercise → Run → assert E0382 in terminal + Diagnostics; assert no solution/expected_error in page source.
- [ ] **P2 — Security review** of rpro-serve (input validation, path traversal, DoS via long runs) + dependency audit.
- [ ] **P2 — Lighthouse / a11y audit** of the web GUI.

## H. Lessons (#7) — 🔒 gated on Paul
- [ ] 🔒 Author Lessons 2–8 — blocked on **Paul's L1 calibration + Phase-5 matrix review** (hard rule). Cannot proceed without his review.

---

### Loop discipline
ONE heavy workflow at a time (rate limits). Small items: do directly per tick. Big
items (content phases, edu-engine, exercise authoring): one workflow, monitor across
ticks, then next. Commit every landed item; refresh `project-loop-state` memory each tick.
