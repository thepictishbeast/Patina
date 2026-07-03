# Study Guide — learning Rust with Tempered Studio

A short guide to *how* to work through the Patina curriculum. It's 37 lessons for
absolute beginners, and it runs **fully offline** — no internet, no account, no AI
required. Take it in order; each lesson is a small step that builds on the last.

## The method: predict, then run

The whole platform is built on one habit: **guess what the compiler will say
before you run.** For each exercise you predict *pass or fail* — and if it fails,
*which* error — then you run it and read the **real compiler output by hand**.

That prediction is the point. It turns a red error message from something that
happens *to* you into something you were *expecting* — which is how the borrow
checker stops being scary and starts being a tool. The app never types the fix
for you; it guides (a hint ladder you earn one rung per attempt, book pointers,
the glossary) so the understanding is yours.

**Baby steps.** Every lesson introduces one idea, with the smallest possible
example. Don't skip ahead — Rust's later ideas (ownership, lifetimes, traits)
lean hard on the earlier ones.

## Your loop for each topic

1. **Read the lesson** (📚 Learn → Lessons) — the idea, in plain language.
2. **Practice it** (📝 Practice tab) — predict, then run the matching exercise; read
   the real error; fix it; run again.
3. **Self-check** with the phase **quiz** — predict every answer before you reveal it.
4. Keep the phase **cheatsheet** open for quick reference while you work.
5. **Read deeper** in the bundled book — every lesson links to the exact chapter
   of *The Rust Programming Language* (or The Cargo Book) in the offline Library.

The app keeps the main screen to just the task and the editor. Everything else
lives one tap away in the **📚 Learn** hub: the lessons (grouped by stage, with a
**Continue** button that jumps to your first unread one), the quizzes and
cheatsheets, the offline book and Library, the glossary, and **Your Rust
Journey** — the whole path by stage, with your progress.

## The path, stage by stage

| Stage | Lessons | What you'll learn |
|---|---|---|
| 1 · Foundations | 1–8 | bindings, `mut`, shadowing, constants, number types & overflow, expressions vs statements, functions, printing |
| 2 · Control Flow | 9–11 | `if`/`else` as an expression, loops, `match` |
| 3 · Text & Collections | 12–14 | `String` vs `&str`, tuples/arrays/slices, `Vec` & `HashMap` |
| 4 · Ownership & Borrowing | 15–17 | moves, references & borrowing, slices in depth |
| 5 · Custom Types & Matching | 18–20 | structs, enums & matching, error handling |
| 6 · Organizing Code | 21–23 | packages/crates/modules, paths & visibility, `use` |
| 7 · Generics, Traits & Lifetimes | 24–26 | generics, traits, lifetimes |
| 8 · Functional & Smart Pointers | 27–29 | closures, iterators, smart pointers |
| 9 · Concurrency | 30–31 | threads, `async`/`await` |
| 10 · Advanced | 32–35 | trait objects & OOP, advanced patterns, advanced features, a capstone web server |
| 11 · Tooling | 36–37 | automated tests, more about Cargo |

## Three editor tiers

- **Learn** — you do the work: predict before Run, read errors by hand, no
  autocomplete or auto-fix. This is where the learning happens.
- **Assist** — a parsed diagnostics panel appears alongside the raw output.
- **Dev** — editor niceties (Tab indents, brackets auto-close) for when you're
  past the basics.

Start in **Learn** and stay there until the by-hand loop feels natural.

## Running your code

- **Desktop / CLI:** your local `rustc` compiles and runs each exercise.
- **On your phone:** install [Termux](https://f-droid.org/packages/com.termux)
  (`pkg install rust`), and the app compiles with the phone's real `rustc` —
  natively, offline. **Check** type-checks fast; **Run** compiles and runs.

There's no substitute for typing it yourself and reading what the compiler tells
you. That's the whole game. Have fun — and don't stop.
