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

# Rustlings Pro

> **Status: v0 scaffold.** Workspace + data model + install script + first exercise. CLI runs; TUI book reader + exercise runner land next.

A Rust learning environment that extends [rustlings] with:

- **Book-references on every exercise.** When you're stuck, `rpro hint` doesn't dump a generic clue — it points you at the specific chapter and section of [*The Rust Programming Language*][book] (a.k.a. "the Rust Book") that explains the concept. Every exercise — rustlings-derived and Rustlings-Pro-original — carries a typed list of book refs.
- **The Rust Book embedded for in-app reading.** No tab-switching to a browser. `rpro book` opens a TUI reader; bookmarks, highlights, and annotations live in `~/.rustlings-pro/`.
- **Deeper exercises beyond the rustlings ceiling.** Once you've finished rustlings' coverage, Rustlings-Pro continues with exercises drawn from real PlausiDen-codebase patterns: typed sieve rules, Iced GUI state shape, Axum + Maud handlers, async-imap clients, etc. Same shape (compile-error → fix → next), more advanced.
- **CLI-first, deliberately.** A basic in-app editor exists for the times you need it, but the design pushes you back to the terminal: read the compiler error, fix the file, run the exercise. The intent is to build CLI fluency alongside Rust fluency, not abstract it away.
- **Rust all the way down.** No JS, no Electron, no Python helpers. CLI in clap, TUI in ratatui, future GUI in Iced — same toolkits the PlausiDen ecosystem uses, so what you learn here applies to the project codebases.

[rustlings]: https://github.com/rust-lang/rustlings
[book]: https://doc.rust-lang.org/book/

## Targets

| Platform | Status |
|---|---|
| Linux x86_64 | v0 target |
| Linux aarch64 | v0 target |
| Termux on Android (aarch64) | v0 target — Termux is a Linux-like userland on Android |
| macOS | works (Rust toolchain is identical), not a primary target |
| Windows | works (Rust toolchain is identical), not a primary target |
| Native Android (no Termux) | future — needs the GUI on Android, separate effort |

## Quick start

> **One-line install (when v0 ships a release):**
> ```sh
> curl -fsSL https://rustlings-pro.example/install.sh | sh
> ```
>
> Today, build from source (Rust toolchain ≥1.85):

```sh
git clone https://github.com/thepictishbeast/Rustlings-Pro.git
cd Rustlings-Pro
cargo install --path crates/rpro-cli
rpro init
```

`rpro init` creates `~/.rustlings-pro/`, downloads the rustlings exercise set, fetches the Rust Book content, and prints what to run next.

## Commands (planned for v0)

| Command | What it does |
|---|---|
| `rpro init` | One-time setup: clone exercises, fetch the book, write state files. |
| `rpro` | Default — opens the TUI: progress dashboard, current exercise, book references. |
| `rpro exercise list` | Every exercise with status (locked / current / done) and short description. |
| `rpro exercise next` | Jump to the next unfinished exercise. |
| `rpro exercise run` | Compile + run the current exercise file. |
| `rpro exercise hint` | Show book references for the current exercise. |
| `rpro book` | TUI Rust Book reader: chapter list, scroll, bookmark, highlight, annotate. |
| `rpro book search <term>` | Full-text search across the book content. |
| `rpro progress` | Summary of completed / pending exercises + estimated remaining time. |

## Why "book references on every exercise"

Stock rustlings says "go read the Rust Book" without saying *which page*. Rustlings-Pro replaces that with a typed pointer: the runner knows the exercise is about borrow checking, the exercise file's frontmatter says "this is § 4.2 — References and Borrowing", and `rpro hint` opens the in-app reader at that section.

Every original Rustlings-Pro exercise (the ones beyond rustlings) ships with the same metadata, so the depth-of-content scales without losing the signpost.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md). Workspace has 5 crates:

```
crates/
  rpro-state/   bookmarks, progress, annotations on disk (~/.rustlings-pro/)
  rpro-runner/  exercise discovery + cargo invocation + result parsing
  rpro-book/    Rust Book content loader + markdown render → ratatui spans
  rpro-tui/     ratatui shell shared by book reader + exercise screen
  rpro-cli/     binary `rpro` — top-level command dispatch
```

A future `rpro-gui` crate will host the Iced desktop GUI; the same `rpro-state` files back both surfaces, so progress made in the CLI shows up in the GUI and vice versa.

## Book Updates

> **Note:** The Rust Book is embedded directly into this repository. Be sure to periodically `git pull` the cloned `rust-book` subdirectory or re-clone it from GitHub to stay up-to-date with the latest Rust language features and documentation updates!

## License

Dual MIT / Apache-2.0 — same as the Rust ecosystem default and the rustlings + Rust Book repos.

The embedded Rust Book content is © its authors, redistributed under MIT/Apache-2.0 per the upstream license at <https://github.com/rust-lang/book>.
