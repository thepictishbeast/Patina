# Review Guide — the autonomous batch awaiting your review

Paul — this is a **reading path**, not a status dump (the blow-by-blow is in
`docs/IMPROVEMENT-LOOP.md`). It exists because ~25 autonomous ticks of work now sit unmerged
and the highest-value next steps are **gated on three decisions only you can make**. Reviewing
in the order below should take well under an hour and unblock the rest.

## The three decisions (the actual blockers)
1. **Merge `textbook-integration` → `main`?** Everything below sits on that branch; it has
   **never** merged to `main`. Does it match the pedagogy you want?
2. **Sign off the DRAFT matrix rows 49–62?** The textbook stops at Phase-6 "Organizing"
   *because of the corpus's own "show Paul the matrix first" gate* — rows 1–48 are DECIDED
   (human-reviewed) and became lessons; rows 49–62 are DRAFT (AI-authored: generics, traits,
   lifetimes, closures/iterators, smart pointers, concurrency/async, advanced + capstone, tests,
   cargo). **No lessons get written from them until you review them.** This is the single thing
   that unblocks the back half of the book.
3. **The smaller scope calls:** rust-analyzer IDE (#19, large), platform/distribution (#23,
   mostly env-gated), the `--solution` flag rename, and the clippy *nursery* lint policy (#20).

## Where things live
- **Study corpus** — the `rust-study` repo (`github.com/thepictishbeast/rust-study`), branch
  `main`. The lessons themselves. (On `main`, but unreviewed per the gate above.)
- **The app** — this repo (Tempered-Studio), branch **`textbook-integration`** (never merged).

## Suggested reading path (~45 min)
1. **Learner's-eye view first:** `rust-study/STUDY-GUIDE.md` — the reading order a student
   follows (Phases 1–6, Lessons 1–23, with quizzes/cheatsheets/kata linked in sequence).
2. **Judge the pedagogy on one hard lesson:** read `rust-study/lessons/15-ownership-and-moves.md`
   end to end — it's the centerpiece concept. Check the **7-part shape**: why it exists → the
   idea → tiny examples → real compiler errors → *predict-then-run practice the learner writes* →
   what surprised you → sources. If this lesson is right, the format is right.
3. **Confirm "never hand the answer":** open `rust-study/katas/likes.md` (a spec + hint ladder,
   **no solution**) and the part-5 of any lesson (the practice is always the learner's to write).
   This is Hard Rule #1 from `CLAUDE.md` — verify it held.
4. **The gate artifact:** skim `rust-study/mapping/concept-matrix.md`. Rows **1–48 = DECIDED**
   (what the 23 lessons were built from, one row per concept with the chosen explanation/example/
   error). Rows **49–62 = DRAFT** — this is decision #2. Reviewing these rows is the unblock.
5. **The app, briefly:** the editor's three tiers (Learn blocks autocomplete/auto-fix and the
   parsed-error panel — "read errors by hand"; Assist/Dev get parsed diagnostics + jump-to-line),
   the 54 exercises across 11 phases under `exercises/`, and the 56-term offline glossary
   (`glossary/glossary.toml`).

## What's in the batch (inventory)
- **Textbook (rust-study):** 23 lessons (L1–L23) covering Phase 1 Foundations → Phase 6
  "Organizing" (modules/paths/`use`); 6 review quizzes (phase1–6); 6 cheatsheets (phase1–6);
  the `likes` capstone kata; the `STUDY-GUIDE.md` index.
- **App (this branch):** Learn/Assist/Dev editor tiers + tier-gated diagnostics + diagnostic→
  jump-to-line + Ctrl/Cmd+Enter run; 54 verified exercises (11 phase dirs, incl. error-handling
  and modules); 56-term offline glossary served at `/api/glossary`; book serving; the
  language-seam architecture (rustc-specific parsing isolated in `crates/languages/`).

## Decisions made autonomously — please validate
- Teaches **fully offline** — no internet, no required LLM (BYO-LLM optional only).
- **Never hands the answer:** guide via predict-then-run + *real* compiler errors + book-pointers.
- **No analogies to other languages** (your explicit instruction) — applied throughout.
- **Baby-steps, reads→writes**, phone-friendly lesson length.
- The **matrix DECIDED/DRAFT status** — not the phase number — gates what gets written.

## How to trust it (verification done)
- **Every** lesson/quiz/cheatsheet code snippet was compiled (and runnable ones run) on
  **rustc 1.95.0, edition 2024**; the deliberate failures show the **real** compiler output
  (error code + message), captured live, never paraphrased.
- `cargo audit`: **0 vulnerabilities** (236 deps). Default `cargo clippy`: **clean** (remaining
  warnings are opt-in pedantic/nursery only — see decision #3 on nursery policy).
- Spot-check anything: paste a lesson snippet into the editor, or `rustc --edition 2024 file.rs`,
  and compare to what the lesson claims.

---
*Maintained by the autonomous loop. When in doubt, `docs/IMPROVEMENT-LOOP.md` has the full
per-tick history and the live "Open questions for Paul" list.*
