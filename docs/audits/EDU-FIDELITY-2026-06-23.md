# Educational-fidelity audit — 2026-06-23

Does Tempered Studio actually *teach* the way the charter + the bundled textbooks
intend? Driven against the **live** system: web gui (Playwright, screenshotted +
read), plus the CLI/TUI parity verified across prior loop ticks. Scope: the core
learner loop on `basics/01_immutable_assign` (the seeded first exercise) and the
cross-cutting surfaces (Book, modes, hint ladder, diagnostics).

Method: ran the real flow — load → **lock a prediction** → Run (real `cargo`) →
read the real error → observe the Tutor/Diagnostics/Book panels → Book tab.

## Verdict

The **core pedagogy is faithfully implemented and works live.** The gaps are not
in the engine — they are missing *content surfaces*, and one of them is exactly
what Paul just asked for (built-in term definitions).

## Strengths — charter held (verified live)

- **Predict-then-run, by hand.** The exercise leads with "Predict first: will this
  compile? which error code? what one-word `help:` fix? Lock your guess, then press
  Run — you'll read the *real* compiler output, by hand." Locking a prediction is
  enforced in Learn mode before Run does anything.
- **Real compiler errors, surfaced honestly.** After Run, Diagnostics showed
  **`E0384` "cannot assign twice to immutable variable `count`"** (clickable →
  Explain) and the raw `rustc` text in the embedded terminal ("try `rustc --explain
  E0384`"). No paraphrase-instead-of-error.
- **Never hand the answer.** The Tutor panel states plainly: *"I guide — I never
  type your fix. Press Hint to climb the ladder."* The hint ladder is gated (no hint
  until a genuine attempt), escalates one rung per attempt, and never serves the
  stored `solution_outline` — verified across web/CLI/TUI in prior ticks.
- **Book-pointers.** Every exercise shows its book refs (here ch03-01 variables-and-
  mutability) with a one-line *why*, and an offline Book reader (TOC + full-text
  search + bundled chapters) is one tab away.
- **Baby-steps + soft-gating.** The exercise list shows later exercises `[locked]`;
  they unlock sequentially (with an explicit jump-ahead). Difficulty ramps by phase
  (guarded by the golden-corpus test).
- **Reads → writes + real-time feedback.** Subheader frames "reads → writes"; the
  learner edits the starter and Runs; attempts are tracked ("1 try") and a pass
  marks Done + advances. Live terminal = immediate feedback.
- **Three surfaces, one contract.** Web/CLI/TUI share the same hint text, force-
  attempt gate, and escalation (closed across recent ticks).

## Gaps — prioritized

### G1 — Built-in term definitions / glossary: ABSENT  **[HIGH — Paul's explicit ask]**
`/api/current` returns `concept: "mutability"` as a bare *tag*; nowhere does the UI
define the Rust jargon a no-background beginner hits ("binding", "immutable",
"move", "borrow", "trait", "lifetime", "scope"…). The charter learner has "no coding
background" and must learn **offline, no LLM** — so there is no fallback for an
unknown word. This is the single highest-value *unblocked* content feature, and it
maps directly to Paul's "built-in terms with simple explanations, pulled from the
textbooks." → **task #24** (curated term→plain-explanation map, synthesized from the
bundled Book/CR and attributed; surfaced as tap/hover defs in exercise prose +
lessons, and a CLI/TUI lookup).

### G2 — Learn/Assist/Dev tiers under-differentiated  **[HIGH]**
The three modes currently differ **only** by the Learn predict-gate. Assist's
promised "inline diagnostics" is not implemented — diagnostics render in a side
panel identically in all three modes, never *inline at the error line*. The tiers
don't yet deliver the "progressive assistance" Paul decided on. → **task #18**.

### G3 — Lessons exist only for Phase 1  **[MED — blocked on Paul]**
The "textbook" half is thin: only Phase 1 (L1–L8) lessons exist; Phases 2–9 are
matrix DRAFT. Blocked on the matrix review + Lesson# sign-off (Paul) and, for the
worked examples, on chapter bundling. → **tasks #16/#17**.

### G4 — Content interactivity is exercise-only  **[MED]**
Exercises are strongly interactive (predict→run→read→fix), but the Book reader and
(future) lessons are read-only prose — runnable listings link *out* to the live
Book. Paul wants the **content** interactive: lessons should embed predict→run
snippets and inline term defs, not just text. → folded into **#17 + #24**.

### G5 — Functional/advanced exercise coverage thin  **[MED — blocked]**
`07b-functional` has a single iterators exercise; closures/smart-pointers/async are
absent (their book_refs need un-bundled chapters). → **tasks #14 → #15**.

### G6 — Editor pane is cramped  **[LOW]**
The starter's CONCEPT comment is clipped in the default editor height; the code area
is short. Cosmetic, but the framing comments are pedagogy — worth a taller pane.

## Recommendation

The engine is sound; invest in **content surfaces**, leading with the one that is
both unblocked and explicitly requested: **G1 / the built-in glossary (#24)**.
After that, G2 (tier differentiation, #18) is the next unblocked learner-facing win.
G3/G5 remain gated on Paul's chapter-source + matrix decisions (see "Open questions"
in IMPROVEMENT-LOOP.md).
