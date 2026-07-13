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
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::{Duration, Instant};

use rpro_lang::{DiagLevel, Diagnostic, LspSpec, Span};
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

/// Collect the diagnostics the server reports for one file of `code`.
///
/// Spawns the server described by `spec`, runs `initialize` (with `root_uri` as
/// the workspace so a Rust server can resolve `std`/deps), opens `file_uri` with
/// `code`, and returns that file's diagnostics — mapped to
/// [`rpro_lang::Diagnostic`] — once the server's analysis settles. Servers clear
/// diagnostics on open then republish after analysing, so we keep the LATEST
/// publish and return only once the server's reported work (`workDoneProgress`)
/// has drained — not on the empty clear that precedes it. An empty `Vec` means the
/// file is clean. The exchange is bounded by `timeout`; on expiry the child is
/// killed and whatever diagnostics arrived so far are returned.
///
/// `root_uri` and `file_uri` are `file://` URIs; `file_uri` must live under
/// `root_uri`. This is the diagnostics half of the Dev-tier IDE (task #19): the
/// caller (the serve layer) already has the exercise's real workspace on disk and
/// points the server at it, so this crate stays free of temp-dir management.
///
/// # Errors
///
/// Same failure modes as [`server_info`]: [`LspError::Spawn`],
/// [`LspError::NoStdio`], [`LspError::Protocol`], and [`LspError::Timeout`].
pub fn diagnostics(
    spec: &LspSpec,
    root_uri: &str,
    file_uri: &str,
    code: &str,
    timeout: Duration,
) -> Result<Vec<Diagnostic>, LspError> {
    // Quiet windows used by the collection loop below. IDLE is the fallback for a
    // server that reports no progress at all: settle on a quiet stream once we hold
    // a publish. SETTLE confirms settling once the server's reported work has
    // drained; it must exceed the server's inter-task gaps so a lull between two
    // work items isn't mistaken for completion. The server clears diagnostics on
    // `didOpen` then republishes once analysis finishes, so waiting for the work to
    // drain captures that final state instead of the empty clear that precedes it.
    const IDLE: Duration = Duration::from_millis(1500);
    const SETTLE: Duration = Duration::from_millis(800);

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

    let mut stdin = child.stdin.take().ok_or(LspError::NoStdio)?;
    let stdout = child.stdout.take().ok_or(LspError::NoStdio)?;

    // A reader thread parses every framed message and forwards it, so the main
    // thread can bound its waits (blocking `read_message` can't be timed out).
    // It ends at EOF — when the child exits or is killed below.
    let (msg_tx, msg_rx) = mpsc::channel::<Value>();
    let reader = thread::spawn(move || {
        let mut r = BufReader::new(stdout);
        while let Ok(msg) = read_message(&mut r) {
            if msg_tx.send(msg).is_err() {
                break;
            }
        }
    });

    let deadline = Instant::now() + timeout;
    let options: Value = spec
        .init_options
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(Value::Null);

    let outcome = (|| -> Result<Vec<Diagnostic>, LspError> {
        write_msg(
            &mut stdin,
            &json!({
                "jsonrpc": "2.0", "id": 1, "method": "initialize",
                "params": {
                    "processId": Value::Null,
                    "rootUri": root_uri,
                    "capabilities": {
                        // Advertise workDoneProgress so the server reports when it
                        // is busy (indexing, building, checking). We settle only
                        // once that reported work drains — see the collection loop.
                        "window": { "workDoneProgress": true },
                        "textDocument": {
                            "synchronization": { "didSave": false, "dynamicRegistration": false },
                            "publishDiagnostics": { "relatedInformation": false }
                        }
                    },
                    "initializationOptions": options,
                }
            }),
        )?;
        // Drain messages until the initialize response arrives (id 1).
        loop {
            let left = deadline
                .checked_duration_since(Instant::now())
                .ok_or(LspError::Timeout(timeout))?;
            match msg_rx.recv_timeout(left) {
                Ok(m) if m.get("id").and_then(Value::as_i64) == Some(1) => break,
                Ok(_) => {}
                Err(RecvTimeoutError::Timeout) => return Err(LspError::Timeout(timeout)),
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(LspError::Protocol(
                        "server exited during initialize".to_owned(),
                    ));
                }
            }
        }

        write_msg(
            &mut stdin,
            &json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }),
        )?;
        write_msg(
            &mut stdin,
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": { "textDocument": {
                    "uri": file_uri, "languageId": "rust", "version": 1, "text": code
                }}
            }),
        )?;

        // Keep the LATEST publish for our file and return once analysis has truly
        // settled. The subtlety: a server clears diagnostics on `didOpen` (an empty
        // publish) BEFORE it has analysed anything, then does its work (indexing,
        // building, checking) and republishes the real result. Settling on that
        // first empty publish would report "clean" for broken code. So we track the
        // server's reported work via `workDoneProgress` (begin/end) and only settle
        // once that work has drained — i.e. the server is genuinely finished, not
        // merely paused between the clear and the real check.
        //
        let mut latest: Option<Vec<Diagnostic>> = None;
        let mut active_work: i32 = 0; // net begin−end of workDoneProgress tasks
        let mut saw_work = false; // did the server ever report progress?
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                break;
            }
            // While work is outstanding, wait up to the deadline for it to finish;
            // once the server is idle, a short quiet window confirms settling.
            let quiet = if saw_work && active_work == 0 {
                SETTLE
            } else {
                IDLE
            };
            match msg_rx.recv_timeout(left.min(quiet)) {
                Ok(m) => {
                    if let Some(d) = progress_delta(&m) {
                        saw_work = true;
                        active_work = (active_work + d).max(0);
                    }
                    if let Some(diags) = publish_for(&m, file_uri) {
                        latest = Some(diags);
                    }
                }
                // Quiet window elapsed. Settle when we hold a publish AND the server
                // is not mid-work: either it finished everything it reported
                // (saw_work && active_work == 0) or it reports no progress at all.
                Err(RecvTimeoutError::Timeout)
                    if latest.is_some() && (!saw_work || active_work == 0) =>
                {
                    break;
                }
                Err(RecvTimeoutError::Timeout) => {} // still working — keep waiting
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        Ok(latest.unwrap_or_default())
    })();

    // Best-effort clean shutdown, then guarantee the child dies and is reaped so
    // the reader thread's pipe closes and it can join.
    let _ = write_msg(
        &mut stdin,
        &json!({ "jsonrpc": "2.0", "id": 2, "method": "shutdown" }),
    );
    let _ = write_msg(&mut stdin, &json!({ "jsonrpc": "2.0", "method": "exit" }));
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();
    let _ = reader.join();
    outcome
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

