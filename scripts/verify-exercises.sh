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
# Runtime-panic exercises must be RUN, so their binary needs an exec-OK dir —
# /tmp is often mounted noexec. Prefer $HOME/.cache (exec-OK here and in CI).
EXEC_BASE="${TS_EXVERIFY_EXECDIR:-$HOME/.cache}"
mkdir -p "$EXEC_BASE" 2>/dev/null || true
RUNDIR="$(mktemp -d "$EXEC_BASE/ts-exrun.XXXXXX" 2>/dev/null || mktemp -d)"
trap 'rm -rf "$WORK" "$RUNDIR"' EXIT

fails=0
checked=0
echo "== Tempered Studio — exercise error-code + runtime-panic verification =="

for toml in exercises/*/*.toml; do
  [ -e "$toml" ] || continue
  rs="${toml%.toml}.rs"
  name="$(basename "$rs")"
  if [ ! -f "$rs" ]; then
    echo "  FAIL — $toml has no sibling .rs"; fails=$((fails + 1)); continue
  fi
  # Each exercise teaches a compile error code, a runtime panic, OR a codeless
  # compile error matched by message (an error rustc emits with no E#### code).
  exp="$(grep -E '^[[:space:]]*expected_error_code' "$toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')"
  panic="$(grep -E '^[[:space:]]*expected_runtime_panic' "$toml" | head -1 | sed -E 's/.*"(.*)".*/\1/')"
  errmsg="$(grep -E '^[[:space:]]*expected_error_message' "$toml" | head -1 | sed -E 's/.*"(.*)".*/\1/')"

  if [ -n "$exp" ]; then
    # Compile-error exercise: assert the taught code appears in the diagnostics.
    checked=$((checked + 1))
    out="$("$RUSTC" --edition 2024 --crate-type bin -o "$WORK/out" "$rs" 2>&1)"
    codes="$(printf '%s' "$out" | grep -oE 'error\[E[0-9]{4}\]' | grep -oE 'E[0-9]{4}' | sort -u | tr '\n' ' ')"
    if printf '%s ' "$codes" | grep -qw "$exp"; then
      echo "  ok   — $name emits $exp"
    else
      echo "  FAIL — $name: expected $exp, got [${codes:-none}]"; fails=$((fails + 1))
    fi
  elif [ -n "$panic" ]; then
    # Runtime-panic exercise: must COMPILE clean, then PANIC with the taught
    # message when run (the predict-then-run "it compiles — but does it panic?").
    checked=$((checked + 1))
    if ! "$RUSTC" --edition 2024 --crate-type bin -o "$RUNDIR/out" "$rs" 2>"$WORK/cerr"; then
      echo "  FAIL — $name: a runtime-panic exercise must COMPILE, but rustc rejected it:"
      sed 's/^/      /' "$WORK/cerr" | head -3; fails=$((fails + 1)); continue
    fi
    runout="$("$RUNDIR/out" 2>&1)"
    if printf '%s' "$runout" | grep -qF "$panic"; then
      echo "  ok   — $name panics: \"$panic\""
    else
      echo "  FAIL — $name: expected a panic containing \"$panic\", got: $(printf '%s' "$runout" | tr '\n' ' ' | head -c 160)"
      fails=$((fails + 1))
    fi
  else
    # Codeless compile-error exercise: rustc rejects it with a message carrying no
    # E#### code (e.g. "missing type for `const` item"). It is still a broken-on-
    # purpose exercise, so assert it FAILS to compile; and when it declares an
    # `expected_error_message`, assert that message appears — the codeless analogue
    # of the runtime-panic substring check.
    checked=$((checked + 1))
    cout="$("$RUSTC" --edition 2024 --crate-type bin -o "$WORK/out" "$rs" 2>&1)"; rc=$?
    if [ "$rc" -eq 0 ]; then
      echo "  FAIL — $name: declares no expected_error_code/panic yet COMPILES CLEAN — a teaching exercise must fail (add an expected_* field)"; fails=$((fails + 1))
    elif [ -n "$errmsg" ]; then
      if printf '%s' "$cout" | grep -qF "$errmsg"; then
        echo "  ok   — $name fails (codeless): \"$errmsg\""
      else
        echo "  FAIL — $name: expected error message \"$errmsg\", got: $(printf '%s' "$cout" | tr '\n' ' ' | head -c 160)"; fails=$((fails + 1))
      fi
    else
      echo "  warn — $name fails to compile but declares no expected_error_message (add one for a precise check)"
    fi
  fi
done

echo
if [ "$fails" -eq 0 ]; then
  echo "EXERCISES OK ✓ ($checked verified)"
else
  echo "EXERCISE VERIFY FAILED: $fails mismatch(es)"; exit 1
fi
