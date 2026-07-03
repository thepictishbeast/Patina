# Audit & test status (board task #12)

Single source of truth for "is Tempered Studio tested?" Re-run the commands below
to reproduce; every number here is from a clean run, not recall.

> As of `textbook-integration` (v0.4 candidate, re-verified 2026-07-03). Refresh
> when crates, gates, or the smoke contract change.

## Summary

| Area | Status |
|---|---|
| Workspace build + unit/integration tests | ✅ **154 passing, 0 failing** |
| seam-grep gate (language-seam purity) | ✅ CLEAN |
| wasm32 pure-core gate | ✅ builds |
| E2E smoke (web server contract) | ✅ **18/18** |
| **Browser E2E (Playwright, isolated throwaway store)** | ✅ **50/50, deterministic** (`scripts/e2e.sh`) |
| **a11y: axe-core WCAG 2 A/AA, every view, BOTH themes** | ✅ **16/16 view-audits** (in the browser suite) |
| Curriculum integrity (every exercise emits its taught outcome) | ✅ **71/71** (`scripts/verify-exercises.sh`) |
| CLI E2E (`rpro` init/list/check/explain/book-search/hint/next/progress) | ✅ **10/10** (`scripts/smoke-cli.sh`) |
| Web GUI pure transforms (`mdToHtml`, `highlightRust`, `hlLines` — incl. XSS invariants) + app-script parse-check + concept→lesson guard | ✅ **53/53** (`scripts/test-gui.mjs`) |
| Book-ref anchor integrity (every `anchor` resolves to a chapter heading slug) | ✅ **115/115** (`scripts/verify-book-anchors.mjs`) |
| Security review + dependency audit | ✅ `docs/SECURITY.md` (posture sound) |
| Packaging pipeline | ✅ verified by inspection (`docs/DISTRIBUTION.md`) |
| `unsafe` code | ✅ **0** (`unsafe_code = "forbid"` workspace-wide) |

## Automated gates (local mirror: `scripts/check.sh` — 12 gates; CI: `.github/workflows/`)

- **`seam-gates.yml`** — (a) `wasm32-pure-core`: the pure crates compile to
  `wasm32-unknown-unknown`; (b) `seam-grep`: no toolchain/error-code literals leak
  outside `crates/languages/`.
