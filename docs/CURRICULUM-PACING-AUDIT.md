# Curriculum Pacing Audit — the lesson-split matrix (2026-07-07)

The foundation artifact for Paul's 2026-07-07 heavy-improvements directive (board **#36/#37**):
every subject in **baby steps**, incrementally, **never jumping ahead**; lessons **short** (depth
belongs in the bundled Book via "read more" links); **more lessons**, not longer ones; and the
learning material and exercises **following along with each other**.

**Method.** Six parallel auditors read all 38 lessons in full (guideline: ~600–900 words teaching
ONE idea = ok; 900–1400 = trim, offload depth to Book refs; >1400 or >2 atomic concepts = split),
returning structured findings per lesson: atomic concepts, forward references (a concept *used*
before the lesson that teaches it), split proposals, offload-to-book passages, and practice-section
coupling. Mechanical companion data (word counts, the `CONCEPT_LESSON` lesson→exercise map) was
extracted separately. Agent claims were spot-checked against the files before adoption.

## Headline numbers

| | |
|---|---|
| Lessons audited | **38** |
| Verdicts | **7 ok · 7 trim · 24 split** |
| Proposed new lessons from splits | **63** → a **~77-lesson** curriculum |
| Lessons with forward references | **23** (41 individual refs) |
| Lessons with NO paired in-app exercise | **13** |
| Practice sections that hand off to the in-app exercise | **0 of 38** (all route to `cargo new`/playground) |

## The two systemic findings (bigger than any single lesson)

