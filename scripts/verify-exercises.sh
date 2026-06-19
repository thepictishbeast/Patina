#!/usr/bin/env bash
# Tempered Studio — curriculum integrity check.
#
# The central pedagogical invariant of this app is: *every exercise fails with
# the exact compiler error it claims to teach.* That claim lives in each
# exercise's `.toml` (`expected_error_code`) and is enforced only by hand at
# authoring time. This script enforces it continuously: it compiles every
# exercise's single-file program with the real toolchain and asserts the taught
# error code appears in the diagnostics.
#
# Catches curriculum drift — an edited exercise that no longer fails, a typo in a
# `.toml`, or a toolchain change that shifts an error code.
#
# Usage:  scripts/verify-exercises.sh
# Honors $RUSTC (defaults to `rustc` on PATH). Exits nonzero on any mismatch.
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

RUSTC="${RUSTC:-rustc}"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/ts-exverify.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

fails=0
checked=0
echo "== Tempered Studio — exercise error-code verification =="

for toml in exercises/*/*.toml; do
  [ -e "$toml" ] || continue
  rs="${toml%.toml}.rs"
  name="$(basename "$rs")"
  if [ ! -f "$rs" ]; then
    echo "  FAIL — $toml has no sibling .rs"; fails=$((fails + 1)); continue
  fi
  # Pull the taught error code out of the toml (e.g. expected_error_code = "E0382").
  exp="$(grep -E '^[[:space:]]*expected_error_code' "$toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')"
  if [ -z "$exp" ]; then
    echo "  skip — $name (no expected_error_code)"; continue
  fi
  checked=$((checked + 1))
  # Compile the single-file program the same way it was authored (edition 2024).
  out="$("$RUSTC" --edition 2024 --crate-type bin -o "$WORK/out" "$rs" 2>&1)"
  codes="$(printf '%s' "$out" | grep -oE 'error\[E[0-9]{4}\]' | grep -oE 'E[0-9]{4}' | sort -u | tr '\n' ' ')"
  if printf '%s ' "$codes" | grep -qw "$exp"; then
    echo "  ok   — $name emits $exp"
  else
    echo "  FAIL — $name: expected $exp, got [${codes:-none}]"
    fails=$((fails + 1))
  fi
done

echo
if [ "$fails" -eq 0 ]; then
  echo "EXERCISES OK ✓ ($checked verified)"
else
  echo "EXERCISE VERIFY FAILED: $fails mismatch(es)"; exit 1
fi
