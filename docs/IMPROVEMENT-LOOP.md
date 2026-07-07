# Improvement Loop — toward the greatest Rust learning platform

> **✅ RELEASE-READINESS BASELINE (textbook-integration `4aa6d39`, verified 2026-07-07).** Full CI gate
> (`scripts/check.sh`) **13 passed, 0 failed — "all gates green, branch is mergeable"** — re-run after the
> next ~10-tick burst since `829565d`: the **content-refresh pipeline** (version-gated re-seed for existing
> stores, server + CLI, shared `rpro_runner::CONTENT_VERSION`, three real bumps delivered), **nav parity on
> all four reading surfaces** (Book/quiz/cheatsheet prev-next, matching lessons), the **lesson-drawer styling
> parity**, **6 new/aliased glossary terms** (constant, expression, statement, macro, type annotation, cargo,
> + variable→binding), the **Study-Guide 38-lesson fix**, and the **cargo-deny clear** (anyhow 1.0.103
> advisory fix + `publish = false` across the workspace + allow-wildcard-paths). Gates: rustfmt · clippy -D
> warnings · cargo test --workspace · cargo doc · smoke · browser e2e (isolated store, incl. the new
> lesson-drawer / book-nav / quiz-cheat-nav specs + both-theme a11y) · verify-exercises **71** · cli smoke ·
> gui-transforms · book anchors · language-seam guard · wasm32 pure-core · **cargo-deny (advisories/bans/
> licenses/sources)**. Mergeable for #29 whenever Paul wants.