1. **Every practice section routes AWAY from the app.** All 38 lessons end in "open a playground
   or `cargo new …`" — even when a matching in-app exercise exists (25 lessons have one). This is
   simultaneously the #1 forward-reference source (`cargo` isn't taught until L37) **and** the #1
   coupling failure (#37). Fix once, systemically: practice sections hand the learner into the
   matching **in-app exercise** (falling back to the Sandbox for free-play), with `cargo new` kept
   only as an "on your own machine" alternative note.
2. **The early lessons quietly lean on untaught basics** — `println!("{x}")` placeholders from L1
   (taught L8), method calls `.len()`/`.trim()` in L3 (methods/strings arrive L7/L12), `&str`
   glosses in L2, `: i32` annotations in L1 before number types (L5). Fix pattern: either teach the
   tiny piece explicitly where first used (a one-line "read this as…" box) or rewrite the example
   to avoid it. The matrix marks each.

## ID strategy — splits land WITHOUT renumbering

Split-out lessons take **letter-suffixed ids** (`16-references…` keeps its slot; the new sibling
becomes `16b-mutable-references…`). Verified against the app's machinery: ids sort lexicographically
(`16-` < `16b-` < `17-`) so list order, prev/next nav, and Continue all keep working, and
`lessonToQuiz(parseInt(id))` still groups `16b` into phase 4 — **no renumber, no mapping rewrite,
no broken links**. A final cosmetic renumber can happen once, at the very end, if Paul wants one.

## The split matrix

Verdicts: **ok** = leave (fix any forward refs only) · **trim** = shorten + Book refs, same lesson
count · **split** = replace with the listed smaller lessons.

| Lesson | Words | Verdict | Concepts | Fwd refs | Becomes |
|---|---|---|---|---|---|
| `00-hello-world` | 879 | ok | 5 | 1 | — |
| ✅ `01-bindings-and-immutability` (trimmed in Batch A `1762b79`, 1214→1095w — row belatedly marked; found unmarked during the pt.9 audit-of-the-audit) | 1136 | trim | 5 | 3 | — |
| `02-mutability` | 695 | ok | 2 | 1 | — |
| `03-shadowing` | 713 | ok | 3 | 1 | — |
| `04-constants` | 643 | ok | 4 | 1 | — |
| ✅ `05-number-types-and-overflow` → `05-scalar-types` + `05b-integer-overflow` (done `5960f71`) | 725 | split | 5 | 1 | → **The scalar types: integers, floats, bool and char**<br>→ **Integer overflow — your first runtime panic** |
| `06-expressions-statements-semicolon` | 682 | ok | 4 | 2 | — |
| `07-functions` | 647 | ok | 5 | 0 | — |
| `08-comments-and-printing` | 712 | ok | 5 | 2 | — |
| ✅ `09-if-else-expressions` → `09-if-else` + `09b-if-as-expression` (done `2a441fc`; sources-comparison meta cut; &str gloss made forward-safe; coupling moved to 09b) | 1440 | split | 4 | 1 | → **if / else if / else — making the program choose**<br>→ **if is an expression — branching that produces a value** |
| ✅ `10-loops` → `10-loop-and-break` + `10b-while` + `10c-for-and-ranges` (done `0f89993`; ALL 3 fwd-refs fixed: += taught, print! taught, array off-by-one DEFERRED to the 13 row; labels + .rev() → Book §3.5; NEW E0571/E0308/E0425 pitfalls; couplings placed 10 + 10c) | 1380 | split | 7 | 3 | → **loop — repeat until you break**<br>→ **while — repeat while a condition holds**<br>→ **for and ranges — walk each item** |
| ✅ `11-match-intro` (trimmed `2932223`: sources-meta cut, coin-sorter + if-contrast + philosophy → one-liners + Book §6.2; 1305→1210w) | 1276 | trim | 5 | 0 | — |
| ✅ `12-string-vs-str` (trimmed `8403d8e`: slice-syntax deferred to L13, + & UTF-8 → Book) | 1278 | trim | 4 | 2 | — |
| ✅ `13-tuples-arrays-slices` → `13-tuples` + `13b-arrays` + `13c-slices` (done `62154e8`; ⚠ADOPT debt PAID — the L10 off-by-one walk lives in 13b beside the dual out-of-bounds; UTF-8 boundary panic → Book Ch.8.2 per audit; NEW E0308-arity + slice-range-panic pitfalls) | 1412 | split | 5 | 0 | → **Tuples — grouping mixed values**<br>→ **Arrays — a fixed row of one type**<br>→ **Slices — a borrowed window into a sequence** |
| ✅ `14-vec-hashmap` → `14-vec` + `14b-hashmap` (done `76ac338`; ALL 3 fwd-refs fixed: &mut/* loop → Phase 4 + Book §8.1, use-import glossed forward-safe backed by E0433, unwrap_or bonus replaced with plain .get(); NEW pop-Option example; couplings placed 14 ×2 + 14b) | 1301 | split | 6 | 3 | → **Vec — the growable list**<br>→ **HashMap — the lookup table** |
| `15-ownership-and-moves` | 1517 | split | 6 | 0 | → **Ownership, Scope, and Drop**<br>→ **Moves — assignment hands ownership over**<br>→ **Copy and Clone** |
| ✅ `16-references-and-borrowing` → `16-shared-references` + `16b-mutable-references` (override split, done `934d47c`) | 1393 | split | 5 | 1 | — |
| ✅ `17-slices-in-depth` (trimmed `8403d8e`: first_word → signature + Book §4.3; coercion glossed) | 1356 | trim | 4 | 2 | — |
| ✅ `18-structs` → `18-defining-structs` + `18b-methods-and-impl` + `18c-derive-debug` (done `5dbd4c9`) | 1358 | split | 6 | 0 | → **Defining Structs**<br>→ **Methods and impl Blocks**<br>→ **Printing Your Own Types: derive(Debug)** |
| ✅ `19-enums-and-matching` → `19-enums` + `19b-option` + `19c-match-in-depth` + `19d-concise-matching` (done `4fda08a`) | 1452 | split | 5 | 1 | → **Enums: One-of Types**<br>→ **Option<T>: Rust Has No Null**<br>→ **match Patterns in Depth**<br>→ **Concise Matching: if let, while let, let…else** |
| ✅ `20-error-handling` → `20-result` + `20b-panic-unwrap-expect` + `20c-question-mark` (done `244b624`) | 1406 | split | 4 | 2 | → **Result<T, E>: Errors Are Values**<br>→ **panic!, unwrap and expect**<br>→ **The ? Operator** |
| ✅ `21-packages-crates-modules` → `21-packages-and-crates` + `21b-modules` + `21c-modules-in-files` (done `2605311`; pub fwd-ref → copy-now-understand-in-L22 gloss; cardinality + mod.rs style → Book §7.1/§7.5; NEW E0601 + E0425 walls) | 1507 | split | 4 | 1 | → **Packages and Crates**<br>→ **Modules and the Module Tree**<br>→ **Splitting Modules into Files** |
| ✅ `22-paths-and-visibility` → `22-paths` + `22b-privacy-and-pub` + `22c-pub-structs-enums` (done `7f26510`; NEW E0425 forgot-super wall pairing its exercise; E0603/E0616 kept; pub(crate)+E0451+prefer-absolute → Book §7.3 per audit; couplings placed by expected_error_code) | 1485 | split | 4 | 0 | → **Paths: Naming Items Across Modules**<br>→ **Privacy and pub**<br>→ **pub on Structs and Enums** |
| ✅ `23-the-use-keyword` (trimmed `248a13a`: nested/self+Write example, glob+HashSet example, pub-use discussion → Book §7.4 pointers; ALL 3 fwd-refs died with the cuts) | 1210 | trim | 5 | 3 | — |
| ✅ `24-generics` → `24-generic-functions` + `24b-generic-types` (done `a1f8907`; E0369 wall = the destination, fwd-ref resolved) | 2270 | split | 6 | 1 | → **Generic Functions: the <T> Placeholder**<br>→ **Generic Structs, Enums, and Methods** |
| ✅ `25-traits` → `25-traits-declare-implement` + `25b-trait-bounds` (done `656a946`; 25b closes 24's E0369 arc) | 1888 | split | 5 | 0 | → **Traits: Declare and Implement**<br>→ **Trait Bounds: Requiring Behavior of a Generic T** |
| ✅ `26-lifetimes` → `26-lifetime-annotations` + `26b-lifetime-elision` + `26c-lifetimes-in-structs` (done `f52cce7`; both iterator fwd-refs rewritten w/ find/match) | 2344 | split | 6 | 2 | → **Lifetime Annotations: the longest Function**<br>→ **Lifetime Elision: the Annotations You Never Write**<br>→ **Lifetimes in Structs** |
| ✅ `27-closures` → `27-closure-syntax` + `27b-closure-capture` (done `09ae769`; all 3 fwd-refs killed: sort_by_key, thread-free move, Fn-family → Book) | 1992 | split | 5 | 3 | → **Closures: Unnamed Inline Functions**<br>→ **Closure Capture and move** |
| ✅ `28-iterators` → `28-iterator-cursor` + `28b-adapter-chains` (done `a153b62`; exercises 2/3 by theme: ownership vs annotation) | 2031 | split | 6 | 0 | → **Iterators: the next() Cursor**<br>→ **Adapter Chains: map, filter, collect** |
| ✅ `29-smart-pointers` → `29-box` + `29b-deref-drop` + `29c-rc` + `29d-refcell` (done `acf2972` — Batch C complete) | 2297 | split | 7 | 0 | → **Box<T>: Values on the Heap**<br>→ **Deref and Drop: the Traits Underneath**<br>→ **Rc<T>: One Value, Many Owners**<br>→ **RefCell<T>: Interior Mutability** |
| ✅ `30-threads-and-concurrency` → `30-spawning-threads` + `30b-channels` + `30c-shared-state` (done `a07b7d7`; new use-after-send E0382 pitfall) | 2651 | split | 6 | 0 | → **Spawning Threads: spawn, join, and move**<br>→ **Channels: Passing Data Between Threads**<br>→ **Shared State: Arc and Mutex** |
| ✅ `31-async-await` → `31-async-syntax` + `31b-futures-are-lazy` (done `5437bb8`; impl Future<Output=> fwd-ref → read-only gloss, runtime-origins → Book Ch.17.1) | 2235 | split | 6 | 1 | → **Async Syntax: async fn, .await, and async Blocks**<br>→ **Futures Are Lazy: Why Async Needs a Runtime** |
| ✅ `32-trait-objects-and-oop` → `32-trait-objects` + `32b-encapsulation` + `32c-states-as-types` (done `3d96fd6`; E0038 → rule-of-thumb + Book Ch.18.2, size_of → read-more; NEW E0616 pitfall in 32b) | 2828 | split | 5 | 0 | → **Trait Objects & Dynamic Dispatch**<br>→ **Encapsulation: Private Fields, Public Methods**<br>→ **States as Types: Making Broken States Impossible** |
| ✅ `33-advanced-patterns` → `33-refutability` + `33b-guards-bindings-nested` (done `fce023e`; flagship walk tightened → Book Ch.19.3, ref footnote → 1-line read-more; NEW E0004 guards-exhaustivity pitfall in 33b) | 2343 | split | 4 | 0 | → **Refutability: When a Pattern Can Fail**<br>→ **Guards, @ Bindings & Nested Patterns** |
| ✅ `34-advanced-features` → `34-unsafe` + `34b-operator-overloading` + `34c-macro-rules` (done `4917a9f`; superpowers 2–5 → Book Ch.20.1, adv-traits depth → Ch.20.2; NEW E0369 pitfall in 34b closing L31's Output= IOU; NEW trailing-comma macro pitfall in 34c) | 2317 | split | 4 | 0 | → **unsafe: A Small Audited Escape Hatch**<br>→ **Operator Overloading & Associated Types**<br>→ **Declarative Macros: macro_rules!** |
| ✅ `35-capstone-web-server` → `35-single-threaded-server` + `35b-thread-pool` (done `6464b3f`; ALL 3 fwd-refs fixed: 'static gloss, assert! gloss, Option::take honest gloss; slow-request now demonstrated; NEW E0596 pitfall; Drop-order + lock-scope → Book Ch.21) | 1442 | split | 5 | 3 | → **Capstone Part 1: A Single-Threaded Web Server**<br>→ **Capstone Part 2: The Thread Pool & Graceful Shutdown** |
| ✅ `36-automated-tests` (trimmed `248a13a`: custom messages + should_panic(expected) → Book Ch.11.1, organization → Ch.11.3 (+ doc-tests now = L37); left/right walkthrough intact) | 1012 | trim | 4 | 0 | — |
| ✅ `37-more-about-cargo` → `37-doc-comments` + `37b-shipping-with-cargo` (done `7ae2fc8`; REAL doc-test pass+fail captures from a live cargo project; publish/workspace mechanics → Book §14.2/§14.3 per audit; 37b carries the course epilogue) | 1133 | split | 4 | 0 | → **Documenting Rust: Doc Comments & Doc-Tests**<br>→ **Shipping with Cargo: Profiles, Publishing & Workspaces** |
**Editorial override — `16-references-and-borrowing`:** the auditor said *trim* (1393 w, 5 concepts),
but this lesson is the practice hub of the whole curriculum — **10 mapped exercises across 8 concept
tags** (most of any lesson). Overridden to **split** into: **"Shared references: borrow to read"**
(`&T`, many-readers, the function-move fix) and **"Mutable references and the borrowing rules"**
(`&mut T`, shared-xor-mutable E0499/E0502, dangling E0106, borrow-ends-at-last-use), so each side
pairs with its own exercise cluster. (+1 lesson → the ~77 total above.)

## Forward references — the complete repair list

Each is a concept *used before it is taught* (curriculum order). Fix = teach-inline-minimally,
rewrite the example, or (where marked in the matrix) the split itself resolves the ordering.

- **00-hello-world** — practice step says 'cargo new hello' — Cargo is never taught before use (Cargo depth is L37; the app's Practice tab is the intended venue)
- **01-bindings-and-immutability** — ': i32' annotation example and typing rep before L5 teaches number types (glossed inline as 'Rust's default whole number')
- **01-bindings-and-immutability** — println!("...{crew_size}") placeholder interpolation used with zero explanation — printing/formatting is taught in L8 (this recurs in L2–L5)
- **01-bindings-and-immutability** — practice step 3 has the learner apply 'let mut' and predict its two-line runtime output before L2 teaches mut (goes beyond a deferred mention — the §4 fixed program is run and predicted)
- **02-mutability** — glosses "five" as `&str` while walking the E0308 output ('"five" is text (&str), not a number') — string slices aren't taught until L12, and understanding the error line relies on accepting that gloss
- **03-shadowing** — uses method calls reply.trim() and reply.len() in the core example — and practice step 2 requires the learner to write .len() — before functions (L7), method-call syntax, or strings (L12) are taught
- **04-constants** — declares 'const MAX_LIVES: u32' and practice asks the learner to 'pick u32 or i32' one lesson before L5 teaches integer types (u32 glossed inline as 'unsigned — never-negative — whole number')
- **05-number-types-and-overflow** — the §4 aside leans on 'cargo run --release' and the debug-vs-release build distinction, which the curriculum never teaches before this point (Cargo depth is L37)
- **06-expressions-statements-semicolon** — defines `fn plus_one(x: i32) -> i32` (typed parameter + return type) in the pitfall, and practice step 3 asks the learner to WRITE a function `-> i32` — functions are taught in lesson 07; lessons 00-05 only ever use main
- **06-expressions-statements-semicolon** — `println!("{}", plus_one(5))` uses the positional {} placeholder — placeholders are taught in lesson 08 (never used in 00-05)
- **08-comments-and-printing** — `let nums = [1, 2, 3];` — array literal used in the E0277 pitfall, and practice step 3 asks the learner to make an array; arrays are taught in lesson 13
- **08-comments-and-printing** — "returns the finished `String`" — names the String type, taught in lesson 12
- **09-if-else-expressions** — minor: pitfall 2's explanation glosses "`&str` is a piece of text" — &str is taught in lesson 12 (compiler output forces the mention, but the gloss teaches it early)
- **10-loops** — `let a = [10, 20, 30]`, `a[index]`, `a.len()`, `for value in a` — the whole off-by-one pitfall relies on arrays, indexing, and a method call; arrays are taught in lesson 13
- **10-loops** — `print!("{n} ")` in the ranges example — only println!/format! were taught (lesson 08); print! is never introduced
- **10-loops** — `counter += 1` / `number -= 1` — compound assignment used without ever being taught (lessons 02/05 only use plain reassignment)
- **12-string-vs-str** — uses slice syntax `&owned[0..5]` in the core part-3 example and requires the learner to WRITE a slice in practice #2 ('Slice a view'), one lesson before L13 teaches slices
- **12-string-vs-str** — `s1 + &s2` uses an unexplained `&s2` borrow expression before L16 teaches references (the &str-as-view framing is deferred, but here & is applied to a String operand with no framing)
- **14-vec-hashmap** — iterates `for n in &mut v { *n += 50; }` — mutable references and the * dereference operator used (and required in practice #1's &v loop) two lessons before L16 teaches references and borrowing
- **14-vec-hashmap** — `use std::collections::HashMap;` is taught and required here, nine lessons before L23 teaches the use keyword
- **14-vec-hashmap** — practice #3 bonus `.get(word).copied().unwrap_or(0)` — Option combinators / unwrap_or before L20 error handling
- **16-references-and-borrowing** — the dangling-reference demo surfaces `error[E0106]: missing lifetime specifier` — the lifetime concept in the error text isn't taught until L26, so the lesson shows an error it cannot yet explain
- **17-slices-in-depth** — the first_word example uses `s.as_bytes()` (yielding &[u8]) and the byte literal `b' '` — byte strings/bytes-of-text machinery is never taught anywhere in the curriculum
- **17-slices-in-depth** — calls `first_word(&sentence)` passing a &String to a &str parameter — deref coercion is silently relied on and never explained (smart-pointer territory, L29)
- **19-enums-and-matching** — generic type-parameter notation glossed before generics (L24): 'you cannot use an Option<T> where a T is expected' relies on reading T as a placeholder, never taught
- **20-error-handling** — turbofish generic-argument syntax before generics (L24): `input.parse::<i32>()` — `::<>` never appears in L00-17 and is used unexplained
- **20-error-handling** — generic definition syntax before L24: shows `enum Result<T, E> { Ok(T), Err(E) }`, a parameterized enum definition, with T/E as placeholders never taught
- **21-packages-crates-modules** — `pub` used before L22 teaches visibility: every runnable example needs `pub fn do_something()`, and practice #2 additionally requires `pub` on a nested module ('you'll need pub on inner too') — one-line gloss only, but the code and exercise rely on it
- **23-the-use-keyword** — traits before L25: the nested-path example imports std::io::Write and explains 'write_all comes from the Write trait' — the example's point (importing a trait to get its methods) relies on the untaught trait concept
- **23-the-use-keyword** — HashSet in the glob example — HashMap is taught in L14 but HashSet appears in no lesson
- **23-the-use-keyword** — byte-string literal `b"hi via Write\n"` used without ever being introduced
- **24-generics** — `T: PartialOrd` trait bound in the flagship §3 `largest` example — bounds are L25's subject; the example does not compile without it, so the learner must type unexplained syntax (the lesson defers it explicitly, but the code relies on it)
- **26-lifetimes** — uses `bytes.iter().enumerate()` in the first_word example — iterator adapters are taught in L28, and .iter()/.enumerate() appear nowhere in lessons 00–23
- **26-lifetimes** — uses `novel.split('.').next().unwrap()` in the Excerpt example — calling .next() on an iterator is L28 material
- **27-closures** — uses `thread::spawn(move || …).join().unwrap()` as the motivating move example — threads are taught in L30
- **27-closures** — uses `.iter().map(|n| n * 2).collect()` in a §3 example, and practice item 2 requires .iter()/.map()/.collect()/.enumerate() — iterators are taught in L28
- **27-closures** — explains the E0507 pitfall via the `FnMut` trait name and twice defers to a 'Fn/FnMut/FnOnce in full' next lesson that does not exist — the next file is 28-iterators
- **31-async-await** — uses associated-type binding syntax `impl Future<Output = u32>` in examples and requires the learner to write it in practice task 2, but associated types (`type Output`) are only taught in lesson 34's advanced-traits section
- **35-capstone-web-server** — uses `Option::take` (`self.sender.take()`, `worker.handle.take()`) and attributes it to '(L19)' — Option::take is never taught in L19 or any other lesson
- **35-capstone-web-server** — uses `assert!(size > 0)` in ThreadPool::new before lesson 36 introduces the assert macros
- **35-capstone-web-server** — uses the `'static` lifetime bound in `Box<dyn FnOnce() + Send + 'static>` — `'static` never appears in lesson 26 (lifetimes) or anywhere earlier
## Coupling (#37) — lesson↔exercise follow-along

- **25 lessons** have mapped in-app exercises (via `CONCEPT_LESSON`) that their practice sections
  never mention. The systemic fix above converts every practice section into an explicit handoff:
  *"Now try it — open `ownership/01_move` in Practice"* (deep link), Sandbox for free-play extras.
- **13 lessons have no exercise at all**: `00-hello-world`, `03-shadowing`, `04-constants`,
  `06-expressions-statements-semicolon`, `08-comments-and-printing`, `13-tuples-arrays-slices`,
  `17-slices-in-depth`, `21-packages-crates-modules`, `31-async-await`, `33-advanced-patterns`,
  `35-capstone-web-server`, `36-automated-tests`, `37-more-about-cargo`. Some are inherently
  readerly (00, 21, 35–37); but shadowing, constants, expressions/semicolon, tuples/arrays/slices,
  and slices-in-depth have obvious predict-then-run exercise potential. **Authoring original
  exercises for those is recommended follow-up** (original authorship — distinct from the
  Paul-gated #22, which is about adapting external exercise sets; flagged for Paul in Open
  questions rather than assumed).
- Reverse direction already exists (exercises → "read the lesson") and stays.

## Execution plan (each batch = 1–3 loop ticks; sync rust-textbook + bump CONTENT_VERSION per batch)

1. **Batch A — systemic + early foundations (L00–L08).** The practice-venue handoff fix across all
   38 (one mechanical pass), early forward-ref repairs, `01` trim, `05` split (first runtime panic
   gets its own lesson). Highest reach, lowest risk.
2. **Batch B — the ownership/types spine (L15–L20).** The 16 override split, 18/19/20 splits, 12/17
   trims. This is where beginners drown; baby steps matter most here.
3. **Batch C — the abstraction ramp (L24–L29).** Generics/traits/lifetimes/closures/iterators/
   smart-pointers splits (all six are 1900–2400 w, 2–3 lessons each) + the L26/L27 iterator/thread
   forward-ref rewrites.
4. **Batch D — concurrency & advanced (L30–L35).** Threads/async/trait-objects/patterns/advanced/
   capstone splits; capstone's untaught-`'static`/`Option::take`/`assert!` repairs.
5. **Batch E — remainder.** 09/10/13/14/21/22/37 splits, 11/23/36 trims, sweep-up verification:
   every lesson ≤ ~900 words, one atomic concept, zero forward refs, practice hands off in-app.

Progress tracking: tick entries in `docs/IMPROVEMENT-LOOP.md` reference this file; the matrix rows
get ✅ marks (editing this doc) as batches land.
