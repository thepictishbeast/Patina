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
- ✅ **Soft exercise gating** (`<pending>`): `Progress::is_unlocked` (sequential, +test) +
  select_handler 423 LOCKED unless `force` + gui "jump ahead?" prompt. Verified live.
- ✅ **Runaway-run timeout** (`9434c8c`): default-on 30s cap on every surface (`0` opts
  out). Verified live — `loop {}` returns in ~timeout with a "stopped" note; UI unfreezes.
- ⊘ **Monochrome compiler output** — DEFERRED (not a quick win): forcing color at the seam
  GARBLES the TUI (it renders raw stdout/stderr with no ANSI parser), and rpro-serve runs
  via core.run with no env hook while the workspace forbids unsafe env-mutation. Needs a
  per-surface color flag plumbed through Core/command_plan (web on, TUI off).

### Editor / IDE
- ◐ **3 modes (Learn/Assist/Dev)** — switcher + persistence + Learn predict-gate DONE
  (`158c997`); legacy header `FREE` badge **removed** (`<pending>`) so the switcher is the sole
  mode authority (verified live: header renders, badge gone, no a11y/contrast change). Remaining:
  Learn must BLOCK autocomplete/auto-fix; Assist/Dev get inline diagnostics.
- ✅ **Editable-pane syntax highlighting** (`<pending>`): colored `<pre>` overlay behind a
  transparent textarea, reusing highlightRust; always-on. Verified live.
- ☐ **Full IDE via rust-analyzer** (`LspSpec` defined, unconsumed) — big; Rust FOSS.

### Content / corpus
- ☐ **Lessons for Phases 2–9** (only Phase 1 / L1–L8 exist; matrix DRAFT rows 49–62 done).
- ☐ **Matrix: re-review rows 51–62** (rate-limited), add the missing back-references the
  DECIDED rows use, fix the `&amp;amp;` double-escape in row 58, decide Lesson# assignment.
- ◐ **More exercises / functional phase** — created `07b-functional-and-smart-pointers` and
  relocated the misplaced iterators exercise (was in 09-advanced) into it. Also relocated
  `09-advanced/03_move_out_of_borrow` (E0507, core ownership) → `04-ownership/04_move_out_of_borrow`
  (id `advanced/…`→`ownership/…`, difficulty `advanced`→`beginner` to match the phase); ramp is now
  move → borrow → dangling → move-out-of-borrow. `09-advanced` is down to `01_unsafe_deref`.
  TODO: closures + smart-pointers exercises (smart-pointers needs Book ch15 bundled — not currently
  in book/); grow async coverage in 08-concurrency.
- ◐ **Integrate Rust by Example / Rustlings / Cookbook / Exercism** — licenses verified via gh
  (RBE Apache-2.0, Rustlings MIT, Cookbook CC0-1.0, Exercism MIT) + cataloged in
  rust-textbook/catalog/EXTERNAL-MATERIALS.md with an integration plan (Rustlings first).
  Next: adapt a few early Rustlings exercises into exercises/ (re-verified + attributed).

### Robustness / audit
- ✅ **Golden-corpus test** (`<pending>`): `crates/rpro-runner/tests/golden_corpus.rs` — a
  fast, toolchain-free `cargo test` over the *real* `exercises/` dir asserting the corpus-wide
  invariants `validate()` can't see (it inspects one .toml in isolation): unique ids, present
  concept, `E####`-shaped error codes, and — via an explicit per-phase golden map of allowed
  `id` area-prefixes **and** `difficulty` tiers — that every exercise sits in the right phase.
  Verified to BITE: perturbing difficulty (`beginner`→`advanced` in ownership) and the id area
  both fail with clear messages; it would have caught both relocated-exercise bugs. Compilation +
  error-code *emission* stay in `verify-exercises.sh`; anchors stay in `verify-book-anchors.mjs`.
- ✅ **Offline-Android Run UX** (`<pending>`): a failed run with an exercise loaded now shows
  a guiding "can't run here — no toolchain; predict/read/study offline" message, not the demo.
- ☐ Clippy hygiene: pre-existing `future not Send` (`?Send` toolchain) + `struct_excessive_bools`.
- ☐ **e2e a11y regressed** (found 2026-06-23): `a11y.spec.js` reports a `color-contrast`
  (WCAG AA, serious) violation on the main view — fails on pristine HEAD too, so it pre-dates the
  FREE-badge removal; task #12 had this green, so something regressed contrast since. Identify the
  low-contrast element (axe lists it) and fix the token.
- ☐ **e2e web.spec stale vs predict-gate** (found 2026-06-23): `web.spec.js` "Run → error code in
  Diagnostics" clicks `#runbtn` without locking a prediction, so the Learn-mode predict-gate
  (`158c997`) blocks the run and it times out. Update the test to lock a prediction first (or run in
  a non-Learn mode), matching the shipped gate.
- ☐ **Stale doc comment in `hint_handler`** (found 2026-06-23): `crates/rpro-serve/src/main.rs`
  ~L482 still says level 3 returns "the solution OUTLINE" — but `ExerciseMetadata::hint` was
  rebuilt (`af5d6fb`) to return book-pointers and NEVER the outline (Hard Rule #1). Doc bug, not a
  real leak, but it describes the forbidden behavior; correct the comment.

### Platform / distribution
- ☐ Web deploy (GH Pages / server) · online Run for Android (remote rpro-serve) ·
  F-Droid + Obtainium · signed APT/dnf repos.

## Open questions for Paul
- **Bundle 3 more Book chapters to unblock functional/advanced exercises** (2026-06-23). The
  functional phase (`07b`) and async are blocked: authoring a **closures** exercise needs
  `book/ch13-01-closures.md`, **smart-pointers** needs `ch15-*`, **async** needs `ch17-*` — none are
  bundled. `NOTICE.txt` already establishes the pattern (vendor chapters from `rust-lang/book` as
  exercises need them, dual MIT/Apache), so the *policy* is settled; the blocker is purely
  mechanical — the authoring env is **offline** and has only rendered PDFs (`rust-textbook/sources/`),
  no Book `src/*.md`. **To unblock in one step:** drop the needed `src/*.md` from `rust-lang/book`
  into `rust-textbook/sources/rust-book-src/` (or grant fetch access to raw.githubusercontent.com).
  Until then the loop works other un-blocked items.

## Audit cadence
Re-run the educational-fidelity audit + CI gate set after each batch of pedagogy
changes. **Throttle agent fan-out** — concurrent heavy workflows hit the rate limit
(2026-06-23); run audits sequentially / small.