- **`ci.yml`** — `fmt`, `clippy -D warnings`, `test`, `doc`, **`e2e smoke`**
  (`scripts/smoke.sh`), **`browser e2e`** (`scripts/e2e.sh` — spawns rpro-serve on
  a THROWAWAY store so mutating specs can never touch a real learner's progress;
  the binary is resolved via `CARGO_TARGET_DIR` so it always tests the current
  build), **`exercise integrity`** (`scripts/verify-exercises.sh` — compiles all
  71 exercises, asserts each emits its taught error code or runtime panic),
  **`cli smoke`**, **`gui transforms`** (`scripts/test-gui.mjs` — the page's pure
  transforms incl. two XSS-safety-by-construction invariants, plus the
  concept→lesson cross-link guard), **`book anchors`** — on every push/PR.
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
bash scripts/e2e.sh             # the 50-test Playwright suite on an isolated store
RUSTC=$TC/rustc bash scripts/verify-exercises.sh   # every exercise emits its taught outcome
bash scripts/smoke-cli.sh       # drives the real `rpro` binary (isolated RPRO_STORE)
node scripts/test-gui.mjs       # web GUI pure transforms + cross-link guards
node scripts/verify-book-anchors.mjs   # every exercise book_ref anchor resolves
```

## Test inventory (154 workspace unit/integration, per-crate lib counts below)

| Crate | Tests | Covers |
|---|---|---|
| `rpro-tui` | 43 | dashboard + Recall panel, exercise view + hint panel, Lessons/Cheatsheets md readers, **Quizzes with the predict-then-reveal gate (answers withheld until `a`; the split NEVER leaks an answer; corpus-asserted: all 11 bundled quizzes carry the gate heading)**, book reader + roadmap, tab/scroll/selection model (TestBackend) |
| `rpro-state` | 30 | progress (skip/reset/select — single-Current invariant), Leitner review + `fold_run`, exercise meta + the 3-rung hint ladder (never the solution), tutor scaffolding, annotations/bookmarks/config |
| `rpro-serve` | 21 | op-whitelist, no-answer-leak, `sanitize_code`, router oneshot tests: no-leak, 400/413, review shape, security headers, hint L1 no-leak + force-attempt gate, book TOC/chapter/traversal-miss/search, select 400/409 |
| `rpro-book` | 12 | chapter loading + `clean_mdbook_source` + `title()` + `search()` |
| `rpro-lang-rust` | 9 | the Rust `Language` seam impl (incl. `book_ref_url`) |
| `rpro-runner` | 8 | discovery, `primary_error_code`, `record_run` + the golden-corpus invariants (ids unique, every concept resolves to a glossary term) |
| `rpro-core` | 6 | the run engine |
| `rpro-cli` | 6 | resolution precedence, seeding helpers, book_ref→chapter invariant, `lesson_stage` thresholds (monotone, 11 stages — mirrors the web grouping) |
| `rpro-storage-fs` | 6 | on-disk store round-trips + `resolve_user_root` |
| `rpro-toolchain-local` | 5 | local process exec (runaway-kill + pipe-drain deadlock-avoidance) |

## Browser E2E — `scripts/e2e.sh` (50/50, deterministic)

Spawns rpro-serve against a fresh throwaway store (never the live one — a
previous setup pointed the suite at the live server and its mutating specs
silently advanced real learner progress; see the isolated-store rationale in
the script header). Covers: the IDE-first navigation + 📚 Learn hub, the
predict-first gate (incl. Enter-to-Run and Ctrl/Cmd+Enter honoring it),
tier differentiation (Learn withholds parsed diagnostics; Assist surfaces the
code + jump-to-line + red gutter marks that clear on edit; Learn never marks),
the RECALL chip lifecycle, lesson/quiz/cheatsheet navigation with progress
tracking, offline purity (ZERO external requests across every surface), theme
defaults (OS preference on first visit, saved choice wins, Android stays
deterministic), and axe-core WCAG 2 A/AA audits of every view in BOTH themes.

## E2E — `scripts/smoke.sh` (18/18)

Builds the server, seeds a throwaway store, serves on loopback, asserts: static
index + vendored xterm served; 71 exercises seeded in learning order with the
first current; `/api/current` leaks neither solution nor expected error; the
hint gate holds at 0 attempts (level 0, "run it first", no leak); a real
`/api/run` executes; the success path end-to-end (fix → compile → pass →
advance); `explain` returns the real `rustc --explain` write-up; `/api/review`
internal consistency; over-limit body → 413; the embedded Book TOC seeds; a
chapter fetch returns cleaned markdown; 3 adversarial book-traversal probes
each miss; `?q=` full-text search returns ranked hits; `POST /api/select`
validates ids.

## Formerly-open items — now closed

- **G63 — Playwright E2E**: ✅ shipped as `scripts/e2e.sh` (50 tests, isolated
  store, deterministic; a `check.sh` gate). The old note said it needed a
  browser-equipped CI job; the local Playwright Chromium covers it here too.
- **G65 — a11y audit**: ✅ the substance shipped as axe-core WCAG 2 A/AA audits
  of every view in both themes, permanently gated (found + fixed 43 serious
  contrast violations in the light theme). A Lighthouse *performance* score
  specifically remains unmeasured — though the two big levers landed (gzip
  serving: the 164 KB shell ships as 48 KB; release-build binary).

## Conclusion

For everything runnable in this environment, the audit is **green and
comprehensive**: 154 workspace tests, the 50-test deterministic browser suite
with both-theme a11y audits, both seam/wasm gates, an 18-assertion server
smoke (incl. the success path and traversal guarantees), 71/71 exercise
integrity, 115/115 book anchors, 53 GUI-transform checks with two
XSS-by-construction invariants, a cited security review with one fix landed,
and a verified packaging pipeline — all reproducible from the commands above.
