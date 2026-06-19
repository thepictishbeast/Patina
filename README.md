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

## Three surfaces, one shared core

All run **fully offline** against your local Rust toolchain — no telemetry, no CDN.
Progress (`~/.rustlings-pro/`) is shared, so what you do in one surface shows in the others.

- **Web** (`rpro-serve`) — a loopback-only [axum] server + a single-file GUI
  (vendored xterm.js, no CDN): editable code pane, predict-first bar, Run/Check/
  Explain, a hint ladder, click-a-diagnostic-to-explain, and a "↻ Recall" spaced-
  repetition queue. Binds `127.0.0.1` only; the op surface is a closed
  whitelist; it never sends the page the answer. See [docs/SECURITY.md](docs/SECURITY.md).
- **CLI** (`rpro`) — `init`, `exercise list/next/hint/skip/reset`, `run/check/test`,
  `explain`, `book`, `progress`.
- **TUI** (`rpro` dashboard) — [ratatui] dashboard, exercise view (raw output +
  diagnostics), book reader, roadmap, with the same hint ladder + Recall panel.

- **Book references on every exercise.** Stuck? The hint ladder points at the
  specific chapter of [*The Rust Book*][book] that explains the concept, never
  the fix — it climbs concept → expected error → solution outline (last resort).

[rustlings]: https://github.com/rust-lang/rustlings
[book]: https://doc.rust-lang.org/book/
[axum]: https://github.com/tokio-rs/axum
[ratatui]: https://ratatui.rs/

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

32 exercises across 9 phases (basics → control-flow → collections → ownership →
types/matching → modules → generics/traits/lifetimes → concurrency → advanced),
each a single-file program that fails with a real, rustc-verified error code,
paired with `.toml` metadata (concept, expected error, book refs, solution
outline). Plus 23 embedded Rust Book chapters (bundled + seeded; readable in the
web/TUI Book tab — code listings link out to the live Book). The structured
lessons are pending (gated on Paul's calibration — see
[docs/BACKLOG.md](docs/BACKLOG.md)).

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

94 unit/integration tests + an end-to-end smoke (API + serving contract) + the
seam/wasm CI gates, all green; a cited security review with one fix landed; a
verified packaging pipeline. The current state is tabulated in
[docs/AUDIT.md](docs/AUDIT.md). Per the AVP-2 disclaimer at the top, none of this
is a certification of safety — it is what is implemented and checked so far.

## License

Dual MIT / Apache-2.0 — same as the Rust ecosystem default and the rustlings + Rust Book repos.

The embedded Rust Book content is © its authors, redistributed under MIT/Apache-2.0 per the upstream license at <https://github.com/rust-lang/book>.
