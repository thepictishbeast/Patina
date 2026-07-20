> # ⚠️ DO NOT USE — UNVERIFIED — UNSAFE ⚠️
>
> This software is **unverified and unsafe for any production use**.
> It is published publicly only for transparency, third-party audit,
> and reproducibility. Treat every commit as guilty until proven
> innocent.
>
> By using this code you accept:
> - **No warranty** of any kind, express or implied.
> - **No fitness** for any particular purpose.
> - **No guarantee** of correctness, safety, or freedom from defects.
> - **Zero liability** on the maintainer for any damages — data loss,
>   security compromise, financial loss, or any consequential damages.
>
> The code is under active engineering development per the
> [Adversarial Validation Protocol v2](https://github.com/thepictishbeast/PlausiDen-AVP-Doctrine/blob/main/AVP2_PROTOCOL.md).
> Every commit's default verdict is **STILL BROKEN**. AVP-2 requires
> a minimum of 36 verification passes before a `SHIP-DECISION:`
> annotation may be considered. **No commit in this repository has
> reached `SHIP-DECISION:` status.**

# Tempered Studio

> Codename **Patina**. The CLI binary is `rpro` (its heritage is "Rustlings Pro";
> the `rpro-*` crate prefix kept that name). Under active engineering per the
> AVP-2 protocol above — capabilities below are **implemented and tested**, not
> certified safe.

A Rust learning environment for a genuine beginner, built around one discipline:
**read the real compiler output by hand.** Every error you see is raw, unedited
bytes from the actual toolchain — never a paraphrase — and the loop trains you to
predict, run, diagnose, and fix:

> **PREDICT → RUN → COMPARE → READ-RAW → DIAGNOSE → GUIDE → EXPLAIN → RETRY → RECALL**

(see [docs/EDUCATION.md](docs/EDUCATION.md)). It is **lesson-agnostic** and
**language-agnostic**: a neutral seam (`rpro-lang`) describes runs/outcomes/
diagnostics, and a per-language adapter (`rpro-lang-rust`) speaks the toolchain —
so the engine never hard-codes `cargo`/`rustc`.

## Four surfaces, one shared core

All run **fully offline** — no telemetry, no CDN, no account, no AI required.
Progress (`~/.rustlings-pro/`) is shared, so what you do in one surface shows in the others.

- **Web** (`rpro-serve`) — a loopback-only [axum] server + a single-file GUI
  (vendored xterm.js, no CDN), gzip-served. **IDE-first**: two tabs — 📝 Practice
  (the task + an editor with line numbers, syntax highlight, and red gutter marks
  on diagnostic lines) and 📚 Learn (one hub for lessons, quizzes, cheatsheets,
  the Book, a 9-book PDF library, the glossary, and Your Rust Journey, with
  per-stage progress). Predict-first gate, Run/Check/Explain, an earned hint
  ladder, click-a-diagnostic-to-explain, jump-to-line, a "↻ Recall" spaced-
  repetition queue, and dark/light themes (WCAG-AA in both, CI-audited). Binds
  `127.0.0.1` only; the op surface is a closed whitelist; it never sends the
  page the answer. See [docs/SECURITY.md](docs/SECURITY.md).
- **Android** ([Tempered-Studio-Mobile]) — the same GUI in a WebView with an
  on-device seam (JNI): the full curriculum offline, and **native compiling on
  the phone's real `rustc`** via an installed [Termux] (no server, no cloud).
- **CLI** (`rpro`) — `init`, `exercise list/next/hint/skip/reset`, `run/check/
  test`, `explain`, `book`, `glossary`, `lessons`, `quizzes`, `cheatsheets`,
  `progress` (with a per-phase breakdown).
- **TUI** (`rpro` dashboard) — [ratatui] dashboard, exercise view (raw output +
  diagnostics), Lessons, Quizzes (answers stay hidden until you reveal them),
  Book reader, Cheatsheets, Journey — same hint ladder + Recall.

**Three editor tiers** — *Learn* (strict: predict before Run, raw errors only,
you do the work), *Assist* (parsed diagnostics alongside the raw output), *Dev*
(editor assists). **The hint ladder never hands the solution**: it climbs
concept → expected error code → back to the source ([*The Rust Book*][book]
chapter that teaches the concept) — the fix is always yours to write.

[rustlings]: https://github.com/rust-lang/rustlings
[book]: https://doc.rust-lang.org/book/
[axum]: https://github.com/tokio-rs/axum
[ratatui]: https://ratatui.rs/
[Tempered-Studio-Mobile]: https://github.com/thepictishbeast/Tempered-Studio-Mobile
[Termux]: https://f-droid.org/packages/com.termux

## Quick start

Build from source (Rust toolchain ≥ 1.85, edition 2024):

```sh
git clone https://github.com/thepictishbeast/Tempered-Studio.git
cd Tempered-Studio

# Web — open http://127.0.0.1:8787/ in a browser (auto-seeds, no init needed):
cargo run -p rpro-serve

# CLI / TUI:
cargo install --path crates/rpro-cli
rpro init          # seed exercises + set the first one current
rpro               # the TUI dashboard
rpro exercise hint # book refs + a laddered hint for the current exercise
```

Full run notes, env knobs, and the smoke test: [RUN.md](RUN.md).

## Content

**89 exercises** across 15 topics (basics → control-flow → collections →
ownership → types/matching → error handling → modules → generics → traits →
lifetimes → closures → iterators → smart pointers → concurrency → advanced),
each a single-file program with a real, rustc-verified outcome (a compile error
code or a runtime panic), paired with `.toml` metadata — concept, expected
outcome, book refs. Every concept resolves to a glossary term AND to its
lesson, both CI-guarded.

The **Patina textbook is bundled and served in-app**: 37 lessons (first `let`
binding → a multithreaded web-server capstone, grouped into 11 stages), 11
predict-then-reveal quizzes, 11 per-phase cheatsheets, a 61-term plain-language
glossary, a Study Guide with a "when you finish" arc — every snippet
compile-verified on rustc 1.95.0, authored in the companion **rust-textbook**
repo and kept in lockstep. Plus 33 embedded Rust Book chapters (web/TUI Book
tab) and a fully-offline **Library of 9 complete books** (PDF, pdf.js): TRPL,
Comprehensive Rust, the Reference, the Rustonomicon, the Cargo Book, the
Edition Guide, the Embedded Book, Rust Design Patterns, and the Rustc Dev
Guide.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md). A language-agnostic **seam** keeps the
core pure and portable (it compiles to `wasm32`), with effects pushed to the edges:

