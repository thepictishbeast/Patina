# Audit & test status (board task #12)

Single source of truth for "is Tempered Studio tested?" Re-run the commands below
to reproduce; every number here is from a clean run, not recall.

> As of `textbook-integration` HEAD. Refresh when crates, gates, or the smoke
> contract change.

## Summary

| Area | Status |
|---|---|
| Workspace build + unit/integration tests | ✅ **122 passing, 0 failing** |
| seam-grep gate (language-seam purity) | ✅ CLEAN |
| wasm32 pure-core gate | ✅ builds |
| E2E smoke (web server contract) | ✅ **18/18** |
| Curriculum integrity (every exercise emits its taught error) | ✅ **32/32** (`scripts/verify-exercises.sh`) |
| CLI E2E (`rpro` init/list/check/explain/book-search/hint/next/progress) | ✅ **10/10** (`scripts/smoke-cli.sh`) |
| Web GUI pure transforms (`mdToHtml`, `highlightRust` — incl. XSS invariant) + app-script parse-check | ✅ **40/40** (`scripts/test-gui.mjs`) |
| Book-ref anchor integrity (every `anchor` resolves to a chapter heading slug) | ✅ **51/51** (`scripts/verify-book-anchors.mjs`) |
| Security review + dependency audit | ✅ `docs/SECURITY.md` (posture sound) |
| Packaging pipeline | ✅ verified by inspection (`docs/DISTRIBUTION.md`) |
| `unsafe` code | ✅ **0** (`unsafe_code = "forbid"` workspace-wide) |
| Browser E2E (Playwright) / Lighthouse a11y | ⏳ CI-gated (needs a headless browser) |

## Automated gates (CI: `.github/workflows/`)

- **`seam-gates.yml`** — (a) `wasm32-pure-core`: the pure crates compile to
  `wasm32-unknown-unknown`; (b) `seam-grep`: no toolchain/error-code literals leak
  outside `crates/languages/`.
- **`ci.yml`** — `fmt`, `clippy`, `test`, `doc`, **`e2e smoke`** (`scripts/smoke.sh`), **`exercise integrity`** (`scripts/verify-exercises.sh` — compiles all 32 exercises, asserts each emits its taught error code), **`cli smoke`** (`scripts/smoke-cli.sh` — drives the real `rpro` binary against an isolated `RPRO_STORE`), **`gui transforms`** (`scripts/test-gui.mjs` — pins the web GUI's pure transforms: `esc`→`mdToHtml` and `highlightRust`, incl. the highlighter's XSS-safety invariant), **`book anchors`** (`scripts/verify-book-anchors.mjs` — every exercise book_ref `anchor` resolves to a chapter heading slug) — on every push/PR.
- **`release.yml`** — tag-triggered `.deb` + `.rpm` packaging (verified; see
  DISTRIBUTION.md).

Local reproduction (matched rustup toolchain `TC=.../stable-*/bin`):
```sh
RUSTC=$TC/rustc RUSTDOC=$TC/rustdoc $TC/cargo test --workspace --offline
RUSTC=$TC/rustc $TC/cargo build --offline --target wasm32-unknown-unknown \
  -p rpro-lang -p rpro-lang-rust -p rpro-state -p rpro-book -p rpro-core
grep -rnE '\b(cargo|rustc|rust-analyzer|clippy|rustfmt)\b|doc\.rust-lang|E0[0-9]{3}' \
  crates/ --include='*.rs' | grep -vE '///|//!' | grep -v 'clippy::' | grep -v 'crates/languages/'
bash scripts/smoke.sh           # builds, seeds, serves, asserts the contract
RUSTC=$TC/rustc bash scripts/verify-exercises.sh   # every exercise emits its taught error code
bash scripts/smoke-cli.sh       # drives the real `rpro` binary (isolated RPRO_STORE)
node scripts/test-gui.mjs       # web GUI pure transforms (mdToHtml + highlightRust XSS invariant)
node scripts/verify-book-anchors.mjs   # every exercise book_ref anchor resolves to a chapter heading
```

## Test inventory (113 unit/integration)