// Map one LSP diagnostic object (0-based `range`, numeric `severity`) to the
// project-wide `rpro_lang::Diagnostic` (1-based `Span`) — the SAME shape the
// compile path already produces, so the Dev editor renders both identically.
fn map_diagnostic(d: &Value, file: &str) -> Option<Diagnostic> {
    let message = d.get("message").and_then(Value::as_str)?.to_owned();
    // LSP severities: 1 Error, 2 Warning, 3 Information, 4 Hint. Absent → Error
    // per the spec ("client should interpret ... as Error"). 3/4 fold to Note.
    let level = match d.get("severity").and_then(Value::as_i64) {
        Some(2) => DiagLevel::Warning,
        Some(3 | 4) => DiagLevel::Note,
        _ => DiagLevel::Error,
    };
    // `code` may be a string (a lint/error code) or a number; normalise to text.
    let code = d.get("code").and_then(|c| match c {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    });
    let span = d.get("range").and_then(|r| r.get("start")).map(|start| {
        let line =
            u32::try_from(start.get("line").and_then(Value::as_u64).unwrap_or(0)).unwrap_or(0);
        let col =
            u32::try_from(start.get("character").and_then(Value::as_u64).unwrap_or(0)).unwrap_or(0);
        Span {
            file: file.to_owned(),
            line: line + 1, // LSP is 0-based; Span is 1-based
            col: col + 1,
        }
    });
    Some(Diagnostic {
        code,
        level,
        message,
        span,
    })
}

// If `msg` is a `textDocument/publishDiagnostics` notification for `uri`, return
// its diagnostics mapped to `rpro_lang::Diagnostic` (an empty Vec means the file
// is currently clean). Any other message → `None` (keep reading).
fn publish_for(msg: &Value, uri: &str) -> Option<Vec<Diagnostic>> {
    if msg.get("method").and_then(Value::as_str) != Some("textDocument/publishDiagnostics") {
        return None;
    }
    let params = msg.get("params")?;
    if params.get("uri").and_then(Value::as_str) != Some(uri) {
        return None; // a publish for some other file (e.g. a dependency)
    }
    let diags = params.get("diagnostics").and_then(Value::as_array)?;
    Some(
        diags
            .iter()
            .filter_map(|d| map_diagnostic(d, uri))
            .collect(),
    )
}

