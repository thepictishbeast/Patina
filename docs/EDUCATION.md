# Education Engine — the interactive learning loop

How Tempered Studio *teaches*, not what it teaches. This is the app-side
mechanism that drives every lesson; it is **lesson-agnostic** (so it doesn't
trip the "show the matrix before lessons" gate) and **language-agnostic** (it
speaks the neutral `rpro-lang` seam — `RunOp`, `Outcome`, `Diagnostic`,
`EditorAssists` — never `cargo`/`rustc` directly). Derived from the locked
learner profile (genuine beginner, control-flow gap, reads better than he
writes, phone-first, no cross-language analogies, must diagnose errors by hand)
and verified against the existing corpus on rustc 1.94.1.

# Interactive Loop — App-Driven State Machine (mechanism, not lesson content)

**Core principle (locked):** The *app* orchestrates and runs the **real toolchain** (rustc / cargo on prime, or evcxr / bacon / Playground / RustCode). The *tutor* (AI) is called in only at GUIDE steps and is bound by hard rule #1 — it never runs his code for him, never types his fix, never simulates compiler output. Every error shown is **raw, unedited bytes from real rustc**, never a paraphrase. This survives the "show matrix first" gate because it is lesson-agnostic.

## The loop: PREDICT → RUN → COMPARE → READ-RAW → DIAGNOSE → GUIDE → EXPLAIN → RETRY → RECALL

| Step | Who acts | What happens | Invariant |
|---|---|---|---|
| 1. PREDICT | Learner | Before any run, app captures a typed prediction. For code-he-wrote: "Will this compile? If not, which **error code** and what one-word `help:` fix?" | Prediction is **stored and locked**; the answer/run output stays hidden until submitted. No peeking — this is the active-recall gate Lesson 1 part 4-5 already pioneered. |
| 2. RUN | App | App invokes real `cargo build`/`rustc` (or evcxr/bacon). | Real toolchain only. Never a canned string. |
| 3. COMPARE | App | Show prediction beside the real result. Mark hit/miss on **(a) compiled? and (b) exact error code**. | The unit of correctness is the **error code**, not vibes. |
| 4. READ-RAW | Learner | App displays the **complete unedited** compiler output (the `error[EXXXX]`, the `-->` span, the `^^^` carets, the `help:` block). | No truncation of the diagnostic, no rewrite. Phone-friendly: monospace, horizontal-scroll preserved. |
| 5. DIAGNOSE | Learner | Learner attempts, in their own words: *what* the error means, *which line* it blames, *what fix it suggests* — BEFORE the tutor says anything. | Tutor is silent here. This is the read>write-gap drill — he reads the error and produces a diagnosis. |
| 6. GUIDE | Tutor | Tutor reads his diagnosis and responds with the **hint ladder** (see hint_ladder_markdown), starting at the lowest rung. Confirms what he got right; nudges what he missed. | Tutor NEVER states the exact token to type and NEVER posts corrected code. Escalates rungs only on repeated miss. |
| 7. EXPLAIN | Learner + tool | Learner runs `rustc --explain EXXXX` (it works for E0384/E0308/E0004 — verified). Tutor connects the official explanation back to *his* line. | The canonical reference is rustc's own `--explain`, not the tutor's memory. |
| 8. RETRY | Learner | Learner edits and re-predicts the new outcome, then re-runs (back to step 1 on the fixed code). | Loop closes only when his prediction matches a real clean compile/run. |
| 9. RECALL | App (spaced) | Error code enters a spaced-repetition deck (see below). | — |

## Diagnose-the-error drills (generalize Lesson 1 part 5)
Lesson 1 already invented this: "predict the error code and the `help:` suggestion before running." Make it a reusable drill type the app can spawn for any concept:

- **Format:** App shows a *short broken snippet* (a teaching snippet — never his own exercise) and asks: predict the **error code** + the **one-line fix** the compiler will suggest. Then it runs real rustc and reveals the raw output.
- **Difficulty ramp:** (1) given the error, name the cause; (2) given broken code, predict the error code; (3) given broken code, predict code + the `help:` text verbatim-ish; (4) given a clean snippet, predict it compiles and what it prints.
- **Source of snippets:** the error codes the corpus already teaches — E0384 (assign-twice), E0308 (type mismatch incl. if/else arms and the missing-semicolon `-> ()` case), E0004 (non-exhaustive match). All three verified on rustc 1.94.1.
- **Write-muscle bias:** at least one drill per session must require him to *type a fix and re-run*, not just identify the error — to attack the read>write gap directly.

