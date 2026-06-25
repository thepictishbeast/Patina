//! Opt-in end-to-end check against a REAL language server.
//!
//! Marked `#[ignore]` so CI never depends on a server being installed. The
//! server's name comes from the Rust plugin's own `lsp()` spec (a dev-dep), so
//! this file hard-codes no tool name. Run it on a machine that has the server:
//!
//! ```text
//! cargo test -p rpro-lsp -- --ignored
//! ```

use std::time::Duration;

use rpro_lang::Language;
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