// If `msg` is a `$/progress` work-done notification, report its effect on the
// count of outstanding server tasks: `+1` when a task begins, `-1` when one
// ends, `None` for a report (mid-task) or any other message. Tracking this net
// count lets the collector tell "the server is still working" (so the initial
// empty diagnostics are not yet final) from "the server has finished" (settle).
fn progress_delta(msg: &Value) -> Option<i32> {
    if msg.get("method").and_then(Value::as_str) != Some("$/progress") {
        return None;
    }
    match msg
        .get("params")
        .and_then(|p| p.get("value"))
        .and_then(|v| v.get("kind"))
        .and_then(Value::as_str)
    {
        Some("begin") => Some(1),
        Some("end") => Some(-1),
        _ => None, // "report" (progress update) or malformed — no net change
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LspError, encode, map_diagnostic, progress_delta, publish_for, read_message, read_until_id,
    };
    use rpro_lang::DiagLevel;
    use serde_json::json;
    use std::io::Cursor;

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

    #[test]
    fn map_diagnostic_translates_severity_range_and_code() {
        // LSP is 0-based; Span is 1-based. Error=1, Warning=2, Info=3, Hint=4.
        let d = json!({
            "severity": 1,
            "code": "some-error-code",
            "message": "mismatched types",
            "range": { "start": { "line": 4, "character": 8 }, "end": { "line": 4, "character": 9 } }
        });
        let got = map_diagnostic(&d, "file:///w/src/main.rs").unwrap();
        assert_eq!(got.level, DiagLevel::Error);
        assert_eq!(got.code.as_deref(), Some("some-error-code"));
        assert_eq!(got.message, "mismatched types");
        let span = got.span.unwrap();
        assert_eq!((span.line, span.col), (5, 9), "0-based LSP → 1-based Span");
        assert_eq!(span.file, "file:///w/src/main.rs");
    }

    #[test]
    fn map_diagnostic_folds_severities_and_defaults_to_error() {
        let warn = map_diagnostic(&json!({ "severity": 2, "message": "unused" }), "f").unwrap();
        assert_eq!(warn.level, DiagLevel::Warning);
        let info = map_diagnostic(&json!({ "severity": 3, "message": "note" }), "f").unwrap();
        assert_eq!(info.level, DiagLevel::Note);
        let hint = map_diagnostic(&json!({ "severity": 4, "message": "hint" }), "f").unwrap();
        assert_eq!(hint.level, DiagLevel::Note);
        // Absent severity → Error, per the LSP spec's client-default rule.
        let none = map_diagnostic(&json!({ "message": "x" }), "f").unwrap();
        assert_eq!(none.level, DiagLevel::Error);
        assert!(none.span.is_none(), "no range → no span");
    }

    #[test]
    fn map_diagnostic_normalises_a_numeric_code() {
        let d = json!({ "severity": 1, "code": 308, "message": "m" });
        assert_eq!(
            map_diagnostic(&d, "f").unwrap().code.as_deref(),
            Some("308")
        );
    }

    #[test]
    fn publish_for_matches_uri_and_maps_each_diagnostic() {
        let uri = "file:///w/src/main.rs";
        let msg = json!({
            "method": "textDocument/publishDiagnostics",
            "params": { "uri": uri, "diagnostics": [
                { "severity": 1, "message": "boom", "range": { "start": { "line": 0, "character": 0 } } },
                { "severity": 2, "message": "meh",  "range": { "start": { "line": 1, "character": 2 } } }
            ]}
        });
        let diags = publish_for(&msg, uri).unwrap();
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].level, DiagLevel::Error);
        assert_eq!(diags[1].level, DiagLevel::Warning);
        // An empty diagnostics array is a valid "file is clean" publish.
        let clean = json!({ "method": "textDocument/publishDiagnostics",
            "params": { "uri": uri, "diagnostics": [] } });
        assert_eq!(publish_for(&clean, uri).unwrap().len(), 0);
    }

    #[test]
    fn progress_delta_counts_begin_and_end_only() {
        let begin = json!({ "method": "$/progress",
            "params": { "token": "t", "value": { "kind": "begin", "title": "Indexing" } } });
        assert_eq!(progress_delta(&begin), Some(1));
        let end = json!({ "method": "$/progress",
            "params": { "token": "t", "value": { "kind": "end" } } });
        assert_eq!(progress_delta(&end), Some(-1));
        // A mid-task "report" makes no net change to the outstanding-work count.
        let report = json!({ "method": "$/progress",
            "params": { "token": "t", "value": { "kind": "report", "percentage": 50 } } });
        assert_eq!(progress_delta(&report), None);
        // Non-progress traffic is ignored.
        let other = json!({ "method": "textDocument/publishDiagnostics", "params": {} });
        assert_eq!(progress_delta(&other), None);
    }

    #[test]
    fn publish_for_ignores_other_uris_and_other_methods() {
        let uri = "file:///w/src/main.rs";
        let other_file = json!({ "method": "textDocument/publishDiagnostics",
            "params": { "uri": "file:///w/src/dep.rs", "diagnostics": [] } });
        assert!(publish_for(&other_file, uri).is_none());
        let other_method = json!({ "method": "window/logMessage", "params": {} });
        assert!(publish_for(&other_method, uri).is_none());
    }

    // The live end-to-end diagnostics check against the real server lives in
    // tests/real_server.rs (alongside the handshake one) so this crate stays
    // language-neutral: the server's name comes from the Rust plugin's `lsp()`
    // spec, never a hard-coded literal here.
}
