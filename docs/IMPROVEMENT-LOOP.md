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
- ✅ **e2e a11y regression fixed** (`<pending>`): the serious `color-contrast` violation was the
  *selected* mode-switcher button — white `#fff` on `--accent` `#f74c00` = **3.5:1** at 11px (needs
  4.5:1); regressed when the switcher landed (`158c997`). Deepened just that button's bg to
  `#c44000` (white = ~5.1:1, self-contained so passes in both themes), preserving the filled-orange
  "selected" affordance. `a11y.spec.js` gate green again; verified live (axe probe + screenshot).
- ☐ **White-on-`--accent` is fragile at small sizes** (note, 2026-06-23): the bright `#f74c00`
  only clears AA for *large* white text — the `.run` button passes solely on size. Any NEW small
  white-on-accent element will silently fail contrast; reuse the deepened `#c44000` (or a token) and
  let `a11y.spec.js` catch it.
- ✅ **e2e web.spec fixed vs predict-gate** (`<pending>`): the "Run → error code" test clicked
  `#runbtn` cold, so the Learn-mode predict-gate (`158c997`) blocked the run → timeout (perma-red,
  masking real regressions). Now locks the honest `fails` prediction first, exercising the real
  predict→Run→diagnostics path. Full e2e suite green (4/4); toolchain round-trip confirmed
  server-side (`/api/run` → E0384). NOTE: the gate's *block* behavior (Run refused with no
  prediction) still has no spec — a coverage gap worth a dedicated test later.
- ✅ **Hint level-3 "solution outline" doc drift fixed across all surfaces** (`<pending>`): the
  rung-3 docs/help everywhere said the ladder shows "the solution outline", but `ExerciseMetadata::hint`
  returns the book/source review and NEVER the stored outline (Hard Rule #1, `af5d6fb`). Corrected 9
  spots in 6 files: rpro-serve handler doc, rpro-cli `--level`/`--solution` `--help` + post-hint tip,
  rpro-tui (render + lib comments), gui comment, the `solution_outline` field doc (rpro-state), and the
  ARCHITECTURE.md example ("Shown only on `rpro hint --solution`" → authoring-ref-never-shown). Behavior
  unchanged; verified live (CLI `--help` + `--solution` run shows the book review, "no shortcut to the
  answer") + rpro-state hint tests (never-leak) + gui node-check. `--solution` confirmed safe (clamps to
  rung 3 = book review).
- ✅ **rpro-serve unit suite un-staled vs its gates** (`<pending>`): two tests were red (and had been
  since the gates shipped — a perma-red suite masking regressions). `hint_level1_serves_without_the_solution`
  expected `level 1` from a 0-attempt store (the force-attempt gate returns `level 0`); `select_switches_
  current_and_validates_id` selected a later exercise without `force` (the soft-gate now returns `423`).
  Both rewritten to exercise the REAL shipped flow — and they now also cover two previously-untested gate
  paths: hint level-0 "run it first" with no attempt, and select 423-locked → force jump-ahead → current.
  Suite now 18/18 green (was 16/2). Test-only; no production change.
- ✅ **CLI run/check/test now update progress (broken core loop fixed)** (`<pending>`): `cmd_exec`
  touched progress NOT AT ALL — no `record_attempt` and (the bigger gap) no `set_done`/advance on a pass,
  so a CLI-only learner could never complete anything (`rpro progress` stuck at 0%, `rpro exercise next`
  walking a never-finished list). Ported the web's logic by calling the SAME shared helper
  `rpro_runner::record_run` from a small `finalize_run_progress` fn: always logs an attempt; on a passing
  Run/Test of the CURRENT exercise marks it Done + advances; Check never advances; failures only log the
  attempt; `set_done` never regresses. **Only the current exercise drives progress** — an ad-hoc
  `rpro run <id>` side-run is left untouched (recording an attempt there would create a phantom 2nd
  `Current`, since `record_attempt` defaults new entries to Current — caught + fixed via live testing).
  Verified live (RPRO_STORE temp): fail→attempt-only, pass→Done+advance+`rpro progress` reflects it,
  side-run→current untouched. Extracted to a helper to stay under clippy `too_many_lines` (no new warning).
- ✅ **CLI force-attempt hint gate** (`<pending>`): `cmd_exercise_hint` now locks the laddered hint until
  the current exercise has ≥1 recorded attempt (parity with rpro-serve `hint_handler`). Book references
  stay ALWAYS visible (like the web's always-open Book tab); only the ladder is gated, and the gate applies
  even to `--solution` (no jumping to the top rung without trying). Verified live: 0 attempts → book_refs +
  "Run it first" (no ladder), `--solution` likewise gated, then `rpro run` → hint unlocks (Hint 1/3).
- ✅ **CLI hint escalation parity** (`<pending>`): `cmd_exercise_hint` now earns ONE rung per attempt
  (`earned = attempts.min(3)`), exactly like rpro-serve — `--level` is clamped to earned, `--solution` jumps
  to the highest EARNED rung (never past it), and the tip honestly says to run again to earn the next rung
  (deeper help comes from trying, not bumping a flag). `--level`/`--solution` help updated to match. The
  cross-surface hint behavior (text + gate + escalation) is now fully consistent across web/CLI. Verified
  live: 1 attempt → `--solution`/`--level 3` both give rung 1/3; 2 attempts → rung 2; 3 → rung 3 last-resort.

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
- **Rename the CLI `--solution` flag?** (2026-06-23). The flag is now honestly documented (it jumps to
  the top hint rung = the book/source review; the literal answer is never printed), but its *name* still
  implies "give me the solution" — arguably at odds with the never-hand-the-answer charter. Options: keep
  (with the honest help), or rename to e.g. `--stuck` / `--last-resort` (a small CLI API change + update
  the 3 exercise-starter comments that reference it). Left as-is this tick (behavior/UX unchanged).

## Audit cadence
Re-run the educational-fidelity audit + CI gate set after each batch of pedagogy
changes. **Throttle agent fan-out** — concurrent heavy workflows hit the rate limit
(2026-06-23); run audits sequentially / small.
