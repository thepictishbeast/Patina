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
  `autocomplete/autocorrect/autocapitalize/spellcheck` off for all modes and has NO Tab/auto-close/
  auto-indent assists to gate.
  Remaining (#18, still open): TRUE **inline-in-editor** diagnostics — surface `dg.span.line` as a
  gutter marker / underline on the offending line in the highlight overlay (Assist/Dev only). The
  data (line spans) is already in the run response; only the editor-overlay rendering is left.
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
- ☐ **Lessons for Phases 2–9** (only Phase 1 / L1–L8 exist; matrix DRAFT rows 49–62 done).
- ◐ **Matrix: re-review rows 51–62** (rate-limited). ✅ Fixed the HTML-entity in row 58 title
  (`Advanced patterns &amp; matching` → literal `&`, matching every sibling row; it was the only
  entity left in the matrix — verified). REMAINING (Paul's review domain): add the missing
  back-references the DECIDED rows use, decide Lesson# assignment for the DRAFT rows.
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
  TODO: async (ch17, may need its own model work); integer-overflow / index-OOB runtime exercises; more
  closures; grow async in 08-concurrency.
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
  NEXT in this phase: matching on `Err` / recover-vs-propagate, custom error types, `unwrap`/`expect` (could
  be a runtime-panic exercise), `?` on `Option`.
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
  chip→channels). 08-concurrency 3→4; corpus 42. NEXT thinnest: 07-generics (3), then 2nd exercises in the
  4-deep phases or error-handling growth.
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
- ◐ **Clippy hygiene (per-crate)**: **5 crates now clippy-clean** — `rpro-book` (`<pending>`: doc-paragraph
  split, unused `.peekable()` dropped, `sort_by`→`sort_by_key(Reverse)`), plus `rpro-lang` + `rpro-lang-rust`
  (`<pending>`: `format_collect` ×2 → `fold`+`push`, identical `display` output confirmed by 7 tests; and the
  cross-cutting `struct_excessive_bools` resolved with a justified `#[allow]` on `EditorAssists` — four
  INDEPENDENT serialized editor-tier flags, where a bitfield/enum would be strictly worse), and the already-clean
  `rpro-storage-fs`/`rpro-runner`. Remaining (6 crates, all genuine fixes except one allow): `rpro-tui` (8),
  `rpro-core` (6), `rpro-serve` (4), `rpro-state` (4 over-long doc paragraphs), `rpro-toolchain-local` (3:
  `match`→`if let`, pass-by-value), `rpro-cli` (2) — mostly doc-paragraphs/redundant-clone/str-lifetime, plus
  the `?Send`-toolchain `future cannot be sent` (×6, rpro-core/serve) which needs the same kind of justified
  `#[allow]`. Once all clean, add a clippy gate to CI (SECURITY.md §4).
- ☐ **`SECURITY.md` is MISSING but referenced** (found 2026-06-24): the clippy/audit notes and prior
  memory cite "SECURITY.md §4", but the file does not exist in the repo. Either CREATE it documenting the
  posture already built (loopback-only bind, op-whitelist, no-leak DTOs, `DefaultBodyLimit`/413,
  path-traversal blocks, cargo-audit clean) — a real artifact — or scrub the dangling references. A clean
  bounded tick on its own. NOTE: security-CI gates (cargo-audit/deny job) were considered and DEFERRED —
  cargo-audit was already clean (preventive, not corrective), cargo-deny isn't installed, and a CI-only
  change can't be verified from the loop env (fails the "verified improvement" bar); revisit on a concrete
  dependency-vuln signal, not preventively.
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