```
crates/
  rpro-lang/             neutral seam: RunOp, Outcome, Diagnostic, Toolchain (pure)
  languages/rust/        the Rust Language adapter (rpro-lang-rust)
  rpro-core/             the run engine over the seam (pure, !Send)
  rpro-state/            progress, review (spaced-rep), hint ladder, tutor scaffolding (pure data)
  rpro-book/             Rust Book content loader + render
  rpro-runner/           exercise discovery + the shared run→progress recorder
  rpro-storage-fs/       on-disk state (effect)
  rpro-toolchain-local/  runs commands via std::process (effect)
  rpro-cli/  rpro-tui/  rpro-serve/   the three surfaces
```

The pure crates (`rpro-lang`, `rpro-lang-rust`, `rpro-state`, `rpro-book`,
`rpro-core`) build for `wasm32-unknown-unknown`; a CI gate enforces that, plus a
grep gate that keeps toolchain names out of the neutral layers.

## Status & verification

A 12-gate CI mirror (`scripts/check.sh`), all green at the current baseline:
rustfmt, clippy `-D warnings`, the workspace test suite, rustdoc, an API/serving
smoke, a **50-test Playwright browser suite run against an isolated throwaway
store** (deterministic — it can never touch a real learner's progress), exercise
integrity (89/89 outcomes re-verified against the real toolchain), CLI smoke,
GUI transform tests (XSS-safety by construction, cross-link guards), book-anchor
resolution, the language-seam grep gate, and the wasm32 pure-core build. Both
themes are axe-audited (WCAG 2 A/AA) on every view. A cited security review with
one fix landed.
Per the AVP-2 disclaimer at the top, none of this is a certification of safety —
it is what is implemented and checked so far.

## License

Dual MIT / Apache-2.0 — same as the Rust ecosystem default and the rustlings + Rust Book repos.

The embedded Rust Book content is © its authors, redistributed under MIT/Apache-2.0 per the upstream license at <https://github.com/rust-lang/book>.
