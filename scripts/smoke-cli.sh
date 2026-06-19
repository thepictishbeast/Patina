#!/usr/bin/env bash
# Tempered Studio — CLI (`rpro`) end-to-end smoke.
#
# Drives the real `rpro` binary against an ISOLATED store (via RPRO_STORE), so
# the command surface a learner actually uses — init seeding, exercise listing,
# by-hand `check`, `explain`, `progress` — is exercised end-to-end, not just its
# helper functions (which the unit tests cover). Complements `smoke.sh` (web).
#
# Honors $RPRO_BIN (defaults to ./target/debug/rpro) and the toolchain on PATH.
# Exits nonzero on the first failed assertion.
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Isolated, exec-capable store (the `check` compile writes a scratch project
# under it; some systems mount /tmp noexec). Keep the real $HOME so cargo/rustc
# find their own caches; only the rpro store is redirected.
BASE="${TS_SMOKE_DIR:-${HOME:-/tmp}/.cache}"
mkdir -p "$BASE" 2>/dev/null || BASE="${TMPDIR:-/tmp}"
STORE="$(mktemp -d "$BASE/ts-cli.XXXXXX")"
export RPRO_STORE="$STORE"
trap 'rm -rf "$STORE"' EXIT

RPRO="${RPRO_BIN:-./target/debug/rpro}"
fails=0
ok()   { echo "  ok   — $1"; }
fail() { echo "  FAIL — $1"; fails=$((fails + 1)); }

echo "== Tempered Studio — CLI smoke (RPRO_STORE=$STORE) =="

echo "-- build --"
cargo build -p rpro-cli --offline 2>/dev/null || cargo build -p rpro-cli

echo "-- assertions --"

# 1. init seeds the store (exercises + book) into the isolated RPRO_STORE.
if "$RPRO" init >/dev/null 2>&1; then ok "rpro init exited 0"; else fail "rpro init failed"; fi
[ -d "$STORE/exercises" ] && ok "exercises/ seeded under RPRO_STORE" || fail "no exercises/ after init"
[ -d "$STORE/book" ] && ok "book/ seeded under RPRO_STORE" || fail "no book/ after init"

# 2. exercise list shows the curriculum with a current exercise.
out="$("$RPRO" exercise list 2>&1)"
if printf '%s' "$out" | grep -q "basics/01_immutable_assign" \
   && printf '%s' "$out" | grep -qi "current"; then
  ok "rpro exercise list shows the seeded curriculum + current"
else
  fail "rpro exercise list"; printf '%s\n' "$out" | head -5
fi

# 3. check the current exercise → real toolchain, surfaces its taught error code.
out="$("$RPRO" check 2>&1)"
if printf '%s' "$out" | grep -qE 'E0[0-9]{3}'; then
  ok "rpro check runs the toolchain + surfaces the error code"
else
  fail "rpro check (no error code)"; printf '%s\n' "$out" | tail -5
fi

# 4. explain returns the official write-up (the diagnose-by-hand payoff).
out="$("$RPRO" explain E0384 2>&1)"
if [ "${#out}" -gt 100 ] && printf '%s' "$out" | grep -qi immutable; then
  ok "rpro explain E0384 returns the real write-up"
else
  fail "rpro explain E0384"
fi

# 5. progress prints a summary without erroring.
if "$RPRO" progress >/dev/null 2>&1; then ok "rpro progress runs"; else fail "rpro progress"; fi

echo
if [ "$fails" -eq 0 ]; then
  echo "CLI SMOKE PASS ✓"
else
  echo "CLI SMOKE FAILED: $fails assertion(s)"; exit 1
fi
