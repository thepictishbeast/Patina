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
- ✅ **Hint contract uniform across ALL THREE surfaces** (web/CLI/TUI) (`<pending>`): same shared
  text (`ExerciseMetadata::hint`) + force-attempt gate (no hint until ≥1 recorded attempt) + escalation
  (one rung earned per attempt, `earned = attempts.min(3)`). Closed in sequence: web always had it; CLI
  got progress-tracking → gate → escalation; TUI now gates the `h`-key climb and caps it per attempt via
  a pure, unit-tested `hint_on_keypress` (the loop can't be driven headlessly, so the decision incl. the
  gate message + `meta.hint` + the no-dead-end "run again" nudge is all under test). Book refs stay
  always-visible on every surface.
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
  mode authority (verified live: header renders, badge gone, no a11y/contrast change).
  ✅ **Tier-gate the Diagnostics panel** (`<pending>`, audit G2 / task #18): `renderDiag` was
  mode-agnostic — it handed the learner a parsed, clickable error code + line in ALL modes,
  including Learn, contradicting Paul's "Learn = by-hand errors only" decision. Now Learn shows a
  "read it by hand — find the error code yourself (switch to Assist)" nudge and withholds the
  parsed panel; **Assist/Dev** get the parsed Diagnostics. `lastCode` resets per `renderDiag`
  (kills cross-run/mode staleness) so predict-feedback still scrapes the real code AFTER a guess
  (active recall preserved); `setMode` re-renders the last run so switching tiers updates the panel
  with NO re-run. Honest tooltips (Learn "read errors by hand"; Assist "parsed diagnostics panel").
  Verified: gui transforms green; live e2e rewritten to a tier-diff test (Learn `#diag` has no
  E-code → switch to Assist → `E0384` appears without re-running) — **3/3 e2e pass**; Playwright
  read + screenshot confirm. **Autocomplete/auto-fix:** already satisfied — the textarea sets
  `autocomplete/autocorrect/autocapitalize/spellcheck` off for all modes.
  ✅ **Dev tier was HOLLOW — gave it a real editor assist** (`<pending>`): `setMode` only branched on
  `'learn'`, so **Assist ≡ Dev** (Dev did nothing extra) while the switcher advertised "Dev: full editor
  assists" — a falsely-advertised tier. Added a **Dev-only Tab→indent / Shift+Tab→dedent** keydown handler
  on the editor textarea (`mode === 'dev'` gate): single-caret Tab inserts 4 spaces; a multi-line selection
  indents every touched line; Shift+Tab strips up to 4 leading spaces; `markEdited()` re-highlights + tracks
  edited-state. **Learn AND Assist keep the default Tab (move focus = no assist)** — per the charter, Learn
  blocks editor assists, and editor conveniences are the Dev differentiator, so all three tiers are now
  distinct. Dev tooltip made honest ("Tab/Shift+Tab to indent (rust-analyzer later)"). Verified LIVE via
  Playwright: Dev Tab → 4 spaces at caret; Dev multi-line → all lines indented; Dev Shift+Tab → dedented;
  Learn Tab → no-op; `fn main(){...}` body indented cleanly + highlight overlay tracks it (screenshot);
  gui transforms green; e2e 4/4 (no regression).
  ✅ **Dev auto-indent on Enter** (`<pending>`): extended the same Dev-only keydown handler — Enter starts
  the new line at the current line's indentation, one level deeper after an opening `{`, and splits a
  `{<caret>}` into a tidy block (open brace, indented middle line, closing brace on its own line). Together
  with Tab-indent this is the minimum viable code editor — a Dev learner can write nested Rust without
  re-typing indentation. Learn/Assist keep the default Enter (plain newline, no assist). Verified LIVE
  (Playwright): match-indent, +level-after-`{`, split-`{}`, Learn no-assist; built a nested
  `fn main(){ if true { println!() } }` by Enter alone (4→8-space nesting compounds correctly) + highlight
  tracks (screenshot); gui transforms green; e2e 4/4. Dev tooltip updated. **Dev tier is now a genuinely
  usable editor (Tab/Shift+Tab + Enter auto-indent).**
  ✅ **Dev bracket auto-close** (`702d63c`): extended the same Dev-only keydown handler — typing `( [ {`
  inserts the matching close with the caret between them, but ONLY when the next char won't be glued onto a
  word (so `(` before `foo` stays `(foo`, never `()foo`); typing `) ] }` over an existing close skips it
  instead of doubling; Backspace inside an empty `()`/`[]`/`{}` deletes both. Composes with the Enter
  handler (type `{` → `{}` → Enter splits the tidy block). Learn AND Assist keep the bare textarea (charter:
  Learn blocks autocomplete/auto-fix), so the tiers stay distinct. Verified LIVE (Playwright,
  `tests/e2e/dev-brackets.spec.js`, 5 cases incl. Learn+Assist untouched) + screenshot (`vec![1, 2, 3]`
  built via one `[` keystroke); web + a11y e2e unchanged; node --check + GUI transforms green. **Dev code
  editor is complete (Tab/Shift+Tab + Enter auto-indent + bracket auto-close); next IDE step is
  rust-analyzer (#19).**
  ✅ **Diagnostic → jump-to-line** (`9e21076`, bounded slice of #18): each parsed diagnostic in
  Assist/Dev now shows two buttons — the CODE (→ Explain) and the LINE `L<n>` (→ move the editor caret
  to that line + select it + scroll into view). **Found a stale premise while doing it**: the backlog
  claimed "line spans are already in the run response" — they were NOT; the Rust plugin's
  `parse_diagnostics` scraped only the error/warning HEADER and returned `span: None`, so the gui's
  existing `· L<line>` never populated. Fixed at the seam: `crates/languages/rust` now scans forward to
  the `--> file:line:col` line and attaches `Span{file,line,col}` (new `parse_arrow_location` + 2 unit
  tests; rustc parsing stays inside `crates/languages/` per the seam gate). The view uses
  `setSelectionRange` (CHARACTER-offset based → **wrap-immune**, sidestepping the pixel-mapping that made
  a gutter overlay fragile). Also de-nested the row (was itself `role=button`; now a plain container with
  sibling `.diagcode`/`.jumpline` buttons → avoids axe nested-interactive). Learn still withholds the
  whole panel. Verified: rpro-lang-rust 9/9 + core/runner green, fmt+seam clean, clippy adds none; LIVE
  `tests/e2e/dev-diag-jump.spec.js` 3/3 (jump moves caret, no nesting, Learn withholds) + web/a11y/brackets
  9/9 unchanged + screenshot (L47 → line selected & scrolled).
  Remaining (#18, optional): a TRUE inline gutter marker / underline on the offending line in the
  highlight overlay — lower-value now that click-to-jump exists, and still the fragile pre-wrap pixel-map
  job; defer unless it proves worth it.
- ✅ **Ctrl/Cmd+Enter runs from the editor** (`3dd40b3`): the web editor lacked the standard "write code,
  press Ctrl+Enter to Run" shortcut. Added as the FIRST check in the editor keydown handler, ahead of the
  `mode !== 'dev'` gate → works in EVERY tier (Learn included): running your code is the core action, not
  an autocomplete-style assist Learn blocks. Calls `runOp('run')` (the Run-button path), so the Learn
  predict-first gate still applies — Ctrl+Enter with no locked guess shows "predict first", not a free run.
  `return`s before the Dev Enter auto-indent → no stray newline. Robust by construction (key→action, no
  pixel mapping, so the pre-wrap layout is irrelevant — unlike the deferred gutter overlay). Run button
  title advertises it. Verified LIVE (`tests/e2e/editor-run-shortcut.spec.js`, 3 cases: Assist runs, Learn
  gated-then-runs, Dev no-newline) + web/a11y/brackets/jump 12/12 unchanged + screenshot (keyboard-only run,
  sidebar "3 tries"). gui-only + spec, no crates/*.rs.
- ✅ **RECALL chips → active-recall prompts** (`9ee4b10`): the spaced-repetition (Leitner) RECALL sidebar
  listed each due error code as a DEAD `<span>` titled "a compiler error to refresh" — clicking did nothing.
  Completed the half-built feature: each due code is now a button (click or Enter/Space) that reveals its
  explanation via the existing Explain op. Framed as active recall (charter-aligned): the chip shows the code
  (recall what it means), activating it reveals the answer = the self-check — same predict-then-reveal shape
  as predict-first Run, applied to review. Reuses the diagnostics `explainFromEl` helper; delegated handler on
  `#recallCodes` matches only `.rc[data-code]` (the "all mastered"/meta spans stay inert). Due codes are real
  rustc error codes (`record_run` folds `primary_error_code` into the queue on a passing/overcome run), so
  `--explain` applies. **First, ground-truthed two non-gaps (dry audits = successful): all 54 exercise
  concepts resolve to glossary terms; the review model is sound — then found the RECALL UI was the actual
  dead-end.** gui-only + spec, no crates/*.rs. Verified LIVE: pass `01_immutable_assign` → RECALL shows an
  E0384 button → keyboard-activate sends an Explain request for E0384 → rustc explanation renders. New
  `tests/e2e/recall-chip.spec.js` green; full e2e 16/16; node --check + transforms green; screenshots.
- ✅ **Exercise-list phase-group headers (`8791aba`, 2026-06-25)** — found via a live-GUI UX pass: the
  sidebar was a flat **60-item** scroll (`phase/name` prefixes, no visual grouping) — hard to scan or orient
  within. `renderList` now emits a quiet uppercase `.exphase` label whenever the id's phase prefix changes
  (BASICS / CONTROL FLOW / OWNERSHIP / …). CSS uses `--fg-dim` (≥4.5:1) and headers are `aria-hidden` (each
  item's button label already names its full phase/id → no a11y/contrast change). Verified LIVE (rpro-serve +
  Playwright): headers render for every phase, all 60 items present + clickable, a11y + web + tier e2e 4/4,
  screenshot read. Faithful quirk: "iterators" shows twice because the 07b ramp interleaves iterators ↔
  smart-pointers on disk (`02_collect`→`03_box`…→`06_into_iter`) — grouping-by-adjacency reflects file order
  honestly; it's a content-ordering artifact (renumbering 07b would churn ids), not a gui bug. **Same pass
  also confirmed (NOT bugs): the dev `~/.cache/ts-serve` store held a STALE 32-exercise copy (re-seeded from
  the live 60 — packaged builds bundle fresh, so prod is unaffected); editor drafts are correctly keyed
  per-exercise (`lsKey(id)`).**
- ✅ **07b exercise order fixed → matches the lesson ramp (`fff1d97`, 2026-06-25)** — the "iterators twice"
  quirk noted above was a real ordering wart, not just cosmetic: the two later iterator exercises (into_iter
  E0382, sum E0283) were files `06`/`07`, so byte-sort put them AFTER the smart-pointers block, splitting the
  iterators area and mismatching the **L27 closures → L28 iterators → L29 smart-pointers** lesson order.
  `git mv` 06→`02b`, 07→`02c` so all three iterators sort right after `02_collect`, before smart-pointers.
  IDs live in the tomls (unchanged) → golden_corpus + progress keys unaffected; only the discover/gating
  ORDER changes, to the correct ramp. Verified: golden_corpus green, verify-exercises 60/60; live
  /api/exercises → closures → iterators(01,02,03) → smart-pointers; GUI now shows ONE iterators header
  (Playwright snapshot). Also drove the **hint ladder live (charter audit — PASS, no bug):** attempts==0 →
  "run it first" (forces a try); then ONE rung earned per attempt (process → error-code → book-sections); the
  literal fix is never served at any rung. NOTE for a future tidy (low-value, NOT this tick): a couple phases
  still list out of id-number order by filename byte-sort (control-flow shows 01,02,05,03,04; types 01,02,03,05,04)
  — each area stays contiguous so no duplicate headers; pure sequence polish if ever worth it.
- ✅ **Editable-pane syntax highlighting** (`<pending>`): colored `<pre>` overlay behind a
  transparent textarea, reusing highlightRust; always-on. Verified live.
- ☐ **Full IDE via rust-analyzer** (`LspSpec` defined, unconsumed) — big; Rust FOSS.

### Content / corpus
- ✅ **Built-in glossary (audit G1, Paul's explicit ask)** (`<pending>`): shared `rpro-glossary` crate
  (flat `glossary/glossary.toml`, alias-aware case/separator-insensitive lookup, seeded into the store like
  `book/`/`exercises/` so it ships to Android). **Covers all 33 exercise concepts** (verified live every
  concept resolves) — terms drafted by a bounded workflow from the bundled chapters + adversarially verified
  (grounded, CONCEPTUAL never an exercise's fix, ATTRIBUTED, no foreign-language analogies; a hardened test
  enforces no-leak + no-HTML-entity + no-foreign-language on every term). **On ALL THREE surfaces:** web
  (tappable `concept` chip → definition box + "read more in the Book"), CLI (`rpro glossary [term]`), TUI
  (`g` key → definition in the shared help slot, TestBackend-verified). Tasks #24, #26 done. Future: add
  terms as new exercises land; optional inline-in-prose term highlighting (deferred — fiddly).
- ◐ **Lessons for Phases 2–9** — **STARTED Phase 2** (rust-textbook `c8dab96`). **Gate re-examined: it
  was being treated as wholly Paul-gated, but the "show Paul the matrix first" gate is MATRIX-STATUS-scoped,
  not phase-numbered.** Phases 2–6 ride DECIDED (human-reviewed) matrix rows 1–48 — the same reviewed footing
  Phase 1's L1–L8 were authored on, under the calibration Paul released 2026-06-22 ("finish the tasks / do
  what's best"; write-first + baby-steps). So Phases 2–6 lessons are UN-GATED; only Phases 7–9 (DRAFT rows
  49–62) still wait on Paul's matrix review. Authored **L9 — `if`/`else if`/`else` as an expression**
  (`lessons/09-if-else-expressions.md`, rust-textbook/main) — Phase 2 = control flow is the learner's stated
  CURRENT GAP per CLAUDE.md. 7-part format + write-first + baby-steps (builds on L6 semicolon/expression);
  every snippet compile-run on rustc 1.95.0/ed2024; both E0308 failure demos verbatim; no Python/foreign
  analogies; no answer-leak; BOOK/CR/BLOG attributed. **L10 — loops** (`10-loops.md`, rust-textbook/main
  `dc7bb05`): `loop`+break-value+labels · `while` · `for`+ranges (matrix rows 16–18); same format/rules;
  every snippet compile-run on 1.95.0/ed2024 (loop→20, while→LIFTOFF, ranges→1 2 3/1 2 3 4/3 2 1, label→2);
  off-by-one `while index <= len` panic reproduced verbatim ("index out of bounds…") with `for` as the fix.
  **L11 — `match` intro** (`11-match-intro.md`, rust-textbook/main `b2baebd`): literal arms, `_` wildcard,
  value-returning, exhaustiveness (matrix row 19); CR intro + BLOG's E0004 non-exhaustive demo + BOOK's
  coin-sorting-machine metaphor; deep patterns deferred to Phase 5. Snippets compile-run on 1.95.0/ed2024;
  E0004 "`2_u8..=u8::MAX` not covered" reproduced verbatim. **Phase 2 lessons COMPLETE: L9 ✓ L10 ✓ L11 ✓.**
  **R2 capstone — `likes` kata SPEC** (`katas/likes.md`, rust-textbook/main `f78ac02`): the BLUEPRINT-named
  Phase-2 capstone (count→branch→build string; ties L11 match + L8 format!). Authored as a SPEC not a
  solution (CLAUDE.md rule 1 — kata is the learner's): task + `fn likes(names: &[&str]) -> String` signature
  + 5 count rules + verified answer-key table (reference solution compiled privately on 1.95.0, kept OUT of
  the repo) + a hint ladder (structure only, never the arm bodies) + a `main` test harness. R2 now:
  cheatsheet (pre-existing) ✓ · `likes` kata spec ✓ · quiz ✓ (`quizzes/phase2.md`, rust-textbook/main
  `e17ae50` — 10 Qs on if/loop/while/for/match, predict-first with a verified answer key; every snippet
  compile-run on 1.95.0). **★ PHASE 2 FULLY COMPLETE: lessons L9–L11 + cheatsheet + `likes` kata + quiz.**
  **Phase 3 STARTED — L12 `String` vs `&str`** (`12-string-vs-str.md`, rust-textbook/main `90ad7b1`; matrix
  row 20 DECIDED). [PHASE-4 LEAN]: surface-level owned-vs-borrowed (deep borrow rules → Phase 4); BOOK
  trade-off + build-a-String; CR unified example; legit fail `s[0]`→E0277 verbatim; `+`-moves-`s1` as a
  Phase-4 foreshadow only. Snippets compile-run on 1.95.0. **L13 — tuples + arrays + slices** (`13-tuples-
  arrays-slices.md`, rust-textbook/main `1e32f61`; rows 21–23): fixed-shape collections; slice = "a view
  whose length drops out of `&[T]`"; the standout dual-OOB reproduced verbatim (const `a[5]`→compile error
  `unconditional_panic`; computed `a[pick()]`→runtime panic) + mid-char-boundary slice panic. **L14 — Vec +
  HashMap** (`14-vec-hashmap.md`, rust-textbook/main `4509870`; rows 24–25): growable collections — Vec
  (push/pop, `[]`-vs-`.get()` as a design choice, `*n+=` mutate) + HashMap (`use std::collections::HashMap`,
  the `entry().or_insert` word-count). Legit fails verbatim: forgot-import E0433 (compiler gives the fix);
  `v[100]` runtime panic (vs L13 array compile error) vs `.get(100)`→None. **★ PHASE 3 LESSONS COMPLETE:
  L12 ✓ L13 ✓ L14 ✓.** **R3 Phase-3 quiz** (`quizzes/phase3.md`, rust-textbook/main `791276c` — 10 Qs,
  predict-first + verified answer key, every snippet compile-run on 1.95.0) + **cheatsheet**
  (`cheatsheets/phase3.md`, `fc7f6ed`). **★ PHASE 3 FULLY COMPLETE: L12 ✓ L13 ✓ L14 ✓ · quiz ✓ · cheatsheet
  ✓.** (Textbook progress: **Phases 1, 2, 3 fully done** — lessons + quizzes + cheatsheets; Phase 1 lessons
  L1–L8 predate the quiz format.) **Phase 4 STARTED — L15 ownership & moves** (`15-ownership-and-moves.md`,
  rust-textbook/main `a0a5595`; matrix rows 26–27): the heart of Rust — ownership/move (E0382 centerpiece
  reproduced verbatim) + Copy-vs-Clone + stack/heap + Drop; cross-language framing stripped per hard rule 3;
  cashes in the Phase-3 foreshadows. **L16 — references & borrowing** (`16-references-and-borrowing.md`,
  rust-textbook/main `71bde5d`; row 28): the clean fix to L15's move — `&T` shared / `&mut T` exclusive, the
  two rules verbatim + "shared xor mutable" + NLL; the three errors verbatim (E0499 two-`&mut`, E0502 `&mut`-
  while-`&`, E0106 dangling). **L17 — slices in depth** (`17-slices-in-depth.md`, rust-textbook/main
  `6e7d241`; row 29): a slice is a borrow → it pins the collection → mutating-while-borrowed is a compile
  error; the `first_word`+`s.clear()` E0502 payoff (cashes the Phase-3 row-23 forward-ref) + the usize-index
  compiles-silently-wrong contrast + the Vec push-while-borrowed E0502. **★ PHASE 4 LESSONS COMPLETE: L15 ✓
  L16 ✓ L17 ✓.** **R4 Phase-4 quiz** (`quizzes/phase4.md`, rust-textbook/main `fc36d24` — 10 Qs on
  move/E0382, Copy/Clone, borrow, E0499/E0502/E0106, slice-pins; predict-first + verified answer key, every
  snippet compile-run on 1.95.0) + **cheatsheet** (`cheatsheets/phase4.md`, `b974c15`). **★ PHASE 4 FULLY
  COMPLETE: L15 ✓ L16 ✓ L17 ✓ · quiz ✓ · cheatsheet ✓.** (Textbook: **Phases 1–4 fully done** — 17 lessons,
  4 quizzes [p1 pending], 4 cheatsheets.) **Phase 5 STARTED — L18 structs** (`18-structs.md`, rust-textbook/
  main `a03a730`; rows 30–32): your own types — define/instantiate/update/tuple/unit + methods/`impl`
  (`&self`/`&mut self`/`self` tied to Phase-4 borrow intent) + assoc-fns + `derive(Debug)`; errors verbatim
  (`{:?}`-no-derive E0277 with the compiler's fix, `&str`-field E0106). **L19 — enums + matching**
  (`19-enums-and-matching.md`, rust-textbook/main `7524804`; rows 33–36): enums + enum-`impl` (`match self`)
  · `Option<T>` ("no null") · deep `match` (binding/`|`/ranges/guards/exhaustive) · `if let`/`while let`/
  `let…else`; cashes Phase-2 #19; errors verbatim (E0004 None-not-covered, E0277 Option-as-T). **★ Phase 5
  DECIDED lessons COMPLETE: L18 ✓ L19 ✓.** **R5 Phase-5 quiz** (`quizzes/phase5.md`, rust-textbook/main
  `d6e98b9` — 10 Qs over L18/L19; predict-first + verified answer key incl. E0282 bare-None; every snippet
  compile-run on 1.95.0) + **cheatsheet** (`cheatsheets/phase5.md`, `181280e`). **★ R5 done (over L18/L19):
  quiz ✓ · cheatsheet ✓.** **★ L20 error-handling AUTHORED → matrix #37 DECIDED → Phase 5 lessons COMPLETE**
  (`20-error-handling.md` + matrix #37 + TRACKER, rust-textbook/main `25a750e`): the deferred E/M+L done in
  one tick — narrowed BOOK Ch.9/CR's whole-chapter superset to the L20 slice (Result `Ok`/`Err`; four
  readings `match`/`unwrap_or`/`unwrap`-`expect`/the `?` operator; framed as the sibling of L19's `Option`;
  custom-error/`Box<dyn Error>`/`From` deferred to a later phase). Anchor = `str::parse`; every snippet
  compile-run on 1.95.0 (match→`parsed: 42`, `unwrap_or`→`42 0`, `?`→`Ok(42)`/`Err(ParseIntError…)`); two
  fails verbatim (E0277 `?`-in-non-Result-fn, runtime panic on `unwrap`-of-`Err`); 7-part, write-first, no
  analogies/leak. (Textbook: **Phases 1–5 lessons all done — L1–L20**; 20 lessons, 5 quizzes [p1 pending],
  5 cheatsheets.) NEXT (un-gated): **extend R5 to cover L20** (Result/`?`/E0277 quiz Qs + an error-handling
  cheatsheet section), then **Phase 6 — Organizing & generics** (modules/generics/traits/lifetimes/tests/
  cargo — the LAST un-gated phase before the matrix-review-gated DRAFT Phases 7–9). **★ R5 EXTENDED to cover
  L20 → PHASE 5 FULLY COMPLETE** (rust-textbook/main `2399d3d`): `quizzes/phase5.md` +Q11–Q15 (now 15 Qs —
  Result `match`, `unwrap_or`, `?`-non-Result-fn E0277, `?`-returns-`Ok`, unwrap-on-`Err` compile-vs-runtime
  panic; every new snippet compile-run on 1.95.0 this tick) and `cheatsheets/phase5.md` +Error-handling
  section; both "not covered yet" notes removed. **Phase 5 = lessons L18–L20 ✓ · quiz (15 Qs) ✓ · cheatsheet
  ✓.** **★ PHASE 6 STARTS — L21 authored** (`21-packages-crates-modules.md`, rust-textbook/main `654e8e0`):
  ground-truthed the Phase-6 matrix → only the **Organizing slice (modules, rows 41–48) is DECIDED/un-gated**;
  **generics/traits/lifetimes/tests/cargo = DRAFT rows 49+, Paul-gated, SKIPPED**. L21 folds rows 41–43:
  packages & crates (bin vs lib crate, crate roots, cardinality) · modules + the `crate`-rooted tree
  (sibling/child/parent, private-by-default) · modules-in-files (`mod garden;` → `src/garden.rs`; "mod is not
  include"). 7-part, write-first, baby-steps (`pub`/paths introduced minimally, full privacy deferred to L22).
  Snippets compile-run on 1.95.0 (`foo`/`bar`→`In the foo module`/`In the bar module`; nested tree builds
  clean); failing demo verbatim **E0583** "file not found for module `garden`" + the help line naming
  `garden.rs`/`garden/mod.rs`. No analogies/leak. Metaphor = filesystem dir-tree (BOOK's own). **★ L22 authored**
  (`22-paths-and-visibility.md`, rust-textbook/main `1723c6e`; rows 44–46): paths (absolute `crate::` vs
  relative `self`/`super`; prefer absolute) · privacy-by-default + `pub` (child sees ancestors' privates; the
  "`pub mod` ≠ `pub` contents" surprise; `pub(crate)`) · `pub` on structs vs enums (struct keeps fields private
  → needs a public constructor; enum publishes all variants; "privacy is module-based not type-based"). 7-part,
  write-first, baby-steps (continues L21; `use` deferred to L23). Snippets compile-checked on 1.95.0 (abs+rel,
  `super`, outer/inner runs, `Breakfast`, `pub enum`, `pub(crate)`); two errors verbatim **E0603** (private fn —
  the two-strike) + **E0616** (private field). No analogies/leak. **★ L23 authored → PHASE 6 ORGANIZING
  LESSONS COMPLETE** (`23-the-use-keyword.md`, rust-textbook/main `4be44bc`; rows 47–48): `use` shortcut +
  function-vs-type idiom · scope-locality (root `use` doesn't reach a child) · name clashes (parent-qualify or
  `as`) · `pub use` re-export (simpler public API) · nested `{b,c}` / `{self, …}` / glob `*` (sparingly).
  7-part, write-first, baby-steps. Snippets compile-checked on 1.95.0 (`as`→`two Results, no clash`,
  nested→`Less`, self→`hi via Write`, glob→`1 1`); error verbatim **E0433** scope-locality (+ the orphaned
  root `use`'s unused-import warning). Metaphor = `use` ≈ symbolic link. No analogies/leak. **Phase 6
  Organizing lessons: L21 ✓ L22 ✓ L23 ✓; remaining R6 (quiz + `cheatsheets/phase6.md` over L21–L23).**
  **★ R6 part 1: Phase-6 quiz authored** (`quizzes/phase6.md`, rust-textbook/main `a05a4c0`): 14 Qs over
  L21–L23 (crate roots, foo/bar, E0583, tree vocab, abs+rel paths, E0603, `super`, E0616, `pub enum`, `use`
  scope-locality E0433, `as`, `pub use`, `{self, Write}`, fill-ins). Predict-first + separate verified
  Answers; every snippet compile-checked on 1.95.0; 14/14 balanced, no leak. **★ R6 part 2: `cheatsheets/phase6.md`
  authored → PHASE-6 ORGANIZING SLICE COMPLETE** (rust-textbook/main `6e5a577`): concise quick-ref over L21–L23
  (packages/crates, module tree, modules-in-files/E0583, paths, `pub`/privacy/E0603, pub-struct-fields E0616/
  pub-enum, `use`/scope-locality E0433/`as`/`pub use`/nested/glob; metaphors filesystem-tree + symbolic-link),
  facts all from verified L21–L23. **R6: quiz ✓ · cheatsheet ✓.** **★ UN-GATED LESSON LANE NOW EXHAUSTED** —
  textbook has Phases 1–5 fully done + Phase-6 Organizing fully done: **23 lessons (L1–L23), quizzes p2–p6,
  cheatsheets p1–p6**. Everything remaining (Phase-6 generics/traits/lifetimes/tests/cargo + Phases 7–9) is
  **DRAFT matrix rows 49–62 = Paul-gated** (do NOT author without his matrix review). Loop now rotates to
  housekeeping (`quizzes/phase1.md`, phase2-cheatsheet 1.94.1→1.95.0) / editor lane (#18) until Paul reviews.
  **★ Housekeeping: `quizzes/phase1.md` authored** (rust-textbook/main `d6b1ad0`) — the one missing quiz; now
  **quizzes phase1–6 all present**. 13 Qs over Phase-1 foundations (L1–L8): E0384 immutable-reassign, shadowing
  vs `mut` (E0308), `const`, integer-div-truncates, `as`-truncates, overflow debug-panic/release-wrap, block-as-
  expression, trailing-`;` E0308, fn-return, tuple Debug. Every snippet compile-checked on 1.95.0; 13/13, no
  leak. **★ Housekeeping: bumped `cheatsheets/phase1.md` + `phase2.md` cite 1.94.1→1.95.0** (rust-textbook/main
  `cde8dab`) — HONEST bump: re-verified both cheatsheets' load-bearing snippets on 1.95.0 first (phase1: E0384,
  shadowing, `-5/3`, `as`-truncate, overflow, block-expr, E0308; phase2: if-expr, E0308 arms, break-value=6,
  label, for-rev, match, E0004). No 1.94.1 left in cheatsheets. **Also checked #18 (editor inline diagnostics):
  essentially DONE** — tier-gated diag panel, diagnostic→jump-to-line (CODE/LINE buttons), Ctrl+Enter all
  shipped; the only remainder is a true inline gutter/underline, which the backlog explicitly DEFERS (fragile
  pixel-map, low-value now that click-to-jump exists). **Minor note:** `lessons/01-...md` cites "(1.94.1),
  unedited" for its displayed E0384 block — byte-identical on 1.95.0 EXCEPT 1.95.0 adds a secondary
  `unused_assignments` warning L1 omits, so refreshing that cite is a pedagogy call on the calibration lesson
  (show the warning or not?), not a trivial bump — left as-is. (Review/merge ask under "Open questions for Paul".)
  **★ In-lesson prev/next navigation** (rust-textbook/main `ad13cb8`, 2026-06-25) — found via a textbook pass:
  ZERO of the 37 lessons had inter-lesson nav (each ended at Sources), so an offline reader had to bounce back
  to STUDY-GUIDE between every lesson — the corpus's PRIMARY mode is offline markdown on a phone. Added a
  consistent footer to every `lessons/NN-*.md`: `[← prev] · [↑ Study Guide] · [next →]`, generated from each
  lesson's real H1 title + filename (correct by construction), L1 omits prev / L37 omits next, idempotent via a
  `<!-- lesson-nav -->` sentinel. Pure navigation (no lesson content touched). Verified: **109/109 nav links
  resolve, 0 broken.** STUDY-GUIDE stays the hub (linked from every footer). Complements the STUDY-GUIDE below.
  **★ `STUDY-GUIDE.md` authored** (rust-textbook/main `26390ec`) — closed a real learner-facing navigation gap:
  the textbook README was builder-facing ("read lessons in order", no index), so a phone learner had no map of
  the 23 lessons / 6 phases / quizzes / cheatsheets / kata. New guide gives the phase-by-phase reading order
  (Phase 1 Foundations → Phase 6 Organizing) with every lesson/quiz/cheatsheet/kata linked in sequence + how-to-
  study + offline "if stuck" help; README now points to it. **All links verified (0 missing; 23/23 lessons,
  6/6 quizzes, 6/6 cheatsheets, 1/1 kata).** **#20 clippy reframed:** the TS workspace's default `clippy::all` is
  CLEAN (0 warnings) — the 30 warnings are all opt-in pedantic (20) + nursery (10); nursery is the Paul-gated
  lint-policy question → corrective clippy debt is effectively DONE, the aspirational/gated tail awaits Paul.
  **★ Glossary expanded 52 → 56 terms** (`glossary/glossary.toml`): added the four head-terms a learner of the
  shipped lessons would look up and not find — **crate** (L21/ch7.1), **panic** (L20/ch9.1), **if let**
  (L19/ch6.3), **tuple** (L13/ch3.2). Conceptual, no analogies, no exercise-answer leak, attributed (source +
  book_chapter) — house style. **Verified:** `cargo test -p rpro-glossary` 3/3 (shipped TOML loads + leak-guard
  + alphabetical-stable); re-seeded the store and confirmed all 4 resolve via `rpro glossary <term>` lookup.
  Seam-guard N/A (scopes `crates/**/*.rs`; this is a data file). Glossary task (#24) was "complete" — this is
  additive vocabulary coverage for the now-23-lesson corpus.
  **★ Convergence CONFIRMED + `docs/REVIEW-GUIDE.md` authored** (advisor-directed, after 4 filler ticks): ran a
  saturation pass to prove convergence rather than assume it — **(a) `cargo audit` = 0 vulnerabilities** (236
  deps, exit 0; #21's *corrective* part is a verified no-op, only the CI-enforce gate remains deferred); **(b)
  exercise coverage of approved phases is saturated** — `06-modules/` already has private_module/private_fn/
  use_scope/super_path matching L21–L23, all 11 phase dirs populated (54 exercises); **(c) lesson cross-refs
  clean** (no `Lesson N` ref > L23). So the high-value lane is genuinely gated on Paul, confirmed. Highest-
  leverage un-gated move that does NOT grow the unmerged pile = a reviewer's **reading path**: wrote
  `docs/REVIEW-GUIDE.md` (the 3 decisions only Paul can make, ~45-min read order starting from STUDY-GUIDE →
  the L15 centerpiece → the never-hand-answer check → the matrix DECIDED/DRAFT gate, + the verification done).
  All referenced paths verified to resolve. Linked from the top of "Open questions for Paul". Zero new product
  surface. **#21 reframed:** corrective audit clean; `cargo deny` + CI-enforce stay deferred (preventive gate).
  **Verify-and-hold tick + glossary 56 → 58:** ground-truth confirmed NO Paul activity since the milestone (TS
  `main` still at old `89dac0c` — batch unmerged; both repos clean+synced; no new pushes), so convergence holds.
  Filled two genuine un-gated lookup gaps for shipped lessons L13/L17: **slice** (`&[T]`, a borrowed view; ch4.3)
  and **array** (`[T; N]`, fixed-length; ch3.2) — neither resolved before. Conceptual, no analogies, no leak,
  attributed; chosen aliases avoid stealing "string slice" from the existing `&str` term (regression-checked:
  `string slice` still → that term). `cargo test -p rpro-glossary` 3/3; re-seeded + CLI-verified all three.
- ✅ **Matrix: rows 49–62 reconciled to ground truth (#16 DONE, 2026-06-25).** Earlier fixed the
  HTML-entity in row 58 title (`Advanced patterns &amp; matching` → literal `&`). This tick: the
  DRAFT rows were stale (status DRAFT "pending Paul's review", Lesson# `—`) even though all 14 had
  since been authored + adversarially verified + shipped as lessons. Filled the **Lesson #** column
  (49→L24 … 62→L37, sequential, matches STUDY-GUIDE) and flipped **Status** DRAFT→**SHIPPED** —
  deliberately NOT `DECIDED` (reserved for the human-reviewed rows 1–48; Paul released the show-me-first
  gate but did not line-by-line review, so SHIPPED keeps that split honest). Updated 5 section headers
  `⟨DRAFT⟩`→`⟨SHIPPED⟩` + 5 banner leads (dropped the false "pending review" framing, KEPT the
  verification provenance). Verified: 0 DRAFT hits left, all 14 tails `| L## | SHIPPED |`. rust-textbook
  main `4e51b9b`. Matrix now internally consistent end-to-end.
- ◐ **More exercises / functional phase** — created `07b-functional-and-smart-pointers` and
  relocated the misplaced iterators exercise (was in 09-advanced) into it. Also relocated
  `09-advanced/03_move_out_of_borrow` (E0507, core ownership) → `04-ownership/04_move_out_of_borrow`
  (id `advanced/…`→`ownership/…`, difficulty `advanced`→`beginner` to match the phase); ramp is now
  move → borrow → dangling → move-out-of-borrow. `09-advanced` is down to `01_unsafe_deref`.
  ✅ Added the first **closures** exercise (`<pending>`): `07b/01_closure_type_lock` (E0308 — a closure's
  param/return types are inferred from the FIRST call then locked; baby-step intermediate). Renumbered
  the iterators exercise → `02_collect_annotation` (id unchanged) so the ramp is closures → iterators.
  book_ref → the freshly-vendored `ch13-01-closures`; its `concept` chip resolves to the "closure"
  glossary term. Verified: 33 exercises compile+emit codes, 52/52 anchors, golden-corpus + live (run→E0308,
  chip→def).
  ✅ Added the first **smart-pointers** exercise: `07b/03_recursive_box` (id `smart-pointers/01_recursive_box`,
  advanced) — a recursive cons-list `enum List { Cons(i32, List), Nil }` the compiler can't size → **E0072**
  ("recursive type has infinite size"; E0391 is the real secondary drop-cycle). Lesson: a self-containing
  type needs a fixed-size heap handle; framing comment points at the E0072 `help:` line without naming the
  fix. book_ref → freshly-vendored `ch15-01-box` # enabling-recursive-types-with-boxes; added a "Box"
  glossary term (33→34) aliased to the concept so the chip resolves. Ramp is now closures → iterators →
  smart-pointers. Verified: verify-exercises 34/34 (emits E0072), book anchors, golden-corpus, glossary 3/3,
  live (run→E0072, chip→"Box"). Commit `950349f`.
  ✅ Added the second **smart-pointers** exercise: `07b/04_shared_ownership_rc` (id
  `smart-pointers/02_shared_ownership_rc`, advanced) — two cons-lists `b` and `c` both want the same
  tail `a`, but a `Box` moves it into `b`, so `c`'s use → **E0382** "use of moved value". Lesson: a
  `Box` gives ONE owner; sharing needs reference-counted ownership. Reuses the E0382 move error the
  learner already met (in 04-ownership/01_move) but in a NEW context — last time clone/borrow fixed
  it; here both lists must OWN the shared tail, which is what `Rc` is for. Framing points at the
  E0382 message + the "many owners / counts references" concept without naming `Rc`. book_ref →
  freshly-vendored `ch15-04-rc` # sharing-data; added an "Rc" glossary term (34→35) aliased to
  `shared-ownership`/`reference-counting`. Ramp: closures → iterators → Box → Rc. Verified:
  verify-exercises 35/35 (emits E0382), book anchors 54/54, golden-corpus, glossary 3/3 (35 terms),
  live (select 200 → /api/current concept `shared-ownership` + book_ref ch15-04-rc#sharing-data →
  run E0382 → chip resolves to "Rc").
  ✅ Added the second **closures** exercise: `07b/01b_fn_once_move` (id `closures/02_fn_once_move`,
  intermediate) — a `move` closure `award` hands its captured `prize` to announce(), giving the value
  away, so it is `FnOnce`; calling `award()` twice → **E0382** "use of moved value: `award`". Lesson:
  a closure that moves a capture OUT can run only once. Reuses E0382 yet again in a NEW frame (the
  *closure* is consumed, not a plain value). Filename `01b_` sorts 2nd in the phase (path-order =
  learning-order via `source.cmp`) with NO renames, so the ramp is closures(type-lock) →
  closures(FnOnce) → iterators → Box → Rc. book_ref → `ch13-01-closures` # moving-captured-values-out-of-closures;
  added an "FnOnce, FnMut, and Fn" glossary term (35→36) aliased to `closure-move-capture`. Verified:
  verify-exercises 36/36 (emits E0382), book anchors, golden-corpus, glossary 3/3, live.
  ✅ Added the third **smart-pointers** exercise — the first one on top of the runtime-outcome model:
  `07b/05_refcell_runtime_borrow` (id `smart-pointers/03_refcell_runtime_borrow`, advanced). Two
  `borrow_mut()` guards held at once → the second one PANICS ("already borrowed") at RUN time — the code
  COMPILES (RefCell moves the borrow check to runtime). `expected_runtime_panic = "already borrowed"`;
  the framing teaches the mechanism but does NOT quote the panic phrase (error *codes* are lookup keys
  worth naming, a panic *message* is a discovery). Completes the core smart-pointers trio Box → Rc →
  RefCell. Vendored `ch15-05-interior-mutability`; book_refs → enforcing-borrowing-rules-at-runtime +
  tracking-borrows-at-runtime; added a "RefCell and interior mutability" glossary term (36→37) aliased to
  `interior-mutability`. Verified: verify-exercises 38/38 (runtime branch asserts the panic), anchors,
  golden-corpus, glossary 3/3, live (select 200 → run passed:false + "already borrowed" in stderr → chip
  resolves). This is the 2nd runtime-outcome exercise, proving the model generalizes beyond unwrap-on-None.
  ✅ Added two **iterators** exercises (2026-06-25), filling the L28 gap — `07b` had only the lone
  `collect`-annotation iterator exercise. (1) `07b/06_into_iter_moves` (id `iterators/02_into_iter_moves`,
  intermediate): `v.into_iter().map(...).collect()` then `println!("{v:?}")` → **E0382** — `into_iter`
  consumes the collection; the fix is `iter()` (borrows, `&i32` items, `|x| x*2` still works via auto-deref).
  Teaches the three iterator-makers' ownership split (into_iter/iter/iter_mut). (2) `07b/07_sum_type_annotation`
  (id `iterators/03_sum_annotation`, intermediate): `let total = v.iter().sum();` with no type pin → **E0283**
  — `sum` is generic over its output, same shape as `collect`; fix on the binding or via turbofish. Both reuse
  the validated book_ref `ch13-02-iterators#methods-that-consume-the-iterator`. `07b` now 8 exercises. Verified:
  golden-corpus (ids unique, area `iterators` ∈ 07b, E####-shaped, concept present) + both emit their claimed
  codes on rustc 1.95.0. Commit `4562dd2` → textbook-integration. TODO: async (ch17, may need its own model
  work); integer-overflow / index-OOB runtime exercises; more closures; grow async in 08-concurrency.
  ✅ Added an **ownership** exercise (2026-06-25) — `04-ownership/05_double_mut_borrow` (id
  `ownership/05_double_mut_borrow`, **E0499**, beginner): two simultaneous `&mut count` borrows alive
  together → "cannot borrow `count` as mutable more than once at a time". This is the PURE form of the
  borrow rule (one `&mut` at a time), distinct from `02_borrow`'s mixed mut/immut **E0502** — the most
  pedagogically central phase (the charter's "take ownership slowly") was missing it. golden_map pins
  04-ownership to area `ownership` + Beginner only, which E0499 fits; reused the validated ch04-02
  `mutable-references` + `the-borrow-checker` anchors (the Book shows this exact two-`&mut` error). The
  fix the learner finds: sequence the borrows (a `&mut` ends at its last USE), so only one is ever live →
  `count = 2`. Verified: E0499 emits on 1.95.0, golden_corpus green, verify-exercises **57/57**. Commit
  `003e31b`. 04-ownership now 5 exercises (E0382/E0502/E0515/E0507/E0499 — the borrow-rule set complete).
  ✅ Added an **error-handling** exercise (2026-06-25) — `05b-error-handling/05_question_mark_option_in_result`
  (id `error-handling/05_question_mark_option_in_result`, **E0277**, intermediate): `?` used on an `Option`
  (`.last()`) inside a `Result`-returning fn → "the `?` operator can only be used on `Result`s, not `Option`s,
  in a function that returns `Result`". The Option/Result `?`-interop confusion — a top beginner stumble — and
  a DISTINCT frame from the phase's two existing E0277s (02 propagation-needs-From, 04 custom-error-From): here
  `?` is on the wrong CARRIER (Option, not Result), not a missing trait impl. The real compiler message names
  the `.ok_or(...)` fix, so the exercise teaches reading the compiler's own suggestion (not spoon-fed in the
  outline). Reused 02's validated ch09-02 anchors (the-operator-shortcut + propagating-errors). golden_map: 05b
  = area `error-handling`, beginner|intermediate. Verified: emits E0277 on 1.95.0, golden_corpus green,
  verify-exercises **58/58**, book anchors 91/91. Commit `3181a73`. 05b now 5 exercises.
  ✅ Added a **basics/mutability** exercise (2026-06-25) — `01-basics/06_mutable_method_needs_mut`
  (id `basics/06_mutable_method_needs_mut`, **E0596**, beginner): `.push` (a `&mut self` method) on a
  non-`mut` Vec → "cannot borrow `scores` as mutable, as it is not declared as mutable". Fills a
  CORPUS-WIDE gap (E0596 was absent entirely, yet it's one of the most common first errors). Placed beside
  `01_immutable_assign` (E0384) as the two faces of forgetting `mut`: E0384 = rebinding (`x = 6`), E0596 =
  mutating in PLACE (`x.push()`/`&mut x`) — same "two faces of one rule" pairing as the ownership E0502/E0499
  set. Compiler's help shows the `let mut` fix → exercise teaches reading the suggestion (outline hints, not
  literal). Reused 01's validated ch03-01 `variables-and-mutability` anchor + ch04-02 `mutable-references`.
  golden_map: 01-basics = `basics` + Beginner. Verified: emits E0596 on 1.95.0, golden_corpus green,
  verify-exercises **59/59**, book anchors 93/93. Commit `c5b27c9`. 01-basics now 6 exercises.
  (Corpus-wide error-code coverage now 27 distinct codes + 5 runtime panics across 59 exercises.)
  ✅ Added a 2nd **ownership** exercise (2026-06-25) — `04-ownership/06_assign_while_borrowed`
  (id `ownership/06_assign_while_borrowed`, **E0506**, beginner): a single `&total` borrow alive while the
  code reassigns `total` → "cannot assign to `total` because it is borrowed". COMPLETES the borrow story —
  distinct from the conflict codes (E0502 `&`/`&mut`, E0499 two-`&mut`): here there's only ONE reference, and
  the violation is mutating the ORIGINAL while it's borrowed (an active borrow freezes the value). Same
  "borrow ends at last use" fix lever as the rest of 04 (finish reading `watcher` before the reassignment).
  Reused 04's validated ch04-02 `mutable-references` + ch10-03 `the-borrow-checker` anchors. golden_map:
  04-ownership = `ownership`+Beginner. Verified: emits E0506 on 1.95.0, golden_corpus green, verify-exercises
  **60/60**, anchors 95/95. Commit `61ed117`. 04-ownership now 6 exercises (E0382/E0502/E0515/E0507/E0499/E0506).
  Corpus = **28 distinct error codes + 5 runtime panics across 60 exercises.**
- ✅ **Runtime-outcome exercise model** (`<pending>`, do-what's-best — not a Paul fork): the model used
  to assume every failure is a COMPILE error. A whole class of core lessons are RUNTIME panics instead
  (RefCell's `BorrowMutError`, integer overflow, `unwrap()` on `None`, index-OOB) — now supported.
  Pre-check first proved `passed` already means "compiled AND ran AND exit 0", so a compiles-but-panics
  starter is `passed:false` for FREE (runner already correct). The full slice: `ExerciseMetadata` gained
  `expected_runtime_panic: Option<String>` (server-side only, mutually exclusive with `expected_error_code`,
  `validate()` rejects both — unit-tested); the no-leak DTOs are allowlists so it's never served (asserted
  live + in the oneshot test); `golden_corpus` accepts the new field + checks it's non-empty and code-free;
  `verify-exercises.sh` gained a runtime branch (compile→expect-clean→run→assert the panic substring, in an
  exec-OK dir since /tmp is noexec); hint rung-2 has a runtime variant (points at the panic, never quotes
  it). First runtime exercise: **`05-types-and-matching/03b_unwrap_none`** (id `types/05_unwrap_none`,
  `.max()` of an empty `Vec` → `None.unwrap()` → panic; sorts after `03_option_value`). The gui predict gate
  was RELABELED compiles/fails → **pass/fail** (the logic `!!out.passed` was already pass-based) so feedback
  reads honestly for both kinds — verified live: predicting "fails" on the runtime exercise reads "✓ outcome:
  right". Verified: workspace build, rpro-state 30 / rpro-serve 18 / golden-corpus green, verify-exercises
  37/37 (runtime branch asserts the panic), anchors, fmt clean, clippy no new warnings, e2e 4/4 (hardened
  the tier test to pin a compile-error exercise), no-leak confirmed live, Playwright drove both exercise
  kinds. **DEFERRED (labelled):** the two-axis prediction ("it compiles — but does it panic?") is the
  pedagogically-complete version (a separate tick); the pass/fail relabel is honest, which is the ship bar.
- ✅ **NEW phase: error handling (`05b-error-handling`)** (`<pending>`): the curriculum had Option
  (phase 05) but NOTHING on `Result` or the `?` operator — error handling (Book ch9) is a CORE topic
  that was missing ENTIRELY. Created the phase (byte-sorts between `05-types-and-matching` and
  `06-modules` via the established sub-letter convention) with its first exercise `01_result_is_not_t`
  (id `error-handling/01_result_is_not_t`, beginner): `to_number` returns `Result<i32, ParseIntError>`,
  bound straight to an `i32` → **E0308** — mirrors `05/03_option_value` (transfer the Option shape the
  learner just saw to Result, "one error many faces"). `golden_map()` gained the
  `("05b-error-handling", &["error-handling"], &[Beginner, Intermediate])` entry (room to grow); vendored
  `ch09-02-recoverable-errors-with-result`; added a "Result and recoverable errors" glossary term (37→38)
  aliased to `result-extract-value`. ROADMAP de-staled (32/9 → 39/11 exercises/phases). Verified:
  seam-clean, golden-corpus, verify-exercises 39/39 (emits E0308), anchors, glossary 3/3, live (discovery
  order 05→05b→06, run E0308, chip→Result).
  ✅ Added the 2nd error-handling exercise `02_question_mark_propagates` (id
  `error-handling/02_question_mark_propagates`, intermediate): `?` used in `first_number` which returns a
  plain `i32` → **E0277** "the `?` operator can only be used in a function that returns `Result` or
  `Option`". This is the error-*propagation* half that pairs with 01's error-*unpacking* — together they
  form the error-handling core (handle it here, or send it up). book_refs → `the-operator-shortcut` +
  `propagating-errors` (the slug for "The `?` Operator Shortcut" collapses the removed-`?` double-space to
  one hyphen). Added a "the ? operator" glossary term (38→39) aliased to `error-propagation`. The phase now
  spans beginner(01)→intermediate(02), exercising the `[Beginner, Intermediate]` golden_map tier. Verified:
  verify-exercises 40/40 (emits E0277), anchors, golden-corpus, glossary 3/3, live (run E0277, chip→? op).
  ✅ Added the 3rd error-handling exercise `03_unwrap_err_panics` (id `error-handling/03_unwrap_err_panics`,
  beginner) — a RUNTIME-panic exercise (uses the runtime-outcome model, so the phase now mixes compile-error
  AND runtime): `raw.parse().unwrap()` on non-numeric text → the code COMPILES then PANICS "called
  `Result::unwrap()` on an `Err` value". This COMPLETES the error-handling arc: 01 unpack (E0308) → 02
  propagate (E0277) → 03 the-lazy-unwrap-crashes (runtime) — the consequence that motivates handling errors
  at all. `expected_runtime_panic`; no new chapter (ch09-02 vendored); book_refs → shortcuts-for-panic-on-error
  + recoverable-errors-with-result. Added an "unwrap and expect" glossary term (42→43). Verified:
  verify-exercises 44/44 (runtime branch asserts the panic), anchors, golden-corpus, glossary 3/3, live
  (05b order 01→02→03, run passed:false + panic in stderr, no field leak, chip→unwrap). 05b 2→3; corpus 44.
  ✅ Added the 4th error-handling exercise `04_custom_error_from` (id `error-handling/04_custom_error_from`,
  intermediate): `parse_count` returns `Result<i32, AppError>` (a custom error enum), but `s.parse()` fails
  with a `ParseIntError` and there's no `From<ParseIntError> for AppError`, so `?` can't convert →
  **E0277** "`?` couldn't convert the error to `AppError`" (probed clean single; starter has an incidental
  unused-variant warning the fix removes). Teaches how REAL Rust error handling works: define your own error
  type, `impl From<TheirError>` so `?` auto-converts. Fix compiles+runs clean ("count is 42"). No new chapter
  (ch09-02 vendored); book_refs the-operator-shortcut + propagating-errors. Added a "custom error types and
  From" glossary term (44→45). **Error-handling phase ARC now complete & real-world: unpack (E0308) →
  propagate-with-? (E0277) → unwrap-crashes (runtime panic) → custom-error+From (E0277).** Verified:
  verify-exercises 47/47 (E0277), anchors, golden-corpus, glossary 3/3, live (05b 01→02→03→04, run E0277,
  chip→custom-error). 05b 3→4; corpus 47. NEXT in phase: matching on `Err` / recover-vs-propagate, `?` on `Option`.
  ✅ **Filled a MISSING core collection: HashMap** (`<pending>`): the collections phase (03) had Vec, String,
  and iterators but NO `HashMap` — a core collection (Book ch8-03), absent from the whole corpus. Added
  `03-text-and-collections/05_hashmap_insert` (id `collections/05_hashmap_insert`, beginner): `scores.insert(team, 10)`
  moves the `String` `team` INTO the map → **E0382** "use of moved value" (probed clean single). Teaches
  HashMap create/insert + that inserting an owned value transfers OWNERSHIP. **Chose insert-move (E0382) over
  the `.get()`→Option lesson to avoid forward-referencing Option** (taught in phase 05, after collections);
  E0382/move is already introduced in phase 03 (`04_vec_moved`), so this fits the existing sequencing. Vendored
  `ch08-03-hash-maps`; book_refs managing-ownership-in-hash-maps + creating-a-new-hash-map. Added a "HashMap"
  glossary term (45→46; covers insert-ownership AND get-returns-Option as a fuller reference). Verified:
  verify-exercises 48/48 (E0382), anchors, golden-corpus, glossary 3/3, live (03 order ...→05, run E0382,
  chip→HashMap). Collections 4→5; corpus 48.
  ✅ **Operator overloading (09-advanced 3→4)** (`<pending>`): added `04_operator_overload_add` (id
  `advanced/04_operator_overload_add`, advanced): `a + b` on a custom `Point` with no `Add` impl → **E0369**
  "cannot add `Point` to `Point`" (probed clean single). Teaches that operators map to `std::ops` traits —
  `a + b` is `a.add(b)` — and you overload `+` for your own type with `impl Add` (`type Output` + `add`).
  Fix compiles+runs ("...= Point { x: 4, y: 6 }"). E0369 reused (vs generics/01_bound's trait-bound context)
  in a new frame. No new chapter (ch20-02 vendored); book_ref using-default-generic-parameters-and-operator-overloading.
  Added an "operator overloading" glossary term (46→47). Verified: verify-exercises 49/49 (E0369), anchors,
  golden-corpus, glossary 3/3, live (09 01→02→03→04, run E0369, chip→operator-overloading). 09-advanced 3→4;
  corpus 49.
  ✅ **Supertraits (09-advanced 4→5; corpus hits 50)** (`<pending>`): added `05_supertrait_display` (id
  `advanced/05_supertrait_display`, advanced): `trait Labelled: fmt::Display` requires Display as a SUPERTRAIT,
  but `Sku` impls `Labelled` without `Display` → **E0277** "`Sku` doesn't implement `Display`" (probed: dual
  E0277 — the supertrait bound on the impl line + the default method's `{self}` usage — same root cause, both
  fixed by one `impl Display`; coherent like DST's E0277+E0308). Teaches `trait A: B` (B is the supertrait, so
  A's methods can rely on B's behaviour). Fix compiles+runs ("[SKU-7]"). No new chapter (ch20-02 vendored);
  book_ref using-supertraits. Added a "supertrait" glossary term (47→48). Verified: verify-exercises 50/50
  (E0277), anchors, golden-corpus, glossary 3/3, live (09 01→05, run E0277, chip→supertrait). **09-advanced =
  unsafe/orphan/DST/operator-overload/supertrait (5); CORPUS = 50 exercises / 11 phases.**
  CONSIDERED clippy rpro-tui this tick (rotate) but its 8 warnings are judgment-y (3 too_many_lines need
  `#[allow]`, 2 similar_names, 2 map_or_else) + the CI gate stays deferred = low-ROI; took the clean
  content pick instead. NEXT: `.get()`→Option HashMap (later phase), iterators depth, match-on-Err, async (ch17).
  ✅ **Trait objects / dynamic dispatch (07-generics-traits-lifetimes; corpus 50→51)** (`c7f6ae8`): the corpus
  taught STATIC dispatch (generic bounds, trait bounds) but had NOTHING on `dyn Trait` — a core gap. Added
  `02c_trait_object_dyn` (id `traits/03_trait_object_dyn`, intermediate; `02c_` sorts with the traits group):
  `vec![Circle {..}, Square {..}]` where both impl `Shape` → **E0308** "mismatched types — expected `Circle`,
  found `Square`" (the first element fixes the Vec's element type; the `Square` is rejected). Teaches WHY trait
  objects exist (a homogeneous `Vec<T>` can't hold a mix); the fix is `Vec<Box<dyn Shape>>` and calling `area()`
  through the box is dynamic dispatch. Framing guides via the symptom + Book pointer WITHOUT writing
  `Box<dyn Shape>` (that's in the server-side solution_outline; verified absent from /api/current). Vendored
  `book/ch18-02-trait-objects.md`; two book_refs (using-trait-objects-... + performing-dynamic-dispatch), both
  resolve. Added a "trait object" glossary term (48→49; aliases incl. dyn / dynamic dispatch / trait-objects).
  Verified: verify-exercises 51/51 (E0308), anchors 80/80, golden-corpus, glossary 3/3, LIVE (selected exercise,
  predict "fails"→failed, Assist→E0308, chip→Trait Object, no leak, screenshot). **07 now pairs static dispatch
  (generic_bound/trait_bound) with dynamic dispatch (trait_object_dyn); CORPUS = 51 exercises / 11 phases.** NEXT:
  match-on-Err / `?` on Option, iterators depth (map/filter/lazy), generic structs, async (ch17).
  ✅ **Enums carry data — bind it in `match` (02-control-flow; corpus 51→52)** (`41630b6`): the only enum
  exercise (`02_match_exhaustive`) used a FIELDLESS enum (`Light`), so the corpus never had the learner
  define a DATA-CARRYING enum and destructure it — the core algebraic-data-type skill behind Option/Result
  and domain modelling. Added `02b_match_enum_data` (id `control-flow/05_match_enum_data`, beginner; `02b_`
  sorts right after `02_match_exhaustive` → enums grouped): `Shape::Rectangle` (a tuple variant carrying
  `(f64, f64)`) matched as a bare unit pattern → **E0532** (NEW code), whose `help:` names the pattern shape
  `Shape::Rectangle(_, _)`. The `Circle(r)` arm is already correctly bound right above the broken one =
  worked-example scaffold (reads→writes). Fix `Rectangle(w, h) => w * h` compiles+runs. **NO-LEAK CARE
  (caught by the live grep):** reworded the 2nd book_ref so the always-visible `why` no longer spells out
  `Rectangle(w, h)` (the compiler's own help shows `(_, _)`; the pointer mustn't pre-give the names), and the
  new "enum with data" glossary term (49→50) uses a DIFFERENT example (`Event { Click, KeyPress, Closed }`)
  so the chip teaches the concept without being this exercise's answer key. book_refs ch06-01#enum-values +
  ch06-02#patterns-that-bind-to-values (both already vendored). Verified: verify-exercises 52/52 (E0532),
  anchors 82/82, golden-corpus, glossary 3/3, LIVE (predict fails→failed, Assist→E0532 + L26 jump,
  chip→Enum With Data, no leak, screenshot). No crates/*.rs. **CORPUS = 52 exercises / 11 phases.** NEXT:
  match-on-Err / `?` on Option, iterators depth, generic structs, index-OOB runtime panic (collections), async.
  ✅ **Index-out-of-bounds RUNTIME panic (03-text-and-collections; corpus 52→53)** (`6b6f797`): filled TWO
  gaps — the collections phase had NO runtime-outcome exercise, and index-OOB is THE canonical Vec panic (a
  core memory-safety lesson: Rust bounds-checks at run time and panics rather than reading garbage like C).
  Added `06_index_out_of_bounds` (id `collections/06_index_out_of_bounds`, beginner): `scores[3]` on a
  3-element Vec COMPILES, then panics at run time. `expected_runtime_panic = "index out of bounds"` (robust
  substring) — the 4th runtime-outcome exercise (after unwrap-None, unwrap-Err, RefCell) and the FIRST in
  collections. Framing teaches zero-based indexing + the run-time bounds check and asks the learner to land
  the lookup in bounds WITHOUT naming the fix; solution_outline (server-side) gives a valid index AND the
  `.get()`→Option safe path, but per the phase-sequencing rule `.get()`/Option is only OFFERED, not required
  (Option is taught in phase 05). book_ref ch08-01#reading-elements-of-vectors (already vendored); new
  glossary term "indexing and bounds checking" (50→51). Verified: verify-exercises 53/53 (runtime branch
  compiles-clean→runs→asserts panic), anchors 83/83, golden-corpus, glossary 3/3, LIVE (predict fails→failed,
  panic in terminal, chip→Indexing And Bounds Checking, no leak, screenshot). No crates/*.rs.
  **CORPUS = 53 exercises / 11 phases.** NEXT: match-on-Err / `?` on Option, iterators depth (map/filter/lazy),
  generic structs, integer-overflow runtime panic, async (ch17).
  ✅ **Integer overflow panics in debug (01-basics; corpus 53→54)** (`8145831`): fundamental ch3 gap + first
  runtime-outcome exercise in 01-basics. `fn area(w: u8, h: u8) -> u8 { w * h }` called with 20*20=400 COMPILES
  then panics at run time ("attempt to multiply with overflow") AT the learner's `w * h` line. **Design catch:
  a LITERAL overflow (`200u8 + 100u8`) is a COMPILE error (deny-by-default `arithmetic_overflow` const-eval), so
  the values flow through fn params rustc can't const-fold → a true runtime panic, kept within basics scope
  (functions + integers, no loop/array forward-ref).** Teaches the fixed range + the debug overflow check (vs
  release wrap / C UB) and asks the learner to widen the type WITHOUT naming the fix; solution_outline (server-
  side) gives u16/u32 + checked_/saturating_/wrapping_. book_ref ch03-02#integer-types (already vendored); new
  glossary term "integer overflow" (51→52). Verified: verify-exercises 54/54, anchors 84/84, golden-corpus,
  glossary 3/3, LIVE (predict fails→failed, panic in terminal, chip→Integer Overflow, no leak, screenshot). No
  crates/*.rs. **CORPUS = 54 exercises / 11 phases.** NEXT: match-on-Err / `?` on Option, iterators depth,
  generic structs, async (ch17).
  ✅ **Breadth: 3rd advanced exercise (09-advanced 2→3)** (`<pending>`): added `03_unsized_str` (id
  `advanced/03_unsized_str`, advanced) — `fn first_char(text: str)` takes `str` BY VALUE → **E0277** "the
  size for values of type `str` cannot be known at compilation time" (the headline; a coherent secondary
  E0308 from the call, both fixed by `str`→`&str`). Teaches **dynamically sized types / `Sized`** — the
  "why `&str` not bare `str`" aha, one of the most fundamental advanced concepts. Probed: E0277 is first;
  the `&str` fix compiles+runs ("first char is f"). Vendored `ch20-03-advanced-types`; book_ref →
  dynamically-sized-types-and-the-sized-trait. Added a "dynamically sized types (DSTs) and Sized" glossary
  term (43→44). Verified: verify-exercises 45/45 (E0277), anchors, golden-corpus, glossary 3/3, live
  (09-advanced 01→02→03, run E0277+E0308, chip→DST). 09-advanced 2→3; corpus 45. Thinnest phases now all
  ≥3 except 05b(3); NEXT: 2nd lifetimes (07, hardest topic, only 1), custom error types, or more advanced.
  ✅ **2nd LIFETIMES exercise (07 lifetimes 1→2)** (`<pending>`): lifetimes is the hardest topic but 07 had
  only `01_longest` (function lifetimes). Added `03b_struct_lifetime` (id `lifetimes/02_struct_lifetime`,
  intermediate; `03b_` sorts after `03_lifetime_longest` → lifetimes grouped): a struct `Excerpt { part: &str }`
  holds a reference with no lifetime → **E0106** "missing lifetime specifier" (probed clean single). Teaches
  **lifetimes in struct definitions** (Book ch10-03 §"In Struct Definitions") — the `struct Excerpt<'a> { part:
  &'a str }` fix (compiles+runs after). Distinct from `01_longest` (function vs struct lifetimes; reuses E0106
  in a new context). No new chapter (ch10-03 vendored); book_ref → in-struct-definitions. **Reused the
  existing "lifetime annotations" glossary term** — just added `struct-lifetime` as an alias (no new term).
  Verified: verify-exercises 46/46 (E0106), anchors, golden-corpus, glossary 3/3, live (07 lifetimes 01→02,
  run E0106, chip→lifetime-annotations via alias). 07 lifetimes 1→2; corpus 46. Checked first that
  `ownership/03_dangling` already covers E0515 (no dup). NEXT: custom error types, supertraits/operator-
  overloading (ch20-02 vendored), 3rd lifetimes (method definitions), or async (ch17).
- ✅ **Breadth, not depth: filled the sparsest phase `09-advanced`** (`<pending>`, advisor decision rule —
  once a topic's core is covered, prefer breadth = the thinnest reachable phase over a 3rd variant of the
  one just touched). 09-advanced had only `01_unsafe_deref`; added `02_orphan_rule_newtype` (id
  `advanced/02_orphan_rule_newtype`, advanced): `impl Display for Vec<String>` (foreign trait on foreign
  type) → **E0117** "only traits defined in the current crate can be implemented for types defined outside"
  — the coherence/orphan rule. Probed clean single E0117 even with a `println!` (rustc still records the
  rejected impl for resolution, so no secondary E0277). The fix is the **newtype pattern** (wrap the foreign
  type in a one-field struct you own) — a genuinely important Rust idiom. Vendored `ch20-02-advanced-traits`;
  book_refs → `ch10-02-traits` # implementing-a-trait-on-a-type (the rule) + `ch20-02` #
  implementing-external-traits-with-the-newtype-pattern (the fix). Added "the orphan rule and the newtype
  pattern" glossary term (39→40). Verified: verify-exercises 41/41 (emits E0117), anchors, golden-corpus,
  glossary 3/3, live (09-advanced now 01→02, run E0117, chip→orphan-rule term). 09-advanced 1→2; corpus
  41 exercises. NEXT thin phases: 08-concurrency (3), 07-generics (3).
  ✅ **Breadth, tick 2: 08-concurrency was all SHARED-STATE, no MESSAGE PASSING** (`<pending>`): the 3
  existing exercises (spawn-move E0373, Rc-not-Send E0277, Arc-Mutex E0594) all cover sharing memory; the
  channels/message-passing model (Book ch16-02) was absent — no exercise AND ch16-02 unvendored. Added
  `08-concurrency/04_channel_send` (id `concurrency/04_channel_send`, advanced): `tx.send(message)` moves
  the value into the channel, then using `message` again → **E0382** "borrow of moved value" (probed clean).
  Lesson: send MOVES ownership to the receiver — channels are the move-don't-share half of concurrency.
  Vendored `ch16-02-message-passing`; book_refs → transfer-data-between-threads-with-message-passing +
  transferring-ownership-through-channels. Added "channels and message passing" glossary term (40→41;
  reworded to drop a Go-proverb echo — charter: no foreign-language analogies). Verified: verify-exercises
  42/42 (emits E0382), anchors, golden-corpus, glossary 3/3, live (08-concurrency now 01→04, run E0382,
  chip→channels). 08-concurrency 3→4; corpus 42.
  ✅ **Breadth, tick 3: 07-generics-traits-lifetimes** (`<pending>`): had one each of generics(E0369)/
  traits(E0277)/lifetimes(E0106). Added a 2nd TRAITS exercise `02b_trait_in_scope` (id
  `traits/02_trait_in_scope`, intermediate; `02b_` sorts after `02_trait_bound`, keeping traits grouped):
  `Circle` implements `Area` in a module, but `main` calls `c.area()` without `use`-ing the trait →
  **E0599** "no method named `area` found …" — the impl exists, it's just not IN SCOPE. Great
  read-the-compiler exercise: the E0599 `help:` line literally names `use crate::geometry::Area;`. book_ref
  → ch10-02-traits # implementing-a-trait-on-a-type (the Book's "must bring the trait into scope" note).
  Added "trait methods must be in scope" glossary term (41→42). Verified: verify-exercises 43/43 (E0599),
  anchors, golden-corpus, glossary 3/3, live (07 order generics→traits→traits→lifetimes, run E0599,
  chip→trait-in-scope). 07-generics 3→4; corpus 43. NEXT: 2nd exercises in the now-4-deep phases, or
  error-handling growth (match-on-Err, custom errors, unwrap/expect runtime), or async (ch17).
- ◐ **Integrate Rust by Example / Rustlings / Cookbook / Exercism** — licenses verified via gh
  (RBE Apache-2.0, Rustlings MIT, Cookbook CC0-1.0, Exercism MIT) + cataloged in
  rust-textbook/catalog/EXTERNAL-MATERIALS.md with an integration plan (Rustlings first).
  Next: adapt a few early Rustlings exercises into exercises/ (re-verified + attributed).
  **VERDICT (2026-06-24, examined at convergence):** actionable but **deliberately DEFERRED while
  Paul-gated.** The next slice (adapt Rustlings exercises) *grows the unmerged review pile*, and the
  `exercises/` corpus is already saturated against the shipped lessons (54 exercises, all 11 phase
  dirs) — so adapting more now is low-marginal-value + pile-growing, the quadrant to avoid mid-review.
  Better to land it **after** Paul reviews the batch, so adapted exercises align with his merge/scope
  decisions (and don't enlarge what's blocking him). Not blocked-blocked; just correctly low-priority
  until review. (Examined per advisor's "is #22 actionable or just unpicked?" — answer: actionable,
  parked-by-choice. Loop should not re-examine each tick.)

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
- ◐ **Clippy hygiene (per-crate)**: **7 crates now clippy-clean** — `rpro-book`, `rpro-lang`, `rpro-lang-rust`
  (per prior ticks; `format_collect`→`fold`, a justified `#[allow(struct_excessive_bools)]` on `EditorAssists`),
  the already-clean `rpro-storage-fs`/`rpro-runner`, plus (`<pending>`) **`rpro-state`** (4 `too_long_first_doc_paragraph`
  → split the summary line on review.rs/tutor.rs) and **`rpro-toolchain-local`** (3: `match`→`if let` in the
  try_wait poll loop; `push_str(&format!)`→`write!` via `std::fmt::Write`; `map_spawn_err` now takes
  `&std::io::Error` — both callers pass `&e`). All corrective, mechanical, no `#[allow]`; verified 0 warnings +
  tests (30 / 5) + fmt + seam-guard. Remaining (4 crates): `rpro-tui` (8), `rpro-core` (6), `rpro-serve` (4),
  `rpro-cli` (2 `similar_names` — judgment call: rename or `#[allow]`). rpro-core/serve carry the `?Send`-toolchain
  `future cannot be sent` (×6) which needs a justified `#[allow]`. **Adding the clippy `-D warnings` CI gate is
  a SEPARATE, LATER task and stays DEFERRED — it's preventive + CI-only (unverifiable from the loop env), same
  bar that defers cargo-audit's CI job.** This tick only reduced existing debt.
  **GROUND-TRUTH UPDATE (2026-06-24, tick 29): the remaining ~24 warnings are dominated by aggressive
  `clippy::nursery` lints** — the workspace `Cargo.toml` sets BOTH `pedantic = warn` AND `nursery = warn`
  (priority −1). nursery is officially unstable/false-positive-prone, and that's what these are: `future_not_send`
  (×6, on `Core`'s async methods that await through `Box<dyn Toolchain/Storage>` — but the crate is explicitly
  wasm-safe/single-threaded/loopback, so the futures are never sent across threads = false positive),
  `option_if_let_else` (style churn — the `if let/else` is often clearer than a nested `map_or_else` closure),
  `redundant_clone` (on `state.store_root.clone()` — you can't move out of `&state`, so the clone may be
  required). Only a few are clean corrections (`doc_markdown` missing-backticks ×2, the `too_long_first_doc`
  reflows). **So the real decision is a LINT-POLICY one, not per-warning churn: either (a) keep nursery and
  scatter justified `#[allow]`s, or (b) relax `nursery` to `allow`/remove it (pedantic alone is the usual bar).
  That's a project-wide quality-bar call → flagged for Paul, LOW priority, deferred alongside the `-D` gate.**
  Did NOT churn the debatable ones this tick.
- ✅ **`SECURITY.md` CORRECTION + freshen** (`<pending>`): the prior "SECURITY.md is MISSING" claim was
  WRONG — it lives at **`docs/SECURITY.md`** (the "§4" the notes cite); the earlier check grepped the repo
  root only. It's a thorough review and was just **re-freshened** to current state (the doc's own rule:
  "re-run when the endpoint set / wire protocol changes" — and they had): (1) the **Answer-leak** row now
  lists `expected_runtime_panic` among the omitted server-side fields (runtime-outcome model; the
  `current_json_omits_the_answer` test asserts all three); (2) added a **`/api/glossary`** Controls row
  (read-only, term = alias map-key via `Glossary::get`, never path-joined); (3) **§4** updated — `cargo
  audit` now runs locally and is **CLEAN (0 vulns / 236 deps / 1138 advisories)**, dropped the stale
  "crates.io unreachable" line. cargo-deny still uninstalled → cargo-audit+deny CI job stays a tracked
  task (#21), revisit on a concrete dependency-vuln signal not preventively.
- ✅ **e2e a11y regression fixed** (`<pending>`): the serious `color-contrast` violation was the
  *selected* mode-switcher button — white `#fff` on `--accent` `#f74c00` = **3.5:1** at 11px (needs
  4.5:1); regressed when the switcher landed (`158c997`). Deepened just that button's bg to
  `#c44000` (white = ~5.1:1, self-contained so passes in both themes), preserving the filled-orange
  "selected" affordance. `a11y.spec.js` gate green again; verified live (axe probe + screenshot).
- ✅ **White-on-`--accent` token (`59cb2f7`, 2026-06-25)** — closed the 2026-06-23 note: the bright
  `#f74c00` only clears AA for *large* white text, and the contrast fix lived as a magic `#c44000`
  literal in `.modesw button.sel`. Promoted it to a documented `:root` token **`--accent-deep: #c44000`**
  (white ≈ 5.1:1 vs the bright accent's ~3.5:1) with a comment telling future code to use it — NOT
  `--accent` — under any small `color:#fff` element. Behavior-identical (`var()` → same `#c44000`); the
  existing `a11y.spec.js` axe gate is the catch for anything that ignores the guidance. Verified LIVE
  (rpro-serve + Playwright): selected button computes `rgb(196,64,0)`/white, a11y + web + tier e2e 4/4,
  screenshot read. (The `.run` button keeps the bright-accent gradient — it passes on size; if it ever
  shrinks, switch it to `--accent-deep`.)
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
- ✅ **GATE RESOLVED (2026-06-25): the "show Paul the matrix first" lesson gate is RELEASED.** Paul: "finish
  your tasks, stop ticking", "you dont need me to baby sit", "just keep going so you dont need me." The loop
  now authors the remaining DRAFT-matrix lessons (rows 49–62) AUTONOMOUSLY — no review needed. **DONE: all 37
  lessons (L1–L37) + 11 phase reviews (quiz+cheatsheet each) shipped to rust-textbook/main, and the concept-matrix
  rows 49–62 are reconciled (DRAFT→SHIPPED, Lesson# L24–L37 filled — `4e51b9b`).** The matrix's `DECIDED` token
  stays reserved for the human-reviewed rows 1–48; rows 49–62 are `SHIPPED` (AI-authored + adversarially verified +
  gate-released), an honest distinction kept visible should Paul ever want to review-and-promote them. The
  review/merge of `textbook-integration`→`main` is still a Paul decision when he wants it, but it no longer blocks
  the loop. (`docs/REVIEW-GUIDE.md` remains a reading path if/when you do review.) The note below is kept for history.
- ⚠️ ~~**MILESTONE + the un-gated lesson lane is now EXHAUSTED — your review unblocks the rest**~~ *(SUPERSEDED — gate released above)* (updated
  2026-06-24). Since the earlier note, the lessons lane re-opened (the "show Paul the matrix first" gate is
  **matrix-status-scoped**, not phase-numbered: DECIDED rows 1–48 are un-gated; only DRAFT rows 49–62 wait) and
  has now been driven to completion for every un-gated phase. **What's done and waiting on `textbook-integration`:**
  1. **rust-textbook: Phases 1–5 complete + Phase-6 "Organizing" complete** — **23 lessons (L1–L23)**, quizzes
     phase2–6, cheatsheets phase1–6, the `likes` kata. Every snippet compile-run on rustc 1.95.0/ed2024;
     real compiler errors captured verbatim; 7-part format; write-first (no solutions); no analogies; no
     answer-leak. **This is the single biggest learner-facing deliverable — please review the pedagogy & merge.**
  2. **Tempered-Studio: 48 exercises + editor/glossary/runtime-model changes**, all unmerged on
     `textbook-integration` (which has never merged to `main`). Do they match your intent? OK to merge?
  3. **THE REMAINING GATE — your matrix review (the real blocker now):** Phase-6 **generics/traits/lifetimes/
     tests/cargo** and **Phases 7–9** are mapped but **DRAFT (rows 49–62)**, AI-authored, not human-reviewed.
     Per the corpus's "show Paul the matrix first" rule, **no lessons there until you sign off the DRAFT rows.**
     This is what stops the loop from finishing the textbook — please review rows 49–62 when you can.
  4. **Scope calls:** rust-analyzer IDE (#19, big), platform/distribution (#23, mostly env-gated), `--solution`
     rename (below).
  With the un-gated lessons done, the loop now rotates to lower-value bounded work (housekeeping: the missing
  `quizzes/phase1.md`, a stale toolchain cite; editor polish #18). Your review of #1–#3 unlocks the high-value
  path. (Fresh PushNotification sent — the prior one predates the entire L9–L23 lesson body.)
- ✅ **RESOLVED — Book chapter sources** (2026-06-23). The chapter-bundling blocker is gone: `curl` to
  `raw.githubusercontent.com/rust-lang/book/main/src/*.md` works in this env, so authentic chapter
  markdown can be vendored the right way (Paul: "do what's best — authentic sources"). NOT WebFetch —
  that's a question-answering fetch that can't return verbatim source. **Vendored `ch13-01-closures.md`**
  (verified: loads, in TOC, serves cleaned via `display_markdown`, renders live). `ch15-*` (smart-pointers)
  and `ch17-*` (async) follow the same proven path as those exercises are authored. Tasks #14/#15 unblocked.
- **Rename the CLI `--solution` flag?** (2026-06-23). The flag is now honestly documented (it jumps to
  the top hint rung = the book/source review; the literal answer is never printed), but its *name* still
  implies "give me the solution" — arguably at odds with the never-hand-the-answer charter. Options: keep
  (with the honest help), or rename to e.g. `--stuck` / `--last-resort` (a small CLI API change + update
  the 3 exercise-starter comments that reference it). Left as-is this tick (behavior/UX unchanged).

## Audit cadence
Re-run the educational-fidelity audit + CI gate set after each batch of pedagogy
changes. **Throttle agent fan-out** — concurrent heavy workflows hit the rate limit
(2026-06-23); run audits sequentially / small.

### Audits run
- ✅ **Educational-fidelity audit 2026-06-23** → `docs/audits/EDU-FIDELITY-2026-06-23.md`.
  Live-driven (web gui via Playwright + screenshots; CLI/TUI parity from prior ticks).
  **Verdict: the core pedagogy is faithfully implemented and works live** (predict-gate,
  by-hand real errors, guide-only Tutor, gated+escalating hint ladder, book-pointers,
  soft-gating, reads→writes). Top gaps: **G1 built-in term defs / glossary ABSENT**
  (Paul's explicit ask — highest-value *unblocked* content feature, task #24); G2 Learn/
  Assist/Dev under-differentiated (#18); G3 lessons Phases 2-9 (blocked, #16/#17); G4
  content interactivity exercise-only (#17/#24); G5 functional/advanced exercises thin
  (blocked, #14/#15); G6 editor pane clips framing comments (#25).