| Crate | Tests | Covers |
|---|---|---|
| `rpro-state` | 28 | progress (incl. skip/reset/**select** — single-Current invariant + done-count preserved), Leitner review + `fold_run`, exercise meta + hint ladder, tutor scaffolding, annotations/bookmarks/config |
| `rpro-tui` | 29 | dashboard + Recall panel, exercise view + hint panel, book reader (incl. mdBook-directive cleanup + blockquote rail + **inline markdown → styled spans**: `` `code` ``/bold/italic/links, no raw markers, intraword-`_` safe) + roadmap render, tab/scroll/selection model (TestBackend) |
| `rpro-serve` | 17 | op-whitelist, no-answer-leak, `sanitize_code`, status mapping + **router oneshot tests**: /api/current no-leak, unknown-op→400, oversized-body→413, /api/review shape, security headers, hint L1 no-leak, **book TOC + chapter fetch + traversal-is-a-miss + `?q=` search (ranked hits, blank→empty)**, **/api/select switch + unknown-id→400 + done→409** |
| `rpro-book` | 12 | chapter loading + `clean_mdbook_source` (directive→link-out, hidden-line drop, `##`-unescape, fence normalize, non-rust passthrough) + `title()` + **`search()`** (counts, frequency ordering, case-insensitive, blank→empty, snippet marker-strip/truncate) |
| `rpro-lang-rust` | 7 | the Rust `Language` seam impl (incl. `book_ref_url`) |
| `rpro-runner` | 7 | discovery, `primary_error_code`, `record_run` (tempfile integration) |
| `rpro-core` | 6 | the run engine |
| `rpro-storage-fs` | 6 | on-disk store round-trips + `resolve_user_root` (RPRO_STORE override → home fallback) |
| `rpro-toolchain-local` | 5 | local process exec (incl. run-timeout: runaway-kill + pipe-drain deadlock-avoidance) |
| `rpro-cli` | 5 | CLI helper logic: current-exercise resolution, `resolve_exercise` precedence (explicit id → current → first; unknown id errors), `write_if_missing` no-overwrite, `copy_tree` seeding (files + nested subdirs); **+ data invariant: every exercise `book_ref` resolves to a bundled chapter** |

## E2E — `scripts/smoke.sh` (18/18)

Builds the server, seeds a throwaway store, serves on loopback, asserts: static
index + vendored xterm served; 32 exercises seeded in learning order with the
first current; `/api/current` leaks neither solution nor expected error; the L1
hint never reveals the solution; a real `/api/run` executes; **the success path
end-to-end** — a correct fix for the first exercise compiles, runs, passes, and
advances the learner to the next (the app's core payoff, through the real
toolchain); **`explain` returns the real `rustc --explain` write-up** for a code
(the "diagnose by hand" payoff); `/api/review` is internally consistent (`len(due)+mastered==tracked`,
now with the overcome code tracked after the pass); an over-limit body → 413;
the embedded Book TOC seeds (≥10 chapters); a chapter fetch returns cleaned
markdown (no raw mdBook directives, listings linked out); and **3 adversarial
book-traversal probes** (`../../etc/passwd`, percent-encoded, `/etc/passwd`) each
miss the chapter map and return null — never a file from disk; **`/api/book?q=`
full-text search returns ranked hits** (the same `rpro_book::Book::search` the
CLI uses); and `POST /api/select` switches the current exercise (validated
against discovered ids, unknown id → 400).

This is the committed E2E coverage of the **API + serving contract**. The same
guarantees (no-leak, op-whitelist→400, oversized-body→413, security headers) are
*also* now asserted at the unit level via `tower::ServiceExt::oneshot` against the
extracted `build_router`, so they run in the standard test gate, not only this
bash script. The bash smoke doesn't run a browser; the **security-critical pure
page JS** (`mdToHtml`/`highlightRust`) is now covered headless by
`scripts/test-gui.mjs` (the XSS strip-spans invariant), so the remaining
Playwright gap is the page's *interactive* JS (DOM wiring, event handlers,
localStorage), not its sanitisation logic.

## Open items (browser-tooling-gated, tracked as CI tasks)

- **G63 — Playwright E2E** (drive the real page: editor, Run/Hint buttons,
  localStorage, predicted-vs-actual). The npm registry is reachable here, but no
  headless browser is installed and such a test only reproduces in a
  browser-equipped CI job — so it belongs in CI, not the offline sandbox. The
  app-side a11y/keyboard wiring it would exercise is already unit-covered in
  `rpro-tui` and present in `gui/` (ARIA roles, aria-live, keyboard handlers).
- **G65 — Lighthouse a11y/perf audit** of the web GUI. The a11y *implementation*
  shipped (keyboard nav + ARIA + phone reflow); the Lighthouse *score* needs a
  headless browser → same CI-task disposition as G63.

These mirror how `clippy` is handled: the toolchain/tooling isn't installable in
this sandbox, so the check lives in CI rather than being run here.

## Conclusion

For everything runnable in this environment, **#12 is green and comprehensive**:
122 tests, both seam/wasm gates, an 18-assertion E2E smoke (incl. the success
path: fix → run → pass → advance, the Book reader's path-traversal guarantee,
and full-text search), a cited security review with one fix landed,
and a verified packaging pipeline — all reproducible from the commands above. The
only outstanding audit pieces (Playwright, Lighthouse) require a headless browser
and are tracked as CI tasks, not in-sandbox gaps.