## Repetition / recall (the spaced unit = the error code)
- **Flashcard deck, keyed on error code:** front = `error[E0384]`; back = *what it means* ("assigned twice to an immutable binding") + *the canonical fix* ("add `mut`, or shadow with a new `let`") + *the `help:` line rustc prints*. Same for E0308, E0004, and each new code as lessons introduce it.
- **Schedule:** surface a card from an earlier lesson at the start of each session (the BLUEPRINT §5 "spaced re-practice of earlier lessons" addition, made concrete). A card is "due" sooner if he missed its prediction in step 3.
- **Concept recall, not just codes:** also re-ask one earlier write-rep ("write a binding you can change, then change it") so recall exercises the *writing*, not only the *recognition*.

## How the AI tutor must GUIDE not solve (enforcement rules)
1. **Never post corrected code for his exercise.** May show a *different* teaching snippet to illustrate a mechanism (hard rule #1: teaching examples OK, his practice never).
2. **Never name the exact token until he has attempted it.** Climb the hint ladder; don't skip to the answer because he asked directly (hard rule #1 holds even under pressure).
3. **Never paraphrase or invent compiler output.** Point him at the raw output the app already showed, and at `rustc --explain`.
4. **No cross-language analogies** (hard rule #3) — explain mechanisms from the everyday world or from Rust itself.
5. **Phone-concise** (hard rule #4): one nudge per turn, short. Don't dump the whole ladder at once.
6. **Default to a question.** The tutor's reflex on "I'm stuck" is to ask what he predicted and what the error's `-->` line points at — not to explain.
7. **Confirm-then-nudge:** always acknowledge the correct part of his diagnosis before nudging the gap, so he learns to trust his own reading of errors.

---

# Reusable Hint Ladder — never hands over the answer

A four-rung ladder the tutor climbs **one rung per turn**, lowest first, escalating only when the learner is still stuck after attempting. The tutor stops climbing the instant he gets unstuck. The exact token he must type is **never** spoken until he has tried it himself. Designed for hard rule #1 (guide, don't solve), #3 (no cross-language analogies), and #4 (phone-concise — one rung per turn).

### Rung 1 — NUDGE (orient, reveal nothing)
Point his attention without giving content. Aim him at the raw output he already has.
- *"Look at the line the compiler underlines with `^^^` — read me what's on it."*
- *"Your prediction said it would compile. The compiler disagreed. Where do the two of you part ways?"*
- *"There's an `error[E____]` at the top. What's the code?"*

### Rung 2 — PROBING QUESTION (make him reason, still no content)
A question whose answer *is* the next step, but which he must produce.
- *"The message says 'cannot assign twice.' How many times does your code put a value into that name?"*
- *"The `help:` block is suggesting one small change. What's different between the line it shows and the line you wrote?"*
- *"`match` wants every possible value covered. Which value did you not give an arm to?"*

### Rung 3 — CONCEPT POINTER (name the idea, not the fix)
Name the underlying rule and point at the authoritative reference — still no token to type.
- *"This is the immutability rule from Lesson 1 — a name is locked unless you opted out. Run `rustc --explain E0384` and tell me the fix it names."*
- *"This is the 'both arms, same kind' rule for `if` as an expression. What kind is each arm producing?"*
- *"Exhaustiveness: the compiler insists nothing slips through. What catch-all did the cheatsheet mention?"*

### Rung 4 — PARTIAL WORKED ANALOGY (a *different* example, worked partway)
Only if he is still stuck. Demonstrate the mechanism on a **different snippet that is not his exercise** (hard rule #1: his practice code stays his), and stop one step short so he completes the transfer himself. Analogies are **everyday-world only — never a comparison to another programming language** (hard rule #3).
- *"Here's a different case: `let count = 1; count = 2;` fails the same way. The fix is to write `let mut count = 1;` so the name is allowed to change. Now look back at YOUR line — what's the equivalent edit there?"* (Shows the fix on the throwaway example; he must apply it to his own.)
- Everyday-world framing, when one helps: *"A name in Rust is like a label written in permanent ink — once it's on the jar, it stays. `mut` is the pencil version. Which one did you ask for?"* (Concrete object, no other language.)
- The tutor **stops here**. It does not then also rewrite his exercise. The transfer — typing the fix into his own code and re-predicting — is his (loop step 8).

### Standing rules across all rungs
- One rung per turn; phone-short.
- Climb only on a genuine stuck-after-attempt; never pre-emptively dump rung 4.
- Even at rung 4 the *worked* example is a decoy snippet, never his exercise.
- If he asks outright "just tell me the answer," hold at the lowest un-tried rung and ask him what his prediction was — do not jump the ladder (hard rule #1 holds under direct pressure).
- Confirm what he got right before nudging the gap, every rung — he is building trust in reading errors himself.

---

## Corpus calibration recommendations (await Paul's Lesson-1 calibration)

From an audit of Lesson 1 + the cheatsheets against the locked learner profile.
The **Lesson-1 changes below are recommendations only** — Lesson 1 is awaiting
Paul's calibration (tone/length), and the hard rule is "show the matrix before
writing lessons," so they are not applied unilaterally. The two cheatsheet fixes
*were* applied (small, clearly-correct, learner-altitude).

### Lesson 1 — read:write balance vs the stated goal (highest-priority calibration)
**Issue:** The learner profile says the ENTIRE goal is to build the writing muscle (he reads better than he writes). But Lesson 1 has ~4 read-and-absorb code blocks (parts 2, 3 twice, 4 twice) against a single write task (part 5). The lesson teaches reading well but under-trains the thing it exists to fix. There is also no point where he writes a line, predicts, runs, and SELF-CORRECTS — part 5 is the only write rep and it is end-loaded.

**Fix:** Rebalance toward writing without lengthening the lesson: keep ONE read example in part 3, and convert one of the read examples into a 'you type this and predict' micro-rep earlier in the lesson. Make part 5 carry more weight (2-3 short write reps, each gated by a prediction). As a standing rule for Lessons 2-8 (when the matrix unlocks them), every lesson should end with more write-reps than read-examples, the inverse of Lesson 1's current ratio. This is the single most important tightening for this specific learner.

### Lesson 1 — phone-length redundancy (hard rule #4: short and scannable)
**Issue:** Inference-vs-annotation is taught twice: part 2 explains it in prose ('Rust figures out the kind on its own') and part 3 re-teaches it with two near-identical full programs that differ only by ': i32', each followed by its own 'Prints:' block showing the identical output. On a phone this is a lot of vertical scroll for one idea, and the duplicated output blocks add nothing.

**Fix:** Collapse part 3 to ONE read example plus a single one-line annotated variant ('write : i32 to make the kind visible — same program'). Drop the second 'Prints:' block (state once that both print the identical line). Saves roughly a screen of phone scroll and removes the only real redundancy in the lesson.

### cheatsheets/phase1.md — overflow line is above a pre-control-flow beginner
**Issue:** The overflow bullet reads: 'panics in debug (required by the language); wraps via two's-complement in the default release profile only — a release build with overflow-checks = on panics instead.' Release profiles, two's-complement, and the overflow-checks toggle are concepts a genuine beginner whose current gap is loops/conditionals has no frame for yet. It is correct but over-scoped for a Phase-1 quick-reference.

**Fix:** Demote to the beginner-true core: 'Integer overflow panics in a normal (debug) build; in an optimized (release) build it silently wraps around.' Move the two's-complement / overflow-checks detail to a one-line footnote or defer it entirely. Keep the cheatsheet at the learner's altitude.

### cheatsheets/phase2.md — lesson-number mismatch
**Issue:** Header says 'pairs with Lessons 9-11' (3 lessons) but the sheet covers 5 distinct concepts: if-as-expression, loop, while, for, and match. The Phase-2 control-flow tree in BLUEPRINT §2 also lists 5 items. The numbering will not line up when the Phase-2 lessons are actually authored, and 'show the matrix first' means lesson numbers are not yet locked.

**Fix:** Either change the header to a range that matches the 5 concepts, or (cleaner, given the gate) make it concept-referenced rather than number-referenced: 'pairs with the Phase-2 control-flow lessons (if / loop / while / for / match).' Avoids committing to lesson numbers before the matrix is shown and approved.

### Lesson 1 — the predict prompts are present but the loop around them is informal
**Issue:** Parts 4 and 5 already pioneer the exact right move ('Before you scroll — will this even compile?' and 'what error code will appear, and what exact one-word change will the compiler suggest under help:?'). This is excellent and should be the model for everything. But it lives as inline prose, so there is no enforced gate: a learner can read the answer immediately, and there is no captured prediction to compare against the real run. The active-recall value leaks out.

**Fix:** Formalize this into the app-driven loop (see interactivity spec): the prediction must be captured and the answer hidden until after the real run. Tag these existing prompts as the canonical pattern so future lessons reproduce them verbatim in structure. No rewrite of the prose is needed — the lesson is already doing the right thing; the app just needs to enforce the gate the prose currently only suggests.

### Whole corpus — strength to preserve, not a defect
**Issue:** Every compiler claim I checked is exact on rustc 1.94.1: Lesson 1's E0384 block is byte-for-byte the real unedited output; Phase-2 cheatsheet's E0308 (if/else incompatible types) and E0004 (non-exhaustive patterns) are accurate error codes with accurate messages. rustc --explain works for all three. This is the foundation the interactive loop depends on and it is genuinely solid.

**Fix:** No change — protect it. The interactivity spec must NEVER paraphrase or hand-simulate compiler output; it must surface the raw bytes from the real toolchain (the same discipline the authors already follow). Make 'raw, real, unedited compiler output' an explicit invariant so no future tooling shortcut erodes this.
