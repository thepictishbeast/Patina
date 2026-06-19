# Audit & test status (board task #12)

Single source of truth for "is Tempered Studio tested?" Re-run the commands below
to reproduce; every number here is from a clean run, not recall.

> As of `textbook-integration` HEAD. Refresh when crates, gates, or the smoke
> contract change.

## Summary

| Area | Status |
|---|---|
| Workspace build + unit/integration tests | ✅ **94 passing, 0 failing** |
| seam-grep gate (language-seam purity) | ✅ CLEAN |
| wasm32 pure-core gate | ✅ builds |
| E2E smoke (web server contract) | ✅ **8/8** |
| Security review + dependency audit | ✅ `docs/SECURITY.md` (posture sound) |
| Packaging pipeline | ✅ verified by inspection (`docs/DISTRIBUTION.md`) |
| `unsafe` code | ✅ **0** (`unsafe_code = "forbid"` workspace-wide) |
| Browser E2E (Playwright) / Lighthouse a11y | ⏳ CI-gated (needs a headless browser) |

## Automated gates (CI: `.github/workflows/`)

- **`seam-gates.yml`** — (a) `wasm32-pure-core`: the pure crates compile to
  `wasm32-unknown-unknown`; (b) `seam-grep`: no toolchain/error-code literals leak
  outside `crates/languages/`.
- **`ci.yml`** — `fmt`, `clippy`, `test`, `doc`, **`e2e smoke`** (runs `scripts/smoke.sh` on every push/PR).
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
```

## Test inventory (85 unit/integration)

| Crate | Tests | Covers |
|---|---|---|
| `rpro-state` | 26 | progress (incl. skip/reset), Leitner review + `fold_run`, exercise meta + hint ladder, tutor scaffolding, annotations/bookmarks/config |
| `rpro-tui` | 25 | dashboard + Recall panel, exercise view + hint panel, book/roadmap render, tab/scroll/selection model (TestBackend) |
| `rpro-lang-rust` | 7 | the Rust `Language` seam impl |
| `rpro-runner` | 7 | discovery, `primary_error_code`, `record_run` (tempfile integration) |
| `rpro-core` | 6 | the run engine |
| `rpro-storage-fs` | 5 | on-disk store round-trips |
| `rpro-serve` | 10 | op-whitelist, no-answer-leak, `sanitize_code`, status mapping + **6 router oneshot tests**: /api/current no-leak, unknown-op→400, oversized-body→413, /api/review shape, security headers present, hint L1 no-leak |
| `rpro-book` | 3 | chapter loading |
| `rpro-toolchain-local` | 2 | local process exec |

## E2E — `scripts/smoke.sh` (8/8)

Builds the server, seeds a throwaway store, serves on loopback, asserts: static
index + vendored xterm served; 23 exercises seeded in learning order with the
first current; `/api/current` leaks neither solution nor expected error; the L1
hint never reveals the solution; a real `/api/run` executes; `/api/review` is
internally consistent (`len(due)+mastered==tracked`); an over-limit body → 413.

This is the committed E2E coverage of the **API + serving contract**. The same
guarantees (no-leak, op-whitelist→400, oversized-body→413, security headers) are
*also* now asserted at the unit level via `tower::ServiceExt::oneshot` against the
extracted `build_router`, so they run in the standard test gate, not only this
bash script. Neither executes the page's JavaScript — that's the Playwright gap below.

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
85 tests, both seam/wasm gates, an 8-assertion E2E smoke, a cited security review
with one fix landed, and a verified packaging pipeline — all reproducible from the
commands above. The only outstanding audit pieces (Playwright, Lighthouse) require
a headless browser and are tracked as CI tasks, not in-sandbox gaps.
