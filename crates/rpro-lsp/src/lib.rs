//! A minimal Language Server Protocol client for the [`LspSpec`] seam.
//!
//! Given an [`LspSpec`] (which says *which* server to launch and how), this
//! crate spawns that server and runs the JSON-RPC handshake LSP requires —
//! `initialize` → `initialized` → `shutdown` → `exit` — then returns what the
//! server reported about itself in [`ServerInfo`].
//!
//! This is the foundation of the Dev-tier IDE (task #19). A real handshake is
//! stronger evidence than a version shell-out: it confirms the executable not
//! only exists but actually speaks the protocol. The framing helpers here
//! (`Content-Length` headers + JSON bodies) are the same ones a future
//! streaming-diagnostics client will reuse.
//!
//! It is an EFFECT crate: it spawns a child process and does blocking stdio, so
//! — like the local toolchain effect crate — it is intentionally not wasm-safe
//! and is kept out of the pure-core wasm build.

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rpro_lang::LspSpec;
use serde_json::{Value, json};
use thiserror::Error;

/// The default deadline for a full handshake before the server is killed.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// What a language server reports about itself in its `initialize` response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerInfo {
    /// The server's self-reported name.
    pub name: String,
    /// The server's self-reported version, if it sent one.
    pub version: Option<String>,
}

/// Why a handshake did not complete.
#[derive(Debug, Error)]
pub enum LspError {
    /// The server executable could not be spawned (not installed / not on `PATH`).
    #[error("could not launch language server {server:?}: {source}")]
    Spawn {
        /// The executable name that failed to launch.
        server: String,
        /// The underlying spawn error.
        source: std::io::Error,
    },
    /// The spawned server's stdin or stdout handle was unavailable.
    #[error("language-server stdio was unavailable")]
    NoStdio,
    /// An I/O or message-framing error while talking to the server.
    #[error("language-server protocol error: {0}")]
    Protocol(String),
    /// The server did not finish the handshake within the deadline.
    #[error("language server did not respond within {0:?}")]
    Timeout(Duration),
    /// The `initialize` response carried no `serverInfo` object.
    #[error("language server returned no serverInfo")]
    NoServerInfo,
}

/// Spawn the server described by `spec`, run the LSP handshake, and return its
/// [`ServerInfo`]. The exchange is bounded by `timeout`; on expiry the child is
/// killed and [`LspError::Timeout`] is returned.
///
/// # Errors
///
/// Returns [`LspError::Spawn`] if the executable cannot be launched,
/// [`LspError::NoStdio`] if its stdio is unavailable, [`LspError::Protocol`] on
/// an I/O or framing failure, [`LspError::Timeout`] if the handshake exceeds
/// `timeout`, and [`LspError::NoServerInfo`] if the server omits `serverInfo`.
pub fn server_info(spec: &LspSpec, timeout: Duration) -> Result<ServerInfo, LspError> {
    let mut child = Command::new(&spec.server)
        .args(&spec.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|source| LspError::Spawn {
            server: spec.server.clone(),
            source,
        })?;

    let stdin = child.stdin.take().ok_or(LspError::NoStdio)?;
    let stdout = child.stdout.take().ok_or(LspError::NoStdio)?;
    let init_options = spec.init_options.clone();

    // The handshake does blocking reads; run it on a worker so the caller can
    // enforce a deadline. On timeout we kill the child, which closes the pipes
    // and unblocks the worker's read.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(handshake(stdin, stdout, init_options.as_deref()));
    });

    if let Ok(result) = rx.recv_timeout(timeout) {
        let _ = child.wait();
        result
    } else {
        let _ = child.kill();
        let _ = child.wait();
        Err(LspError::Timeout(timeout))
    }
}