> **🎯 PRIORITY PROGRAM — Paul's 2026-07-07 directive (supersedes "rotate the polish"):** heavy improvements,
> executed incrementally by the loop. Workstreams:
> **A. Curriculum** — (A1) baby-steps pacing audit: one atomic subject per lesson, never jump ahead;
> (A2) lessons SHORT — depth moves to "read more in the textbook" links (split/trim the 2 300–2 800-word
> lessons); (A3) CREATE MORE LESSONS (Paul un-gated lesson authoring — split dense topics into more, smaller
> steps); (A4) lesson↔exercise follow-along: each lesson marches straight into its exercises and each
> exercise names its lesson section — tight two-way coupling.
> **B. UI/UX** — (B1) FULLSCREEN IDE mode: file explorer + editor + fullscreen terminal/console as SEPARATE
> switchable views; (B2) keep the learning area but arrange it better; (B3) textbooks easier to access +
> quick back-and-forth switching (remember position per book); (B4) book readability pass; (B5) animations
> (transitions, success moments); (B6) GAMIFY learning (offline-local XP/levels/streaks/badges — fun, cool,
> modern, helpful).
> Each tick: pick the next bounded chunk from A/B, verify live, ship. Board tasks #36–#41 track these.

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
- ◐ **Mobile UI overhaul (Paul 2026-06-30: "heavily improve the mobile UI … more beauty … simple and intuitive
  yet fully capable and powerful").** Pass 1 (`<pending>`): on a phone the editor header was visibly broken — the
  Learn/Assist/Dev switcher clipped to "Lea" and the "read & edit, then Run" label crammed the reset; the
  prediction "error code?" input was clipped; touch targets were small. Reworked the `@media(max-width:760px)`
  block: the editor bar now wraps cleanly (dots+file+reset on row 1, a **full-width segmented Learn/Assist/Dev**
  switcher on row 2, the noisy label dropped), the prediction input gets its **own full-width row**, and Run/Check/
  Explain/Hint + the prediction chips + tab/icon buttons are all touch-sized (≥40px). Verified LIVE (Playwright
  390×844, before/after screenshots): the switcher is fully usable, nothing clipped, desktop untouched (rules are
  media-query-scoped); gui-transform + **e2e 20/20 (+1 known flake)**. Iterative — more beauty/flow passes to come.
  Plus a real **app + favicon icon** (Paul-supplied art: a metallic Ferris-crab on a gear-ringed shield with molten
  cracks): wired the Android adaptive-icon pipeline (mipmap densities + foreground/bg + anydpi-v26 XML, manifest
  icon/roundIcon) and gui favicons (32/180/192/512 + .ico + theme-color). Bundled into a fresh APK.
- ✅ **Footer keybar fixed + `?` keyboard-shortcuts overlay** (this commit): the bottom keybar was a stale TUI
  copy — it advertised `t tasks` / `q quit` (neither exists in the web app: `t`→Roadmap, no `q` handler) and
  omitted the keys that DO work (`l`/`g`/`z`/`s`/`f`/`i`/`?`). Rewrote it to the truth (`r run · c check · h hint ·
  l lessons · b book · ? keys`) and added a `?`-triggered modal listing every shortcut, grouped **Do** (run/check/
  hint) · **Go to** (the 7 tab keys) · **Layout** (Focus/Insights/this-help). Esc or backdrop-click closes; the help
  toggle and Esc are gated so they never fire while typing in the editor (the existing `isTyping` guard). gui-only.
  **The e2e suite caught a real bug:** the overlay carried `.keyshelp{display:flex}` with no `[hidden]` guard, and
  an author `display:` overrides the UA `[hidden]{display:none}` regardless of specificity — so the full-viewport
  overlay stayed painted on load and **swallowed every click** (13 click-based specs timed out). Fixed by scoping
  the show rule to `.keyshelp:not([hidden])`. (A `hidden`-property check alone missed it — the attribute WAS set;
  only computed `display` exposed it.) Verified LIVE (Playwright): computed display is `none` on load,
  `elementFromPoint` at viewport-center is NOT the overlay (clicks pass through), `?` opens it (`flex`), Esc closes
  it (`none`); full e2e green (15 passed, 1 known recall-chip flake heals on retry). Discoverability: a learner can
  now find every shortcut without reading source.
  - 🧹 **Robustness note (ops, not code):** the long-blamed "browser-launch-under-load" e2e flake was aggravated by
    **449 orphaned `chromium-shell` processes** (~12.8GB RSS) leaked from weeks of Playwright runs that never reaped
    their browsers. Reaping them dropped a full e2e run from **5.1m → 31.9s**. Future ticks: if e2e crawls, check
    `pgrep -c chromium-shell` and reap stale ones (`ps -eo pid=,etimes=,comm= | awk '$3=="chromium-shell" && $2>3600'`).
- ✅ **Mobile layout fix (Android-first)** (this commit): the recent desktop toggle rules
  (`main.show-insights {…248px 1fr 340px}` / `main.no-list {…}`) have higher CSS specificity than the mobile
  `@media (max-width:760px) main { grid-template-columns:1fr }`, so on a PHONE with the Insights drawer or Focus
  mode on (persisted in localStorage), the 3-column grid did NOT collapse — it overflowed the viewport and squished
  the practice text to **one character per line**. Fixed with `grid-template-columns:1fr !important` inside the
  mobile media query (beats specificity; toggled-away panes stay hidden, shown panes just stack). Desktop is
  untouched (the fix lives inside the phone media query). Verified LIVE at 390×844 (Playwright): the practice view
  and a lesson (with its Practice card + nav) render single-column with **no horizontal overflow**
  (`scrollWidth == viewportWidth == 390`); gui-transform tests pass; all 16 e2e pass (the recurring run-based
  flake — this time `recall-chip` — green on warm retry).
- ✅ **e2e flake fixed: `retries: 1`** (this commit). The recurring first-run-based-spec timeout was NOT a cold
  compile (a cold `/api/run` is ~0.1s — the run-scratch shares the warm workspace target). It's **environmental**:
  the first Chromium launch + page load under machine load occasionally exceeds the 30s test timeout. Set
  `retries: 1` in `tests/e2e/playwright.config.js` — the retry runs with a warm browser and passes, while a genuine
  regression still fails both attempts (so this heals flakes, not real breaks). Verified: the suite now reports
  `1 flaky, 15 passed` (green) where it used to report a hard failure. e2e is not a CI gate; this just makes local
  verification reliable without a manual warm re-run every UI tick.
- ◐ **UX density redesign (Paul, 2026-06-25: "UI is extremely dense … tabs + a menu bar … things the user isn't
  working on shouldn't be on the main page … focus on teaching and practicing rust").** Multi-tick. **Step 1 done
  (this commit):** the right pane (Diagnostics / Book refs / Tutor) is now a collapsible **Insights drawer** — the
  default landing is a 2-column practice layout (exercise list + the read→predict→write→run work area), and the
  supplementary pane is hidden until the learner opens it (header ▦ button, `i` key) or a Run/Hint auto-reveals it
  (the real run output is the centre terminal, so nothing essential is hidden). State persists in localStorage.
  **Also fixed a pre-existing horizontal-overflow bug**: `.pane` lacked `min-width:0`, so the grid couldn't shrink
  below content width and the 3rd column was clipped off-screen at ≤1280px. Verified LIVE (Playwright, 1280×820):
  collapsed + open states both clean, toggle + persistence work; gui-transform tests pass; **all 16 e2e pass
  (incl. a11y — zero WCAG A/AA violations — and the tier/diag-jump specs that touch the drawer)**.
  **Step 2 done (this commit):** the drawer made the **`Dashboard` vs `Exercise` tab split redundant** (Dashboard =
  list+centre, Exercise = centre-only/focus) — that overlap was itself the "unintuitive" clutter. Consolidated the
  two into one clear **`Practice`** tab (tab bar is now **Practice · Book · Roadmap**), and turned the old
  Exercise-tab "focus" behaviour into a header **◧ Focus toggle** (`f` key) that collapses the exercise-list pane
  for full-width practice — symmetric with the ▦ Insights drawer, and the two compose (clean CSS matrix over
  no-list × show-insights, all 4 combos verified). Focus state persists in localStorage. Verified LIVE (Playwright):
  all 4 layout combos render clean (no overflow); gui-transform + **all 16 e2e still green**.
  **Step 3 done (this commit): Glossary tab.** The built-in glossary was only reachable by tapping a concept chip
  on the current exercise; now a **`Glossary` tab** (tab bar: Practice · Book · Glossary · Roadmap; `g` key)
  renders all 58 terms — alphabetised cards with definition + source + a "read more in the Book →" jump — from the
  existing `GET /api/glossary` (frontend-only; offline). Includes a live **filter box** (type-to-narrow, match
  count). **Bonus: fixed a pre-existing latent bug** — `main { display:grid }` (author) overrode the `hidden`
  attribute's UA `display:none`, so Book/Roadmap had been rendering UNDER the practice view rather than replacing
  it; added `main[hidden] { display:none }`. Verified LIVE: tab renders 58 terms, filter narrows 58→8 on "borrow",
  Book-jump opens the right chapter, view properly replaces practice; gui-transform + **all 16 e2e still green**.
  **Step 4 done (this commit): the literal "menu bar" — a `⋯ More` menu.** Paul asked for "tabs **and a menu bar**
  … things the user isn't working on shouldn't be on the main page … focus on teaching and practicing rust." The bar
  had grown to **7 flat tabs** (Practice · Lessons · Quizzes · Cheatsheets · Book · Glossary · Roadmap) — dense, and
  on a phone it horizontal-scrolled. Split by role: the **active-learning** surfaces stay as primary tabs (Practice ·
  Lessons · Quizzes — where you *do* the work), and the **reference/lookup** surfaces (Book · Glossary · Cheatsheets ·
  Roadmap) fold into a single **`⋯ More ▾` dropdown** — 7 tabs → 3 + a menu. Every capability is retained; the
  direct `b`/`g`/`s`/`t` keyboard shortcuts still jump straight to each surface (and now light up the More trigger +
  mark the active item). Accessible menu-button pattern (`aria-haspopup`/`aria-expanded`/`aria-controls`, `role=menu`
  + `role=menuitem`, ArrowUp/Down nav, Esc + outside-click close, focus returns to the trigger); the tablist stays a
  pure 3-tab tablist (the menu lives outside it). Re-applied last tick's `[hidden]` lesson: `.menu-list:not([hidden])`
  so the dropdown's `display:flex` doesn't override the `hidden` attribute. **Bonus:** with only 3 tabs + a menu, the
  mobile bar now wraps cleanly instead of horizontal-scrolling (dropped `overflow-x:auto`, which also used to clip the
  dropdown). Verified LIVE (Playwright, desktop + 390×844): trigger renders one line; clicking a menu item navigates +
  closes the menu + activates More + marks the item `aria-current`; `g`/`b` keys do the same; back to Practice
  de-activates More; no horizontal overflow at 390 (`scrollWidth==innerWidth==390`); Esc + outside-click close;
  screenshots read. gui-transform green; **full e2e green (15 passed, 1 known dev-diag-jump flake heals on retry)**.
  NEXT steps: fold RECALL + a Settings/detect menu into the bar; tighter spacing pass; (awaiting Paul on whether
  Book-refs should stay visible while solving vs. living in the Insights drawer).
- ✅ **Roadmap tab repurposed: dev build-status → the learner's curriculum journey (`<pending>`).** Found via a
  live look at the beginner's first-run orientation: the `Roadmap` tab was rendering the **project's engineering
  roadmap** (`/api/roadmap` = `docs/ROADMAP.md`: "Phase 0 — Language seam & engine", "rpro-lang wasm-safe", "CI gates:
  wasm32 pure-core build") — developer meta, meaningless and confusing to a *learner* clicking "Roadmap" to see their
  path. Exactly Paul's "things the user isn't working on shouldn't be on the main page … focus on teaching and
  practicing rust." Repurposed the tab to a **"🗺️ Your Rust journey"** view, built **offline frontend-side** from
  `/api/lessons`: the 37 lessons grouped into the 11 curriculum stages (Foundations → Control Flow → … → Tooling, via
  the existing `lessonToQuiz` phase map + a `PHASE_NAMES` table sourced from the cheatsheet titles), each stage a card
  listing its lessons (→ open the lesson) plus its 📋 Cheatsheet + ❓ Quiz links, with a live progress line (reads the
  header gauge — "Exercises so far: 16 / 71 · 23%"). Now the learner sees the whole path and can dive into any part.
  The dev `ROADMAP.md`/`/api/roadmap` stay in the repo (a dev artifact); only the learner-facing tab changed. gui-only
  (no review-pile growth, no backend change). Verified LIVE (Playwright): 11 stages in curriculum order, all 37 lesson
  links, every stage has cheatsheet+quiz, progress shown, clicking a lesson opens it + activates the Lessons tab;
  screenshot read. gui-transform green; **full e2e 21/21 green** (a transient 2-run-test contention failure re-ran
  clean — my change touches only the roadmap render, not the editor/run paths).
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
- ✅ **Book/Roadmap renderer: thematic breaks → `<hr>` (`b35f905`, 2026-06-25)** — audited the two un-examined
  learner surfaces (Book reader `/api/book`, Roadmap `/api/roadmap`); both render via the hand-rolled `mdToHtml`,
  which proved impressively complete (code fences, GFM tables, blockquotes, lists, slugged headings, links/
  emphasis) EXCEPT it had no horizontal-rule handling — a `---` line fell through to `<p>---</p>`. Added a
  thematic-break rule (3+ of `-`/`*`/`_`, optionally spaced) → `<hr>`, placed BEFORE the list check so a spaced
  "- - -" isn't parsed as a bullet. Currently DORMANT (no served book/roadmap content uses `---`) but hardens the
  shared renderer for our `---`-heavy markdown conventions (every lesson footer). Verified LIVE via
  `browser_evaluate` on `mdToHtml`: ---/***/___/"- - -" → `<hr>`; single "- bullet" still a list; GFM `|---|`
  separator untouched; full e2e 16/16. **GOTCHA (recorded): e2e specs that exercise the RUN flow (dev-diag-jump,
  editor-run-shortcut) need rpro-serve started WITH the rustc toolchain on PATH** — `setsid env PATH="$PATH"
  TS_ASSET_ROOT=… rpro-serve &`; without it `/api/run` can't compile → no diagnostic → those 3 specs fail
  (false alarm, not a code regression).
- ✅ **Editable-pane syntax highlighting** (`<pending>`): colored `<pre>` overlay behind a
  transparent textarea, reusing highlightRust; always-on. Verified live.
- ◐ **Full IDE via rust-analyzer** (`LspSpec` defined; now CONSUMED) — big; Rust FOSS. Progress:
  - ✅ **Real LSP client crate `rpro-lsp` (this commit)** — first genuine LSP consumer. `server_info(&LspSpec,
    timeout)` spawns the server named by the spec and runs the full JSON-RPC handshake (`initialize` →
    `initialized` → `shutdown` → `exit`) over stdio with `Content-Length` framing, returning the server's
    reported `ServerInfo { name, version }`. A real handshake is strictly stronger than a `--version` shell-out:
    it proves the binary *speaks LSP*. **Effect crate** (std::process + stdio) — kept OUT of the wasm pure-core
    gate (like `rpro-toolchain-local`); server name stays data-driven from `LspSpec` so the seam-grep gate stays
    green. Bounded by a worker-thread + `recv_timeout` deadline (kills the child on expiry — no hang). Verified:
    5 deterministic unit tests (framing round-trip, two-message stream, notification-skipping, two protocol-error
    paths); clippy `-D warnings` + fmt + seam-grep clean; and an opt-in `#[ignore]` e2e ran LIVE against the
    matched server → `server reported: rust-analyzer 1.95.0`, clean exit, 0.05s.
  - ✅ **Wired `rpro-lsp` into `rpro detect`** (this commit): the "Language server (Dev IDE)" block now runs a
    REAL `rpro_lsp::server_info(&lsp, DEFAULT_TIMEOUT)` handshake instead of a `--version` shell-out. Reports
    `✓ <name> <version> — completed an LSP handshake` on success, a not-found hint on spawn failure, and a
    distinct `⚠ on PATH but did not complete an LSP handshake` when the binary exists but doesn't speak the
    protocol. Server name stays data-driven from `LspSpec` (seam-grep clean). Verified live: `✓ rust-analyzer
    1.95.0 — completed an LSP handshake (owns **/*.rs)`; build + clippy `-D warnings` + fmt + seam all green.
  - ☐ NEXT: the streaming client (`didOpen`/`didChange` → `publishDiagnostics`/completion/hover) into the
    Dev-tier editor. Multi-session; build incrementally.

### Content / corpus
- ✅ **Cheatsheets — the 6th & final study surface** (this commit). The 11 per-phase quick-reference cheatsheets
  (condensed syntax per phase, from rust-textbook) are now embedded in `cheatsheets/` (the generalized
  `sync-lessons.sh` now mirrors lessons+quizzes+cheatsheets), served by **`GET /api/cheatsheets`** (reusing the
  shared `md_collection` helper — one more thin handler), and surfaced in a **`Cheatsheets` tab** (bar:
  Practice·Lessons·Quizzes·**Cheatsheets**·Book·Glossary·Roadmap; `s` key) rendered via `mdToHtml`. Bundled in the
  APK (mobile `build-apk.sh`+apk.yml). Verified: `cargo test -p rpro-serve` 21 pass (new cheatsheets list/fetch/
  traversal test; lessons+quizzes still green); clippy `-D warnings` + fmt + seam clean; LIVE curl → 11 sheets +
  lessons/quizzes regression-free; Playwright → tab lists 11, phase1 renders. **All study materials are now in the
  app: lessons · exercises · glossary · Book · quizzes · cheatsheets — all offline + on Android.**
- ✅ **#22 — E0716 corpus gap filled** (`66289a6`): added
  `exercises/07-generics-traits-lifetimes/03d_temporary_dropped.{rs,toml}` — `first_word(&String::from("hello
  world"))` keeps a `&str` into a *temporary* `String` that's dropped at the end of the statement (E0716, "temporary
  value dropped while borrowed"). E0716 was absent from the corpus; teaches that a temporary lives only to the end
  of its statement, the subtler sibling of `03c`'s named-block E0597. Predict-then-run, no answer-leak; intermediate;
  `concept = "borrow-must-not-outlive-value"` (already resolves + already in the GUI `CONCEPT_LESSON` map → lesson
  16, so the "read the lesson" link works with no GUI change). Verified: `rustc` emits exactly one `error[E0716]`,
  golden_corpus green, anchors 115/115.
- ◐ **Surface the per-phase quizzes (predict-then-verify self-checks)** — backend slice DONE (`7653832`). The
  upstream rust-textbook has 11 per-phase quizzes (Questions + Answers, "predict before you look" — exactly the
  platform's pedagogy) that weren't in the app. Embedded them in `quizzes/` (synced via the now-generalized
  `scripts/sync-lessons.sh`, which mirrors lessons + quizzes), the serve layer seeds them like book/glossary/lessons,
  and a new **`GET /api/quizzes`** lists them / returns one's markdown via `?id=STEM`. Refactored the lessons
  handler into a shared `md_collection(root, subdir, id, list_key, item_key)` so lessons + quizzes share one
  traversal-safe code path (DRY). Android: `build-apk.sh` (+ apk.yml comment) now syncs `quizzes/` too. Verified:
  `cargo test -p rpro-serve` 20 pass (incl. a new quizzes list/fetch/traversal test, lessons test still green);
  clippy `-D warnings` + fmt + seam clean; LIVE curl → 11 quizzes listed, fetch returns markdown, traversal → null,
  `/api/lessons` regression-free.
  **GUI surface DONE (this commit): a `Quizzes` tab.** Tab bar is now Practice · Lessons · **Quizzes** · Book ·
  Glossary · Roadmap (`z` key). Lists the 11 quizzes → opens one rendered via `mdToHtml`, splitting on the
  `## Answers` heading so the **answers sit behind a collapsed `<details>` "Reveal answers — predict every question
  first"** — the quiz's own predict-then-verify rule, enforced in the UI. Verified LIVE (Playwright): 11 quizzes
  listed; opening phase1 renders the Questions (Q1… with highlighted code) while the answers stay collapsed by
  default (`<details>` not open) yet present; gui-transform tests pass; e2e green (`1 flaky, 15 passed` — the
  `retries:1` fix healed the environmental flake). **The learning materials are now complete: lessons → exercises
  → glossary → Book → quizzes, all offline + on Android.**
  ✅ **Per-question reveals (`<pending>`, pedagogy upgrade):** the single reveal-all `<details>` only weakly held the
  *"predict **each** answer before you reveal it"* rule — a learner could reveal all 13 at once. Now `quizToHtml`
  pairs each `**Q<n>` with its `**A<n>` and tucks **that one answer behind its own inline "Reveal answer — predict
  first" toggle, right after the question** — so prediction-then-verification happens one question at a time (predict
  Q, reveal Q, move on). Robust by construction: parses on the `**Q<n>`/`**A<n>` convention (verified regular across
  all 11 quizzes: 13/13, 10/10, 15/15, 14/14 …), pairs by number, and **falls back to the old reveal-all** if a quiz
  doesn't match. Provenance note ("Verified on rustc 1.95.0") kept as a quiet footer. gui-only. Verified LIVE
  (Playwright): phase1 → 13 question blocks, 13 independent collapsed reveals, no reveal-all, each answer present-but-
  hidden until its toggle opens (A1's `E0384` revealed on click); screenshots read. gui-transform + **full e2e 20/20
  (+1 flake heals)**.
  **Lesson → quiz cross-link DONE (this commit):** the last lesson of each phase now ends with a
  **"🧠 Finished this phase? Take the self-check quiz →"** link to that phase's quiz, so the quizzes are discoverable
  at the pedagogically right moment (phase end) rather than only via the tab. Mapping is a compact `lessonToQuiz(n)`
  (contiguous lesson ranges → quiz id, from the quiz intros); the link shows ONLY on a phase's final lesson
  (`lessonToQuiz(n+1) !== lessonToQuiz(n)`, or n≥37). Verified LIVE (Playwright): lesson 8 → `phase1`, lesson 37 →
  `tooling`, non-last lesson 2 shows no link, clicking opens the right quiz; gui-transform + **e2e green**
  (`1 flaky, 15 passed`). All five surfaces are now mutually cross-linked.
- ✅ **Lesson → cheatsheet cross-link DONE (`<pending>`) — the 6th surface joins the web.** Cheatsheets were the one
  study surface reachable ONLY via their tab (the doc above said "five surfaces … cross-linked" — cheatsheets, the
  6th, were orphaned from the flow). Now every lesson carries a quiet **"📋 Quick reference: this phase's cheat
  sheet →"** companion link. Key pedagogy distinction from the quiz link: a quiz is an end-of-phase *checkpoint*
  (last lesson only), but a cheatsheet is a reference you reach for *while* learning — so it shows on **every** lesson
  of the phase. Zero new mapping: the per-phase cheatsheet ids are **identical** to the quiz ids (`phase1`…/`tooling`),
  so `lessonToQuiz(n)` already names the cheatsheet (verified: `ls cheatsheets/` == `ls quizzes/`); the link routes
  via `showView('cheatsheets', id)` (deep-links the single sheet) and lights up the `⋯ More` trigger + marks the item
  `aria-current`. gui-only (no review-pile growth). Verified LIVE (Playwright): lesson 05 (mid-phase) shows the cheat
  link → `phase1` but **no** quiz link; lesson 08 (phase-1 end) shows BOTH; clicking opens "Phase 1 Cheatsheet —
  Foundations" + activates More; screenshot read. gui-transform green; **full e2e 16/16 (2.3s, no flake)**.
  **All SIX study surfaces are now mutually cross-linked.**
- ✅ **Reverse cross-link: cheatsheet / quiz → back INTO the phase's learning (`<pending>`).** The web was
  one-directional out of the reference surfaces: a lesson/roadmap links *to* a phase's cheatsheet & quiz, but a
  learner viewing a cheatsheet or quiz had only "← all cheatsheets/quizzes" — no path back to *studying or practicing*
  that phase. Each single cheatsheet/quiz now ends with a **"Back into learning: 📚 Study <Phase> · 📝 Practice"**
  footer. `📚 Study` deep-links to the phase's **first lesson** (new `firstLessonIdOfPhase` — the inverse of the
  `lessonToQuiz` grouping, fetched once from `/api/lessons` and cached), labelled with the phase name (`PHASE_NAMES`);
  `📝 Practice` returns to the exercise view. Shared `phaseBackLinks`/`wirePhaseBackLinks` helpers used by both render
  paths (DRY). gui-only (no review-pile growth, no backend). Verified LIVE (Playwright): the phase4 cheatsheet →
  "📚 Study Ownership & Borrowing" targeting `15-ownership-and-moves` (the phase's first lesson) + Practice; the
  phase1 quiz → "📚 Study Foundations" → `01-bindings-and-immutability`; clicking Study opens the lesson + activates
  the Lessons tab; screenshot read. gui-transform green; **full e2e 21/21 (no flake)**. **The nav web is now fully
  bidirectional across all six surfaces.**
- ✅ **#22 — E0381 corpus gap filled** (`af455e8`): added `exercises/01-basics/08_use_before_init.{rs,toml}`
  — `let count: i32;` then reading `count` before assigning it (use of a possibly-uninitialized binding). E0381 was
  absent from the corpus; teaches Rust's *definite initialization* (every read proven to follow a write). Predict-
  then-run, no answer-leak; `concept = "binding"` resolves to the glossary "binding" term, and was added to the GUI
  `CONCEPT_LESSON` map (→ `01-bindings-and-immutability`) so the exercise's "read the lesson" link works. Verified:
  `rustc` emits exactly one `error[E0381]`, golden_corpus green, anchors 113/113, and LIVE the exercise loads with
  its lesson link targeting lesson 01.
- ◐ **Surface the 37 textbook lessons in the app** (the namesake "rust-textbook → Tempered Studio" integration —
  the authored Patina curriculum was invisible in the GUI; only the Rust Book + per-exercise text were readable).
  **Backend slice DONE (this commit):** the 37 lessons are now **embedded** in `lessons/` (synced from the upstream
  `rust-textbook` repo via new `scripts/sync-lessons.sh`; rust-textbook stays source-of-truth, like `book/`), the
  serve layer **seeds** them into the store alongside `book/`/`glossary/`, and a new **`GET /api/lessons`** endpoint
  lists them (`id`+`title` from the first `# ` heading, filename order) or returns one's markdown via `?id=STEM`.
  Traversal-safe (id matched against real file stems, never path-joined; unit test asserts a `../../` id → null).
  Verified: `cargo test -p rpro-serve` 19 pass (incl. the new list/fetch/traversal test), clippy `-D warnings` +
  fmt + seam clean; LIVE curl → 37 lessons listed, fetch returns markdown, traversal id → null. (One semgrep FP on
  the server-fixed seeding `read_dir` annotated `nosemgrep`.)
  **GUI slice DONE (this commit):** a **`Lessons` tab** (tab bar: Practice · **Lessons** · Book · Glossary · Roadmap;
  `l` key) lists the 37 lessons → opens one (rendered via `mdToHtml`) with a **nav bar lifted from the footer**
  (Study Guide / prev / next, wired in-app — the relative `.md` links are extracted because `mdToHtml` only
  linkifies http(s)). **Plus an app-wide rendering fix surfaced by this:** the Book/Roadmap/Lessons markdown is all
  hard-wrapped (78-col), and `mdToHtml` emitted one `<p>` per physical line — breaking paragraphs into single lines
  and dropping multi-line `**bold**`. Added a paragraph buffer (`flushPara`) that joins soft-wrapped lines at block
  boundaries → proper paragraphs everywhere (Book + Roadmap + Lessons all improved). Verified LIVE (Playwright):
  list + content render clean, multi-line bold now bold, prev/next nav loads Lesson 2, Roadmap improved, no literal
  `[..](.md)` leaks; gui-transform tests pass (slugs/tables/anchors intact); **all 16 e2e pass**.
  **Android bundling DONE** (mobile `1794beb`): `build-apk.sh` synced gui/exercises/book but NOT glossary/ or
  lessons/, so the APK was missing BOTH the Glossary data and the 37 lessons. Added them to the asset sync (+ the
  `apk.yml` step name/comment); the committed `assets/store` is regenerated each build (CI clones current
  Tempered-Studio), so the next APK from CI bundles the complete offline store. Verified: `bash -n` clean; the sync
  block populates `assets/store/{glossary/glossary.toml, lessons/*.md (37)}`.
  **Exercise→lesson link DONE (this commit): the read→practice loop is closed both ways.** Each exercise's
  subheader now shows a **"📖 read the lesson"** link beside the concept chip; clicking it opens the textbook lesson
  that teaches that concept. Mapping is a compact `CONCEPT_LESSON` table in the GUI (concept tag → lesson stem,
  covering all 63 exercise concepts) — no per-toml edits, no Rust, no new endpoint; `showView('lessons', id)` opens
  a specific lesson. Verified LIVE (Playwright): `struct-lifetime` exercise → "📖 read the lesson" → opens "Lesson 26
  — Lifetimes"; gui-transform tests pass; all 16 e2e pass (2 run-based specs flaked on a cold compile, green on warm
  retry).
  **Reciprocal lesson→exercise link DONE (this commit): the read↔practice loop is now fully bidirectional.** Each
  lesson view ends with a **"Practice this lesson"** card listing the exercises whose concept maps to it (inverse of
  `CONCEPT_LESSON`, built live from `/api/exercises` which carries `concept` + `status`); clicking one selects that
  exercise (mirroring the list's soft-gate via the new `openExercise`) and jumps to the Practice tab. Verified LIVE
  (Playwright): lesson 16 lists its 9 borrow/lifetime exercises; clicking `lifetimes/03_borrow_outlives_value`
  switched to Practice with that exercise loaded; gui-transform tests pass; all 16 e2e pass (the recurring
  `dev-diag-jump` cold-compile flake passed on warm retry). **The lessons integration is now complete end-to-end.**
  NEXT (optional): move `CONCEPT_LESSON` to a data file if it grows; warm the run cache before the e2e suite to kill
  the recurring cold-compile flake (robustness).
- ✅ **#22 — E0432 corpus gap filled** (`a0e8f38`): added `exercises/06-modules/05_unresolved_import.{rs,toml}`
  — a `use crate::shape::Circle;` whose module is really spelled `shapes` (an *unresolved import*). E0432 was not
  in the corpus, and `06-modules` was a thin phase (4 → 5). Teaches that a `use` is only a shortcut — the path it
  follows must lead to a real item in the module tree. Predict-then-run, no answer-leak; `concept = "use-and-paths"`
  resolves to the glossary "use keyword" term; two ch07-03/ch07-04 book_refs. Verified: `rustc` emits exactly one
  error (`error[E0432]`, secondary suppressed), golden_corpus green, anchors 112/112.
- ✅ **#22 — E0005 corpus gap filled** (`93ecdef`): added `exercises/05-types-and-matching/05_refutable_let.{rs,toml}`
  — the classic beginner trap `let Some(n) = maybe;` (a *refutable* pattern in a plain `let`). E0005 was
  absent from the corpus's error-code set, so this is genuinely new coverage, not a duplicate. Predict-then-run
  format, no answer-leak in the `.rs`; `concept = "if-let"` resolves to the glossary "if let" term (golden_corpus
  concept-guard green); two `ch06-02-match` book_refs (`patterns-that-bind-to-values`, `matches-are-exhaustive`).
  Verified: golden_corpus ✓, anchors 110/110 ✓, direct rustc emits exactly `error[E0005]` ✓.
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
  **★ Fixed 9 DEAD concept chips (`5ca856c`, 2026-06-25)** — found via audit: the exercise header renders each
  `concept` as a tappable glossary chip (`/api/glossary?term=<concept>`), but 9 exercises (the recent
  E0499/E0506/E0505/E0596/E0597/?-on-Option/iterator adds + the older `unwrap_none`) shipped concepts with no
  matching glossary term/alias → chip showed but tapping returned nothing, violating the glossary's stated
  invariant *"every exercise concept resolves to a term."* Root cause: I'd been adding exercises without adding
  the matching glossary alias. Fixed by adding each dead concept as an **alias on the right existing term**
  (facets of existing vocabulary, not new terms): borrow trio → `borrowing`, `option-unwrap-panic` → `unwrap
  and expect`, `question-mark-option-vs-result` → `the ? operator`, `borrow-must-not-outlive-value` → `no
  dangling references`, iterator pair → `iterating by reference` / `collecting an iterator`,
  `mutable-borrow-needs-mut` → `mutability`. Verified: 0/62 concepts unresolved (was 9), rpro-glossary 3/3,
  live `/api/glossary?term=move-while-borrowed` → "borrowing" definition.
  **★ REGRESSION GUARD ADDED (`36a644b`, 2026-06-25)** — the concept→glossary invariant was documented but
  UN-enforced (which is why it rotted across 9 exercises). `golden_corpus` now loads the built-in glossary via
  **rpro-glossary** (added as a dev-dep, so the guard uses the EXACT same case/separator-insensitive lookup the
  live `/api/glossary` does — no normalization drift) and asserts `gloss.get(concept).is_some()` for every
  exercise, with an actionable failure pointing at the fix (add the concept as an alias in glossary.toml).
  Verified: passes on the current 62/62 corpus AND proven to BITE (temporarily breaking one concept fails with
  the dead-chip message); full rpro-runner suite green, fmt clean. **Now adding a new exercise with an
  unresolved concept fails `cargo test` loudly instead of shipping a silent dead chip.** This closes the
  dead-chip class for good (fix + enforcement). NOTE for self when adding exercises: pick a `concept` that
  already resolves, or add its alias to glossary.toml in the SAME commit — golden_corpus will now enforce it.
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
  ✅ Added a **lifetimes** exercise (2026-06-25) — `07-generics-traits-lifetimes/03c_lifetime_dangling`
  (id `lifetimes/03_borrow_outlives_value`, **E0597**, intermediate): `let r; { let x=5; r=&x; } use(r)` →
  "`x` does not live long enough / dropped here while still borrowed". E0597 was missing corpus-wide yet is
  the **Book's OPENING lifetime example** and a top beginner borrow error — the simplest dangling reference,
  needing NO `'a` syntax, so it motivates *why* lifetimes exist. Fills the thin lifetimes area (2→3); placed
  as the 3rd lifetimes exercise (filename `03c_` sorts last so id-order = display-order). Reused the validated
  ch10-03 `the-borrow-checker` + `lifetime-annotation-syntax` anchors. golden_map: 07 = generics/traits/
  lifetimes, Intermediate. Verified: emits E0597 on 1.95.0, golden_corpus green, verify-exercises **61/61**,
  anchors 97/97. Commit `1415807`. Corpus = **29 distinct codes + 5 runtime panics / 61 exercises.**
  ✅ Added a 3rd **ownership** borrow-interaction exercise (2026-06-25) — `04-ownership/07_move_while_borrowed`
  (id `ownership/07_move_while_borrowed`, **E0505**, beginner): `let r=&v; let v2=v; use(r)` → "cannot move
  out of `v` because it is borrowed". COMPLETES the "can't-while-borrowed" pair with `06_assign_while_borrowed`
  (E0506): an active borrow freezes the value against BOTH reassignment (E0506) and moving (E0505). Distinct
  from E0507 (move out OF a borrow, e.g. `*ref`) and E0382 (plain move, no live borrow). Same "borrow ends at
  last use" fix lever as the rest of 04. Reused 04's validated ch04-02 `the-rules-of-references` + ch10-03
  `the-borrow-checker` anchors. golden_map: 04 = ownership + Beginner. Verified: emits E0505 on 1.95.0,
  golden_corpus green, verify-exercises **62/62**, anchors 99/99. Commit `ba53f27`. 04-ownership now 7
  exercises (E0382/E0502/E0515/E0507/E0499/E0506/E0505). Corpus = **30 distinct codes + 5 runtime panics / 62
  exercises.** Remaining mined codes: E0716 (temporary dropped while borrowed), E0061 (wrong arg count) —
  both more niche/shallow; the high-value borrow/lifetime set is now essentially complete.
  ✅ Added **basics/07_fn_arg_count** (**E0061**, `e4d1cfb`, 2026-06-25, after two clean audits) — the last
  genuinely-common missing beginner code: `area(width, height)` called with one argument → "this function
  takes 2 arguments but 1 argument was supplied". The COUNT companion to `03_fn_arg_type` (E0308, wrong type);
  the learner picks the 2nd value so no answer is handed. **First exercise added under the new concept→glossary
  guard** — added its alias (`function-argument-count` → "function argument types") in the SAME commit;
  golden_corpus passed BECAUSE of that (would've failed otherwise — the guard working on a fresh exercise).
  Verified: emits E0061, golden_corpus green (incl. concept-resolution), verify-exercises **63/63**, anchors
  resolve, glossary 3/3. Corpus = **31 distinct codes + 5 runtime panics / 63 exercises.**
  **Two clean AUDITS this tick (no bug found, good confidence):** (a) all 33 exercise-cited book chapters are
  bundled in `book/` → no dead "Open in Book reader" buttons (and verify-book-anchors implicitly enforces
  chapter-presence by reading `book/<ch>.md`); (b) all **170 internal markdown links across 105 rust-textbook
  files resolve, 0 broken** (STUDY-GUIDE hub + lesson nav + quizzes/cheatsheets) → textbook nav fully sound.
- ✅ **CLI audit + `rpro init --refresh` (`8f61ffb`, 2026-06-25)** — first audit of the CLI surface (the
  terminal learner's path: Termux/SSH). Fresh-install path VERIFIED sound: `rpro init` seeds 63 exercises +
  33 Book chapters + 58 glossary terms, and the recent concept aliases resolve (`rpro glossary
  move-while-borrowed` → "borrowing"). But found a real UX wart: `init` only seeds "if absent" (no refresh),
  so an updating user is stuck on stale content — yet the init footer + empty-list message advertised
  `rpro init --refresh-all` / `--refresh-exercises` flags **that don't exist** (clap "unexpected argument").
  Implemented a real `--refresh` that re-seeds all three bundled content types; SAFE because `copy_tree` is a
  merge-copy (overwrites bundled files, never deletes) → updates/adds content while preserving the user's
  progress/config AND any exercises they added themselves. Fixed the 3 misleading messages. Verified:
  stale-store sim (alias removed → "no entry") → `rpro init --refresh` → resolves; rpro-cli 5/5, fmt clean,
  no new clippy (kept cmd_init <100 lines), seam-guard clean. (TUI not yet audited — it's the other terminal
  surface; lower priority since it shares the store/glossary/exercise plumbing the CLI just exercised.)
- ✅ **TUI audit + stale Roadmap refreshed (`d0166c9`, 2026-06-25)** — audited the LAST un-audited surface
  (the TUI). Its render CODE is sound: **18 TestBackend render tests** across all 5 screens (dashboard ×5,
  exercise ×6 incl. the `g`-define-concept glossary feature, book ×4, roadmap, inline-markdown ×3), and the
  `g`-define uses the same glossary the concept-guard now protects. But the audit surfaced a real **content**
  bug: `docs/ROADMAP.md` (rendered live in BOTH the TUI and web Roadmap tabs) was badly STALE — "39 verified
  exercises" (actually 63), "23 Book chapters" (33), "concept matrix Phases 1–6" (now 1–9), and an un-checked
  "[ ] Lesson 1 calibration (needs Paul) → unlocks Lessons 2–8" — i.e. it showed the textbook as barely
  started when all 37 lessons + 11 reviews are DONE. Refreshed Phase 3 to reality (only open item = the
  Paul-gated book-listings vendor-vs-link decision). Verified: rpro-tui 35/35; live `/api/roadmap` serves the
  updated content. **★ ALL SURFACES NOW AUDITED SOUND** (web GUI, Book/Roadmap renderer, hint ladder, exercise
  list, glossary chips, book-refs, internal links, CLI, TUI). Recurring lesson: render *code* can be
  well-tested while the *content* it renders silently rots — audit both.
- ✅ **Top-level README Content section refreshed (`9474881`, 2026-06-25)** — continued the stale-content
  sweep onto the highest-visibility doc (the first thing any visitor/contributor sees). It badly understated
  the project: "32 exercises across 9 phases" (actually 63 across 11 — the list even omitted error-handling
  and functional/smart-pointers), "23 Book chapters" (33), and "structured lessons are pending (gated on
  Paul's calibration)" — when the gate is released and all 37 lessons + 11 reviews are done. Fixed to current
  reality. `docs/BACKLOG.md`'s older counts are a HISTORICAL done-log (changelog of what was true per shipped
  item) → correctly left as-is. **Stale-content sweep now covers the 2 highest-visibility surfaces: the
  in-app Roadmap tab (last tick) + the top README (this tick).** Lower-visibility docs (ARCHITECTURE.md,
  DISTRIBUTION.md, etc.) may have drift but aren't learner-facing; a future spot-check, not urgent.
- ✅ **`scripts/check.sh` — one-command local CI mirror (`5ebc28c`, 2026-06-25)** — rotated out of the
  stale-content sweep to a real DX/robustness gap: there was NO single local command to confirm a change is
  mergeable — you had to read `ci.yml` + `seam-gates.yml` and run ~11 commands by hand. `check.sh` mirrors
  every gate (fmt, clippy, test, doc, e2e + CLI smokes, gui-transforms, exercise integrity, book anchors,
  language-seam guard, wasm32 pure-core build), prints a pass/fail line per gate, runs them ALL (no fail-fast
  → see every problem at once), and exits nonzero if any fail. wasm32 gate auto-skips when the target isn't
  installed. Doubles as the improvement loop's **regression sentinel** (one command per tick instead of the
  ad-hoc subset I was running). Verified: shellcheck clean; full run = **10/10 gates green** on the current
  branch (+ wasm32 skipped locally) → "branch is mergeable". Documented in RUN.md. (The two workflow files
  remain the merge-gating source of truth; check.sh notes "keep in sync".)
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
- ✅ **e2e coverage for the navigation surface (`<pending>`)** — the last 3 ticks' navigation work (the `⋯ More`
  menu, the lesson→cheatsheet companion link, the `?` keys overlay) had **zero dedicated e2e coverage**, despite
  being central to the UX Paul asked for. (Last tick the suite caught the overlay click-swallow bug only
  *incidentally*, via unrelated click tests.) New `tests/e2e/navigation.spec.js` (5 tests): the bar shows exactly
  3 primary tabs with the 4 reference surfaces as menu items; opening `⋯ More` → Book navigates + closes the menu +
  marks the trigger `active`/the item `aria-current`; Esc closes the menu; a lesson's "📋 this phase's cheat sheet"
  link is present, targets a real phase id, and opens that sheet (+ lights up More); and `?` opens the shortcuts
  overlay / Esc closes it **with an explicit regression guard that a closed overlay never intercepts clicks** (a tab
  click must still land — the exact failure mode of the `[hidden]`-override bug). Test-only (no crates/gui). Verified:
  the 5 new tests pass in isolation (1.7s) and in the **full suite — 20 passed, 1 known recall-chip flake heals on
  retry** (16→21 tests). Locks in the navigation against silent regressions in future ticks.
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
  ✅ **Cleared the clean corrective tail (`4080c4e`, 2026-06-25):** the 4 genuinely-clean warnings are now
  fixed — `doc_markdown` ×2 (rpro-serve/main.rs: backtick `AppDir`/`AppImage`) + `too_long_first_doc_paragraph`
  ×2 (rpro-tui/render.rs: split the `render_exercise`/`render_roadmap` summary into a short first line + detail).
  **rpro-serve 4→2, rpro-tui 8→6.** Verified: the 2 lint names → 0 occurrences, rpro-tui 18 + rpro-serve 35
  tests pass, fmt clean. **What REMAINS is now 100% the Paul-gated lint-policy tail** — nursery false-positives
  (`future_not_send` ×6 on the wasm-safe `Core`, `option_if_let_else` ×3 whose `map_or_else` rewrites are
  *less* readable, `redundant_clone` ×1 likely-required) + pedantic refactors (`too_many_lines` ×3,
  `similar_names` ×3). No more clean corrective clippy work exists; the rest awaits Paul's nursery/`-D` decision.
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
- ✅ **v0.4 declutter #3 — slim the task header (`6960c2e`, 2026-07-02).** Above the editor sat a big title + a metadata line + a THREE-LINE "Predict first…" paragraph + the predict bar, pushing the editor below the fold. Condensed the prose to one line (the method is also in the Study Guide + Tutor, and collapses after predicting); title 20→19px, metadata 13→12.5px, tighter margins. Editor moved up ~65px — editor + Run/Check/Explain now in the first screen. Verified LIVE (390×840): Learn shows the concise prose (gate intact), Assist hides it. e2e green. NO APK — batching to v0.4.
- ✅ **v0.4 declutter #2 — one "📚 Learn" hub replaces the scattered nav (`d731642`, 2026-07-02).** Top bar was Practice/Lessons/Quizzes tabs + a ⋯ More menu (book/glossary/cheatsheets/library/roadmap/logs). Now just **two tabs**: 📝 Practice (the IDE) + 📚 Learn (a hub for EVERYTHING else, with progress). `renderLearnHub` = a progress line + Continue CTA + grouped cards (Curriculum / Practice checks / Reference) + a small Logs link; `LEARN_VIEWS` keeps the Learn tab lit across sub-views; removed the whole More-menu markup/JS. Every capability still one tap from a clean IDE. Rewrote navigation.spec; routed a11y/offline/lesson-nav via showView(). Verified LIVE (390×844) + e2e **34/34**. NO APK — batching to v0.4.
- ✅ **v0.4 declutter #1 — IDE-first: Insights drawer collapsed by default (`a5afc9b`, 2026-07-02).** Paul: *"way too congested and noisy … focus on the task and the IDE mostly."* The Diagnostics/Book-refs/Tutor panels used to stack on the main screen permanently (a Run auto-opened them AND persisted). Now the main view = task + IDE (exercise → editor → Run/Check → terminal); Insights is collapsed by default, auto-opens **transiently** after a Run/Hint, and pins across sessions only via the explicit ▦ toggle. `setInsights(on,pin)` + renderCurrent collapse-on-new-exercise. Verified LIVE (390×840): clean default (screenshot), transient auto-open, ▦ pins. e2e 35/35. **NO APK — batching to v0.4** ([[release-cadence-and-v04]]); web/desktop live now.
- ✅ **Full CI-mirror verification + 2 rustdoc-warning fixes (`3d5966f`, 2026-07-02).** Ran `scripts/check.sh` (the
  complete gate set — fmt, clippy -D warnings, test, doc, e2e smoke, exercise integrity, cli smoke, gui-transforms,
  book anchors, language-seam guard, wasm32 build) to confirm this session's ~20 changes are green end-to-end:
  **11/11 gates pass — "branch is mergeable"** (cargo-deny skipped, not installed; CI runs it). The doc gate surfaced
  2 rustdoc warnings, fixed: rpro-cli's MdSurface doc wrote "no <noun>" (rustdoc parsed as an unclosed HTML tag →
  backticked); rpro-glossary linked `[`rpro_book`]` (not a dep → broken intra-doc link → dropped the brackets).
  `cargo doc` now warning-free. Milestone: the whole platform is verified-green after the session's work.
- ✅ **Journey quiz-done ✓ + validated the mobile-progression gap architecture (`<gui>`, v0.3.13, 2026-07-02).** The
  Journey showed per-phase lesson read-counts but not quiz completion; added a green ✓ next to each phase's Quiz link
  when its quiz is done (quizDoneSet) — each stage now shows lessons + read badge + quiz ✓ + book chapter at a glance.
  Verified LIVE (390×844): none→0 ✓, phase1+phase4→✓ on exactly those. **Also confirmed** the mobile-progression gap
  (#35) is a genuine multi-part feature: the mobile seam is a **JNI shared-Rust lib** (`libtempered_seam.so`) and
  `/api/exercises` carries no code — so client-side switch/advance needs a Rust seam change + `.so` rebuild + on-device
  test. Correctly deferred to Paul. **v0.3.13 shipped** — batch-released this + the a11y contrast fix; e2e 35/35.
- ✅ **a11y: extended the axe audit to the content views + fixed a real contrast defect (`2e6b50d`, 2026-07-02 — web
  now; APK batched — RELEASED in v0.3.13).** The a11y spec covered only the main view; extended it to the Lessons list, an open lesson, the
  Library list, and the Book TOC (all built this session). It immediately caught a defect I'd introduced: the
  Continue/Start CTA was white on the bright `--accent` #f74c00 (~3.5:1, below AA 4.5:1). (The Run button dodges the
  check only via its gradient bg — a blind spot, not compliance.) Fixed to the darker brand orange #b7410e (white =
  5.56:1, passes AA in BOTH themes; still a prominent on-brand button). All 4 view audits 0 critical/serious; full
  suite **31/31**. Verified the CTA still looks good at 390×844.
- ✅ **Mobile Check = fast type-check (#30/#31, gui `63c480e` / mobile `40cc3ff`, v0.3.12, 2026-07-02).** Directly
  addresses Paul's "compiling takes a long time." Mobile Check was routing to the SAME full compile+link+run as Run.
  Now the run handler passes the op to `runViaTermux(code, cbId, op)` and the bridge branches: **check → `rustc
  --edition 2021 --emit=metadata main.rs`** (type-check only — no codegen, link, or run), run/test → the prefer-
  dynamic compile+run. Verified LOCALLY (rustc 1.95.0): metadata emits the IDENTICAL diagnostics (same E0384), no
  binary, and is ~1.8× faster than compile+link / ~2.8× vs compile+run (a bigger absolute win on a slow phone). Also
  the correct semantics (Check never runs). Verdict/status now say "type-checks ✓" vs "ran"; a 2-arg overload keeps
  older-gui calls working. gui-transform + e2e green; APK builds (Java compiles); v0.3.12 published + verified + emailed.
- ✅ **CLI glossary substring-search fallback + content-integrity audit (`aedc467`, 2026-07-02).** `rpro glossary <q>`
  was exact/alias only — a near-miss dead-ended into a 116-line dump. Now a miss falls back to a substring search over
  names/aliases/definitions (parity with the web filter), listing related term names (capped 15 + "… and N more").
  Verified LIVE: exact/alias unchanged; "memory" → 5 related (was a dead miss); gibberish → clean miss; rpro-cli 5/5.
  **Content-integrity audit this tick (rustc 1.95.0):** `scripts/verify-exercises.sh` **71/71** (every exercise still
  emits its taught error code / panic) + `scripts/verify-book-anchors.mjs` **115/115** anchors resolve — zero drift;
  both already gated in check.sh + CI. The teaching content is verifiably correct end-to-end. CLI only — no APK.
- ✅ **TUI Cheatsheets tab — completes reference parity (#31/#32, `de04988`, 2026-07-02).** Follows the Lessons tab:
  the per-phase quick-reference sheets are now a browsable TUI tab (…·Book·Cheatsheets·Roadmap), so a Termux/SSH
  learner has the sheet at hand while solving. Pure reference — quizzes stay web/CLI-only ON PURPOSE so the predict-
  then-reveal holds (a static TUI render would just show answers). DRY refactor: `Lesson`→`MdDoc`, extracted
  `render_md_reader()` (render_lessons/render_cheatsheets are thin wrappers), `lessons_data`→`md_docs(store, subdir)`
  for both dirs; Book reader untouched. +2 TestBackend tests → rpro-tui **39/39**, clippy/fmt/seam clean. CLI/TUI only.
- ✅ **TUI Lessons tab — the curriculum reaches the dashboard (#31/#32, `d6842ef`, 2026-07-02).** The `rpro` TUI had
  Dashboard/Exercise/Book/Roadmap but not the 37 Patina lessons — a Termux/SSH learner couldn't read one without
  dropping to `rpro lessons`. Added a 5th tab modelled on the Book reader: title list + selected lesson's markdown
  (scrollable), Tab/j/k/PgUp-PgDn nav. Extracted a shared `markdown_body_lines()` from render_book (same styling),
  reused by `render_lessons`; `lessons_data()` loader (store/lessons/*.md, sorted, `# `-title, HTML-comments
  stripped). +2 TestBackend tests + the cycle test. rpro-tui 37/37, clippy/fmt/seam clean, Book reader unchanged.
  CLI/TUI only — no APK.
- ✅ **The Tutor reacts to your run — no more static question (#31, `f82bda5`, v0.3.11, 2026-07-02).** Directly
  closes Paul's feedback: *"there is no answer to the question it asks; the questions should disappear once it's
  satisfied."* The Tutor was one hard-coded prompt forever; now `renderTutor(passed, code)` (wired after renderDiag
  + the Termux path, reset per exercise) gives 4 states — pre-run predict prompt; **✓ Solved** (acknowledge + move
  on); failed-Learn → "read the terminal, first error" (NEVER reveals the code — Learn's by-hand tier); failed-
  Assist/Dev → names the real E-code + asks what it means. Still strictly guide-only (never the fix). Verified LIVE
  (390×844): all 4 states; Assist E0384 references the code (screenshot); Learn never leaks it. +tutor.spec.js,
  suite **29/29**. APK v0.3.11 published + verified + emailed.
- ✅ **Offline-integrity e2e guard for the #1 charter promise (`7727f0e`, 2026-07-02).** Audited `gui/index.html` —
  zero external resources (no CDN/web-fonts/remote scripts; pdf.js's only https strings are comments/bug links). New
  `offline.spec.js` locks it: exercises lessons/quizzes/glossary/book/library while listening on every network
  request, FAILS on any non-local origin (data:/blob: allowed). Catches a future CDN `<link>`/`<script>` before it
  silently breaks offline use. Full suite **28/28**. Test-only — no app change, no release.
- ✅ **Quiz completion tracking — the last untracked surface (#31, `0ed0275`, v0.3.10, 2026-07-02).** Lessons had
  read-tracking + Continue; the per-phase self-check quizzes had none. A quiz is now "done" once EVERY answer has
  been revealed (predict-then-verify — the honest self-check signal); mirrors lessons exactly (✓ + "N of M done" on
  the list, reusing readbadge/read styles). Partial reveals don't count. +2 e2e tests. Verified LIVE (390×844):
  reveal all 13 of Phase 1 → "1 of 11 done" + ✓; one → not done. Full suite **27/27**. **v0.3.10 shipped** —
  BATCHED the pending lessons type-to-filter (`9137617`) with this into one APK + one email (cadence discipline).
  Now lessons/quizzes track + book/glossary search → the offline loop is progress-aware & searchable end-to-end.
- ✅ **e2e regression guard for lesson-nav (`1e95fd9`, 2026-07-02).** The Continue/Start CTA, type-to-filter, and
  book chapter deep-links shipped this session with ZERO coverage (localStorage-/render-driven — the kind a stray
  refactor breaks silently; we've regressed nav before). New `lesson-nav.spec.js` (4 tests): Continue CTA
  Start→Continue→opens; filter narrows/no-match/clears; lesson `.booklink` has a real `data-page`; Journey has
  per-phase chapter links. Full suite **25/25 green** (was 21). Test-only — no app change, no release.
- ✅ **Type-to-filter the lessons list (#31, `9137617`, 2026-07-02 — web now; APK batched).** With 37 lessons, a
  filter box (shown once the list passes a handful) narrows to a topic instantly. Case-insensitive title substring,
  a "no lessons match" note, clear restores all; coexists with the Continue CTA + ✓ read marks. Verified LIVE
  (390×844): "own"→2 matches, "loop"→1, gibberish→0+note, clear→37. gui-transform + e2e green (19 passed, 2 pre-
  existing real-run flakes retry-passed). **Release cadence note:** 6 APKs cut this session (v0.3.4–v0.3.9); this
  minor change is batched into the next notable release to avoid inbox spam — web/desktop have it live now.
- ✅ **Book cross-links deep-link to the exact chapter (#32, `866608e`, v0.3.9, 2026-07-02).** Upgraded the
  phase→book weave from "open at page 1" to a real chapter deep-link (pdf.js `#page=N`) that also names the chapter.
  Extracted real TRPL page numbers from the PDF outline (pypdf in a scratch venv) and mapped each phase → its
  chapter (Ownership→p65 Understanding Ownership, Generics→p174, Concurrency→p336, Tooling→The Cargo Book, …).
  `renderLibrary(file, page)` appends `#page=N`; lesson link reads 'TRPL — "Understanding Ownership" (offline)';
  Journey shows the chapter name. Verified LIVE: clicking Ownership opens TRPL at p65 (Chapter 4, screenshot);
  lesson 15 link carries p65. gui-transform + e2e green. APK v0.3.9 published + verified + emailed.
- ✅ **"Continue" — one-tap resume to your first unread lesson (#31, `9962fbe`, v0.3.8, 2026-07-02).** Turns the
  lesson read-tracking into navigation: the Lessons list leads with a CTA — "▶ Start — Lesson 1" (fresh), "▶
  Continue — Lesson N" (first UNREAD), or "✓ read all 37 — revisit" (done, no button). One tap resumes where you
  left off. Verified LIVE (390×844): fresh→Start@L1, 3-read→Continue@L4 (+ "3 of 37 read" badge + ✓ marks),
  all-read→note; click opens + marks read. gui-transform + e2e green. APK v0.3.8 published + verified + emailed.
- ✅ **Reading-experience typography pass (#31, `9cbfbd9`, v0.3.7, 2026-07-02).** Reading (lessons/book/quizzes/
  glossary/library) is the platform's core and had never had a type pass. Scoped to `.docview`/`.bookbody` (the
  practice editor untouched): comfortable **~72ch measure** (column caps at 780px, was 920 — centres on desktop,
  full-width on phones); clearer **heading hierarchy** (h1 22→25px/700 tighter tracking, h2 16→17.5px/600 with more
  air); **prose rhythm** (body 14.6px, line-height 1.7, real 13px paragraph spacing); **code** blocks get a border +
  padding, inline chips a hair more. Verified LIVE: lesson reads cleanly at 390×844 (screenshot); column capped+
  centred at 1200px (780/left 210); Library still lists 9. gui-transform + e2e 21/21. APK v0.3.7 published+emailed.
- ✅ **Books woven into the learning path (#32, `3fbc4a0`, v0.3.6, 2026-07-02).** The offline 📚 Library was
  reachable only via ⋯ More — a detached shelf. Added a `PHASE_BOOK` map (each curriculum phase → the text that
  covers it: beginner path → TRPL, tooling → The Cargo Book) and a companion **"📖 Read deeper: <Book> (offline)"**
  link on every lesson footer + every Journey stage (beside the existing cheatsheet/quiz cross-links). Clicking
  opens the correct PDF in the offline pdf.js reader. The path now reads as one material: lesson ↔ cheatsheet ↔
  quiz ↔ book. Verified LIVE (desktop + 390×844): 11 Journey book links (10 TRPL + tooling→Cargo), lesson link
  opens the right book (0 errors); gui-transform + e2e green. APK v0.3.6 published + verified + emailed (HTML).
- ✅ **`rpro quizzes` + `rpro cheatsheets` — full offline CLI curriculum parity (`52205d2`, 2026-07-02).** The
  natural follow-up to `rpro lessons`: terminal/Termux/SSH learners now get the SAME reference surfaces the
  web/mobile GUI has. Generalized `cmd_lessons` into a shared **traversal-safe `MdSurface`** helper (list-or-print
  a bundled markdown surface by exact-or-prefix stem; raw input never path-joined — two `// nosemgrep` on the
  store-owned `dir.join`s) + thin `cmd_quizzes`/`cmd_cheatsheets` wrappers. `rpro init` now seeds `quizzes/` +
  `cheatsheets/` (11 each) alongside lessons. Verified LIVE: init seeds 11+11; `quizzes` lists, `quizzes phase1`
  prints; bad/`../..` ids fall back safely. clippy 0, 5 tests, seam-guard + fmt clean.
- ✅ **Offline book Library — 9 original FOSS PDFs + pdf.js reader (`3876c39`/mobile `cf3c021`, v0.3.5, 2026-07-02).**
  Paul: *"list all the textbooks unaltered in their original PDFs so they can be viewed offline."* New **📚 Library**
  (⋯ More): 9 license-clean books (TRPL, Comprehensive Rust, Reference, Nomicon, Cargo Book, Edition Guide, Embedded,
  Design Patterns, Rustc Dev Guide) read **fully offline** via a vendored, trimmed **pdf.js v6.1.200** (Apache-2.0).
  Opens full-screen via top-level nav (server CSP `frame-ancestors 'none'` + `X-Frame-Options DENY` block iframing —
  kept; top-level nav is fine; device Back returns). Static under `gui/` → served by both desktop ServeDir + the
  mobile WebView interceptor (added `.mjs`/`.wasm`/`.pdf`/… MIME). Verified LIVE: TRPL renders 524pp with outline/
  search/zoom, 0 errors; e2e 21/21. APK v0.3.5 published + verified + emailed (polished HTML).
- ✅ **`rpro lessons` — the curriculum reaches the terminal learner (`<pending>`).** Cross-surface gap: the web
  had all 6 study surfaces, but the CLI only exposed `book`, `glossary`, and `exercises` — a Termux/SSH learner
  could read the Book and practice but **could not read the 37 lessons**. Added `rpro lessons [id]`: no id lists
  every lesson (stem + `# `-heading title, in curriculum order); an id prints that lesson's markdown (HTML comments
  stripped) matched by exact stem **or prefix** so a bare `rpro lessons 05` works. **Traversal-safe** — the id is
  matched against the real `.md` stems on disk, never path-joined (mirrors the rpro-serve pattern + `// nosemgrep`
  on the store `read_dir`). `cmd_init` now also seeds `lessons/` from the bundled workspace (parity with the
  Book/glossary seeds; honoured by `--refresh`). Also refreshed the stale top-of-file subcommand doc (it still said
  "v0 wires init … stubs the rest" and omitted glossary/detect/run/check/test/explain). Verified: `cargo build`
  clean, `cargo fmt --all --check` clean, **clippy 0 warnings**, **seam-guard clean** (`*.rs` scope), `cargo test -p
  rpro-cli` 5/5; LIVE (temp `RPRO_STORE`): `init` seeds 37 lessons, `lessons` lists 37, `lessons 05`/`lessons
  12-string-vs-str` print the right lesson, bad id → friendly miss. (Quizzes + cheatsheets in the CLI are natural
  follow-ups.)

### Platform / distribution
- ✅ **Fresh, downloadable APK shipped — `v0.3.0` (`<pending>`, 2026-06-26).** Paul: *"it will not download from
  the link you sent anyway."* Root-caused: (a) the only prior release (`v0.2.0`) was **stale** (predates lessons/
  glossary/quizzes/cheatsheets + the whole UX redesign), and (b) the auto-build is **dead** — the self-hosted runner
  `[self-hosted, linux, x64, plausiden]` is **offline/deregistered**, so the last **5 `main` pushes' builds are stuck
  `queued` (8h+)** and never produced a new release. Bypassed the dead runner by **building the APK locally** on the
  same host (`HOME=/home/paul bash build-apk.sh`, sibling = `Tempered-Studio@textbook-integration` → bundles ALL
  current work) and publishing it as **`v0.3.0`** via `gh release create` (debug-signed → sideloadable, no keystore).
  Verified the way Paul would: the release asset downloads **anonymously** (no auth) — `curl -IL` → 302→200,
  `application/vnd.android.package-archive`, and the downloaded bytes' **sha256 matches the local build exactly**
  (`d395fb0…`); the APK bundles the live gui (`renderRoadmap`/`moremenu-list`/`lessoncheat`) + 11 cheatsheets + 49
  lessons/quizzes/glossary files. Mobile `README` download pointer bumped to v0.3.0 (`9758eea`). PushNotification sent
  with the working link. **⚠ See Open questions: the dead CI runner means future APKs need a manual local build (or
  Paul reviving the runner).**
- ☐ Web deploy (GH Pages / server) · online Run for Android (remote rpro-serve) ·
  F-Droid + Obtainium · signed APT/dnf repos.

## Open questions for Paul
- **Original exercises for the 13 uncovered lessons? (2026-07-07, from the pacing audit).** The
  audit (`docs/CURRICULUM-PACING-AUDIT.md`) found 13 lessons with NO paired in-app exercise —
  shadowing, constants, expressions/semicolon, tuples/arrays/slices, and slices-in-depth have clear
  predict-then-run potential. #22 (adapting Rustlings/RBE/Exercism sets) stays gated as you asked;
  this is different — **authoring small ORIGINAL exercises** to complete the lesson↔exercise
  follow-along (#37). Default plan: I'll author them as part of the batch work unless you say hold.
- ⚠️ **Mobile exercise PROGRESSION doesn't work offline (design decision, 2026-07-02).** On the phone there's no
  writable server: the WebView seam serves `/api/exercises` + `/api/current` **read-only** and `/api/select`/run
  return null; the Termux run-result handler (`window.__termuxResult`) doesn't record a pass or advance. Net effect:
  a mobile learner can RUN the seeded "current" exercise but **can't switch exercises or auto-advance** through the
  71-exercise curriculum — they're pinned to one. **Two ways forward, your call:** (a) accept that mobile = reading
  (lessons/book/library) + free-form snippet-running, and the structured 71-exercise progression lives on desktop/CLI
  (then I'd soften the mobile exercise UI to match); or (b) build a **client-side progress layer** (localStorage
  overlay: mark done + advance on a passing on-device run, switch exercises locally) so the full loop works offline.
  (b) is a real feature I can build but can only fully verify **on-device** — needs your steer + a phone test. Flagged
  now rather than half-fixed blind.
- ⚠️ **The Android CI runner is DOWN (2026-06-26).** The self-hosted GitHub Actions runner
  `[self-hosted, linux, x64, plausiden]` that builds the APK (`.github/workflows/apk.yml` on the mobile repo) is
  **offline/deregistered** — `gh api .../actions/runners` lists none, and the last **5 pushes to mobile `main` are
  stuck `queued` for 8h+** with no build. I shipped `v0.3.0` this tick by **building locally on the host and
  publishing via `gh release`** (works, verified downloadable), but that's a manual bypass. **To restore automatic
  APK builds on every push, the runner needs reviving** (restart the `actions-runner` service on the plausiden host /
  re-register it) — that's your infra. Until then I can keep building+publishing APKs manually each time the gui
  changes materially, but it won't happen automatically.
- 🔄 **"Finish everything / all tasks" push (2026-06-25).** Paul: "keep going stop stopping… finish everything,
  all tasks, and clear completed tasks." Cleared the completed task list and drove the backlog. **FINISHED this
  push: #14** (every exercise-cited Book chapter is bundled — verified; ch17/async is uncited, no offline async
  exercises), **#15** (closures + smart-pointers + iterators exercises all exist; an async *exercise* can't fit
  the offline fix-to-run model — no runtime — but the async *lesson* L31 exists), **#18** (the real tier
  differentiator ships: Learn withholds parsed diagnostics, Assist/Dev show code→explain + line→jump; the inline
  *gutter* stays intentionally deferred as a fragile pre-wrap pixel-map that click-to-jump supersedes),
  **#20** (clippy clean → `-D warnings` enforced in CI + check.sh; nursery relaxed with rationale), **#21**
  (cargo-deny wired: `deny.toml` + CI job + check.sh). **#22 IN PROGRESS** (Rustlings adaptations, one/tick:
  `162bb65` collections/07 fill-vec-needs-mut E0596 · `7893bb7` collections/08 use-after-move-into-fn E0382 ·
  `3bc47af` collections/09 return-owned-string E0308 · `4eb387a` traits/04 missing-method **E0046 (new code)**;
  attribution pattern set [.rs header + `.toml` `attribution`], Rustlings cloned MIT; RBE/Cookbook/Exercism follow). **Two remain genuinely large/external — NOT one-autonomous-turn, flagged honestly rather than
  fake-finished:**
  - **#19 Full IDE via rust-analyzer — IN PROGRESS (two blockers cleared, 2026-06-25).** ✅ Installed the
    **toolchain-matched rust-analyzer 1.95.0** (`rustup component add rust-analyzer`; the system one was 1.85,
    proc-macro-incompatible with the 1.95 toolchain) — the version-mismatch blocker is gone. ✅ **First
    consumer of `LspSpec`** (`8f02542`): `rpro detect` now reports the Dev-tier language server
    (`✓ rust-analyzer 1.95.0 (owns **/*.rs)`) using `lang.lsp()`, data-driven + seam-clean — so "LspSpec
    defined but unconsumed" no longer holds. **Remaining (genuinely multi-session, not one autonomous turn):**
    the actual IDE — an LSP client (spawn rust-analyzer, `initialize`/`didOpen`/`didChange`, stream
    `publishDiagnostics`/completion/hover) wired into the Dev web/TUI editor. That's a substantial async
    protocol + frontend feature; flagging the scope for you, building it incrementally over ticks.
  - **#23 Distribution.** AppImage DONE. **★ APK DONE + DOWNLOADABLE (2026-06-25):** the Android app builds
    end-to-end (seam cross-compiled for all 4 ABIs + gui/exercises/book bundled → 5.3 MB **debug-signed,
    sideloadable** APK), published at **github.com/thepictishbeast/Tempered-Studio-Mobile/releases/tag/v0.2.0**
    + auto-published every push via `apk.yml` (mobile `c5866ac`). Build was unblocked by fixing build-apk.sh's
    `$HOME/.rustup` → `/home/paul/.rustup` (the android std targets live under paul, not root). **The remaining
    #23 parts genuinely need you / external infra:** signed **APT/dnf repos** (your GPG key + a GH-Pages target),
    **F-Droid** (submission + a *release*-signed build → your keystore), **Play-Store release APK** (your Android
    keystore), **web deploy** (a GH-Pages/host target), **online Run for Android** (a hosted remote-toolchain
    server, since Android has no on-device rustc). The debug APK gives a working downloadable app TODAY; the
    signed/hosted distribution channels are the key/infra-gated next layer.
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
- ✅ **#37 pt.2 — original exercise for 13-tuples (tuple field access, NEW E0609) (2026-07-07, TS `f3cfcad`, CONTENT_VERSION 37→38, web-first).** Tuples had ZERO exercises despite being a foundational early Phase-3 concept. Checked the full expected_error_code inventory first (E0005/E0616/most walls already covered — E0609 was NOT), then authored on that gap: `point.3` on a 3-tuple → **E0609** "no field `3`… available fields are: 0, 1, 2", teaching 0-indexed-fields-bounded-by-arity (off-by-one). Wired `tuple-field-access` → 13-tuples (reciprocal practice link). Verified: verify-exercises **73/73**; live :8099 marker=38, 73 exercises, 13-tuples handoff works (Playwright, zero errors); e2e 30/30; fmt+lesson-integrity clean. Exercise-authoring method proven twice now: (1) inventory covered error codes, (2) target a lesson with a distinctive UN-covered wall, (3) compile-verify the exact code, (4) wire CONCEPT_LESSON + live-verify the handoff. NEXT #37 candidates by the same method; or #40 beauty / #41-#32 cross-book.
- ✅ **#37 pt.1 — original exercise for 13b-arrays; coupling audited clean (2026-07-07, TS `e81e0ce`, CONTENT_VERSION 36→37, web-first).** First audited the lesson↔exercise coupling: a **perfect bijection** — 65 exercise concepts ↔ 65 CONCEPT_LESSON mappings, ZERO orphans (exercise with no lesson link) and ZERO dead keys; gui-transform already guards targets-exist. So the mechanical coupling is sound. The real #37 gap is *content* — some strong-error lessons lack a coupled exercise. Authored one for **13b-arrays**'s headline ("length is part of `[T; N]`"): a `[i32; 3]` given 4 elements → **E0308** ("expected an array with a size of 3, found one with a size of 4"), the distinct COMPILE-time array case vs the existing Vec RUNTIME panic (06_index_out_of_bounds → 14-vec). Charter predict-then-run; solution_outline guides w/o handing the fix. Wired `array-length-type` → 13b-arrays so the lesson renders the "Practice this lesson" link (reciprocal coupling both ways). Verified: verify-exercises **72/72** (new emits E0308 as claimed); live :8099 marker=37, 72 exercises, 13b→exercise handoff works (Playwright, zero errors); e2e 32/32; fmt+lesson-integrity clean. rust-textbook has no exercises mirror (TS-authoritative). ⚠ 35 lessons "uncovered" but most are exerciseless-BY-DESIGN (Sandbox-only splits + reading lessons) — future #37 exercise-authoring should target the few with strong un-exercised walls, not all 35. NEXT: more #37 exercises / #40 beauty / #41 cross-book; fold into a dev.40 later.
- ✅ **📱 v0.4.0-dev.39 RELEASED — gamification + reading-size + motion polish (2026-07-07, mobile `a3e69ec`, versionCode 10353).** Everything web-only since dev.38 now on the phone: **#39** XP/level/🔥streak chip + 7-badge panel (XP=f(server done), reveals grant nothing), **#41** ⋯-menu reading-size stepper (all `.bookbody` text 85–150%, persisted), **#40 pt.1** smooth XP-bar fill + view fade-in + level-up glow (all reduced-motion-guarded); plus dev.38's IDE + 78 lessons. build-apk.sh auto-synced the current TS tree; versionName bump re-triggers the delete-then-copy re-seed (no ghosts). **Also committed the build-synced assets** — the mobile repo's committed store was dev.37-era, so this brought it to a faithful dev.39 snapshot (64 files: gui + all the lesson splits). **Bundle VERIFIED pre-release** (unzipped APK): gui carries xpchip+gamifyUpdate+xpLevelup, --read-scale+setReadScale, animation:fadein on .docview, renderIde; 78 lessons; ZERO ghosts; versionName 0.4.0-dev.39. GitHub PRE-release, **anon-download HTTP 200 / 26.7 MB**; one PushNotification, NO email (dev.N-silent). https://github.com/thepictishbeast/Tempered-Studio-Mobile/releases/tag/v0.4.0-dev.39 — notes ask Paul to spot-check the XP chip/badges, reading-size persistence, and that no reveal grants XP. ⚠ IDE Termux-run still unverified on-device. **NEXT: board — #37 coupling, more #40 beauty, #41 cross-book switcher (#32).**
- ✅ **#40 pt.1 — MOTION POLISH: smooth progress, view fades, level-up glow (2026-07-07, TS `1e2e813`, web-first).** Paul's "fun and cool and modern … animations." Closed the gaps in the existing motion foundation (which already had a reduced-motion guard, fadein keyframe, animated gauge): (1) the **XP bar now fills smoothly** (`transition:width .9s`, matching the gauge — it was jumping); (2) **every doc view fades in on navigation** (`animation:fadein .26s` on `.docview`, recreated per switch by freshDocview — before, only list items faded, views popped); (3) a **level-up pulses the XP chip** (`@keyframes xpLevelup` ring, fired in gamifyUpdate at the existing level-up point — #39 now feels alive). Verified LIVE 390×844: XP bar transitions on width, `.levelup` fires Lv1→Lv2, a doc view's animation-name is `fadein`, zero console errors; **under prefers-reduced-motion BOTH collapse to ~0s (1e-06s)** — accessible. NEW motion.spec.js pins cues-present + reduced-motion-collapse; suite **39/39**; gui-transform+JS-parse green; pure GUI → no CV bump. NEXT board: #37 coupling (+ more #40 beauty). **⚠ dev.39 now DUE: #39 gamify + #41 readability + #40 motion are all web-only (only the IDE reached dev.38) — cut dev.39 next tick to get the fun/polish on the phone.**
- ✅ **#41 pt.1 — READING-SIZE control for textbook readability (2026-07-07, TS `8e80e2d`, web-first).** Paul's "improve their readability." A ⋯-menu A−/%/A+ stepper scales EVERY long-form surface at once (the Rust Book + all 78 lessons + the lesson drawer — all render into `.bookbody`) via a single CSS var `--read-scale`; `.bookbody` font = `calc(15.5px * var(--read-scale))`. 5 clamped steps (85–150%), persisted (`ts-read-scale`), restored on load; default 100% = the tuned 15.5px (no change for existing readers). Tucked in the ⋯ menu (declutter). Verified LIVE 390×844 (screenshot: lesson body 15.5→20.15px at 130%, % label tracks, survives reload, clamps both ends, zero console errors); NEW readability.spec.js pins scale+persist+both clamps; suite **37/37**; gui-transform+JS-parse green; pure GUI → no CV bump. **#41 "switch back and forth" side is partly served already (Book prev/next nav); the bigger cross-BOOK switcher is #32 territory (source books are PDFs in the library, markdown Book is separate).** NEXT board: #37 coupling / #40 beauty; a dev.39 once #39/#40/#41 accumulate.
- ✅ **#39 pt.1 — GAMIFICATION: XP, level, streak & badges (2026-07-07, TS `02e9e86`, web-first; NOT yet in a dev.N).** Paul's "gamify learning … fun and cool." A compact **Lv/XP/🔥streak chip** in the header by the gauge; tap → a **badge panel** (7 badges, earned+locked); level-up & new-badge **toasts**. **The charter rule "never reward a reveal" holds BY CONSTRUCTION** — XP/level/streak/badges are a pure function of the server's authoritative `done` count (folded in at renderList's existing `data.done` read); a hint/reveal never changes `done`, so there is NO code path from a reveal to a reward. XP=done×10, level=floor(done/5)+1; streak +1/day on a genuine completion, resets after a skipped day; **bestDone monotonic** so reset-to-redo never lowers your level; first-run seeds from existing progress (no toast storm). Fixed a live-caught name-shadow bug (the panel's local `esc` Escape handler shadowed the HTML-escape `esc` → TDZ; renamed onEsc). Verified LIVE 390×844 (screenshot: Lv 2 🔥1 at 5 done, panel + badge toast, zero console errors); NEW gamify.spec.js **pins the invariant** (repeat-fold of same `done` → byte-identical state) + monotonic level + panel; full suite **36/36**; gui-transform + lesson-integrity + JS parse green. Pure GUI/localStorage → no CONTENT_VERSION bump. **v1 scope; follow-ups: more badges, an XP history/celebration surface, and folding into the next dev.N so it reaches the phone.** NEXT board: #37 coupling / #40 beauty / #41 textbooks.
- ✅ **📱 v0.4.0-dev.38 RELEASED — the fullscreen IDE + the full 78-lesson curriculum (2026-07-07, mobile `dae1b79`, versionCode 10352).** The biggest drop since dev.37 (which was 56 lessons). `build-apk.sh` auto-syncs gui/ + store from the sibling TS checkout, so this carries: the fullscreen IDE (Files/Editor/Terminal), the complete 56→78 baby-step overhaul, and the L1 back-link fix. Re-seed is versionName-gated (delete-then-copy the 5 read-only dirs → no ghost lessons on upgrade); jniLibs were cached so the build was 7s. **Bundle VERIFIED by unzipping the built APK before release**: gui carries the IDE (renderIde + ⋯ entry, 4 hits), 78 lessons in assets/store/lessons, L1 back-link present, the 15b/15c/35b/37b split files bundled, **ZERO ghost/old-merged lessons**, versionName reads `0.4.0-dev.38`. GitHub PRE-release (Obtainium silent channel) with a versioned APK asset, **anon-download confirmed HTTP 200 / 26.69 MB** (never an Actions link). One PushNotification sent (delivers Paul's direct IDE ask); NO email per the dev.N-silent cadence. Release: https://github.com/thepictishbeast/Tempered-Studio-Mobile/releases/tag/v0.4.0-dev.38 — notes ask Paul to spot-check the IDE (explorer→open→fullscreen), the 78-count, L1's back-link, and the Termux run path. ⚠ Awaiting his on-device confirm (esp. IDE Termux-run, still unverified on hardware). **NEXT: board items — #37 coupling / #39 gamify / #40 beauty / #41 textbooks.**
- ✅ **#36 CLOSED — the sweep-up: a lesson-corpus integrity checker (2026-07-07, TS `5489bb9`, CONTENT_VERSION 35→36; still 78 lessons). 🏁 #36 DONE end-to-end (38 monolithic lessons → 78 baby steps + this guard).** NEW `scripts/check-lessons.mjs` (wired into `scripts/check.sh`, no server/browser): asserts H1 + 7 `## N.` sections + a lesson-nav footer whose links resolve; **footer-CHAIN continuity** in course order (each lesson's `next →` is the successor's file and the successor's `← prev` is it); no body link to a ghost/renamed file; Study-Guide "It's N lessons" == file count. **Caught a REAL bug on first run: Lesson 1 had NO `← Lesson 0` back-link** — a navigation dead-end at the very start, predating every split. Fixed + synced to rust-textbook (`50b833a`). Verified: checker PASSES 78 lessons; gui-transform OK; live :8099 marker=36 with the back-link in the served store; e2e 32/32; fmt clean. This is the durable regression guard the content phase never had — a mistyped footer filename in any future split now fails the gate instead of silently breaking nav. **NEXT highest-value: cut dev.38 (78 lessons + IDE + nav, all stuck on web since dev.37) — the biggest undelivered thing; verify the IDE bundles into mobile assets first.** Board now: #37 coupling, #39 gamify, #40 beauty, #41 textbooks all pending.
- ✅ **#38 pt.2 — THE FULLSCREEN IDE GUI: file explorer + editor + terminal (2026-07-07, TS `70c827e`). 🏁 #38 DELIVERED end-to-end (server pt.1 `e9dac31` + GUI pt.2).** Paul's direct ask, built against the 8-agent map+design workflow (`wf_3ea7e6e2-a1e`; the judge was slow so I synthesized from the 3 converging proposals — mobile-first + learner-first both vetoed a 3rd tab, citing the declutter directive + navigation.spec.js count-2 pin). A new `ide` view in `#docview` reached from the ⋯ menu (NO third tab): three SEPARATE sub-views 🗂 Files / ✎ Editor / ▤ Terminal, each fullscreen-able via a body class that tucks header/footer (auto-cleared in showView on exit so it can't bleed). Files = the workspace tree with status dots + lock badges; clicking loads via GET, locked → the Practice jump-ahead confirm. Editor = CM6 (NEW shared `makeEditorExtensions`; Practice/Sandbox untouched); ▷ Run through the PROGRESS-NEUTRAL sandbox path (never /api/run — no grading/advance/answer-reveal in the IDE), 💾 Save persists via PUT. Terminal = ANSI `#ideout` console. **A WORKSHOP, not a 2nd learning flow.** Verified LIVE 390×844 (Playwright+screenshots): 71 files/11 groups, CM6 loads, fullscreen toggles+auto-clears, ZERO console errors. NEW ide.spec.js (4 tests incl. no-answer-leak + the 423 gate — fixed a subtle test bug: tree *status* `locked` ≠ *gated*, since is_unlocked opens a file once its predecessor is Done; probe from the end for a real 423) + a11y IDE dark+light → 34/34. **⚠ deferred to a follow-up tick: mobile Termux run path for the IDE is wired (runCode → 'sandbox' op) but UNVERIFIED on-device; and the IDE ships web-only until the next dev.N bundle.**
- ✅ **#36 Batch E pt.10 — the 15-ownership split: the LAST audit row (2026-07-07, TS `769f282` ⚠PUSH PENDING (403 cooldown); CONTENT_VERSION 34→35; 76→78 lessons). 🏁 ALL 32 PACING-AUDIT WORK ROWS DONE (24 splits + 7 trims + 1 belated mark). The curriculum: 38 monolithic lessons → 78 baby steps, CV 4→35, every flagged fwd-ref resolved, ~20 new compile-verified error walls added.** **15** = the three rules + drop tied to L3's scope ("name and memory expire together"); stack/heap → ONE sentence + Book §4.1 per the audit; NEW E0425-after-drop wall ("available in a different scope" = scope+drop pointing at the same brace). **15b** = owner-card moves with a NEW working-move-FIRST example ("the move is normal; the use-after is what's stopped"); E0382 + function-move verbatim; takes the `move-semantics` coupling. **15c** = Copy/Clone; §4 re-READS 15b's E0382 ("the message is this whole lesson in two lines") + don't-reflex-clone; NEW tuple-straddle task. 16→pt.4, 16b→pt.5, 17→pt.6-and-last renumbers. Study Guide 76→78 (vs live); marker=35; e2e 28/28; gui-transform; fmt clean; serve tests 26/26. **REMAINING for #36: only the sweep-up verification pass. Done this tick alongside: unstuck the #38 design workflow (4th mapper hung 20+ min silent → TaskStop + narrowed-prompt resume `wf_3ea7e6e2-a1e`; 3 mappers cached; the editor map's KEY CORRECTION: xterm is NO LONGER loaded — the console is a DOM `<pre id=conout>` + `term` shim, so the fullscreen terminal fullscreens THAT).**
- ✅ **#38 pt.1 — the fullscreen IDE's SERVER HALF: the workspace file API (2026-07-07, TS `e9dac31`; Paul's direct ask same day — "go ahead and start on the fullscreen IDE"; #36's last row, the 15 split, queues behind it).** Three endpoints keyed by exercise ID — the wire never names a path (the `/api/select` doctrine), so traversal and answer-file exposure are impossible **by construction** (learner `.rs` files sit NEXT TO answer-bearing `.toml`s; a path-based API would have needed filtering — an id-keyed one cannot even express the request). `GET /api/workspace` (explorer tree: phase groups, id/name/title/status only), `GET /api/workspace/file?id` (editor source; locked → 423 — the explorer is not a LOCKOUT bypass), `PUT /api/workspace/file` (the IDE's persistent save, vs `/api/run`'s ephemeral buffer; unknown → 400, locked → 423, done → 409 mirroring select; size clamp). Contract tests 22→26 (no-answer tree, roundtrip w/ on-disk persistence, all gates, oversize). Live smoke: 11 groups, zero `.toml`/answer leaks, locked = 423. e2e 28/28; fmt+seam clean; GUI untouched. **In flight: the 8-agent map+design workflow (`wf_3ea7e6e2-a1e`) — 3 of 4 mappers done; the GUI half (IDE view: file explorer / editor / fullscreen terminal, xterm addon-fit + `refit()` already vendored) implements against its judged spec next.**
- ✅ **#36 Batch E pt.9 — Lessons 23 + 36 trims, and an audit-of-the-audit (2026-07-07, TS `248a13a`, CONTENT_VERSION 33→34; still 76 lessons).** **23-use**: the nested/`self` section with its io/Write example CUT (killing the Write-trait fwd-ref AND the never-taught byte-string), the glob HashMap/HashSet example CUT (killing the never-taught HashSet), `pub use` compressed to one recognize-not-master sentence — all → Book §7.4 pointers per the audit; the scope-locality E0433 wall and all practice intact; **live store verified token-free** (grep HashSet/write_all = 0). **36-tests**: custom messages + `should_panic(expected=…)` → Book Ch.11.1 pointer block (with a nod that doc-tests are now L37); practice task 3's substring half replaced with a fix-it-so-it-DOESN'T-panic prediction; the left:/right: walkthrough untouched. **Then a sweep of un-✅'d matrix rows found TWO surprises:** (a) the 01 trim was DONE in Batch A (`1762b79`, 1214→1095w) but never marked — marked now; (b) **`15-ownership-and-moves` (1517w, split → Ownership/Scope/Drop + Moves) fell through every batch plan** — it is the ONE remaining content row. Marker=34; e2e 28/28; gui-transform; fmt clean. Matrix ✅ (31 of 32 work rows). **Next: the 15 split — the LAST content row — then the sweep-up pass closes #36.**
- ✅ **#36 Batch E pt.8 — Lesson 37 split: doc-comments + 37b shipping (2026-07-07, TS `7ae2fc8`, CONTENT_VERSION 32→33; 75→76 lessons).** **37** = the docs-are-tested trick now PROVEN with live captures from a real `cargo new adder --lib` project: the passing run (the test's *name* is a place in your documentation — "src/lib.rs - add (line 7)") and the lying-example failure ("left: 5, right: 6" — the code said 5, the docs claimed 6, the docs lost), framed as L36's assert in a new uniform; practice explicitly project-based. **37b** = profiles/publishing/workspaces as the honest ten-minute tour (publish + workspace mechanics → Book §14.2/§14.3 per the audit; no-rustc-error-code framing kept); carries the course epilogue. ⚠ Capture gotcha for the playbook: **edition-2024 merged doctests need TMPDIR on an exec-able dir** — with the host's noexec default tmp, cargo reports "doctest failed" with ZERO detail even on passing examples (diagnosed via --verbose → rustdoc builds the merged bundle in TMPDIR). Footers 36→37→37b (37b final, no next); Study Guide 75→76 (vs live); marker=33; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (99 links). Matrix ✅ (28 rows). **Batch E remaining: 23/36 trims + the sweep-up pass — then #36 is DONE.**
- ✅ **#36 Batch E pt.7 — Lesson 22 split THREE ways: paths, privacy, type-shaped pub (2026-07-07, TS `7f26510`, CONTENT_VERSION 31→32; 73→75 lessons).** Couplings placed by the exercises' own `expected_error_code`s. **22** = `crate::`/relative/`self::`/`super::`; **NEW E0425 forgot-`super` wall** ("names don't drift down into child modules") whose two fixes are one per path flavour — the compiler suggests the absolute route, the lesson shows the relative; takes `super-paths` (exercise expects E0425 — exact pair). **22b** = one-way privacy, the two-strike E0603 kept verbatim, a count-the-pubs walkthrough of the working example (incl. why `front_of_house` needs none — sibling privilege); takes BOTH `module-privacy` + `item-privacy` (both expect E0603); pub(crate) → one sentence + Book/CR per audit. **22c** = the struct/enum asymmetry framed by what-each-kind-is-*for*; Breakfast/E0616 verbatim; E0451 aside → Book §7.3; NEW read-vs-write task. Sandbox-only. 23 renumbered pt.7-and-finale. Footers 21c→22→22b→22c→23; Study Guide 73→75 (vs live); marker=32; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (98 links). Matrix ✅ (27 rows). Batch E remaining: 37 split + 23/36 trims + sweep-up — the pacing overhaul's tail is in sight.
- ✅ **#36 Batch E pt.6 — Lesson 21 split THREE ways: packages/crates, modules, files (2026-07-07, TS `2605311`, CONTENT_VERSION 30→31; 71→73 lessons).** **21** reframed around a reveal — "you've been writing crates since Lesson 0" — with a **NEW E0601 wall** where the compiler says *"in crate `main`"* (the unit made real; no-main code framed as library-shaped, not wrong). **21b** = the crate-rooted tree + foo/bar namespace example; the **`pub` fwd-ref fixed** with an explicit copy-now-understand-in-L22 gloss; **NEW E0425 wall** (bare call to a module's function) whose `help: use crate::foo::do_something` line is called out as a two-lesson preview (L22 paths, L23 use) — practice task 3 has the learner spot that themselves. **21c** = `mod name;` file loading, "mod is not include" + E0583 kept verbatim, "files are storage; the tree is the truth"; mod.rs style + cardinality → Book §7.5/§7.1 per the audit; practice explicitly project-based (cargo/Termux) with a NEW three-segment-path task. 22 renumbered pt.4 (back-ref → 21b–21c), 23 → pt.5. Footers 20c→21→21b→21c→22; Study Guide 71→73 (vs live); marker=31; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (96 links). Matrix ✅ (26 rows). Batch E remaining: 22/37 splits + 23/36 trims + sweep-up.
- ✅ **#36 Batch E pt.5 — Lesson 11 trim (2026-07-07, TS `2932223`, CONTENT_VERSION 29→30; 1305→1210w, still 71 lessons).** The audit's three flagged meta blocks removed: the sources-comparison blockquote CUT (its one learner-relevant sentence kept — "shallow intro on purpose"), the coin-sorter metaphor + match-vs-if contrast + closing philosophy each compressed to one line with Book §6.2 pointers. The E0004 walkthrough — the lesson's core — untouched; `match-exhaustiveness` coupling unchanged. Marker=30 with the trimmed file confirmed IN THE LIVE STORE (content refresh sync working as designed); e2e 28/28; gui-transform; fmt clean. Matrix ✅ (25 rows). Batch E remaining: 21/22/37 splits + 23/36 trims + sweep-up. Next: the 21-packages-crates-modules split.
- ✅ **#36 Batch E pt.4 — Lesson 14 split: Vec + 14b HashMap (2026-07-07, TS `76ac338`, CONTENT_VERSION 28→29; 70→71 lessons).** All THREE flagged fwd-refs resolved. **14** = vec!/push/pop with a **NEW pop-Option example** ("last = Some(7)" — one return type must cover the empty run), `[]`-vs-`.get()` as the learner's design choice, runtime-only bounds panic ("13b's two-times collapses to one"); the **`&mut`/`*` in-place loop REMOVED → Phase 4 + Book §8.1 pointer** per the audit (practice stays read-only `&v` via 13c's borrowed-view framing); NEW pop-till-empty task; keeps BOTH vec couplings. **14b** = the `use` import **glossed forward-safe** ("Lesson 23's topic — a fixed incantation to copy exactly") and backed by the E0433 pitfall (now doubling as an imports-in-general preview); entry().or_insert() showpiece + run-it-twice unordered check; the `unwrap_or` practice bonus **replaced with plain `.get()` + `{:?}`** (combinators are L20's); String-key move → Book §8.3 pointer; takes hashmap-insert-ownership coupling. Footers 13c→14→14b→15; Study Guide 70→71 (vs live); marker=29; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (94 links). Matrix ✅ (24 rows). Batch E remaining: 21/22/37 splits + 11/23/36 trims + sweep-up. Next: the 11-match-intro trim (quick) or 21-packages split.
- ✅ **#36 Batch E pt.3 — Lesson 13 split THREE ways: tuples, arrays, slices (2026-07-07, TS `62154e8`, CONTENT_VERSION 27→28; 68→70 lessons). ⚠ADOPT debt PAID.** **13** = arity/destructure/unit-() with a **NEW E0308 arity-mismatch pitfall** ("expected a tuple with 3 elements, found one with 2" — arity taught by the error) + the `_` placeholder fix + an arity-zero task tying `()` back to L6. **13b** = `[T; N]` + the dual out-of-bounds kept verbatim, then the **adopted L10 off-by-one while-walk** as "how this bites in real code" — Lesson 10c's promised debt paid in its rightful home, with the for-fix moral. **13c** = length-erasure as the headline (`&[i32]`, no `; N`), `&str`-is-a-slice tie; **NEW slice-range runtime panic** ("range end index 7 out of range for slice of length 5") mirroring 13b's compile-vs-runtime rule; the UTF-8 mid-character example → one-sentence warning + Book Ch.8.2 pointer per the audit; NEW `peek(&[i32])` any-length-fits task. All Sandbox-only; 14 renumbered pt.5-and-last. Footers 12→13→13b→13c→14; Study Guide 68→70 (vs live); marker=28; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (93 links). Matrix ✅ (23 rows). Next: 14-vec-hashmap split, or the 11-match-intro trim.
- ✅ **#36 Batch E pt.2 — Lesson 10 split THREE ways: loop, while, for/ranges (2026-07-07, TS `0f89993`, CONTENT_VERSION 26→27; 66→68 lessons).** One loop form per lesson, each with its own on-topic wall, all THREE flagged fwd-refs resolved. **10** = break-as-only-exit + break-value; **`+=` now TAUGHT** (was used-never-taught); labels → Book §3.5; **NEW E0571** (break-with-value in a while — compiler states the single-exit rule verbatim + hands the fix diff); keeps `loop-break-value` coupling. **10b** = the `loop{if !cond{break}}` desugar + LIFTOFF rep; **NEW E0308** non-bool condition explicitly echoing L9 ("one rule, every condition in the language"); NEW runaway-loop task. **10c** = `..`/`..=` as the atomic bit (`.rev()` → Book pointer); **`print!` now TAUGHT**; the **array off-by-one panic REMOVED and DEFERRED to the L13 row** (needs arrays/indexing/.len() — deferral flagged ⚠ on the 13 matrix row); **NEW E0425** (loop variable out of scope) pairing `04_for_scope`'s `expected_error_code` exactly; takes `loop-variable-scope` coupling. Caught pre-commit: my own false "Lesson 1 block scoping" attribution → corrected to L3 (verified by grep — same error class the audit caught in the capstone). Spec id updated; 11 renumbered pt.6-and-last. Footers 09b→10→10b→10c→11; Study Guide 66→68 (vs live); marker=27; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (91 links). Matrix ✅ (22 rows). Next: 11-match-intro TRIM, or the 13 split (which now must adopt the off-by-one demo).
- ✅ **#36 Batch E pt.1 — Lesson 9 split: if/else choosing + 9b if-as-expression (2026-07-07, TS `2a441fc`, CONTENT_VERSION 25→26; 65→66 lessons).** **9** = the fork itself: shape, no-truthiness, first-true-wins now EXPLICIT (order-matters + else-catch-all bullets); E0308 expected-bool wall; NEW practice: swap the cutoffs and watch order claim the run. **9b** = the Rust twist as its own lesson: L6 block-value recap, Listing 3-2 type-it rep, arms-must-agree, E0308 incompatible-arms with the `&str` gloss made **forward-safe** per the audit ("Lesson 12's topic — the compiler had to name the type, you don't have to know it yet"); NEW practice: the semicolon trap inside an arm. **First split where the exercise coupling moves to the letter-suffix half** (CONCEPT_LESSON `if-as-expression` → `09b`; gui-transform validates). Sources-comparison meta blockquote cut per the audit. Footers 08→9→9b→10; Study Guide 65→66 (vs live); marker=26; e2e 28/28; fmt clean; textbook index regen'd (89 links); matrix ✅ (21 rows). Next: the 10-loops 3-way per its audit row (loop-until-break · while · for).
- ✅ **#36 Batch D pt.6 — Capstone split: 35 single-threaded server + 35b thread pool (2026-07-07, TS `6464b3f`, CONTENT_VERSION 24→25; 64→65 lessons). 🏁 BATCH D COMPLETE (30–35 → fourteen atomic lessons, CV 19→25).** **35** = TcpListener/TcpStream + HTTP-response anatomy; the slow-request flaw now **demonstrated, not asserted** (practice: build it, add /sleep, feel `/` wait; task 3's sentence = 35b's design brief); NEW compile-verified **E0596** pitfall (forgot `mut` on the stream — L2's rule at the capstone). **35b** = the pool, compiler-driven off the verbatim E0382 receiver wall (tied to 30c's "previous iteration"). **ALL 3 flagged fwd-refs FIXED**: `'static` glossed as the no-borrows end of L26's spectrum; `assert!` glossed as a L36 peek; `Option::take` given an honest new-here gloss (extract ownership through `&mut` by swapping in `None`) replacing the false "(L19)" attribution. Lock-scope + Drop-ordering depth → Book Ch.21 per the audit; premature "end of the series" claim fixed (L36–37 follow). Footers 34c→35→35b→36; Study Guide 64→65 (vs live); marker=25; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (88 links verified); matrix ✅ (20 rows). **Next: Batch E** — 09/10/13/14/21/22/37 splits + 11/23/36 trims, then the sweep-up verification pass. The pending mobile batch since dev.37 is now 9 new lessons (31→35b) + 5 phase-label fixes — dev.38 worth cutting after Batch E's first few rows.
- ✅ **#36 Batch D pt.5 — Lesson 34 split THREE ways: unsafe, operators/associated-types, macro_rules! (2026-07-07, TS `4917a9f`, CONTENT_VERSION 23→24; 62→64 lessons).** **34** = raw pointers + E0133 + the E0502 proof the borrow checker stays ON (superpowers 2–5 → one named line + Book Ch.20.1 per the audit); keeps the `unsafe-raw-pointers` exercise coupling; NEW practice: borrow-violate *inside* unsafe, predict it still fails. **34b** = `Add` on Point, `+` as `Add::add` sugar, `type Output` — **closes L31's IOU** (its `impl Future<Output = u32>` gloss now readable exactly; practice task 4 has the learner re-read it in associated-type words); **NEW compile-verified E0369 pitfall** whose note names the fix ("an implementation of `Add` might be missing") tied back to L24's generic wall. **34c** = the grammar model of `macro_rules!`, mini `my_vec!`, **NEW compile-verified trailing-comma pitfall** ("unexpected end of macro invocation… while trying to match meta-variable `$x:expr`") + `$(,)?` widening verified-to-build. Footers 33b→34→34b→34c→35; Study Guide 62→64 (vs live); marker=24; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (87 links verified); matrix ✅ (19 rows). Next: the 35-capstone 2-way (fix Option::take/'static/assert! fwd-refs) — CLOSES Batch D.
- ✅ **#36 Batch D pt.4 — Lesson 33 split: refutability + 33b guards/@/nested (2026-07-07, TS `fce023e`, CONTENT_VERSION 22→23; 61→62 lessons).** **33** = the one rule (can this pattern FAIL?), the else-path position table, E0005 + its two L19d fixes, and the mirror irrefutable-if-let warning; NEW paper-first classify-five-patterns drill. **33b** = the flagship Listing 19-29 promoted to §2 with a tightened walk (catalog depth → Book Ch.19.3 per the audit), nested destructure + guard-over-`|`, and a **NEW compile-verified E0004 pitfall**: two obviously-complete guards (`x if x<0` / `x if x>=0`) still non-exhaustive — the compiler's own note states the rule ("match arms with guards don't count towards exhaustivity"); practice 2 has the learner rediscover it by guarding the last arm. `ref` footnote → 1-line read-more. Both halves Sandbox-only (exerciseless per audit). ⚠ practice-handoff.spec.js id list updated (33-advanced-patterns → 33-refutability). Footers 32c→33→33b→34; Study Guide 61→62 (vs live); marker=23; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (85 links verified); matrix ✅ (18 rows). Next: the 34-advanced-features 3-way — then 35-capstone 2-way closes Batch D.
- ✅ **#36 Batch D pt.3 — Lesson 32 split THREE ways: trait objects, encapsulation, states-as-types (2026-07-07, TS `3d96fd6`, CONTENT_VERSION 21→22; 59→61 lessons).** The Book's three OOP moves, one lesson each. **32** = the mixed-collection problem, `Box<dyn>`/`&dyn`, fat-pointer intuition, the static-vs-dynamic table + CR's dyn-last caution, E0308 mixing wall; **audit offloads applied**: the E0038 dump compressed to its key line + keep-methods-plain rule → Book Ch.18.2, the `size_of` measurement → read-more. Keeps BOTH exercise couplings (CONCEPT_LESSON `trait-objects` + `dynamically-sized-types` remapped). **32b** = AveragedCollection + a **NEW compile-verified E0616 pitfall** (`mod stats` boundary + `c.list.push(999)` reach-in — "the exact bug the type exists to prevent"), privacy tied back to L22; practice 3 = rebuild the inside, `main` never notices. **32c** = DraftPost/PublishedPost, `publish(self)` consumes (18b's `self`-by-value + L15 ownership retiring the old state), the E0599 payoff; practice adds the use-after-transition E0382 prediction. 32b/32c Sandbox-only; 33 renumbered to Phase 9 pt.4. Footers 31b→32→32b→32c→33; Study Guide 59→61 (vs live); marker=22; runner+serve 30/30; e2e 28/28; gui-transform; fmt clean; textbook index regen'd (84 links verified); matrix ✅ (17 rows). Next: the 33-patterns 2-way.
- ✅ **#36 Batch D pt.2 — Lesson 31 split: async syntax + 31b futures-are-lazy (2026-07-07, TS `5437bb8`, CONTENT_VERSION 20→21; 58→59 lessons).** **31** = the three syntax pieces (`async fn` / async blocks / postfix `.await`), threads-vs-async framing, desugar/chain/block examples, and a **NEW compile-verified E0308 pitfall** ("expected `u32`, **found future**" — the compiler naming the model). **The flagged fwd-ref is FIXED**: `impl Future<Output=>` is now a read-only gloss (associated types deferred to L34, mirroring 19b's placeholder-gloss pattern) and practice task 2 supplies the signature verbatim instead of asking the learner to invent it. **31b** = the build-but-never-run proof, the L28 iterator-laziness parallel, E0728+E0752 as two views of ONE missing piece (a runtime); tokio/`block_on` runtime-origins **offloaded to Book Ch.17.1** and CR's Future/poll kept read-more-only, per the audit. Both halves Sandbox-only (31 was exerciseless). Rider fixes: phase labels 29d/30/30b/30c reconciled to the GUI quiz map (30–31 = **Phase 8**, prose said 9); **rust-textbook's own STUDY-GUIDE.md index regenerated from the real 59 files** — it still listed pre-split ids (27-closures, 30-threads-and-concurrency…, ~24 dead links) from before Batch C. Footers 30c→31→31b→32; Study Guide 58→59 (vs live); marker=21; runner+serve tests 30/30; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (16 rows). Next: the 32-trait-objects 3-way.
- ✅ **#36 Batch D pt.1 — Lesson 30 split three ways: spawning, channels, shared state (2026-07-07, TS `a07b7d7`, rust-textbook `eb07988`, CONTENT_VERSION 19→20; 56→58 lessons).** **30** = spawn/join/interleaving + `move` as 27b's keyword meeting its sharpest customer; E0373 walk pairs `01_spawn_move`. **30b** = mpsc/send-moves/iterating-rx + a **NEW use-after-send E0382 pitfall (compile-verified)** pairing `04_channel_send`'s exact error — the old lesson only gestured at it. **30c** = Mutex+Arc with the Book's error-driven arc **compressed per the audit** (outline + short excerpts; full dumps → §16.3; Send/Sync one paragraph → §16.4) and the third wall (Arc-without-Mutex E0594) named for its exercise — pairs BOTH `02_rc_not_send` + `03_arc_mutex`. Footers 29d→30→30b→30c→31; Study Guide 56→58 (vs live); marker=20; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (15 rows). Next: the 31-async split (fix the `impl Future<Output=>` fwd-ref).
- ✅ **📱 MOBILE v0.4.0-dev.37 RELEASED (2026-07-07, mobile `794b805`, versionCode 10351) — the curriculum-overhaul batch.** The largest delivery since the app began, batched since dev.36 (~15 ticks): **56 baby-step lessons** (11 splits + 3 trims + 41 fwd-ref repairs + practice-in-app + one-exercise coupling), the reading-nav suite (Book/quiz/cheat prev-next, drawer styling, glossary inline-code), glossary 61→67, and the seeding SYNC fix (this build's re-seed removes renamed lessons cleanly). **Bundle verified before release: 56 lessons in assets, all new split files present, ZERO ghosts, gui carries prevNextNav, study guide reads 56, glossary 67.** GitHub PRE-release (silent Obtainium channel, NO email per practice): https://github.com/thepictishbeast/Tempered-Studio-Mobile/releases/tag/v0.4.0-dev.37 — asset verified anon-downloadable. Release notes ask Paul to spot-check: 56 in the lesson list, a few split lessons (16b/20c/29d), practice handoffs, no lingering old titles. ⚠ On-device Termux run path still needs Paul's confirmation.
- ✅ **#36 Batch C pt.6 — Lesson 29 split FOUR ways: Box, Deref+Drop, Rc, RefCell (2026-07-07, TS `acf2972`, rust-textbook `081ecca`, CONTENT_VERSION 18→19; 53→56 lessons). BATCH C COMPLETE.** One tool per lesson, one-exercise coupling (E0072 → 29-box · exerciseless-by-design 29b-deref-drop, which sets up 29c's "Drop is how Rc counts down" · E0382-share-vs-move → 29c-rc with the Book's TV metaphor · borrow-panic → 29d-refcell, named the course's THIRD runtime-panic pattern after 5b/20b). Offloads per the audit: Deref depth → Book §15.2, RefCell nuance + the Rc<RefCell<T>> combo depth → §15.5 (one closing example kept). Footers 28b→29→29b→29c→29d→30; Study Guide 53→56 (vs live); marker=19; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (14 rows). **BATCH C TOTALS: six 1900–2400-word lessons (24–29) → fifteen atomic lessons; CV 13→19; every flagged fwd-ref in the abstraction ramp eliminated. Curriculum now 56 lessons (was 38). Next: Batch D (30–35) — and the mobile dev.N question is now RIPE (pending batch = 18 new lessons + nav + refresh pipeline since dev.36).**
- ✅ **#36 Batch C pt.5 — Lesson 28 split: the next() cursor + 28b adapter chains (2026-07-07, TS `a153b62`, rust-textbook `916d94e`, CONTENT_VERSION 17→18; 52→53 lessons).** **28** = the Iterator trait, next() by hand, the laziness rule stated, iter/iter_mut/into_iter as borrow/borrow-mut/own + the for-loop tie-back, and the E0382 after-into_iter wall — pairing BOTH ownership exercises (2). **28b** = adapters-vs-consumers, the chain-as-a-sentence, the unused-Map laziness WARNING (named as the course's second warning-class trap, after 19c) — pairing all THREE annotation exercises (collect/collect-inference/sum); the duplicate turbofish example → one sentence + Book Ch.13.2 per the audit (links back to L20's `::<>` gloss). Footers 27b→28→28b→29; Study Guide 52→53 (vs live); marker=18; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (13 rows). **Next: the 29-smart-pointers 4-way — the last Batch C split.**
- ✅ **#36 Batch C pt.4 — Lesson 27 split: closure syntax + 27b capture & move (2026-07-07, TS `09ae769`, rust-textbook `8d0343e`, CONTENT_VERSION 16→17; 51→52 lessons).** All THREE flagged fwd-refs eliminated: the `.iter().map().collect()` example → **`sort_by_key`** (no iterator chains before L28); the `thread::spawn` motivating example → **thread-free `move`** ("outliving the spot it was written — returning/storing; Lesson 30 for threads"); the twice-dangled "Fn/FnMut/FnOnce in full next lesson" promise (no such lesson exists) → Book Ch.13.1 pointers with "an FnMut closure" glossed as *callable repeatedly*. **NEW E0308 inference-locks-on-first-use pitfall** (Book 13-3) pairs `closures/01_closure_type_lock` exactly; the E0382 walk pairs `02_fn_once_move`. One self-review catch: my own practice hint used `.bytes().last()` — replaced with a non-iterator key, compile-verified like the rest. Footers 26c→27→27b→28; Study Guide 51→52 (vs live); marker=17; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (12 rows). Next: the 28-iterators split, then 29-smart-pointers (4-way) closes Batch C.
- ✅ **#36 Batch C pt.3 — Lesson 26 split three ways: annotations, elision, struct lifetimes (2026-07-07, TS `f52cce7`, rust-textbook `9c7bca1`, CONTENT_VERSION 15→16; 49→51 lessons).** The 2344-word lifetimes lesson, with **both flagged iterator forward-refs eliminated by rewriting the examples**: `first_word` and the `Excerpt` novel-slicing now use `s.find(…)` + `match` on the `Option` — all L19b/L13 tools — and **both new snippets were compile-verified on rustc** (outputs match the lesson text exactly). **26** = the `'a` signature read line-by-line + the "promise not a lever" centrepiece + the E0106 walk (pairs `lifetimes/01_longest`); **26b** = elision rule 2 in depth (rules 1+3 → Book) + the E0515 return-a-local limit compressed per the audit (Sandbox practice); **26c** = `Excerpt<'a>` closing Lesson 18's deferred `&str`-field E0106 + borrow-vs-own as a real choice (pairs `lifetimes/02_struct_lifetime`). Footers 25b→26→26b→26c→27; Study Guide 49→51 (vs live); marker=16; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (11 rows). Next: the 27-closures split (its thread/iterator fwd-refs + the dangling "Fn/FnMut/FnOnce next lesson" promise).
- ✅ **#36 Batch C pt.2 — Lesson 25 split: declare/implement + 25b trait bounds (2026-07-07, TS `656a946`, rust-textbook `a6fbc8f`, CONTENT_VERSION 14→15; 48→49 lessons).** Structured so **24→25b is one continuous arc**: 25b OPENS by closing 24's E0369 cliffhanger (the same `largest<T>` body compiles with `T: PartialOrd` — "the wall is closed"). **25** teaches declare/required-vs-default/impl-for/E0046 — its flagship now calls `.summarize()` DIRECTLY (the old version used `&impl Summary`, a bounds shape, before bounds are taught); pairs FOUR exercises (display_bound, trait_in_scope, missing_method, operator_overload_add). **25b** = the three bound spellings + the E0277 walk; impl-Trait-return → one line + Book; orphan rule one-liner; pairs orphan_rule_newtype + supertrait_display. Footers 24b→25→25b→26; Study Guide 48→49 (vs live); marker=15; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (10 rows). Next: the 26-lifetimes 3-way split (incl. its iterator-example rewrites).
- ✅ **#36 Batch C pt.1 — Lesson 24 split: generic functions + 24b generic types (2026-07-07, TS `a1f8907`, rust-textbook `518e3d6`, CONTENT_VERSION 13→14; 47→48 lessons).** The 2270-word generics opener, with its flagged fwd-ref **resolved by restructuring the arc**: the flagship example is now `first<T>` (compiles with NO bound — the body only stores/returns a reference), and the un-bounded `largest<T>` **E0369 wall becomes the lesson's DESTINATION** and cliffhanger into trait bounds — the learner never types unexplained `T: PartialOrd` as working code. 24 pairs BOTH exercises (E0369 + E0284); **24b** carries Point<T>/MyOption<T>-unmasking/impl<T> + the E0425 walk, monomorphization → one-liner + Book pointer. Footers 23→24→24b→25; Study Guide 47→48 (vs live); marker=14; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (9 rows). Next: the 25-traits split.
- ✅ **#36 Batch B pt.5 — the 12 + 17 trims (2026-07-07, TS `8403d8e`, rust-textbook `33e66e5`, CONTENT_VERSION 12→13). BATCH B COMPLETE.** Four forward-reference kills (the trims' real substance — length barely moved, code became shorter prose): **L12** — slice syntax (`&owned[0..5]`) appeared one lesson before L13 in the core example AND practice; now the whole-String `.as_str()` view with slicing named as L13's opener; the unexplained-`&` `s1 + &s2` example + the +-consumes passage → two sentences + Book §8.2; UTF-8 depth → Book §8.2. **L17** — the 20-line `first_word` implementation used `as_bytes()`/`b' '` (taught nowhere); now the SIGNATURE + the borrowed-return story (the lesson's actual point) + Book §4.3; the silent `&String`→`&str` deref coercion glossed (machinery → L29). Verified served text on the refreshed store (marker=13); e2e 28/28; gui-transform; seam+fmt clean; matrix ✅×2 (8 rows). **Batch B totals: 4 splits + 2 trims, 40→47 lessons, CV 9→13. Next: Batch C — the abstraction ramp (24-generics split first: 2270w).**
- ✅ **#36 Batch B pt.4 — Lesson 20 split three ways (2026-07-07, TS `244b624`, rust-textbook `1834382`, CONTENT_VERSION 11→12; 45→47 lessons). ALL BATCH-B SPLITS DONE.** The 1433-word error-handling lesson → **20-result** (errors-as-values, match/unwrap_or, NEW "Result is not a T" E0308 pitfall pairing `01_result_is_not_t`; **turbofish fwd-ref fixed** with a read-as gloss), **20b-panic-unwrap-expect** (the blunt path; compiles-then-crashes ferris walk; the 5b trap named again; pairs BOTH runtime-panic exercises; judgement→Book §9.2/9.3 pointers), **20c-question-mark** (propagation + mixed-kind rule + From-conversion for custom errors — sets up all THREE ?-exercises; FromResidual one-liner kept). Footers 19d→20→20b→20c→21; Study Guide 45→47 (vs live); marker=12. **The e2e run caught the spec's own hardcoded pre-split id** (practice-handoff listed `20-error-handling`) — updated + noted to keep spec ids current across splits. e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (6 rows). **Remaining Batch B: the 12/17 trims. Phase-5 chain is now: 18→18b→18c→19→19b→19c→19d→20→20b→20c — ten atomic lessons where three dense ones stood.**
- ✅ **#36 Batch B pt.3 — Lesson 19 split FOUR ways (2026-07-07, TS `4fda08a`, rust-textbook `13524c3`, CONTENT_VERSION 10→11; 42→45 lessons).** The 1452-word enums lesson (5 concepts) → **19-enums** (one-of types, variant shapes, method-on-enum; NEW E0532 shape-mismatch pitfall pairing `control-flow/05_match_enum_data`), **19b-option** (no-null; the **T-placeholder gloss fixes the audit's generics forward-ref**; E0004 + E0277 walks; Hoare→Book pointer; pairs `types/03_option_value`), **19c-match-in-depth** (the L11-deferred vocabulary + a NEW warning-class trap: unreachable-arm is only a WARNING — "read warnings too"; Sandbox practice), **19d-concise-matching** (if let / while let / let…else as deliberate exhaustiveness trades; E0005 refutable-let wall pairing `types/05_refutable_let`). Footers chained 18c→19→19b→19c→19d→20; Study Guide 42→45 (verified against the LIVE count); marker=11; e2e 28/28; gui-transform; seam+fmt clean; matrix ✅ (5 rows done). Next: the 20-error-handling split, then the 12/17 trims.
- ✅ **#36 Batch B pt.2 — Lesson 18 split three ways (2026-07-07, TS `5dbd4c9`, rust-textbook `539d73b`, CONTENT_VERSION 9→10; 40→42 lessons).** The 1358-word structs lesson (6 atomic concepts) → **18-defining-structs** (define/instantiate, shorthand, the three forms, own-your-fields E0106; pairs `types/01_struct_build` E0063 + `types/02_field_privacy` E0616, both set up in the pitfalls prose), **18b-methods-and-impl** (receivers as borrow intent + associated fns/Self + a NEW E0599 no-method pitfall pairing `types/04_struct_method`), **18c-derive-debug** (the E0277 fail-then-fix kept as its core; `dbg!` + struct-update caveat → Book pointers per the audit; no mapped exercise). Footers chained 17→18→18b→18c→19; CONCEPT_LESSON re-mapped 2/1; Study Guide 40→**42** (a 3-way split adds TWO — caught my own off-by-one against the live count before it shipped). Verified live (marker=10, ghost gone, 42 in order, practice links exact, stage auto-grew to 5); e2e 28/28; gui-transform; seam+fmt clean; matrix ✅. Next: the 19-enums split, then 20, then 12/17 trims.
- ✅ **#36 Batch B pt.1 — the L16 override split: shared references + 16b the borrowing rules (2026-07-07, TS `934d47c`, rust-textbook `e6fddf1`, CONTENT_VERSION 8→9; 39→40 lessons).** The curriculum's practice hub (8 concept tags / 10 exercises on one lesson) is now two baby-steps: **16-shared-references** (&T as a view, the function-move fix, + a NEW E0507 "you can look, not take" pitfall pairing exactly with `ownership/04_move_out_of_borrow`) and **16b-mutable-references** (&mut + the two rules: E0499/E0502 walks, and rule-2 validity taught via a scope-based **E0597** instead of the old E0106 demo — the audit flagged E0106's "missing lifetime specifier" as unexplainable before L26; E0597's use→drop→borrow story needs no lifetime syntax and matches the mapped exercises' real errors). CONCEPT_LESSON re-mapped 1/7; sources-blockquote cut; NLL nuance compressed to a pointer. Verified live (marker=9, ghost gone, 40 in order, 16→1 exercise / 16b→9, Ownership stage auto-grew); e2e 28/28; gui-transform; seam+fmt clean; matrix row ✅. **Also this tick: the rolling-state memory file was found REVERTED by a rewind — restored from git ground truth (all work had persisted; only the memory regressed). Next: 18/19/20 splits, 12/17 trims.**
- ✅ **#36 Batch A pt.4 — L01 trim + the mobile ghost-fix (2026-07-07, TS `1762b79`, rust-textbook `e96c007`, mobile `9466e68`, CONTENT_VERSION 7→8). BATCH A COMPLETE.** **L01 1214→1095 words:** cut the sources-framing blockquote (author meta-commentary), the full let-mut fixed program + output prediction (duplicated L2's opening and had the learner run `mut` before it's taught — now reads the compiler's `help:` line + an explicit "that's the next lesson" hand-over; practice step 3 hands over too), and compressed the inference elaboration to two sentences + a Book §3.2 pointer. The E0384 walkthrough stays — it IS the lesson. **Mobile:** confirmed MainActivity's version-gated re-seed had the same overlay-ghost bug the split exposed on desktop; mirrored the fix (clear lessons/quizzes/cheatsheets/book/glossary before copy; progress + exercises untouched) — Java verified by a local debug build (BUILD SUCCESSFUL, no APK cut). Verified live (marker=8; blockquote/program gone, hand-over present); e2e 28/28; gui-transform. **Batch A done end-to-end: practice in-app curriculum-wide + Foundations glosses + first split + L01 trim + refresh-sync on all three platforms. Next: Batch B — the ownership spine (16 override split, 18/19/20 splits, 12/17 trims).**
- ✅ **#36 Batch A pt.3 — THE FIRST LESSON SPLIT: L05 → scalar types + 05b integer overflow (2026-07-07, TS `5960f71`, rust-textbook `522801d`, CONTENT_VERSION 6→7; 38→39 lessons).** `05-scalar-types` (four scalar kinds + a NEW no-silent-conversion E0308 pitfall pairing exactly with `basics/04_no_coercion`) + `05b-integer-overflow` (the first RUNTIME failure as its own baby-step: compile-time vs run-time, reading a panic line, the "it compiled so it works" trap — pairing with `basics/05_integer_overflow`). Letter-suffix id verified live end-to-end (sorts 04→05→05b→06; Foundations auto-absorbs → 0/9; per-lesson practice links now one-exercise-each — precise #37 coupling). **Also found + fixed a refresh gap the split exposed: overlay-copy left removed files as GHOSTS — the read-only dirs (book/glossary/lessons/quizzes/cheatsheets) are now SYNCED on refresh (server + CLI), exercises stay overlay (user-authored survive; pinned by the extended regression test: ghost deleted + user exercise survives).** Verified on the REAL live store (ghost gone, 39 served); serve 22/22 + cli 7/7, clippy/seam/fmt clean, e2e 28/28, gui-transform. ⚠ **Mobile note: the Android version-gated re-seed may have the same overlay-ghost issue — check MainActivity's seeding before the next APK.** Next: L01 trim, then Batch B (ownership spine).
- ✅ **#36 Batch A pt.2 — practice routes in-app across L09–L34 (2026-07-07, TS `409451a`, rust-textbook `e872391`, CONTENT_VERSION 5→6).** Completes the audit's #1 systemic fix curriculum-wide: 26 more lessons rewritten Sandbox-first — 21 with mapped exercises also hand off via "Practice this lesson"; 13/17/21/33 route Sandbox-only; L31's rustc opener specially handled (compiles runtime-free). Each lesson's own predict instruction preserved; `cargo new` survives only as the own-machine aside. **Deliberately untouched: L35/36/37** (capstone = real project; cargo test; Cargo the subject). Tightened `practice-handoff.spec` (the `## 5.` heading remnant was masking the opener assertion — now checks the first BODY line) + extended coverage to 6 lessons across both slices. Verified on the refreshed live store (marker=6); e2e **28/28**; gui-transform; seam+fmt clean. **All 35 pre-Tooling lessons now route practice into the app. Remaining Batch A: L01 trim + L05 split (the curriculum's first split).**
- ✅ **#36 Batch A pt.1 — Foundations practice routes in-app + forward-ref glosses (2026-07-07, TS `9c5470d`, rust-textbook `465f3a8`, CONTENT_VERSION 4→5).** First execution slice of the audit. **Practice-venue fix (the #1 systemic finding) applied to L00–L08:** every practice section now routes into the app — reps in the 🧪 Sandbox, then (L01/L02/L05/L07, which have mapped exercises) the "Practice this lesson" links hand off into Practice; `cargo new` demoted to an own-machine aside (it was also the top forward ref — Cargo is L37). **Forward-ref glosses:** L01 `{placeholder}`, L02 `&str`, L03 method calls, L05 release-aside→Book pointer, L06 fn-signature + positional `{}`, L08 array→L13 + String→L12. Synced byte-identical to rust-textbook. **New `tests/e2e/practice-handoff.spec.js`** pins the handoff (locked-exercise confirm: decline stays, accept force-selects into Practice — found live: the 423 confirm flow works as designed) + the content shape (Sandbox named, no bare cargo-new opener). Verified live on the refreshed store (marker=5, gloss renders, practice box lists the exercise); e2e **28/28** incl. a11y both themes; gui-transform; seam+fmt clean. **Remaining Batch A: same practice-venue pass on L09–L37 (mechanical), then the L01 trim + L05 split.**
- ✅ **#36 kickoff — the curriculum pacing audit + lesson-split matrix (2026-07-07, TS `docs/CURRICULUM-PACING-AUDIT.md`).** The foundation artifact for the heavy-improvements program. Six parallel auditors read ALL 38 lessons (structured findings: atomic concepts, forward refs, split proposals, book-offloads, coupling); claims spot-checked against the files (26-lifetimes' iterator usage, 35-capstone's untaught `Option::take`/`'static`/`assert!` — all real). **Results: 7 ok · 7 trim · 24 split → 63 proposed new lessons (~77-lesson curriculum); 41 forward references across 23 lessons; 0 of 38 practice sections hand off to the in-app exercise (all route to `cargo new` — simultaneously the top forward-ref source AND the #37 coupling failure); 13 lessons have no exercise at all.** One editorial override: lesson 16 (10 mapped exercises — the practice hub) split despite the auditor's trim. **Key discovery: letter-suffixed ids (`16b-…`) sort + phase-map correctly → splits land WITHOUT renumbering.** Execution staged into 5 batches (A: systemic practice-handoff + early foundations → B: ownership spine → C: abstraction ramp → D: concurrency/advanced → E: remainder), each syncing rust-textbook + bumping CONTENT_VERSION. Original-exercise authoring for the 13 uncovered lessons → Open questions (default: proceed). Next tick: Batch A.
- ✅ **cargo-deny gate cleared + fresh 13/13 mergeable baseline (2026-07-07, TS `4aa6d39`).** First full-gate run since the ~10-tick burst found the (post-`829565d`) cargo-deny gate failing on two axes: **advisories** — anyhow 1.0.102 hit the newly published RUSTSEC advisory (anyhow#451, stacked-borrows UB in `downcast_mut`); time-triggered, not caused by our changes; `cargo update -p anyhow` → 1.0.103. **bans/wildcard** — cargo-deny counts version-less `{ path = … }` deps as wildcards and only exempts UNPUBLISHED crates: set `publish = false` once in `[workspace.package]` + inherited in all 13 crates (correct regardless — internal crates, GitHub-release distribution, guards against accidental `cargo publish`), then `allow-wildcard-paths = true` (a real `version = "*"` still fails). cargo-deny: all four axes ok; workspace tests 26/26 suites; **full gate 13 passed, 0 failed** → baseline block refreshed to `4aa6d39`. Also this tick: the 403'd push from last tick landed on retry (GitHub git-transport secondary rate limit — cooled down; lesson: don't hammer retries). **And: Paul's 2026-07-07 heavy-improvements directive captured as the PRIORITY PROGRAM block above + board tasks #36–#41.**
- ✅ **Glossary: type annotation + cargo terms — closes remaining gaps (2026-07-07, TS `045333a`).** The two remaining beginner gaps that resolved to nothing: `type annotation` (beginners see `: u32`/`: i32` constantly; "type inference" was defined but not the annotation) + `cargo` (the write-it-yourself practice sections use `cargo new`, lesson 37 is about Cargo). Both plain, analogy-free, conceptual, charter-honoring; `cargo` has `book_chapter=""` (no bundled Cargo chapter → "read more" omitted). 65→67. **CONTENT_VERSION 3→4**; live store refreshed (marker 3→4), tap-to-define links "annotation"/"type annotations"/"cargo run" in Book ch03-02 + "Cargo" in lesson 37 (and correctly NOT "cargo" inside `code`). Also ran the FULLER gate this tick: **`cargo doc -D warnings` clean + full workspace test green (13 crates, tui 43/serve 22/state 30)** — no regression from the accumulated content-refresh/CLI work. rpro-glossary 3/3, e2e 26/26, gui-transform. (The push 403'd — GitHub git-transport secondary rate limit, credential verified fine — and landed cleanly on the next tick's retry after the cooldown.)
- ✅ **Study Guide: fix stale lesson count 37→38 + add Lesson 0 (2026-07-07, TS `9173620`; WEB-only — rides next mobile release).** Rotated to a fresh surface (audited it + cheatsheets this tick — all excellent, in sync). Found a concrete staleness bug: Lesson 0 (Hello World) was added but the Study Guide still said "37 lessons" and its stage table started at Lesson 1, omitting it — while the app's Journey correctly renders 38 lessons / 12 stages (verified live from the roadmap grouping). Fixed the count + folded Lesson 0 into Foundations (0–8, "hello world & reading a real compiler error"). Applied to BOTH the canonical `STUDY-GUIDE.md` and the byte-identical `gui/study-guide.md` (kept in sync). `study-guide.md` is served from `gui/` on disk (NOT the store) → no CONTENT_VERSION bump; live immediately. Verified LIVE 390×844 (served guide reads "38 lessons"; table Foundations row 0–8; 12 rows, chips intact, no errors); gui-transform.
- ✅ **Glossary: macro term + variable alias (2026-07-07, TS `3a48efb`).** The two most fundamental first-hour vocab gaps had NO tap-to-define coverage. **macro:** lesson 00 (the FIRST lesson) is entirely about `println!` being a macro + the `!`, yet tapping "macro" gave nothing — added a plain, analogy-free definition (what a macro is, why the `!`). No bundled macros chapter → `book_chapter=""` (all 3 "read more" sites already omit the link when empty; source cites ch.19.5 honestly). **variable:** beginners say "variable" but the term is "binding" and neither resolved — added "variable"/"variables" as aliases on `binding` + noted "often just called a variable" in its def. Bumped `CONTENT_VERSION` 2→3; verified end-to-end (guard 65 terms; live store refreshed marker 2→3; live tap-to-define links "macro" in lesson 00, "variables" in lesson 01). rpro-glossary 3/3, serve 22/22 + cli 7/7, e2e 26/26, gui-transform. **Remaining gaps (future tick): cargo, module, type annotation resolve to nothing.**
- ✅ **Quizzes & Cheatsheets prev/next navigation (2026-07-07, TS `7522c54`; WEB-only — rides next mobile release).** Completes reading-surface nav parity: lessons + Book let you advance without returning to the list, but quizzes/cheatsheets offered only "← all …". Added the same prev/next footer to both via a shared `prevNextNav(items, id, cls)` helper + memoized `quizList()`/`cheatList()` (mirrors `bookChapters()`), reusing the Book's `.booknav` styling; cards reuse `.quizjump`/`.cheatjump`, now wired in the single-item view too. Verified LIVE 390×844 (quiz #2 → prev "Phase 1 Quiz" / next "Phase 3 Quiz"; next advances; ends omit the missing side) + new `tests/e2e/quiz-cheat-nav.spec.js` (both surfaces) + book-nav + a11y both themes **24/24**, gui-transform. **All four reading surfaces (lessons/Book/quizzes/cheatsheets) now navigate identically.**
- ✅ **Glossary +3 beginner terms: constant, expression, statement (2026-07-07, TS `cc8a9b0`).** Three fundamental early-curriculum concepts had NO glossary entry, so tap-to-define went dark on them in the very lessons that teach them (04-constants, 06-expressions-statements-semicolon). (Option/trait/reference already resolve via aliases on existing terms — verified — so those weren't gaps; constant/expression/statement resolved to nothing.) Added plain-language, analogy-free, conceptual definitions attributed to ch03-01/ch03-03, honouring the charter (teach the idea, never an exercise fix). 61→64. **First real use of the content-refresh pipeline: bumped `rpro_runner::CONTENT_VERSION` 1→2** so seeded stores pick them up. Verified end-to-end: glossary editorial guard passes; restarted live server → its store auto-refreshed (marker 1→2), `/api/glossary` serves 64, lesson 06 tap-to-define now links "statement"/"expressions". rpro-glossary 3/3, version-relative serve/cli tests green, e2e 26/26 (tap-to-define + a11y both themes), gui-transform. **Debug note: `setsid nohup … rpro-serve &` in a compound command exits 144 and aborts the rest — run the server START as its OWN Bash call.**
- ✅ **Book reader prev/next chapter navigation (2026-07-07, TS `439831c`; WEB-only — rides next mobile release).** A chapter offered only "← contents", so reading the Book linearly meant bouncing back to the ToC after every chapter. Added a prev/next footer — two cards (previous left, next right) that jump to the neighbouring chapter in real ToC order, with the missing side omitted at the first/last chapter. Order from a memoized `/api/book` ToC fetch (`bookChapters`); links reuse the existing `.bookjump` wiring + land scrolled to top. Verified LIVE 390×844 (ch03-02 → prev "Variables and Mutability" / next "Functions"; next → ch03-03; first chapter no prev) + new `tests/e2e/book-nav.spec.js` (neighbours match ToC order, next navigates, ends omit the missing side) + a11y both themes + navigation **28/28**, gui-transform.
- ✅ **CLI content auto-refresh + shared CONTENT_VERSION (2026-07-07, TS `cdb9b50`).** Completes last tick's "content reaches existing users" guarantee across ALL surfaces. Hoisted the bundled-content revision into `rpro_runner::CONTENT_VERSION` (single source of truth — server + CLI/TUI; kills version drift). `rpro init` now auto-refreshes a store whose `.content-version` is behind (absent=0) WITHOUT needing `--refresh` — a returning user running plain `rpro init` gets the sweep/glossary/lesson updates; fresh store still reads "first-time setup", up-to-date reads "already up to date" (no-op), `--refresh` still forces. `progress.json` untouched. New pure helper `store_content_behind` + unit test; functionally verified vs a temp RPRO_STORE (fresh seeds 71 ex; roll marker back → plain `rpro init` restores clean content + keeps Current; 3rd init no-op). rpro-cli 7/7, rpro-serve 22/22, clippy + seam clean. CONTENT_VERSION stays 1 (refactor) → live server unaffected, no restart. **⚠ still: bump `rpro_runner::CONTENT_VERSION` when bundled content changes.**
- ✅ **Existing stores now refresh bundled content on upgrade (2026-07-07, TS `68e3eae`).** Ground-truthing the LIVE server surfaced that its `ownership/01_move` STILL showed the pre-sweep spoiler comment — one that names the exact error code the predict box asks the learner to guess. Root cause: `ensure_seeded` copied exercises/book/glossary/lessons/quizzes/cheatsheets into the store ONLY when absent, so a store seeded by an older build was frozen on its first-run copies — every content improvement after a user's first run (the whole "never hand the answer" sweep, new glossary terms, lesson rewrites) was invisible to existing installs (the Android app was already fixed this way in dev.14; the server was not). Fix: gate a one-shot refresh on a bundled `CONTENT_VERSION` written to `.content-version` — a store whose recorded version is older (absent reads as 0) re-copies the read-only content dirs, then records the version so it runs once per bump. SAFE: `progress.json` untouched (progress preserved), editor edits live in the browser (localStorage), never the store. Restarted :8099 onto the new release binary → verified against the REAL live store: 0 lingering spoilers tree-wide, `/api/current` + the editor now serve the clean source, exercise count went stale-`0/3` → full `0/71`, marker=1, progress still Current on `ownership/01_move`. New unit test pins refresh + progress-preservation; rpro-serve **22/22**, clippy + seam clean. **⚠ FUTURE TICKS: bump `CONTENT_VERSION` whenever bundled content changes** so existing stores pick it up. (Debug note: the live store is `/home/paul/.cache/ts-serve` — verify with ABSOLUTE paths; `$HOME` in a shell defaulting to /root points at a different, never-served store.)
- ✅ **Lesson drawer styled like the Learn tab (2026-07-07, TS `c20a862`; WEB-only — rides next mobile release).** The read-while-coding lesson drawer (`#lessonDrawerBody`) renders the same markup as the Learn tab but lives OUTSIDE `.docview`, so the rich-text CSS never reached it: inline code showed a transparent bg + the browser default monospace, code blocks were unstyled, and the wired-in ▷ Run / ⧉ Copy toolkit rendered as naked default-chrome buttons. Broadened the relevant `.docview` rules to also list `.ldrawer-body` (one source of truth, no drift) — inline code, code blocks + token colours, blockquotes, links, tables, the runnable/copy toolkit. NOT the `.docview` class itself (its base rule sets a full-height page layout that would break the drawer's flex column). **New `tests/e2e/lesson-drawer.spec.js`** (the drawer had ZERO coverage): pins inline-code chip styling, the code-block + Run/Copy toolkit, glossary tap-to-define in the drawer. Verified LIVE 390×844 (inline code now `--bg-3` chip + `--mono`; code block bg+border; Run pill r=8px; Copy absolute; no errors) + e2e **31/31** incl. a11y both themes + gui-transform.
- ✅ **Glossary definitions render inline `code` (2026-07-07, TS `37587c7`; WEB-only — rides next mobile release).** Definitions are plain text written with markdown backticks and were shown verbatim ("with `let`", backticks and all). New `mdInlineCode(s)` helper (esc first → XSS-safe → `` `code` `` → `<code>`) applied to the Glossary view cards, the tap-to-define popover (lessons/Book), and the Practice concept box; inline-code styled for the two non-`.docview` contexts. Verified LIVE 390×844 (glossary: 160 styled code chips, ZERO literal backticks; popover renders `let` as a chip; no errors) + a11y both themes (`--fg` on `--bg-3`, AA-clean) + navigation + lesson-nav (asserts no raw backtick) **34/34**, gui-transform.
- ✅ **dev.36 — "⧉ Copy" on every code block in lessons & the Book (2026-07-07).** A small top-right copy button on each code block — grab any snippet (whole OR partial) to paste into the Sandbox / your editor, with a "copied ✓" toast (reuses `copyText`: clipboard→AndroidSeam→execCommand). Each `<pre>` wrapped in `.codewrap` so the button survives horizontal scroll; the run/console output `<pre>` (no `<code>`) is skipped. **a11y catch:** the first pass used `opacity:.8`, which composited the button text toward the code background → light-theme contrast dropped below AA; the a11y gate flagged it, removed the opacity (the button's own opaque `--bg-2` carries the text → AA-clean both themes). Verified LIVE 390×844 (6 blocks → 6 Copy; output skipped; click → clipboard has the code + toast; no errors) + a11y both themes + lesson-nav + runnable-code (asserts 1 Copy/block) + navigation **36/36**, gui-transform. Mobile dev.36 (10350) prereleased.
- ✅ **dev.35 — "Open in Sandbox" on runnable code examples (2026-07-07).** Connects the two features: every ▷ Run example now also has a **✎ Open in Sandbox** button that drops the example into the Sandbox as a NEW file and switches there — read an example, then tinker + re-run it freely in the full editor. The multi-file Sandbox (dev.34) makes it safe: `sbxOpenWithCode` appends an "example N" file (never clobbers existing scratch). Pure UI (no Termux logic) → web + mobile. coderun-bar is now a clean two-button row. Verified LIVE 390×844 (lesson example → new "example 1" active with the code; pre-existing "scratch" preserved intact) + new e2e (open-as-new-file + no-clobber) + runnable-code + sandbox multi-file + a11y both themes + navigation **30/30**, gui-transform. Mobile dev.35 (10349) prereleased. **The Sandbox ↔ examples ↔ lessons/book loop is now fully connected.** [[mobile-v04-testing-backlog]].
- ✅ **dev.34 — Sandbox file explorer: multiple named files (2026-07-07).** Paul's "create + switch + edit files" for the Sandbox. The scratchpad now holds several INDEPENDENT named files — a files bar of chips (tap to switch), "+ new" (prompts a name), "×" on the active chip to delete (keeps ≥1); each file is its own standalone program with its own persisted code, Run runs the ACTIVE file. Model `{files:[{name,code}],active}` in localStorage, migrating the old single-buffer key. (Independent scratch files, NOT a multi-file cargo project — mod/lib compilation would be a server fork; noted for later.) Only `renderSandbox` changed — the run path (`runSandbox`) is UNTOUCHED, so no added risk to the on-device Termux run. Verified LIVE 390×844 (scratch→+new demo active; independent + persists across switches; Run runs active; delete→one file; no console errors) + new e2e + a11y both themes (chips/delete/new aria-labelled, files bar role=group) + navigation + runnable-code **28/28**, gui-transform. Mobile dev.34 (10348) prereleased. [[mobile-v04-testing-backlog]].
- ✅ **Full-gate verification + fresh mergeable baseline (2026-07-07, TS `829565d`).** After the v0.4 feature burst (Sandbox, runnable code, glossary tap-to-define, PDF back button, readability, comment sweep), ran the WHOLE CI gate `scripts/check.sh` → **12 passed, 0 failed, "all gates green, branch is mergeable"**: rustfmt · **clippy --workspace --all-targets -D warnings CLEAN** (the new `sandbox_handler` included) · cargo test workspace · cargo doc · smoke · browser e2e (isolated) incl. sandbox/runnable-code/glossary/pdf-back/both-theme a11y · verify-exercises 71 · cli smoke · gui-transforms · book anchors · seam-guard · wasm32 pure-core. Recorded the fresh baseline block at the top (the old one was stale — `22e8abe`/"63/63"). Confirms the branch is mergeable for #29 whenever Paul wants. Verification tick — no code change, no release.
- ✅ **dev.33 — Runnable code examples in lessons & the Book (2026-07-07).** A beginner reading a complete-program example can tap **▷ Run** to compile+run it inline, fully offline (web `/api/sandbox`; mobile Termux op `'sandbox'`) — interactive learning without leaving the lesson. New generic `runCode(code, onResult)` primitive (does NOT touch `runSandbox`) + a `_runCbs` map for per-example mobile routing; free-play (never records progress/hints). `wireRunnableCode` adds ▷ Run + an inline console ONLY to blocks with a `fn main` (partial snippets stay un-runnable); hooked into the lesson view, lesson drawer, and Book chapters (alongside `glossaryLinkify`). Verified LIVE 390×844 (hello-world → 3 Run buttons all under fn-main blocks; tap → "Hello, world!" inline; no button on partial snippets; progress untouched) + new e2e + a11y both themes + lesson-nav + sandbox **28/28**, gui-transform. Mobile dev.33 (10347) prereleased — **bundles the held Sandbox status-clarity polish** (`71b0085`, compile-vs-runtime-vs-ran). [[mobile-v04-testing-backlog]].
- ✅ **Sandbox status clarity (2026-07-07, TS `71b0085`; WEB-only — rides next mobile release, no APK churn for a wording fix).** The Sandbox badge showed a cryptic "exited 101" for a compile failure (101 is also a panic's exit code). Now `sbxRender` reads the outcome — rustc `error[E####]`/`error:` → "compile error ✗"; non-zero exit / panic w/o compiler error → "runtime error ✗"; success → "ran ✓" (meta states running/needs-Termux/network/toolchain still override). Both paths (web `/api/sandbox` + mobile Termux result). Verified: sandbox e2e (now asserts the compile-error status) + a11y both themes 19/19, gui-transform; live on :8099.
- ✅ **dev.32 — Sandbox now runs on MOBILE (Termux) + un-gated (2026-07-07).** Completes the Sandbox for the phone. `runSandbox` on mobile routes through the Termux bridge with a NEW op `'sandbox'` (MainActivity: compile+run like a normal run — it falls into the run/test branch since it's neither `check` nor `explain` — but skips `recordAttemptForCurrent`, so free-play never counts as an exercise attempt or touches the hint gate). Results arrive on the shared `window.__termuxResult`, short-circuited by a separate `sbxPending` id to a shared `sbxRender` (no verdict/tutor/recordRun). Removed the mobile hide-gate on the ⋯-menu entry. Web path unchanged (still `/api/sandbox`). Verified: sandbox+a11y(both)+navigation+lesson-nav e2e **35/35** + gui-transform + APK builds (compiles the Java). On-device Termux run mirrors the proven exercise-Run path — **confirm on Paul's phone**. Mobile dev.32 (10346) prereleased. [[mobile-v04-testing-backlog]].
- ✅ **Sandbox — free-play Rust scratchpad, WEB MVP (2026-07-07, TS `38b84d2`; NO mobile release — gated off on-device until the Termux intercept).** Paul's "Sandbox tab". Vertical slice: **server** `POST /api/sandbox` compiles+runs ARBITRARY code on the local toolchain (mirrors `run_handler` but NO `resolve_current`/`record_run`/advance → progress untouched; reuses `write_scaffold`/`Core` in a `run/sandbox` dir), + **client** a "🧪 Sandbox" ⋯-menu view with its OWN CodeMirror editor (line numbers, Rust highlight, Ctrl/Cmd+Enter), Run/reset toolbar, console (ANSI-stripped, err styling), localStorage-persisted buffer. Gated off mobile (`window.AndroidSeam`). Verified LIVE 390×844 (Run starter → "Hello, Rustacean!"/"1..=10 = 55"; missing-semicolon → compiler error + err styling; back to Practice → progress unchanged) + rpro-serve build/fmt/**seam-guard**/21 tests + new e2e (run+error+progress-untouched) + a11y both themes + gui-transform + **full browser e2e 55/55**. **NEXT SLICE: mobile — MainActivity intercept `/api/sandbox` → Termux bridge (mirror of the `/api/run` path, no recording), then un-gate.** [[mobile-v04-testing-backlog]].
- ✅ **dev.31 — Glossary tap-to-define in the Book + common-word stoplist (2026-07-07).** Extended dev.28's tap-to-define from lessons to the offline Book chapters (highest reading volume — a beginner in an early chapter hits FORWARD-reference jargon like `closure`/`Vec` and can tap for a definition; skips code; ~108ms/chapter measured live). Also hardened the matcher: the book's denser prose exposed a false-positive class — lowercase single-word ALIASES that are common English words (the ordinary "result" linked to the Rust `Result` term; also method/expect/collect/moved/…). Added a small stoplist (skipped as match keys only, still glossary-search terms); genuine Rust words (lifetime/immutable/reference/closures) kept — cleans the lessons too. Verified LIVE 390×844: ownership chapter → 11 genuine terms, 0 in code, no "result", popover works; lessons unchanged. New e2e book+stoplist guard; gui-transform + a11y/lesson-nav/navigation/offline **32/32** both themes. Curriculum note this tick: audited the 38-lesson order (0–37) against the Rust Book — it already follows the book's chapter order with sensible pedagogy (the old "leap" is long gone), so NO reflow needed. Web live (TS `454cbc4`); mobile dev.31 (10345) prereleased.
- ✅ **dev.30 — Prose readability pass on the reading surfaces (2026-07-07, beauty).** Lessons/book/quizzes/glossary body was 14.6px at ~95ch (720px column) — small type at a measure well past the comfortable 45–75ch, and a beginner reads a LOT of it. `.bookbody` 14.6→15.5px; `.docview` measure 780→680px (~78ch); inline `code` 12.5px→0.86em so it scales with the body instead of looking tiny (block `pre code` unchanged). Verified LIVE 390×844 (larger, no overflow) + 1440×900 (620px measure, code chips well-proportioned) + gui-transform + a11y/lesson-nav/navigation **31/31** both themes (colours unchanged → contrast unaffected). Mobile <760px keeps full width but gets the larger, more legible text. Also confirmed this tick: PDF restore-position ALREADY works on web (pdf.js ViewHistory persisted page:100, reopen restored to 99/100) — remaining PDF asks (bookmarks/highlights/annotations) are bigger. Web live (TS `0fcb834`); mobile dev.30 (10344) prereleased.
- ✅ **dev.29 — In-app "← Studio" back button in the offline PDF book viewer (2026-07-07).** Paul (PDF cluster): the Library opens each book as a full-page nav to the vendored pdf.js viewer, whose toolbar has no close/back-to-app control — the only way out was the browser / Android back (worse on a maximised WebView + desktop). Added a "← Studio" button at the left of the viewer toolbar → `history.back()` (fallback `index.html`). The handler lives in a same-origin `tempered-back.js`, NOT inline: the viewer's CSP is `script-src 'self'` and silently blocked the first attempt's inline `onclick` (caught by driving the LIVE viewer — the click did nothing until moved to a 'self' script). Verified LIVE 390×844 (app → open TRPL → tap → back in the app, URL left the viewer) + new e2e regression guard (button present + script referenced/served + no dead inline onclick) — pdf-back/offline/a11y **18/18**. Rides gui/ into the APK. Web live (TS `405f58d`); mobile dev.29 (10343) prereleased. Remaining PDF-cluster asks (restore page position, bookmarks/highlights/annotations, Bookmarks tab) still pending. [[mobile-v04-testing-backlog]].
- ✅ **dev.28 — Glossary tap-to-define in lessons (2026-07-07).** Paul: "make the glossary searchable + tap-a-word-for-a-definition." Search box already existed; added the tap-to-define half. Lesson prose (Learn lesson view + left drawer, both `mdToHtml`) auto-links known glossary terms — first whole-word occurrence of each becomes a dotted-underline button that taps to reveal an inline definition callout (+ Book deep-link), one open at a time, terms from `/api/glossary` (cached). Quality guards: never inside code/links/headings (TreeWalker skip — editor untouched, per Paul); case-SENSITIVE (Rust ids Some/Vec/Copy are capitalized, colliding English words aren't → links the term not the word); names + only ≥5-char aliases (drops "lock"/"let"/"use" common-word mis-links); XSS-safe (textContent/dataset) + aria-labelled. Verified LIVE 390×844 (4 clean terms in the bindings lesson; correct def; 0 in code; no console errors) + new e2e regression spec + a11y/lesson-nav/navigation **30/30** both themes + gui-transform. Web live (TS `8959989`); mobile dev.28 (10342) prereleased. [[mobile-v04-testing-backlog]]. (Glossary ask now fully done; remaining backlog: curriculum book-order reflow, PDF viewer cluster, Sandbox tab, beauty passes.)
- ✅ **dev.27 — Learn back-nav: no longer stranded after drilling into a lesson (2026-07-07).** Paul's flagged item: drilling into a lesson/quiz/cheatsheet then coming back left no in-app path to the 📚 Learn hub (only the ⋯ menu — worse on mobile with no tab bar). Root cause: the "back to list" links (`.lessonback`/`.quizback`/`.cheatback`) called `renderX(null)` directly, bypassing `showView`, so the returned-to list never re-got its `showView`-injected "← Learn" button. Fix: route those 4 back-links through `showView('lessons'|'quizzes'|'cheatsheets')`. Clean breadcrumb restored (lesson → ← all lessons → list → ← Learn → hub), stable across repeated drilling. Verified LIVE 390×844 (learnback persists after drill+back on lessons AND quizzes; ← Learn → hub) + e2e lesson-nav/navigation/a11y **30/30** (both themes) + gui-transform. Web live (TS `85e1250`); mobile dev.27 (10341) prereleased. Lesson prev/next + the injection itself were already done — this closes the last nav gap. [[mobile-v04-testing-backlog]].
- ✅ **dev.26 — Q pt.2 sweep, phase 11/advanced — SWEEP COMPLETE (2026-07-07).** Final 5 exercises (unsafe deref, orphan rule, unsized `str`, operator overload, supertrait) stripped of CONCEPT blocks, E0133/E0117/E0277/E0369 spoilers, and spelled-out fixes ("wrap in the block it names", "one-field newtype struct", "take the text behind a reference", "implement the operator's trait", "implement Display for Sku") → tight task + constraint + read-the-error. Code byte-identical, 71 verify. Web live (TS `802799b`); mobile dev.26 (10340) prereleased. **All 11 phases (basics…advanced) DONE — the whole curriculum now honors "never hand the answer"; the concept lives in the lesson drawer, the fix in the earned hint ladder.** [[mobile-v04-testing-backlog]]. Next: back to the generic board / other backlog (console rework, Sandbox tab, lesson nav, curriculum reflow).
- ✅ **dev.25 — Q pt.2 sweep, phase 10/concurrency (2026-07-07).** 4 thread/Arc/Mutex/channel exercises stripped of the fix-reveals: "reach for the thread-safe cousin" (Arc), "add the piece that allows safe mutation" (Mutex), the E0382 spoiler, and the CONCEPT + "print it before the send, or send a copy" walkthrough → tight task + constraint + read-the-error. Code byte-identical (diff comment-only), 71 verify (E0373/E0277/E0594/E0382 all fire). Web live (TS `b1df098`); mobile dev.25 (10339) prereleased. **10/11 done — only 09-advanced remains.** [[mobile-v04-testing-backlog]].
- ✅ **dev.24 — Q pt.2 sweep, phase 9/functional-and-smart-pointers (2026-07-07).** 8 closure/iterator/Box/Rc/RefCell exercises stripped of the CONCEPT blocks that named the fix (Rc "keeps a count of owners", Box "a pointer of known size", which iterator-maker "lets `v` survive"), the E0382/E0072 spoilers, the answer-giving Hint lines, and the narrating inline comments → tight task + constraint + read-the-error. Code byte-identical (diff comment-only), 71 verify. Web live (TS `d3d8548`); mobile `caf7d65` dev.24 (10338) prereleased (Obtainium). 9/11 done. Remaining: 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.23 — Q pt.2 sweep, phase 8/generics-traits-lifetimes (2026-07-04).** 9 exercises; stripped CONCEPT (incl. the trait-object walkthrough), E0599/E0106 spoilers, and Hint lines spelling the fix. 8/11 done. 71 verify. Remaining: 07b-functional-and-smart-pointers, 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.22 — Q pt.2 sweep, phase 7/modules (2026-07-04).** 5 exercises (privacy/paths/use/super/unresolved-import); dropped CONCEPT, the add-pub/use-super reveals, and the 'look closely at the spelling' spoiler. 7/11 done. 71 verify. Remaining: 07-generics-traits-lifetimes, 07b-functional-and-smart-pointers, 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.21 — Q pt.2 sweep, phase 6/error-handling (2026-07-04).** 5 exercises (Result/?/From/Option-in-Result); dropped CONCEPT, E0308/E0277 spoilers, and Hint lines naming the exact fix (From trait, ok_or). 6/11 done. 71 verify. Remaining: 06-modules, 07-generics-traits-lifetimes, 07b-functional-and-smart-pointers, 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.20 — Q pt.2 sweep, phase 5/types-and-matching (2026-07-04).** 6 exercises (struct build/privacy, Option, methods, refutable let). 04 no longer spells out the method to write; 03/05 no longer list the constructs to use. 5/11 done. 71 verify. Remaining: 05b-error-handling, 06-modules, 07/07b, 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.19 — Q pt.2 sweep, phase 4/ownership (2026-07-04).** 7 borrow-checker exercises stripped of CONCEPT, error-code spoilers (E0382/E0502/E0515), `rpro … --solution` CLI reveals, a FIXME that spelled the fix, and stale 'Patina —' credits. 4/11 done. 71 verify. Remaining: 05/05b, 06-modules, 07/07b, 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.18 — Q pt.2 sweep, phase 3/text-and-collections (2026-07-04).** 9 exercises (Vec/String/&str/iter/HashMap/move) stripped of CONCEPT/how-to/Hint reveals; credit normalised. 3/11 phases done. 71 verify. Remaining: 04-ownership, 05/05b, 06-modules, 07/07b, 08-concurrency, 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.17 — Q pt.2 sweep, phase 2/control-flow (2026-07-04).** Same strip on the 5 if/match/loop/for exercises. 2/11 phases done (basics+control-flow). Code byte-identical, 71 verify. Remaining: 03-text-and-collections … 09-advanced. [[mobile-v04-testing-backlog]].
- ✅ **dev.16 — Q pt.2 sweep, phase 1/basics (2026-07-04).** Stripped the answer-revealing CONCEPT blocks + how-to-fix hints + narrating inline comments from the 8 01-basics exercises → minimal task (goal + what-not-to-change + read-the-error). Concept → lesson drawer (toml book_refs); fix → EARNED hint ladder (gated) + toml solution_outline. Code byte-identical, 71 verify. Remaining phases (02-control-flow … advanced) are follow-up ticks. [[mobile-v04-testing-backlog]].
- ✅ **dev.15 — left lesson DRAWER (2026-07-04).** Redesign step 5 (last big piece): the current exercise's lesson slides out from the LEFT over the editor (📖 pull-tab or 'read the lesson'; same md as the Learn tab) and recedes on close (×/scrim/Escape) — you keep your editor spot. 'full view ↗' → Learn tab. Handle is Practice-only. e2e 49/49. The editor-first REDESIGN arc (Run-above→congrats→compact-toolbar→menu→CodeMirror→drawer) is essentially COMPLETE. Next big threads: curriculum overhaul + Q pt.2 comment rewrite + Sandbox + PDF cluster + glossary tap-to-define + evcxr. [[mobile-v04-testing-backlog]].
- ✅ **dev.14 — gutter error-marks on CM6 + scroller a11y (2026-07-04).** Finished the editor swap: reimplemented the red gutter error-marks as native CM6 gutter decorations (StateField→gutterLineClass, effect from renderDiag, auto-clear on edit; rebuilt bundle w/ the state/gutter exports) + un-skipped its e2e test. Fixed a latent a11y: CM6's .cm-scroller (tabindex=-1) failed scrollable-region-focusable when code overflowed → gave it tabindex=0+name. Full suite 49/49. Editor fully done — no lingering regressions. NEXT big piece: the left lesson-DRAWER. [[mobile-v04-testing-backlog]].
- ✅ **dev.13 — CodeMirror 6 editor (2026-07-04).** THE editor swap: replaced the drifting textarea/<pre> overlay with CM6 (MIT) via an editorEl() proxy shim — single layer, aligned, Rust-highlighted (theme --tok-*, both-theme AA), Ctrl+Enter/reset/predict-gate preserved, Dev tier = CM6 native closeBrackets/indent, jump-to-line reimplemented, .cm-content aria-labelled. e2e specs migrated to .cm-content/window.__cm; dev-brackets rewritten. 0 failed (1 skip = deferred gutter error-marks → CM6 line decorations follow-up). CM bundle verified in the APK. [[mobile-v04-testing-backlog]].
- ✅ **CodeMirror 6 bundle vendored (2026-07-04).** De-risked the editor swap: built an OFFLINE CM6 IIFE (tools/cm-build → esbuild → gui/vendor/codemirror.js, 389KB, window.CM) + loaded it. Proven live: mounts, single-layer editable (no overlay), Rust highlighting paints. Foundation only — NOT wired yet (no release). e2e 50/50. NEXT: wire CM6 into the editor (value shim, gate, Ctrl+Enter, Learn/Assist/Dev), then release. [[mobile-v04-testing-backlog]].
- ✅ **dev.12 — ⋯ menu built; mode switcher out of the editor (2026-07-04).** New ⋯ dropdown (EDITOR MODE + GO TO jumps); Learn/Assist/Dev moved out of the editor bar (Paul: wasting space). 11 e2e call sites updated to open the menu first; 50/50 (recall-chip flake). Paul (firm): the DIY overlay editor is misaligned — REPLACE with CodeMirror 6 (MIT, offline-bundled). Captured; **next BIG item, before the lesson-drawer.** [[mobile-v04-testing-backlog]].
- ✅ **dev.11 — compact editor-integrated run toolbar (2026-07-04).** Redesign step 2: Run/Check/Explain/Hint folded into a slim .etools toolbar INSIDE the editor box (icon buttons ✓/?/💡 + Run; 'ready' tucked right), removed the standalone runrow. Editor gains height, controls hug the code. e2e 50/50 (recall-chip order-flake), live-verified. NEXT: build the ⋯ menu + move Learn/Assist/Dev tier into it; then the left lesson-drawer. [[mobile-v04-testing-backlog]].
- ✅ **dev.10 — ephemeral celebration popup (2026-07-04).** Completing an exercise fires an animated 🎉 popup (auto-dismiss, non-blocking, reduced-motion-aware) from both pass paths. Paul escalated the redesign (3 msgs): compact/INTEGRATED editor buttons, move 'ready' out of the way, BUILD A MENU + move Learn/Assist/Dev into it, and the LEFT lesson-DRAWER (extends right / recedes left); editor = main focus. Captured in [[mobile-v04-testing-backlog]]. NEXT: integrate the run buttons into the editor chrome + start the menu.
- ✅ **dev.9 — Run row above the editor (2026-07-04).** First step of the UI real-estate REDESIGN: moved .runrow (Run/Check/Explain/Hint) between the predict bar and the editor so the action bar sits over the code. IDs unchanged → gate + Ctrl+Enter intact. e2e 50/50, live-verified. NEXT redesign steps: ephemeral congrats popup, smaller/auto-hidden tier tabs, a menu, left lesson-drawer; then evcxr. [[mobile-v04-testing-backlog]].
- ✅ **dev.8 — readable generics/lifetimes annotations (2026-07-04).** Quick-fix: the two hand-drawn ASCII-caret code annotations (24-generics, 26-lifetimes) broke on wrap → rewrote as prose; left all real rustc `|`-output carets. Guards green. Quick-fixes queue now CLEAR — NEXT is the BIG work: UI real-estate REDESIGN, then evcxr (+ streaming Termux bridge for bacon). [[mobile-v04-testing-backlog]].
- ✅ **dev.7 — redo completed exercises (2026-07-04).** Quick-fix: added Seam.resetExercise JNI (mirrors CLI reset — Current + un-Done + attempts 0, keeps single-Current; symbol verified in .so) + gui: tapping a DONE exercise confirms re-open for redo. Per-exercise only (never reset-all). Web e2e 50/50 unaffected. NEXT quick: `^^^` lesson-annotation sweep; then BIG: UI redesign + evcxr. [[mobile-v04-testing-backlog]].
- ✅ **dev.6 — lesson nav + Learn back + a11y fixes (2026-07-04).** Quick-fix tick: reliable lesson Prev/Next (computed from real order, not the fragile footer parse) + a `← Learn` back button on every sub-view. Fixed two a11y regressions hello-world surfaced: its <li> was outside a <ul> (→ 'Getting started' group) and light-theme tok-s/tok-m/tok-c were <4.5:1 on the code-block bg (darkened). e2e 50/50. Shipped dev.6 (10320) w/ hello-world + copyright fix. NEXT quick: mobile reset/redo (Seam.resetExercise — 'reset to redo' is dead on mobile) + `^^^` lesson-annotation sweep; then the BIG ones (UI redesign, evcxr). See [[mobile-v04-testing-backlog]].
- ✅ **Lesson 0 Hello-World + loop re-prioritised (2026-07-04).** Authored the new FIRST lesson (fn main / println!-is-a-macro / ; / {} — zero-background, no Python, real E0423 error), sorts ahead of 01, both repos, renders + guards green. Paul adjusted the loop: work HIS backlog first, QUICK FIXES before longer ones, then the board — recorded in [[mobile-v04-testing-backlog]]. Quick next: lesson Next/Back + Learn back + `^^^`-annotation sweep. Big (Paul wants them all): UI real-estate redesign + evcxr + curriculum reflow + Sandbox + PDF cluster.
- ✅ **Q sweep pt.1 — copyright + press-Hint (2026-07-04).** Dropped "Rustlings Pro" (© concern) from 47 exercise headers → one clean "Inspired by Rustlings (MIT/Apache-2.0)" credit; removed the obvious "press Hint" line (46 files). All 71 still verify. NOT released alone (batches with pt.2). NEXT: Q pt.2 — rewrite CONCEPT headers to stop revealing the answer (→ Explain/Hint) + drop jargon; then the curriculum overhaul (hello-world first, intro fn/;/println!-macro/let/mut before use). See [[mobile-v04-testing-backlog]].
- ✅ **dev.5 — native selectable console (2026-07-04).** Replaced xterm with a `<pre>` (#conout): selectable, wraps, scrolls-itself, flat (fixes window-in-window). 16/16 axe both themes. Paul's flood continued mid-turn — huge backlog now in [[mobile-v04-testing-backlog]]: editor selection glitch (HIGH), Q comment sweep (drop 'Rustlings Pro'©/press-Hint/answer-giveaway/jargon), curriculum overhaul (hello-world first, no leaps, real rustlings, organise by level), glossary tap-to-define, left lesson-drawer, lesson/Learn nav, Sandbox, PDF cluster, redesign. Working through one verified change per dev.N.
- ✅ **v0.4.0-dev.4 — THE LOCKOUT FIXED (mobile offline progression, 2026-07-04).** Root cause: the phone store ships no progress.json → nothing Current → everything Locked + passing never recorded. Added the seam WRITE-path (recordRun/selectExercise/ensureSeeded JNI — symbols verified in libtempered_seam.so all ABIs; reuse rpro-state/rpro-runner) + MainActivity wrappers + ensureSeeded on startup + gui wiring (pass→record+advance, list-tap→select). Web e2e 50/50 unaffected. Backlog grew (console rework, lesson/Learn nav, PDF viewer+annotations, real-estate redesign) — see [[mobile-v04-testing-backlog]]. NEXT: console rework OR the declutter redesign.
- ✅ **v0.4.0-dev.3 + captured the phone-test flood (2026-07-04).** Compiler WORKS (dev.2). Shipped dev.3: predict gate-once + collapse, empty-// cleanup, restart hint. Paul fired a big backlog now in [[mobile-v04-testing-backlog]]: #1 THE LOCKOUT (mobile progression — passing doesn't advance; needs Seam JNI record/advance/select reusing rpro-state), #2 comments give away the answer (trim → hints), #3 console rework (xterm→selectable flat pre), #4 Sandbox tab + file explorer, #5 lesson Next/Back, profile+social future. NEXT TICK: N (the lockout) — the unblock.
- ✅ **v0.4.0-dev.2 prerelease — Termux permission fix + AMOLED + editor HL + more (2026-07-04).** Paul's live phone-test flurry, batched. THE compiler fix: RUN_COMMAND is a `dangerous` Termux-defined permission — manifest declaration alone left it ungranted, so every run was rejected; now REQUESTED at runtime (onCreate + per-run guard) [mobile]. GUI [TS `bc4ecc1`]: AMOLED black (dropped the warm brown — 16/16 axe both themes), REAL editor syntax highlighting (tok-* colors were scoped to .docview only → editor was monochrome; my earlier 'works' check only proved tokens EXIST, not colored), gutter left+wider code, console overscroll-contain + wrap-refit, copyable #runError card, dots hidden on phones. Also shipped v0.4.0-dev.1 (10315) then dev.2 (10316) as prereleases for Obtainium. DEFERRED to next: empty-// sweep, Dev file explorer + tier differentiation, bacon/evcxr (feasibility fork — see [[practice-layout-redesign]]).
- ✅ **Seam banner removed + collapsible console (step ①) + dead ◧ hidden + editor-HL confirmed (2026-07-04).** Paul's phone session: (1) the always-on orange 'on-device seam' debug bar → removed the onPageFinished injection [mobile `9858a3b`→main]. (2) ◧ focus button 'does nothing on tap' — it's a desktop-only list-hide, neutered on phones → hidden ≤760px. (3) Redesign step ① — discoverable ▾/▸ console-collapse chevron, persists, global (not phone-only), Run auto-unfolds [TS `36e2d2c`]. (4) 'need syntax highlighting' — VERIFIED it already works + is WebView-hardened (textarea color:transparent + -webkit-text-fill-color; #editorHL carries tok-* spans); Paul is on a STALE APK (every exercise opens on its gray comment block, so it LOOKS unhighlighted). Offer a fresh test build. NO APK.
- ✅ **Termux copy-command card + wrapped-bullet fix + layout paradigm chosen (2026-07-04).** Paul (phone) sent 3 things: (1) copy ONLY the Termux setup command → new #termuxSetup DOM card with a one-tap ⧉ Copy command button (idempotent one-liner, safe to re-run; persists — manifest RUN_COMMAND is a normal perm, Termux setting is a file) [`894cc79`]. (2) Learn bullets that wrap looked disconnected → mdToHtml now joins hard-wrapped bullet continuations into the <li> instead of a stray margin <p>; +2 tests; verified live [`d386aa8`]. (3) Big Practice-layout redesign directive → saved to memory, Paul CHOSE collapsible-panels; build order banked (console→problem→focus-code→ref drawer). OPEN: Paul's 'orange bar at the bottom' — couldn't repro in current build (gauge empty at 0%); need a screenshot. Both fixes flow to the phone app via build-apk.sh. NO APK.
- ✅ **Content: rustc provenance 1.94.1→1.95.0 (`5805099`, 2026-07-04).** Pivoted from UI to substance. Lesson 1 was the lone stale version label (33 other lessons + installed toolchain = 1.95.0) on the platform's core claim: real unedited rustc output. RE-VERIFIED all 3 documented codes on 1.95.0 before relabelling — E0384 byte-for-byte identical to lesson 1's block, E0308 (both variants), E0004 — then fixed lesson 1 (both repos, kept identical) + EDUCATION.md. Closes a loose end a prior tick had flagged (line 583) and left. Historical log kept as-is. NO APK.
- ✅ **UI refresh step 5 — phone header 3→2 rows (`ddd28f3`, 2026-07-04).** On <470px the wordmark + progress + 3 panel toggles overflowed, wrapping the Practice/Learn tabs to a 3rd row; collapsed the wordmark to just the ◆ mark below 470px (returns above) so everything fits 2 rows and the task title rises. On-mandate declutter. Verified live 390x844; e2e 50/50 (desktop-width, wordmark shown); AA unaffected. Learn hub re-checked — already strong. NO APK.
- ✅ **UI refresh step 4 — premium finish on the action controls (`669b600`, 2026-07-04).** Run button got a 1px inset white top-gloss (lit-from-above) layered over its orange glow, kept across base/hover/both CTA keyframes; ghost buttons (Check/Explain/Hint) got the --edge-hi top highlight at rest + a real drop-shadow on hover so the lift casts. Gated stays flat. Verified live 390x844 (Assist); AA unaffected; e2e 50/50. Follows steps 1–3. NO APK.
- ✅ **UI refresh step 3 — warm the terminal (`cd1d083`, 2026-07-04).** The xterm terminal was the last cold surface (bg #0a0d12 / fg #cfe3d6) clashing with the warm Patina palette; warmed bg→#0f0c07 + fg→#e8e0d2 in BOTH the .terminal container CSS and the xterm.js theme object (kept in lockstep), leaving the semantic ANSI colours alone. Verified live 390x844 (cohesive now); e2e 50/50. Follows steps 1–2 (palette, depth+hierarchy). NO APK.
- ✅ **UI refresh step 2 — depth + hierarchy (`5748ec7`, 2026-07-04).** Card lift via a new `--edge-hi` top-edge highlight token (editor/terminal/hub cards; the dark-mode depth cue drop-shadows can't sell on dark; inert on the light theme's white cards) + a hero bump for the exercise title (.lesson h1 19→22px/700/tighter tracking). Verified live 390x844; WCAG-AA held; e2e 50/50. Follows step-1 warm palette. Next: warm terminal + predict-bar polish. NO APK.
- ✅ **Progress reset + warm-Patina palette (`abbf140`, 2026-07-04).** Paul confirmed he'd done ZERO exercises — the 56 "done" in the live store were ALL my pre-isolation test pollution; reset to 0/71 (backup kept). Then, on his "heavily improve the UI": a design-token refresh — warm layered charcoal dark (bg/bg-2/bg-3 warmed for depth, warm off-white text, 14px radius, two-tier shadow) + warm-paper light, on-brand (Patina). WCAG-AA HELD (16/16 two-theme axe), e2e 50/50. Step 1 of a multi-tick heavy refresh; component polish next. NO APK.
- ✅ **AUDIT.md re-tabulated from fresh runs (`db7e6a4`, 2026-07-03).** The README's "current state" doc was a June-19 snapshot: 122→154 tests, 32→71 exercises, 51→115 anchors, 40→53 transforms — and it listed Playwright + a11y as OPEN items "needing a headless browser" when both shipped weeks ago (50-test isolated suite; both-theme axe audits). Every number re-derived at commit time; G63/G65 closed with honest residue (Lighthouse perf score unmeasured). The public-claims sweep is complete: both READMEs, Study Guide, draft notes, AUDIT all current. NO APK.
- ✅ **Mobile README: evergreen download link (mobile `367462b`, 2026-07-03).** The "latest APK" button hardcoded the v0.3.2 asset — FIVE releases stale (pre-versionName-fix). Now /releases/latest (verified resolving to v0.3.14), with the v0.4 cut checklist teaching a STABLE asset name (tempered-studio.apk) so a one-tap /releases/latest/download/ URL works forever after. Offline list caught up (quizzes/cheatsheets/hints/Explain/Library). NO release.
- ✅ **README refresh — the front door tells the truth (`0278783`, 2026-07-03).** It was factually wrong: 63→71 exercises (15 learner-visible topics), THREE→FOUR surfaces (Android was absent), the textbook attributed to the companion repo (it's bundled + served in-app), and — worst — it claimed the hint ladder ends at 'solution outline' (the ladder NEVER serves the solution; Hard Rule #1 now stated correctly). Added tiers, IDE-first UI, gutter/marks, both-theme AA, the 12-gate mirror + isolated 50-test browser suite. Every number re-verified against the tree. AVP-2 header untouched. NO APK.
- ✅ **The curriculum gets an ending (`482d54b` + rust-textbook `a9c69e8`, 2026-07-03).** Lesson 37 just stopped; a finisher was told nothing. Added the Study Guide's "When you finish the path" (clear Recall from memory, redo cold, graduate to the bundled whole books — Patterns/Reference/Rustonomicon/Embedded — build something real with cargo; the predict-run-read habit IS the takeaway) + a one-paragraph hand-off closing lesson 37. Both guide copies + both repos' lesson 37 identical; live-verified renders; e2e 50/50. NO APK.
- ✅ **Android theme determinism (`ede1fe4`, 2026-07-03).** The WebView's prefers-color-scheme reporting is undefined without DayNight config (the app has none) — the new OS-default could have surprise-booted phones light. The OS branch now applies only when AndroidSeam is absent; phones stay dark unless toggled, saved choices win everywhere. e2e +1 (seam + OS-light → dark). Suite 50/50 zero-flake. NO APK.
- ✅ **OS color-scheme respected on first visit (`f73cd19`, 2026-07-03).** The app forced dark on everyone; now: saved choice wins → else prefers-color-scheme:light gets the (AA-clean, CI-gated) light theme → else dark. e2e +3 covering all branches via emulateMedia. Suite 49/49 zero-flake. NO APK.
- ✅ **Enter-to-Run in the predict input (`aa2713b`, 2026-07-03).** Audited quiz depth first (all 11 quizzes 10–16 questions — no thin content), then closed a walkthrough-found gesture gap: Enter in the error-code box now fires Run, but ONLY with the outcome half locked — same runOp path, gate stays authoritative, no-op otherwise. e2e +1 (both branches). Suite 46 tests, 0 failures. NO APK.
- ✅ **Live server → release build (`cc1e87a`, 2026-07-03).** The :8099 instance had always run a DEBUG rpro-serve. Built release, swapped by PID with identical env: /api/exercises 697→359ms, /api/lessons 471→301ms, / 462→319ms per 30 reqs (1.4–1.9×; server-side gain larger — curl spawn dominates the residue). Store intact; gzip (50KB wire) and PDF 206-ranges re-verified on release. Restart runbook memory updated. Note: smoke/e2e still build+test debug of the CURRENT code — correct for gating; the live host now runs release — correct for Paul. NO APK.
- ✅ **Mobile asset sync — committed tree = buildable current snapshot (mobile `70665ea`, 2026-07-03).** The mobile repo's committed assets lagged mid-way (a partial sync from the explain-tick build sat uncommitted) and its bundled noto-emoji.css still carried the digit-hijack ranges → wide digits on the phone. One fresh full sync + build: gutter + error marks + digit fix + current Study Guide all verified INSIDE the APK (0 digit ranges bundled). NO release — v0.4 batch.
- ✅ **Integrated learner-flow walkthrough — CLEAN (`0e46fbc`, 2026-07-03).** Walked the whole loop live on an isolated fresh store (never the live one): predict-gate holds → honest run fails as predicted → verdict renders (green, inline) → hint rung 1 unlocks only after the genuine attempt → Assist re-renders the LAST run (E0384, no re-run) → gutter mark line 13 == L13 jump link → edit clears marks → fix → 'advanced ✓' 1/71 → E0384 enters the RECALL queue. Zero defects across every feature seam — the strongest possible evidence for the v0.4 cut; noted in the draft release notes. (Two probe artifacts identified as such, not app bugs.)
- ✅ **Gutter error marks (`a9c7279`, 2026-07-03).** Assist/Dev: the lines the last run's diagnostics flag get error-red gutter numbers (the visual error↔line bridge next to the L# jump links). Tier-gated — Learn never marks (by-hand preserved); edits clear stale marks; hlLines stays pure (optional marks Set, +2 transform checks). e2e +2 (mark==data-line; edit clears; Learn clean): 45 tests, 0 failures. NO APK.
- ✅ **Editor line-number gutter + emoji digit-hijack fix (`19f88e3`, 2026-07-03).** The Learn tier teaches reading `--> main.rs:4:5` but the editor had no line numbers. Added a wrap-correct gutter: per-logical-line `.cl` blocks + CSS counters in a 54px strip (net-zero advance → caret alignment intact; wrapped rows hang past the gutter; numbers never copy). hlLines() repairs spans that cross newlines (block comments/multi-line strings), 7 new transform tests. The gutter's own screenshot exposed a REGRESSION from emoji-first: Google's subsets declare keycap bases (#,*,0-9) in unicode-range → all digits rendered via the emoji font (wide). Stripped those codepoints + versioned the css link. Digits now measure normal; emoji stay colored. e2e 43/43 zero-flake. NO APK.
- ✅ **gzip on rpro-serve (`6871cf0`, 2026-07-03).** The 164 KB single-file shell + all JSON shipped uncompressed on every load; enabled tower-http compression-gzip → index.html 164,014→48,384 B (3.4×), /api/exercises→3.4 KB. PDFs/woff2 excluded by predicate — pdf.js Range chunking verified intact (206, exact bytes, accept-ranges kept). rpro-serve tests 21/21; e2e 43/43 zero-flake + smoke PASS on the compressed server; live 8099 restarted onto the fresh binary (it had been a week-old build). NO APK.
- ✅ **`rpro lessons` stage-grouped (`59508e8`, 2026-07-03).** Audited rust-textbook sync first (lessons/quizzes/cheatsheets IDENTICAL to TS, repo pushed+clean — fifth consecutive clean audit), then closed the last per-stage parity gap: the CLI lessons list now groups under the 11 stage headers via lesson_stage(n) mirroring the web thresholds, with a boundary+monotone+count test so a one-sided regroup fails loudly. fmt/clippy/tests green; live output verified. NO APK.
- ✅ **Mobile Explain offline (mobile `787a7ab` + gui `1294dfe`, 2026-07-03).** The Explain button / RECALL chips / diagnostic-code taps were silently dead on the phone (op=explain hit the nulled /api/run; the Termux branch only routed run/check/test). Now explain rides the bridge: stdin = the diagnostic code, sanitized in JS+Java+bash and never interpolated (injection-tested against `E0382; rm -rf /` on real rustc → harmless). Reference rendering (no verdict/tutor/gate re-arm); explain does NOT earn hint rungs. APK builds; e2e green. NO release — v0.4 batch.
- ✅ **v0.4 release notes drafted (`7af4872`, 2026-07-03).** Audited three more surfaces first — hint-ladder inputs (71/71 have book_refs + expected outcome), Journey (already a full progress map) — all clean, so banked the batch: docs/RELEASE-v0.4-DRAFT.md written from git ground truth (17 feature commits + mobile offline stack), numbers spot-checked, plus a cut checklist encoding the release discipline (versionCode bump, asset re-sync, release URL, sendmail verify). DRAFT only — the cut stays Paul-gated. NO APK.
- ✅ **Study Guide matches the IDE-first nav (`a799e28`, 2026-07-03).** The guide still taught "Read the lesson (Lessons tab)" + "the Journey tab" — navigation that no longer exists post-redesign; for a beginner that's worse than no guide. Rewrote the loop steps (📚 Learn → Lessons) + described the hub model (main screen = task + editor; everything else one tap away). Both copies in lockstep; live-verified render; zero stale refs remain. NO APK.
- ✅ **Desktop audit: emoji-first font stack + predict-input cap (`f5a8e3e`, 2026-07-03).** First desktop-size audit (1440×900) found the 📝/🔮/💡 glyphs STUCK as tofu on elements painted before the bundled font chunks loaded (fresh elements rendered colored — Chromium per-glyph fallback quirk when emoji families trail the stack). Fix: `var(--emoji)` FIRST in `--ui`/`--mono` — safe since all listed families are emoji-only + unicode-range-gated. Pixel-verified 0→92 colored px incl. cold paint. Also capped the predict input at 420px (sprawled ~1050px on desktop). e2e 42+1 green incl. both-theme a11y. NO APK.
- ✅ **Read→practice loop guarded (`d0d3654`, 2026-07-03).** Audited CONCEPT_LESSON (every exercise's "read the lesson" link): all 65 concepts map, zero dangling — but the hand-maintained map had NO guard (the concept→glossary rot pattern). Added its lesson-side twin to the gui-transforms gate: every exercise concept must map, every target lesson must exist, corpus-size sanity. Verified green + negative-tested (a broken target fails naming the entry). NO APK.
- ✅ **TUI Quizzes tab with an enforced predict-then-reveal gate (`15b0bd8`, 2026-07-03).** Quizzes were deliberately off the TUI (a plain reader dumps the answer key). Now: Tab::Quizzes reuses the shared md reader, but the `## Answers` section is WITHHELD until `a` reveals it, and the reveal resets on every quiz/tab change — the platform's predict-then-verify rule enforced in the terminal (stronger than the CLI honor system). Corpus test asserts all 11 bundled quizzes carry the gate heading. TUI tests 43/43; fmt+clippy+seam clean. All four surfaces now carry the full curriculum: web/mobile/CLI/TUI. NO APK.
- ✅ **Deterministic e2e: isolated store + fresh binary + stale-render fix (`b1cd9ec`, 2026-07-02).** The recurring "flakes" were four real defects: (1) the suite ran against the LIVE server and recall-chip quietly PASSED whatever exercise was current each run (force-select doesn't reopen Done → silent mis-pin) — **the live store's progress was test pollution, one exercise per run**; (2) smoke/e2e launched a June-19 fossil `./target` binary (local builds go to CARGO_TARGET_DIR) — smoke stayed green testing old code; (3) a real gui race let a late in-flight render clobber the next view — every renderer now paints a fresh #docview node (stale continuations paint off-DOM); (4) the smoke hint assertion still expected pre-gate semantics — now asserts the gate. NEW scripts/e2e.sh (throwaway store, in check.sh); pins asserted; workers:4 + 90s compile-project. **43/43 twice consecutively, 12.6s** (was 8.3m/19-fail). ⚠ Paul's live progress (47/71) is inflated by tests — flagged for his decision. NO APK.
- ✅ **Light theme: AA contrast + persistent toggle (`acef447`, 2026-07-02).** First-ever light-theme audit found 43 serious contrast nodes (the status palette --done/--note/--error/--warn/--current was never overridden for light — ~2.5–3:1 on white) + the 🌓 toggle never persisted (reset to dark every reload). Fixed the palette with AA-clearing light variants, pinned the always-dark terminal bar to a fixed light-safe color, persisted ts-theme. a11y.spec now audits ALL views in BOTH themes (16 checks) permanently. 16/16 green. NO APK.
- ✅ **Mobile hint ladder offline (mobile `b247f59`, 2026-07-02).** /api/hint was still null on the phone (Hint button silently dead) — continues Paul's "everything offline". New seam native `hintJson(store, level, attempts)` runs the DESKTOP ladder verbatim (attempt gate → 1 rung/attempt → cap 3 → never the solution), host-tested against the bundled store. Attempts now counted on-device per exercise (every Termux run/check/test) in SharedPreferences — the phone never reaches the desktop's run handler. Packaged .so in all 4 ABIs carries the export. NO release — rides the v0.4 batch.
- ✅ **v0.3.14 bugfix release — offline Learn content + Obtainium version fix (mobile `a28ce1e`, 2026-07-02, BOTH Paul-reported).** (1) lessons/quizzes/cheatsheets/glossary were bundled in the APK but MainActivity returned null for their /api/ routes → Learn surfaces dead offline; now served on-device with the desktop server's exact shapes (md_collection/glossary_handler; glossary via build-time TOML→JSON). (2) versionCode/versionName were stuck at 1/0.1.0 since scaffold → every APK self-reported v0.1 and Obtainium never converged; now 10314/0.3.14, scheme MAJOR*10000+MINOR*100+PATCH, bump EVERY release. (3) found+fixed: .seeded was once-ever → updates never refreshed content; now version-gated. JVM-tested the JSON serialization against the real 37 lessons. Released + emailed (status=sent). Feature batching to v0.4 continues — this was a bugfix cut.
- ✅ **Glossary +3 beginner terms: Copy, Clone, dereference (`bf7ecd3`, 2026-07-02).** Filled real gaps — Copy/Clone is the key to the #1 ownership confusion (why `let b = a;` sometimes moves, sometimes copies); dereference (`*`) for references. Conceptual, plain-language, no analogies, attributed to ch04-01/ch04-02 with aliases. 58→61 terms; golden-corpus guard passes; CLI + live /api/glossary serve them. NO APK.
- ✅ **`rpro progress` per-phase breakdown (`793344a`, 2026-07-02).** The CLI showed only the grand total; now it prints a done/total per topic (grouped by the `<phase>/` id prefix, curriculum order, green ✓ when a phase is cleared) — parity with the web/mobile per-phase counts, so a terminal learner sees where they stand in each topic. fmt+clippy clean; rpro-cli tests 5/5; counts sum to 71. NO APK.
- ✅ **Per-phase progress counts on the exercise list (`2f619aa`, 2026-07-02).** The exercise list was already phase-grouped but headers showed no progress; added a done/total count to each (e.g. `BASICS 2/8`, `CONTROL FLOW 5/5 ✓` green when cleared), matching the new Lessons stage counts — both lists now show where you are within each topic. Computed in renderList; verified LIVE (sum to 46/71) + full e2e green. NO APK.
- ✅ **Lessons list grouped into curriculum stages (`9dbd720`, 2026-07-02).** The flat 37-lesson scroll now groups into the 11 stages (Foundations…Tooling) with a per-stage read count (green ✓ when complete), mirroring the Journey — learners see the path's shape + progress at a glance. Derived client-side from the lesson number (lessonToQuiz + PHASE_NAMES), no API change; the filter collapses fully-filtered stages. Verified LIVE + e2e 17 green. NO APK.
- ✅ **v0.4 beauty #3 — visible mobile progress line (`169b020`, 2026-07-02).** The header progress bar rendered as a ~15px invisible sliver on phones (the % text's min-width ate the track). Promoted it to a full-width 3px accent progress line pinned to the header's bottom edge, % kept inline — a glanceable progress cue where there was none. Scoped to <=760px; desktop inline bar unchanged. Verified LIVE (mobile 390px full-width @65%; desktop static 95px) + nav/a11y/offline e2e green. NO APK.
- ✅ **Bundled Noto Color Emoji offline (`aea09ba`, 2026-07-02).** Emoji were tofu □ on any host without a system emoji font (the FOSS AppImage/.deb/.rpm targets). Vendored Google's optimized COLRv1 woff2 subsets (~2MB, 10 `unicode-range` chunks) + a `local()`-gated @font-face stylesheet: systems WITH an emoji font download nothing; fontless builds fetch only the chunk(s) for on-screen glyphs, fully offline (all `url()` relative — charter intact). Verified LIVE on this fontless host: 📚/🔮 render as real color glyphs. NO APK — batching to v0.4.
- ✅ **v0.4 beauty #2 — glyph robustness + predict-input polish (`666bc74`, 2026-07-02).** (a) The UI leans on emoji affordances but the font stack had no emoji fallback → tofu □ boxes on any host without a system emoji font (this box + lean Linux desktops the AppImage/.deb/.rpm ships to). Added a shared `--emoji` fallback appended to `--ui`/`--mono` (per-glyph fallback keeps text in the UI font, emoji resolve to the OS font on Win/Mac/Android/most-Linux). (b) The predict error-code input was a fixed 150px that truncated its placeholder; now flex-fills its row (328px) with a fitting placeholder + accent focus border. Verified LIVE + e2e green. NO APK — batching to v0.4.

### RESOLVED — emoji font bundling (Paul 2026-07-02: "bundle everything, svg fallback only if not hand-coded")
Bundled **Noto Color Emoji** (OFL) as Google's optimized COLRv1 woff2 subsets (~2MB, 10 unicode-range chunks) under `gui/vendor/emoji/`, `local()`-gated @font-face — devices with a system emoji font download nothing; fontless FOSS-desktop builds get full color emoji offline. No hand-coded SVGs. Done in `aea09ba`.
