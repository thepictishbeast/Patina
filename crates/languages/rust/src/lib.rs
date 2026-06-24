//! `rpro-lang-rust` — the Rust implementation of the [`rpro_lang`] seam.
//!
//! This is the **only** language-specific crate in the workspace: every
//! `cargo` / `rustc` / `E0xxx` / `rust-analyzer` / `doc.rust-lang.org` token
//! lives here, and the seam-grep CI gate enforces that nothing language-shaped
//! leaks out of `crates/languages/`. It is pure (returns plans + parses
//! strings, performs no I/O), so it builds for `wasm32` like the rest of the core.

use rpro_lang::{
    BookRef, CommandPlan, DiagLevel, Diagnostic, ExerciseSource, ExerciseTemplate, LangTool,
    Language, LspSpec, RunOp, Span, ToolKind, ToolchainStatus,
};

/// The Rust language plug-in for Tempered Studio.
#[derive(Debug, Clone, Copy, Default)]
pub struct RustLanguage;

impl RustLanguage {
    /// The files to drop into a scratch project directory so a single snippet of
    /// `code` can be built/checked/tested/run. **Pure**: returns
    /// `(relative path, contents)` pairs; the effectful caller writes them. This
    /// keeps the Rust-specific project layout (the `Cargo.toml`) inside the
    /// language crate, where the seam permits language tokens.
    #[must_use]
    pub fn scaffold(code: &str) -> Vec<(&'static str, String)> {
        let manifest =
            "[package]\nname = \"exercise\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
                        [[bin]]\nname = \"exercise\"\npath = \"src/main.rs\"\n"
                .to_string();
        vec![("Cargo.toml", manifest), ("src/main.rs", code.to_string())]
    }

    /// Build a `cargo <sub> <extra…>` plan rooted at the exercise directory.
    fn cargo(ex: &ExerciseSource, sub: &str, extra: &[&str]) -> CommandPlan {
        let mut args = vec![sub.to_string()];
        args.extend(extra.iter().map(|s| (*s).to_string()));
        let tail = extra.iter().fold(String::new(), |mut acc, s| {
            acc.push(' ');
            acc.push_str(s);
            acc
        });
        CommandPlan {
            program: "cargo".into(),
            args,
            cwd: Some(ex.dir.clone()),
            env: vec![],
            display: format!("cargo {sub}{tail}"),
        }
    }

    /// Build a [`LangTool`] entry.
    fn tool(name: &str, kind: ToolKind, program: &str, args: &[&str]) -> LangTool {
        let tail = args.iter().fold(String::new(), |mut acc, a| {
            acc.push(' ');
            acc.push_str(a);
            acc
        });
        LangTool {
            name: name.into(),
            kind,
            plan: CommandPlan {
                program: program.into(),
                args: args.iter().map(|a| (*a).to_string()).collect(),
                cwd: None,
                env: vec![],
                display: format!("{program}{tail}"),
            },
        }
    }
}

/// Parse a rustc `  --> file:line:col` location line into a [`Span`]. Returns
/// `None` for any line that isn't a well-formed arrow location. The file path is
/// whatever precedes the trailing `:line:col` (so paths are taken verbatim).
fn parse_arrow_location(line: &str) -> Option<Span> {
    let rest = line.trim_start().strip_prefix("-->")?.trim();
    // rsplit twice: trailing `col`, then `line`, leaving the file path.
    let mut parts = rest.rsplitn(3, ':');
    let col = parts.next()?.trim().parse::<u32>().ok()?;
    let line_no = parts.next()?.trim().parse::<u32>().ok()?;
    let file = parts.next()?.trim();
    if file.is_empty() {
        return None;
    }
    Some(Span {
        file: file.to_string(),
        line: line_no,
        col,
    })
}

