#!/usr/bin/env bash
# Tempered Studio — run every CI gate locally, in one command.
#
# Mirrors .github/workflows/ci.yml + seam-gates.yml so you can confirm a change
# is mergeable BEFORE pushing (and so the improvement loop has a one-command
# regression sentinel). Runs ALL gates and reports every failure — it does NOT
# stop at the first. Exits 0 only if everything green.
#
# Needs: the Rust toolchain (cargo/rustc) + node on PATH. The wasm32 pure-core
# gate is skipped (with a note) when the wasm32-unknown-unknown target isn't
# installed locally (`rustup target add wasm32-unknown-unknown` to enable it).
#
# The two workflow files remain the source of truth that actually gates merges;
# keep this in sync with them.
set -uo pipefail
cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)" || exit 1

pass=0 fail=0
failed=()
run() { # run "<label>" <cmd...>
  local label="$1"
  shift
  printf '\n\033[1m▶ %s\033[0m\n' "$label"
  if "$@"; then
    printf '\033[32m  ✓ %s\033[0m\n' "$label"
    pass=$((pass + 1))
  else
    printf '\033[31m  ✗ %s\033[0m\n' "$label"
    fail=$((fail + 1))
    failed+=("$label")
  fi
}

# ── ci.yml gates ───────────────────────────────────────────────────────────
run "rustfmt" cargo fmt --all -- --check
run "clippy (-D warnings)" cargo clippy --workspace --all-targets --locked -- -D warnings
run "test" cargo test --workspace --locked
run "doc" cargo doc --workspace --no-deps --locked
run "e2e smoke" bash scripts/smoke.sh
run "browser e2e (isolated store)" bash scripts/e2e.sh
run "exercise integrity" bash scripts/verify-exercises.sh
run "cli smoke" bash scripts/smoke-cli.sh
run "gui transforms" node scripts/test-gui.mjs
run "book anchors" node scripts/verify-book-anchors.mjs

# ── seam-gates.yml: language-seam guard ──────────────────────────────────────
# No language-specific tokens (cargo/rustc/clippy/E0###/…) outside the language
# crates — keeps the seam clean. Mirrors seam-gates.yml exactly.
seam_grep() {
  local hits
  hits=$(grep -rnE '\b(cargo|rustc|rust-analyzer|clippy|rustfmt)\b|doc\.rust-lang|E0[0-9]{3}' \
    crates/ --include='*.rs' |
    grep -vE '///|//!' |
    grep -v 'clippy::' |
    grep -v 'crates/languages/' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    return 1
  fi
  echo "seam clean — no language tokens outside crates/languages/"
}
run "language-seam guard" seam_grep

# ── seam-gates.yml: wasm32 pure-core build (guarded on target availability) ───
if rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown; then
  run "wasm32 pure-core build" \
    cargo build --locked --target wasm32-unknown-unknown \
    -p rpro-lang -p rpro-lang-rust -p rpro-state -p rpro-book -p rpro-core
else
  printf '\n\033[33m▶ wasm32 pure-core build — SKIPPED\033[0m\n'
  printf '  (target not installed: rustup target add wasm32-unknown-unknown)\n'
fi

# ── supply-chain (deny.toml): advisories + licenses + bans + sources ─────────
# Guarded on availability — CI runs this via EmbarkStudios/cargo-deny-action.
if command -v cargo-deny >/dev/null 2>&1; then
  run "cargo-deny" cargo deny check advisories licenses bans sources
else
  printf '\n\033[33m▶ cargo-deny — SKIPPED\033[0m\n'
  printf '  (not installed: cargo install cargo-deny; CI runs it via the action)\n'
fi

# ── summary ──────────────────────────────────────────────────────────────────
printf '\n\033[1m── %d passed, %d failed ──\033[0m\n' "$pass" "$fail"
if [ "$fail" -ne 0 ]; then
  printf '\033[31m✗ FAILED: %s\033[0m\n' "${failed[*]}"
  exit 1
fi
printf '\033[32m✓ all gates green — branch is mergeable\033[0m\n'
