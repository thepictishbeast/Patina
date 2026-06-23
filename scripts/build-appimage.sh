#!/usr/bin/env bash
# Build a single-file, no-root AppImage — the locked Linux-desktop artifact from
# docs/DISTRIBUTION.md, parallel to the .deb/.rpm produced by release.yml.
#
# Two editions (EDITION env, default `cli`):
#   cli  — the `rpro` CLI/TUI; AppRun execs it, forwarding args. No bundled assets.
#   gui  — the GUI desktop app: bundles `rpro-serve` + gui/exercises/book under
#          usr/share/tempered-studio; AppRun launches the server (which finds the
#          assets via <exe>/../share/tempered-studio — resolve_asset_root) and
#          opens the browser.
#
# Usage:  scripts/build-appimage.sh [OUTDIR]            (default: dist/)
# Env:    EDITION=cli|gui                               which artifact (default cli)
#         ARCH=x86_64                                   target arch tag
#         BIN_OVERRIDE=path/to/binary                   use a prebuilt binary (skip cargo)
#         UPDATE_INFO="gh-releases-zsync|owner|repo|latest|<name>.AppImage.zsync"
#                                                       embed AppImageUpdate/zsync info (+ .zsync)
#         APPIMAGETOOL=path                             use a local appimagetool
#
# Locally reproducible, no root. FUSE is preferred for the pack; falls back to
# --appimage-extract-and-run where FUSE is unavailable.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
OUTDIR="${1:-dist}"
ARCH="${ARCH:-x86_64}"
EDITION="${EDITION:-cli}"
BUILD="$ROOT/target/appimage"            # exec-ok work dir (off any noexec /tmp)
SCAFFOLD="$ROOT/packaging/appimage"

case "$EDITION" in
  cli)
    PKG=rpro-cli;   BIN=rpro;        APPRUN=AppRun
    DESKTOP=tempered-studio.desktop; OUTNAME="Tempered_Studio-$ARCH.AppImage" ;;
  gui)
    PKG=rpro-serve; BIN=rpro-serve;  APPRUN=AppRun.gui
    DESKTOP=tempered-studio-gui.desktop; OUTNAME="Tempered_Studio_GUI-$ARCH.AppImage" ;;
  *) echo "fatal: unknown EDITION '$EDITION' (use cli|gui)" >&2; exit 2 ;;
esac

APPDIR="$BUILD/$EDITION.AppDir"
mkdir -p "$OUTDIR" "$BUILD"
rm -rf "$APPDIR"

# 1. The binary — build the release target unless a prebuilt one was supplied.
if [ -n "${BIN_OVERRIDE:-}" ]; then
  SRCBIN="$BIN_OVERRIDE"
else
  cargo build --release --locked -p "$PKG"
  SRCBIN="${CARGO_TARGET_DIR:-$ROOT/target}/release/$BIN"
fi
[ -x "$SRCBIN" ] || { echo "fatal: $BIN binary not found at $SRCBIN" >&2; exit 1; }

# 2. Assemble the AppDir (binary in usr/bin; top-level AppRun + .desktop + icon).
install -Dm755 "$SRCBIN"                       "$APPDIR/usr/bin/$BIN"
install -Dm755 "$SCAFFOLD/$APPRUN"             "$APPDIR/AppRun"
install -Dm644 "$SCAFFOLD/$DESKTOP"            "$APPDIR/tempered-studio.desktop"
install -Dm644 "$SCAFFOLD/tempered-studio.png" "$APPDIR/tempered-studio.png"
install -Dm644 "$SCAFFOLD/tempered-studio.png" \
  "$APPDIR/usr/share/icons/hicolor/256x256/apps/tempered-studio.png"

# The GUI edition bundles the served assets under the FHS path rpro-serve probes.
if [ "$EDITION" = gui ]; then
  SHARE="$APPDIR/usr/share/tempered-studio"
  mkdir -p "$SHARE"
  cp -r "$ROOT/gui" "$ROOT/exercises" "$ROOT/book" "$SHARE/"
fi

# 3. appimagetool (cached under target/; it is itself an AppImage).
TOOL="${APPIMAGETOOL:-$BUILD/appimagetool-$ARCH.AppImage}"
if [ -z "${APPIMAGETOOL:-}" ] && [ ! -x "$TOOL" ]; then
  echo "fetching appimagetool…"
  curl -fsSL -o "$TOOL" \
    "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-$ARCH.AppImage"
  chmod +x "$TOOL"
fi

# 4. Pack. Prefer the FUSE path; retry via extract-and-run if FUSE is absent.
OUT="$OUTDIR/$OUTNAME"
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
echo "built ($EDITION): $OUT"
( cd "$OUTDIR" && sha256sum "$(basename "$OUT")" )
