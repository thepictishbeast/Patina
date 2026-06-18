# Distribution & Packaging Plan

How every Tempered Studio surface is built, published, and **auto-updated**.
FOSS-first; everything obtainable from GitHub. This is the release-engineering
layer — it sits *on top of* the runnable binaries, so a surface gets a pipeline
only once it actually builds something runnable.

## Targets (locked requirement)

| Surface | Artifact(s) | Primary auto-update channel | Home |
|---|---|---|---|
| **Linux desktop** (Rust app) | **AppImage**, **.deb**, **.rpm** | AppImage: zsync self-update · .deb: signed APT repo · .rpm: dnf repo | `Tempered-Studio` GH Releases + GH Pages repos |
| **Android** (mobile app) | **APK** (+ AAB if ever Play) | **F-Droid** (best FOSS auto-update) + Obtainium-compatible GH Releases | likely a **separate** `Tempered-Studio-Mobile` repo |
| **Web** | static wasm bundle | n/a (always latest on load) | GH Pages |

Minimum bar the user set for mobile: **APK + (AppImage or .deb)**. We exceed it:
APK for the native app, and the desktop AppImage/.deb/.rpm also run on-device
under Termux for the CLI/TUI path.

## Linux desktop — three formats, two update styles

Built from the same `rpro` (CLI/TUI) and future GUI binaries.

- **AppImage** — the "just works", no-root, single-file option.
  - Build: `cargo build --release` → `linuxdeploy` + `appimagetool`.
  - **Auto-update: AppImageUpdate / `--appimage-update`** via embedded update
    info + a `.zsync` file published next to the release asset. Delta updates,
    no repo signing, no root. This is the recommended default updater.
- **.deb** — system-integrated, the user's stated preference for regular updates.
  - Build: `cargo-deb`.
  - **Auto-update: a signed APT repository** hosted on GH Pages
    (`deb [signed-by=…] https://thepictishbeast.github.io/Tempered-Studio stable main`).
    `apt update && apt upgrade` then keeps it current. Requires a GPG release key.
- **.rpm** — Fedora/openSUSE parity.
  - Build: `cargo-generate-rpm`.
  - **Auto-update: a dnf/yum repo** on GH Pages (same hosting pattern as APT).

All three are attached to each GitHub Release; the APT/dnf metadata and the
`.zsync` live on GH Pages so updates pull straight from GitHub.

## Android — APK, FOSS auto-update

- **F-Droid is the FOSS-native auto-update path** — submit to the main F-Droid
  repo (or run our own F-Droid repo on GH Pages); the F-Droid client then
  updates the app like any store. First choice given the FOSS-first rule.
- **GitHub Releases APK** in parallel — works with **Obtainium**, which polls a
  GH repo's releases and auto-installs. Good for power users / pre-F-Droid.
- **In-app updater (fallback)**: check the GH Releases API on launch, prompt to
  download + install the newer APK (needs the install-unknown-apps permission).
- Reproducible builds + a stable signing key are prerequisites for F-Droid.

## Mobile repo split

Per the user: a **separate `Tempered-Studio-Mobile` repo is acceptable** if the
Android toolchain (Termux `rust` package embed, Kotlin host, Android Studio
project) doesn't sit cleanly in the Rust workspace. Decision: **split it** — the
Android host app + its packaging live in their own repo; it consumes the shared
language-seam crates (`rpro-lang`, `rpro-core`, …) via git dependency or a
published crate, so the "one seam, many surfaces" architecture is preserved
across both repos.

## CI / release flow (GitHub Actions)

One `release.yml` triggered on a version tag:
1. Build release binaries on the self-hosted `plausiden` runner.
2. Produce AppImage + .deb + .rpm (desktop) and, in the mobile repo, the signed APK.
3. Create the GitHub Release and attach all artifacts.
4. Refresh the GH Pages APT/dnf repos and the AppImage `.zsync`.
5. (Mobile) trigger the F-Droid repo rebuild.

## Open decisions (user's call, later)

- **Android auto-update channel**: F-Droid (most FOSS-native, slower to publish)
  vs self-hosted F-Droid repo vs Obtainium-only. *Recommendation: F-Droid main +
  GH Releases APK in parallel.*
- **APT/dnf repo signing key** custody (where the GPG private key lives for CI).

## Sequencing

1. **Now-ish (desktop):** the `rpro` CLI/TUI is runnable, so the desktop
   pipeline can start — `cargo-deb` + `cargo-generate-rpm` + AppImage are all
   locally verifiable before any GUI exists.
2. **After the Android app exists:** stand up the mobile repo + APK + F-Droid.
3. **With the web build:** GH Pages deploy of the wasm bundle.