// Drive the four-step handshake over the server's stdio and return its info.
fn handshake<W: Write, R: Read>(
    mut stdin: W,
    stdout: R,
    init_options: Option<&str>,
) -> Result<ServerInfo, LspError> {
    let mut reader = BufReader::new(stdout);

    let options: Value = init_options
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(Value::Null);
    let initialize = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": Value::Null,
            "rootUri": Value::Null,
            "capabilities": {},
            "initializationOptions": options,
        }
    });
    write_msg(&mut stdin, &initialize)?;

    let response = read_until_id(&mut reader, 1)?;
    let info = response
        .get("result")
        .and_then(|r| r.get("serverInfo"))
        .ok_or(LspError::NoServerInfo)?;
    let name = info
        .get("name")
        .and_then(Value::as_str)
        .ok_or(LspError::NoServerInfo)?
        .to_owned();
    let version = info
        .get("version")
        .and_then(Value::as_str)
        .map(str::to_owned);

    write_msg(
        &mut stdin,
        &json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }),
    )?;
    write_msg(
        &mut stdin,
        &json!({ "jsonrpc": "2.0", "id": 2, "method": "shutdown" }),
    )?;
    let _ = read_until_id(&mut reader, 2);
    write_msg(&mut stdin, &json!({ "jsonrpc": "2.0", "method": "exit" }))?;

    Ok(ServerInfo { name, version })
}

// Frame a JSON message with the Content-Length header the protocol mandates.
fn encode(msg: &Value) -> Vec<u8> {
    let body = serde_json::to_vec(msg).expect("a serde_json::Value always serializes");
    let mut out = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
    out.extend_from_slice(&body);
    out
}

// Write one framed message and flush it.
fn write_msg<W: Write>(w: &mut W, msg: &Value) -> Result<(), LspError> {
    w.write_all(&encode(msg))
        .map_err(|e| LspError::Protocol(e.to_string()))?;
    w.flush().map_err(|e| LspError::Protocol(e.to_string()))
}

// Read one Content-Length-framed JSON message.
fn read_message<R: BufRead>(r: &mut R) -> Result<Value, LspError> {
    let mut len: Option<usize> = None;
    loop {
        let mut line = String::new();
        let n = r
            .read_line(&mut line)
            .map_err(|e| LspError::Protocol(e.to_string()))?;
        if n == 0 {
            return Err(LspError::Protocol("server closed the stream".to_owned()));
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break; // a blank line ends the header block
        }
        if let Some(rest) = trimmed.strip_prefix("Content-Length:") {
            len = rest.trim().parse::<usize>().ok();
        }
    }
    let len = len.ok_or_else(|| LspError::Protocol("missing Content-Length header".to_owned()))?;
    let mut body = vec![0u8; len];
    r.read_exact(&mut body)
        .map_err(|e| LspError::Protocol(e.to_string()))?;
    serde_json::from_slice(&body).map_err(|e| LspError::Protocol(e.to_string()))
}

// Read messages until one is the response carrying `id`, skipping any
// notifications the server interleaves before it answers.
fn read_until_id<R: BufRead>(r: &mut R, id: i64) -> Result<Value, LspError> {
    for _ in 0..256 {
        let msg = read_message(r)?;
        if msg.get("id").and_then(Value::as_i64) == Some(id) {
            return Ok(msg);
        }
    }
    Err(LspError::Protocol(
        "too many messages before the response".to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{LspError, read_message, read_until_id};
    use serde_json::json;
    use std::io::Cursor;

    use super::encode;

    #[test]
    fn encode_then_read_roundtrips() {
        let msg = json!({ "jsonrpc": "2.0", "id": 1, "method": "ping" });
        let mut cur = Cursor::new(encode(&msg));
        assert_eq!(read_message(&mut cur).unwrap(), msg);
    }

    #[test]
    fn reads_two_concatenated_messages_in_order() {
        let a = json!({ "id": 1 });
        let b = json!({ "id": 2 });
        let mut stream = encode(&a);
        stream.extend(encode(&b));
        let mut cur = Cursor::new(stream);
        assert_eq!(read_message(&mut cur).unwrap(), a);
        assert_eq!(read_message(&mut cur).unwrap(), b);
    }

    #[test]
    fn read_until_id_skips_notifications() {
        let notif = json!({ "method": "window/logMessage", "params": {} });
        let resp = json!({ "id": 7, "result": {} });
        let mut stream = encode(&notif);
        stream.extend(encode(&resp));
        let mut cur = Cursor::new(stream);
        assert_eq!(read_until_id(&mut cur, 7).unwrap(), resp);
    }

    #[test]
    fn missing_content_length_is_a_protocol_error() {
        let mut cur = Cursor::new(b"\r\n".to_vec());
        assert!(matches!(read_message(&mut cur), Err(LspError::Protocol(_))));
    }

    #[test]
    fn closed_stream_is_a_protocol_error() {
        let mut cur = Cursor::new(Vec::new());
        assert!(matches!(read_message(&mut cur), Err(LspError::Protocol(_))));
    }
}
