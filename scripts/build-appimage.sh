#!/usr/bin/env bash
# Build a single-file, no-root AppImage of the `rpro` CLI/TUI — the locked
# Linux-desktop "just works" artifact from docs/DISTRIBUTION.md, parallel to the
# .deb/.rpm produced by release.yml. Bundles only the self-contained `rpro`
# binary. (The GUI server `rpro-serve` is NOT yet relocatable — it resolves its
# assets via CARGO_MANIFEST_DIR — so the *GUI* AppImage waits on that fix; see
# docs/BACKLOG.md.)
#
# Usage:  scripts/build-appimage.sh [OUTDIR]            (default: dist/)
# Env:    ARCH=x86_64                                   target arch tag
#         RPRO_BIN=path/to/rpro                         use a prebuilt binary (skip cargo)
#         UPDATE_INFO="gh-releases-zsync|owner|repo|latest|Tempered_Studio-*.AppImage.zsync"
#                                                       embed AppImage auto-update info (+ .zsync)
#         APPIMAGETOOL=path                             use a local appimagetool
#
# Locally reproducible: run from anywhere, no root. FUSE is preferred for the
# final pack; the script falls back to --appimage-extract-and-run where FUSE is
# unavailable (e.g. minimal CI containers).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
OUTDIR="${1:-dist}"
ARCH="${ARCH:-x86_64}"
BUILD="$ROOT/target/appimage"            # exec-ok work dir (off any noexec /tmp)
APPDIR="$BUILD/Tempered_Studio.AppDir"
SCAFFOLD="$ROOT/packaging/appimage"

mkdir -p "$OUTDIR" "$BUILD"
rm -rf "$APPDIR"

# 1. The binary — build the release `rpro` unless a prebuilt one was supplied.
if [ -n "${RPRO_BIN:-}" ]; then
  BIN="$RPRO_BIN"
else
  cargo build --release --locked -p rpro-cli
  BIN="${CARGO_TARGET_DIR:-$ROOT/target}/release/rpro"
fi
[ -x "$BIN" ] || { echo "fatal: rpro binary not found at $BIN" >&2; exit 1; }

# 2. Assemble the AppDir (binary in usr/bin; top-level AppRun + .desktop + icon,
#    plus the icon in the hicolor theme path for desktop integration).
install -Dm755 "$BIN"                              "$APPDIR/usr/bin/rpro"
install -Dm755 "$SCAFFOLD/AppRun"                  "$APPDIR/AppRun"
install -Dm644 "$SCAFFOLD/tempered-studio.desktop" "$APPDIR/tempered-studio.desktop"
install -Dm644 "$SCAFFOLD/tempered-studio.png"     "$APPDIR/tempered-studio.png"
install -Dm644 "$SCAFFOLD/tempered-studio.png" \
  "$APPDIR/usr/share/icons/hicolor/256x256/apps/tempered-studio.png"

# 3. appimagetool (cached under target/; it is itself an AppImage).
TOOL="${APPIMAGETOOL:-$BUILD/appimagetool-$ARCH.AppImage}"
if [ -z "${APPIMAGETOOL:-}" ] && [ ! -x "$TOOL" ]; then
  echo "fetching appimagetool…"
  curl -fsSL -o "$TOOL" \
    "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-$ARCH.AppImage"
  chmod +x "$TOOL"
fi

# 4. Pack. Prefer the FUSE path; retry via extract-and-run if FUSE is absent.
OUT="$OUTDIR/Tempered_Studio-$ARCH.AppImage"
ARGS=()
[ -n "${UPDATE_INFO:-}" ] && ARGS+=(-u "$UPDATE_INFO")
ARGS+=("$APPDIR" "$OUT")
export ARCH
if ! "$TOOL" "${ARGS[@]}" 2>"$BUILD/appimagetool.log"; then
  echo "(direct/FUSE run failed — retrying via --appimage-extract-and-run)" >&2
  sed 's/^/  appimagetool: /' "$BUILD/appimagetool.log" >&2 || true
  "$TOOL" --appimage-extract-and-run "${ARGS[@]}"
fi

chmod +x "$OUT"
echo "built: $OUT"
( cd "$OUTDIR" && sha256sum "$(basename "$OUT")" )
