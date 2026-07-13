//! Opt-in end-to-end check against a REAL language server.
//!
//! Marked `#[ignore]` so CI never depends on a server being installed. The
//! server's name comes from the Rust plugin's own `lsp()` spec (a dev-dep), so
//! this file hard-codes no tool name. Run it on a machine that has the server:
//!
//! ```text
//! cargo test -p rpro-lsp -- --ignored
//! ```

use std::fs;
use std::time::Duration;

use rpro_lang::{DiagLevel, Language};
use rpro_lang_rust::RustLanguage;

#[test]
#[ignore = "requires the language server on PATH"]
fn handshakes_with_the_real_server() {
    let spec = RustLanguage.lsp();
    let info = rpro_lsp::server_info(&spec, Duration::from_secs(20))
        .expect("handshake should succeed when the server is installed");
    assert!(
        !info.name.is_empty(),
        "the server should report a non-empty name"
    );
    eprintln!("server reported: {} {:?}", info.name, info.version);
}

// Prove `diagnostics()` end-to-end against the real server: open a file with a
// deliberate type error and confirm an Error diagnostic comes back. The server
// clears diagnostics on open then republishes after analysing, so this also
// exercises the idle-collection that keeps the settled (not the empty) publish.
#[test]
#[ignore = "requires the language server on PATH + writes a temp project"]
fn diagnostics_reports_a_type_error() {
    let spec = RustLanguage.lsp();

    // A minimal on-disk project the server can load (it resolves the workspace
    // via its build tool, then analyses the opened file).
    let dir = std::env::temp_dir().join(format!("rpro-lsp-diag-{}", std::process::id()));
    let src = dir.join("src");
    fs::create_dir_all(&src).expect("create temp src dir");
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("write manifest");
    // A hard type mismatch the analyser flags from inference alone (no build).
    let bad = "fn main() { let n: i32 = \"not a number\"; let _ = n; }\n";
    fs::write(src.join("main.rs"), bad).expect("write source");

    let root_uri = format!("file://{}", dir.display());
    let file_uri = format!("file://{}", src.join("main.rs").display());

    let diags = rpro_lsp::diagnostics(&spec, &root_uri, &file_uri, bad, Duration::from_secs(30))
        .expect("diagnostics should succeed when the server is installed");
    let _ = fs::remove_dir_all(&dir);

    assert!(
        diags.iter().any(|d| d.level == DiagLevel::Error),
        "expected an Error diagnostic for the type mismatch, got: {diags:?}"
    );
    eprintln!("diagnostics reported: {diags:?}");
}
