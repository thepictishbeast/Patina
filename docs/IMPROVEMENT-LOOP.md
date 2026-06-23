# Improvement Loop — toward the greatest Rust learning platform

Living backlog for the open-ended improvement loop (started 2026-06-23). Goal:
make Tempered Studio the greatest Rust learning platform ever. Charter & rules:

- **Teaches Rust completely OFFLINE — no internet, no LLM required.** Deterministic
  teaching (compiler + book + matrix-driven lessons + hint ladder). An LLM is
  *optional, bring-your-own* only; never a dependency.
- **Never hand the answer.** Guide via predict-then-run, real compiler errors,
  escalating book-pointers. Force a genuine attempt first.
- **Gradual baby-steps**, reads→writes (build the writing muscle), no Python analogies.
- **FOSS-first**, prefer Rust FOSS (don't reinvent the wheel) — but write it
  ourselves in Rust when that's genuinely better. Do what's best, not easiest.

## Decisions (Paul, 2026-06-23)
- **Editor modes:** three tiers — **Learn** (syntax highlight + by-hand errors only;
  NO autocomplete/auto-fix/format — the student does the work) · **Assist** (+ inline
  diagnostics + hints on demand) · **Dev** (full assists, later rust-analyzer).
- **Full IDE:** wire **rust-analyzer** (Rust FOSS) via the existing `LspSpec`; build
  ourselves only where better. (Desktop-first or web-editor TBD during impl.)
- **Solutions / hints:** force an attempt before any hint; restrict the solution as
  hard as possible; escalate to book-pointers / "go review" when truly stuck; **never
  hand the literal answer**; keep solutions stored server-side.
- **Predict-then-run gate:** required in **Learn** mode only.
- **Exercise gating:** **soft gate** — lock later exercises until prereqs Done, with an
  explicit "jump ahead anyway".
- **Materials to integrate:** Rust by Example, Rustlings, Rust Cookbook, Exercism Rust
  (+ anything else found) — verify licensing (MIT/Apache/CC) before reuse.
- **AI tutor / LLM:** optional BYO only; the platform must teach fully without one.

## Backlog (status: ✅done · ▶next · ☐todo)

### Critical pedagogy
- ✅ **Hint ladder never hands the solution; force a real attempt first** (`af5d6fb`).
- ✅ **Predict-then-run gate (Learn mode)** (`158c997`): Run/Check reveal nothing until a
  compiles/fails guess is locked; re-arms each run (re-predict on retry). Verified live.
- ☐ **Soft exercise gating:** enforce prereq lock in `Progress::select`/`select_handler`
  + make the 🔒 real + an explicit "jump ahead"; today locked rows are clickable.
- ✅ **Runaway-run timeout** (`9434c8c`): default-on 30s cap on every surface (`0` opts
  out). Verified live — `loop {}` returns in ~timeout with a "stopped" note; UI unfreezes.
- ☐ **Monochrome compiler output:** pass `--color=always` / `CARGO_TERM_COLOR` so the
  GUI's promised ANSI colors actually appear.

### Editor / IDE
- ◐ **3 modes (Learn/Assist/Dev)** — switcher + persistence + Learn predict-gate DONE
  (`158c997`). Remaining: Learn must BLOCK autocomplete/auto-fix; Assist/Dev get inline
  diagnostics; reconcile the legacy header `FREE` badge with the new switcher.
- ☐ **Editable-pane syntax highlighting** (quick win; it's a plain `<textarea>` now) —
  overlay a highlighted `<pre>` (reuse `highlightRust`) behind a transparent textarea.
  Always-on (all modes).
- ☐ **Full IDE via rust-analyzer** (`LspSpec` defined, unconsumed) — big; Rust FOSS.

### Content / corpus
- ☐ **Lessons for Phases 2–9** (only Phase 1 / L1–L8 exist; matrix DRAFT rows 49–62 done).
- ☐ **Matrix: re-review rows 51–62** (rate-limited), add the missing back-references the
  DECIDED rows use, fix the `&amp;amp;` double-escape in row 58, decide Lesson# assignment.
- ☐ **More exercises** — only ~31; iterators/smart-pointers/async thin or absent.
- ☐ **Integrate Rust by Example / Rustlings / Cookbook / Exercism** (+ licensing).

### Robustness / audit
- ☐ **Golden test:** recompile every exercise starter, assert it still emits its
  `expected_error_code` (prevent .rs↔.toml↔lesson drift on toolchain bumps).
- ☐ **Offline-Android Run UX:** tapping Run offline shows a confusing static demo →
  show a guiding "Run needs the desktop app / a connection" message instead.
- ☐ Clippy hygiene: pre-existing `future not Send` (`?Send` toolchain) + `struct_excessive_bools`.

### Platform / distribution
- ☐ Web deploy (GH Pages / server) · online Run for Android (remote rpro-serve) ·
  F-Droid + Obtainium · signed APT/dnf repos.

## Audit cadence
Re-run the educational-fidelity audit + CI gate set after each batch of pedagogy
changes. **Throttle agent fan-out** — concurrent heavy workflows hit the rate limit
(2026-06-23); run audits sequentially / small.
