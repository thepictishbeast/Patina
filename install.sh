#!/bin/sh
# Rustlings Pro install script.
#
# Usage:
#   curl -fsSL https://rustlings-pro.example/install.sh | sh
#
# What it does:
#   1. Verifies you have a Rust toolchain (rustc + cargo) and offers
#      to install rustup if not.
#   2. Downloads the latest Rustlings-Pro source.
#   3. Builds with `cargo install --path crates/rpro-cli`.
#   4. Prints next steps (`rpro init`).
#
# Design intent:
#   - POSIX sh, no bashisms — runs on Termux's busybox sh too.
#   - One job: get the user to a working `rpro` binary.
#   - All actual setup happens via `rpro init` — so the install
#     script stays small and the install logic stays in Rust.

set -eu

INSTALL_TAG="${INSTALL_TAG:-main}"
SRC_DIR="${RUSTLINGS_PRO_SRC:-$HOME/.rustlings-pro-src}"
PINK='\033[1;35m'
DIM='\033[0;90m'
RST='\033[0m'

say() { printf '%b%s%b\n' "$PINK" "→ $1" "$RST"; }
note() { printf '%b%s%b\n' "$DIM" "  $1" "$RST"; }

# ----- 1. Toolchain check ------------------------------------------------

if ! command -v cargo >/dev/null 2>&1; then
    say "No Rust toolchain found."
    note "Rustlings Pro is a Rust application; you need rustup + cargo."
    note "Install rustup with the official one-liner, then re-run this script:"
    note ""
    note "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    note ""
    exit 1
fi

RUSTC_VERSION=$(rustc --version | awk '{print $2}')
say "Rust toolchain present: rustc $RUSTC_VERSION"

# ----- 2. Fetch source ---------------------------------------------------

# If we are running this from within a local clone, use it directly.
if [ -f "$PWD/crates/rpro-cli/Cargo.toml" ]; then
    say "Detected local workspace in $PWD"
    note "Skipping git clone and using local source."
    SRC_DIR="$PWD"
else
    if [ -d "$SRC_DIR/.git" ]; then
        say "Updating existing source at $SRC_DIR"
        git -C "$SRC_DIR" fetch --quiet origin "$INSTALL_TAG"
        git -C "$SRC_DIR" checkout --quiet "$INSTALL_TAG"
        git -C "$SRC_DIR" reset --quiet --hard "origin/$INSTALL_TAG" || true
    else
        say "Cloning source to $SRC_DIR"
        git clone --quiet --branch "$INSTALL_TAG" \
            https://github.com/thepictishbeast/Rustlings-Pro.git \
            "$SRC_DIR"
    fi
fi

# ----- 3. Build + install ------------------------------------------------

say "Building rpro (this takes 1-2 minutes the first time)"
cargo install --quiet --path "$SRC_DIR/crates/rpro-cli" --locked

# ----- 4. Next steps -----------------------------------------------------

say "Installed."
note ""
note "Next: run \`rpro init\` to fetch the exercise set + Rust Book content."
note "      Then \`rpro\` to open the dashboard."
note ""
note "If \`rpro\` is not found, your Cargo bin directory isn't on PATH."
note "Add this to your shell rc (~/.bashrc, ~/.zshrc, ~/.profile):"
note ""
note "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