impl Language for RustLanguage {
    fn id(&self) -> &'static str {
        "rust"
    }

    fn source_ext(&self) -> &'static str {
        "rs"
    }

    fn exercise_template(&self, concept: &str) -> ExerciseTemplate {
        ExerciseTemplate {
            filename: "main.rs".into(),
            contents: format!(
                "// Exercise: {concept}\n// Make it compile and do what the lesson asks.\nfn main() {{\n    // TODO: your code here\n}}\n"
            ),
        }
    }

    /// Additive scrape of `error[E0xxx]:` / `warning:` headers from raw output,
    /// attaching the primary span from the following `--> file:line:col` line.
    fn parse_diagnostics(&self, raw: &str) -> Vec<Diagnostic> {
        let lines: Vec<&str> = raw.lines().collect();
        let mut out = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            let (level, rest) = if let Some(r) = trimmed.strip_prefix("error") {
                (DiagLevel::Error, r)
            } else if let Some(r) = trimmed.strip_prefix("warning") {
                (DiagLevel::Warning, r)
            } else {
                continue;
            };
            let (code, after) = rest
                .strip_prefix('[')
                .and_then(|b| b.find(']').map(|i| (b, i)))
                .map_or((None, rest), |(b, i)| {
                    (Some(b[..i].to_string()), &b[i + 1..])
                });
            let message = after.trim_start_matches(':').trim().to_string();
            if message.is_empty() {
                continue;
            }
            // The location sits on a following `--> file:line:col` line, before the
            // next header. Take the FIRST such line = this diagnostic's primary span.
            let span = lines[i + 1..]
                .iter()
                .take_while(|l| {
                    let t = l.trim_start();
                    !t.starts_with("error") && !t.starts_with("warning")
                })
                .find_map(|l| parse_arrow_location(l));
            out.push(Diagnostic {
                code,
                level,
                message,
                span,
            });
        }
        out
    }

    fn explain_plan(&self, code: &str) -> Option<CommandPlan> {
        Some(CommandPlan {
            program: "rustc".into(),
            args: vec!["--explain".into(), code.into()],
            cwd: None,
            env: vec![],
            display: format!("rustc --explain {code}"),
        })
    }

    fn book_ref_url(&self, r: &BookRef) -> String {
        r.anchor.as_ref().map_or_else(
            || format!("https://doc.rust-lang.org/book/{}.html", r.chapter),
            |a| format!("https://doc.rust-lang.org/book/{}.html#{}", r.chapter, a),
        )
    }

    fn lsp(&self) -> LspSpec {
        LspSpec {
            server: "rust-analyzer".into(),
            args: vec![],
            init_options: None,
            file_glob: "**/*.rs".into(),
        }
    }

    fn command_plan(&self, ex: &ExerciseSource, op: &RunOp) -> CommandPlan {
        match op {
            RunOp::Check => Self::cargo(ex, "check", &[]),
            RunOp::Run => Self::cargo(ex, "run", &["--quiet"]),
            RunOp::Test => Self::cargo(ex, "test", &[]),
            RunOp::Fmt => Self::cargo(ex, "fmt", &[]),
            RunOp::Lint => Self::cargo(ex, "clippy", &[]),
            RunOp::Explain(code) => self
                .explain_plan(code)
                .unwrap_or_else(|| Self::cargo(ex, "check", &[])),
            RunOp::Tool(name) => self
                .tools()
                .into_iter()
                .find(|t| &t.name == name)
                .map_or_else(|| Self::cargo(ex, "check", &[]), |t| t.plan),
        }
    }

    fn detect_plan(&self) -> CommandPlan {
        CommandPlan {
            program: "rustup".into(),
            args: vec!["show".into()],
            cwd: None,
            env: vec![],
            display: "rustup show".into(),
        }
    }

    fn parse_detect(&self, raw: &str) -> ToolchainStatus {
        let present = raw.contains("rustc")
            || raw.contains("toolchain")
            || raw.contains("stable")
            || raw.contains("nightly");
        // Version string, most-specific first — robust across rustup releases:
        //   1. a real `rustc 1.xx.x` line (`rustc --version` or old `rustup show`),
        //   2. the modern `rustup show` `name: <toolchain>` active-toolchain line,
        //   3. the `(active`/`(default)` toolchain row under `installed toolchains`.
        let version = raw
            .lines()
            .find(|l| l.trim_start().starts_with("rustc "))
            .or_else(|| raw.lines().find(|l| l.trim_start().starts_with("name:")))
            .or_else(|| {
                raw.lines()
                    .find(|l| l.contains("(active") || l.contains("(default)"))
            })
            .map(|l| l.trim().trim_start_matches("name:").trim().to_string());
        let components = ["clippy", "rustfmt", "rust-src", "rust-analyzer"]
            .iter()
            .filter(|c| raw.contains(**c))
            .map(|c| (*c).to_string())
            .collect();
        ToolchainStatus {
            present,
            version,
            components,
        }
    }

    fn tools(&self) -> Vec<LangTool> {
        vec![
            Self::tool("clippy", ToolKind::Lint, "cargo", &["clippy"]),
            Self::tool("rustfmt", ToolKind::Format, "cargo", &["fmt"]),
            Self::tool("bacon", ToolKind::Watch, "bacon", &[]),
            Self::tool("evcxr", ToolKind::Repl, "evcxr", &[]),
            Self::tool(
                "miri",
                ToolKind::Sanitize,
                "cargo",
                &["+nightly", "miri", "run"],
            ),
            Self::tool("cargo-expand", ToolKind::Expand, "cargo", &["expand"]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{RustLanguage, parse_arrow_location};
    use rpro_lang::{BookRef, DiagLevel, Language};

    fn ref_for(chapter: &str, anchor: Option<&str>) -> BookRef {
        BookRef {
            chapter: chapter.into(),
            anchor: anchor.map(Into::into),
            why: String::new(),
        }
    }

    #[test]
    fn book_ref_url_with_anchor() {
        assert_eq!(
            RustLanguage.book_ref_url(&ref_for(
                "ch04-01-what-is-ownership",
                Some("ownership-rules")
            )),
            "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules"
        );
    }

    #[test]
    fn book_ref_url_without_anchor() {
        assert_eq!(
            RustLanguage.book_ref_url(&ref_for("ch03-01-variables-and-mutability", None)),
            "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html"
        );
    }

    #[test]
    fn parses_an_error_code() {
        let diags = RustLanguage.parse_diagnostics("error[E0382]: borrow of moved value: `s`");
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code.as_deref(), Some("E0382"));
        assert_eq!(diags[0].level, DiagLevel::Error);
        // No `-->` line in this fragment → no span (the jump target stays absent).
        assert!(diags[0].span.is_none());
    }

    #[test]
    fn parse_diagnostics_attaches_the_primary_span() {
        let raw = "error[E0308]: mismatched types\n  \
                   --> src/main.rs:47:9\n   |\n47 |     let x = y;\n   |         ^ bad\n\n\
                   error: aborting due to 1 previous error";
        let diags = RustLanguage.parse_diagnostics(raw);
        assert_eq!(diags[0].code.as_deref(), Some("E0308"));
        let span = diags[0].span.as_ref().expect("primary span is attached");
        assert_eq!(span.file, "src/main.rs");
        assert_eq!((span.line, span.col), (47, 9));
        // the trailing "aborting" header carries no `-->` of its own → no span
        assert!(diags.last().unwrap().span.is_none());
    }

    #[test]
    fn parse_arrow_location_only_accepts_well_formed_locations() {
        assert!(parse_arrow_location("   | some source code").is_none());
        assert!(parse_arrow_location("  --> src/main.rs").is_none()); // missing line:col
        let s = parse_arrow_location("  --> src/main.rs:3:9").expect("valid location");
        assert_eq!((s.file.as_str(), s.line, s.col), ("src/main.rs", 3, 9));
    }

    #[test]
    fn parse_detect_modern_rustup_show() {
        // Real output from a current rustup (no `rustc 1.x` line; uses `name:`).
        let raw = "Default host: x86_64-unknown-linux-gnu\n\
                   installed toolchains\n--------------------\n\
                   stable-x86_64-unknown-linux-gnu (active, default)\n\n\
                   active toolchain\n----------------\n\
                   name: stable-x86_64-unknown-linux-gnu\n\
                   active because: it's the default toolchain\n";
        let st = RustLanguage.parse_detect(raw);
        assert!(st.present);
        assert_eq!(
            st.version.as_deref(),
            Some("stable-x86_64-unknown-linux-gnu")
        );
    }

    #[test]
    fn parse_detect_prefers_a_real_rustc_version_line() {
        // Old `rustup show` / `rustc --version` style — the rustc line wins.
        let raw = "active toolchain\n----------------\n\
                   stable-x86_64-unknown-linux-gnu (default)\n\
                   rustc 1.94.1 (abc123 2026-01-01)\n";
        let st = RustLanguage.parse_detect(raw);
        assert!(st.present);
        assert_eq!(
            st.version.as_deref(),
            Some("rustc 1.94.1 (abc123 2026-01-01)")
        );
    }

    #[test]
    fn parse_detect_absent_toolchain() {
        let st = RustLanguage.parse_detect("");
        assert!(!st.present);
        assert!(st.version.is_none());
    }

    #[test]
    fn scaffold_emits_manifest_and_entry() {
        let files = RustLanguage::scaffold("fn main() { let x = 1; }");
        assert_eq!(files.len(), 2);
        let manifest = &files.iter().find(|(p, _)| *p == "Cargo.toml").unwrap().1;
        assert!(manifest.contains("edition = \"2024\""));
        assert!(manifest.contains("name = \"exercise\""));
        let entry = files.iter().find(|(p, _)| *p == "src/main.rs").unwrap();
        assert_eq!(entry.1, "fn main() { let x = 1; }");
    }
}
