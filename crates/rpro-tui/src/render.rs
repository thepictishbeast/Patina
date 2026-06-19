//! Screen rendering. Render functions are pure over (`App` + screen data), so
//! they're snapshot-testable with `ratatui`'s `TestBackend` — no live terminal.

use crate::app::{App, Tab};
use crate::status;
use crate::theme::Theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Gauge, List, ListItem, ListState, Paragraph, Tabs, Wrap};
use rpro_book::Book;
use rpro_lang::{BookRef, Diagnostic, EditorAssists, Language};
use rpro_lang_rust::RustLanguage;

/// Display data for the dashboard, decoupled from storage so the render is pure.
#[derive(Debug, Clone, Default)]
pub struct DashboardData {
    /// Exercises completed.
    pub done: usize,
    /// Total exercises discovered.
    pub total: usize,
    /// The current exercise id, if any.
    pub current_id: Option<String>,
    /// The current exercise title, if any.
    pub current_title: Option<String>,
    /// "Up next" exercise ids (locked until the current passes).
    pub up_next: Vec<String>,
    /// Spaced-repetition (RECALL) queue: diagnostic codes due for review,
    /// weakest first. Read from shared `Progress.reviews` — mirrors the web's
    /// "↻ Recall" widget so a web+TUI learner sees one review queue.
    pub due: Vec<String>,
    /// Distinct codes mastered (retired from review).
    pub mastered: usize,
    /// Distinct codes tracked at all. The Recall panel hides when this is 0.
    pub tracked: usize,
}

/// The top tab bar, shared by every screen.
fn tab_bar(app: &App) -> Tabs<'static> {
    let titles: Vec<Line> = Tab::ALL
        .iter()
        .map(|t| Line::from(format!(" {} ", t.title())))
        .collect();
    Tabs::new(titles)
        .select(app.tab.index())
        .highlight_style(Style::new().fg(app.theme.accent).add_modifier(Modifier::BOLD))
        .divider(Span::raw("│"))
        .block(Block::bordered().title(Span::styled(
            " ◆ Tempered Studio ",
            Style::new().fg(app.theme.accent).add_modifier(Modifier::BOLD),
        )))
}

/// Render the dashboard: tab bar, a progress gauge, the current exercise, and
/// an "up next" list.
pub fn render_dashboard(f: &mut Frame, app: &App, data: &DashboardData) {
    let area = f.area();
    // The Recall row only appears once the learner has tracked a concept, so a
    // fresh user sees absence rather than a permanently-empty box.
    let show_recall = data.tracked > 0;
    let mut constraints = vec![
        Constraint::Length(3), // tabs
        Constraint::Length(3), // gauge
        Constraint::Length(4), // current
    ];
    if show_recall {
        constraints.push(Constraint::Length(3)); // recall
    }
    constraints.push(Constraint::Min(0)); // up next
    let rows = Layout::vertical(constraints).split(area);
    let recall_idx = 3;
    let up_next_idx = if show_recall { 4 } else { 3 };

    f.render_widget(tab_bar(app), rows[0]);

    // Progress gauge — Skipped never fills the bar (done / total only).
    #[allow(clippy::cast_precision_loss)]
    let ratio = if data.total > 0 {
        (data.done as f64 / data.total as f64).clamp(0.0, 1.0)
    } else {
        0.0
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let pct = (ratio * 100.0).round() as u16;
    let gauge = Gauge::default()
        .block(Block::bordered().title(" Progress "))
        .gauge_style(Style::new().fg(app.theme.done))
        .ratio(ratio)
        .label(format!("{}/{} · {pct}%", data.done, data.total));
    f.render_widget(gauge, rows[1]);

    // Current exercise.
    let current = match (&data.current_id, &data.current_title) {
        (Some(id), Some(title)) => Line::from(vec![
            Span::styled("▸ ", Style::new().fg(app.theme.current).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{id}  "), Style::new().add_modifier(Modifier::BOLD)),
            Span::styled(title.clone(), Style::new().fg(app.theme.muted)),
        ]),
        _ => Line::from(Span::styled(
            "All caught up — nothing in progress.",
            Style::new().fg(app.theme.done),
        )),
    };
    f.render_widget(
        Paragraph::new(current).block(Block::bordered().title(" Current ")),
        rows[2],
    );

    // Recall — the spaced-repetition queue, mirroring the web's "↻ Recall".
    if show_recall {
        let title = if app.ascii { " Recall " } else { " ↻ Recall " };
        let line = if data.due.is_empty() {
            let check = if app.ascii { "(all mastered)" } else { "all mastered ✓" };
            Line::from(Span::styled(
                format!(
                    "{} concept{} {check}",
                    data.tracked,
                    if data.tracked == 1 { "" } else { "s" }
                ),
                Style::new().fg(app.theme.done),
            ))
        } else {
            let mut spans: Vec<Span> = data
                .due
                .iter()
                .take(8)
                .map(|c| {
                    Span::styled(
                        format!("{c} "),
                        Style::new().fg(app.theme.error).add_modifier(Modifier::BOLD),
                    )
                })
                .collect();
            spans.push(Span::styled(
                format!("· {}/{} mastered", data.mastered, data.tracked),
                Style::new().fg(app.theme.muted),
            ));
            Line::from(spans)
        };
        f.render_widget(
            Paragraph::new(line).block(Block::bordered().title(title)),
            rows[recall_idx],
        );
    }

    // Up next.
    let items: Vec<ListItem> = if data.up_next.is_empty() {
        vec![ListItem::new(Span::styled(
            "  (run `rpro init` to load exercises)",
            Style::new().fg(app.theme.muted),
        ))]
    } else {
        data.up_next
            .iter()
            .map(|id| {
                ListItem::new(Line::from(vec![
                    Span::styled("[ ] ", Style::new().fg(app.theme.locked)),
                    Span::raw(id.clone()),
                ]))
            })
            .collect()
    };
    f.render_widget(
        List::new(items).block(Block::bordered().title(" Up next ")),
        rows[up_next_idx],
    );
}

/// Display data for the exercise view — the by-hand-error surface.
#[derive(Debug, Clone, Default)]
pub struct ExerciseViewData {
    /// Exercise id.
    pub id: String,
    /// Exercise title.
    pub title: String,
    /// Raw stderr — shown verbatim (what the learner reads).
    pub raw_stderr: String,
    /// Raw stdout — shown verbatim, after stderr.
    pub raw_stdout: String,
    /// Parsed diagnostics — additive sidebar, never a replacement for raw.
    pub diagnostics: Vec<Diagnostic>,
    /// Editor assists (drives the Free/Learning badge).
    pub assists: EditorAssists,
    /// A run is in flight (animates the throbber).
    pub running: bool,
    /// Last verdict: `(passed, duration_ms)`.
    pub verdict: Option<(bool, u64)>,
    /// Book references: `(chapter, why)`.
    pub book_refs: Vec<(String, String)>,
    /// The current exercise's metadata, used to compute the hint ladder locally.
    /// (This is the local surface, so it may hold the answer fields; they only
    /// reach the screen when the learner explicitly climbs to that rung.)
    pub meta: Option<rpro_state::ExerciseMetadata>,
    /// The currently-revealed hint rung `(level, max_level, text)`. `None` until
    /// the learner presses `h`.
    pub hint: Option<(u8, u8, String)>,
}

/// Render the exercise view: tab bar, a title/verdict header, the **raw output**
/// (the big, always-present region — the by-hand-error contract) beside an
/// additive **diagnostics** sidebar, and a book-refs footer. Stacks on narrow.
pub fn render_exercise(f: &mut Frame, app: &App, data: &ExerciseViewData) {
    let area = f.area();
    // A hint row only appears once the learner has climbed the ladder (pressed h).
    let show_hint = data.hint.is_some();
    let mut constraints = vec![
        Constraint::Length(3), // tabs
        Constraint::Length(4), // header (id/title + status)
        Constraint::Min(0),    // raw + diagnostics
    ];
    if show_hint {
        constraints.push(Constraint::Length(5)); // hint
    }
    constraints.push(Constraint::Length(3)); // book refs
    let rows = Layout::vertical(constraints).split(area);
    let hint_idx = 3;
    let book_idx = if show_hint { 4 } else { 3 };

    f.render_widget(tab_bar(app), rows[0]);

    // header: id + title, then a status line (running / verdict / prompt).
    let badge = status::mode_badge(data.assists);
    let title_line = Line::from(vec![
        Span::styled(
            format!(" {} ", data.id),
            Style::new().fg(app.theme.accent).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(data.title.clone(), Style::new().fg(app.theme.muted)),
    ]);
    let status_line = if data.running {
        Line::from(Span::styled(
            format!("{} running…", app.throbber.frame()),
            Style::new().fg(app.theme.current),
        ))
    } else if let Some((passed, ms)) = data.verdict {
        let (label, col) =
            if passed { ("✓ passed", app.theme.done) } else { ("✗ failed", app.theme.error) };
        Line::from(Span::styled(format!("{label} in {ms}ms"), Style::new().fg(col)))
    } else {
        Line::from(Span::styled(
            "press r to run · h for a hint — read the real output by hand",
            Style::new().fg(app.theme.muted),
        ))
    };
    f.render_widget(
        Paragraph::new(vec![title_line, status_line])
            .block(Block::bordered().title(format!(" Exercise · {badge} "))),
        rows[1],
    );

    // body: raw output (big) + diagnostics (additive). Stacked on narrow.
    let body = if App::is_narrow(area.width) {
        Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)]).split(rows[2])
    } else {
        Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)]).split(rows[2])
    };

    let raw = if data.raw_stderr.is_empty() && data.raw_stdout.is_empty() {
        "(no output yet — your real compiler output will appear here, verbatim)".to_string()
    } else {
        let mut s = data.raw_stderr.clone();
        if !data.raw_stdout.is_empty() {
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(&data.raw_stdout);
        }
        s
    };
    f.render_widget(
        Paragraph::new(raw)
            .wrap(Wrap { trim: false })
            .scroll((app.scroll, 0))
            .block(Block::bordered().title(" Raw output  (PgUp/PgDn) ")),
        body[0],
    );

    let diag_items: Vec<ListItem> = if data.diagnostics.is_empty() {
        vec![ListItem::new(Span::styled("  (none)", Style::new().fg(app.theme.muted)))]
    } else {
        data.diagnostics
            .iter()
            .map(|d| {
                let code = d.code.clone().unwrap_or_default();
                let loc = d
                    .span
                    .as_ref()
                    .map_or_else(String::new, |s| format!(" L{}:{}", s.line, s.col));
                ListItem::new(Line::from(vec![
                    Span::styled(status::diag_token(d.level), status::diag_style(d.level, &app.theme)),
                    Span::raw(" "),
                    Span::styled(code, status::diag_style(d.level, &app.theme)),
                    Span::styled(loc, Style::new().fg(app.theme.muted)),
                    Span::raw(" "),
                    Span::raw(truncate(&d.message, 40)),
                ]))
            })
            .collect()
    };
    f.render_widget(
        List::new(diag_items).block(Block::bordered().title(" Diagnostics ")),
        body[1],
    );

    // hint ladder (only when the learner has climbed it). The last rung — the
    // solution outline — is flagged "last resort" and coloured as a warning so it
    // reads as the deliberate end of the ladder, not the default.
    if let Some((level, max, text)) = &data.hint {
        let last = *level >= *max;
        let title = if last {
            format!(" Hint {level}/{max} · last resort  (h to re-show) ")
        } else {
            format!(" Hint {level}/{max}  (h for more) ")
        };
        let col = if last { app.theme.error } else { app.theme.note };
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(text.clone(), Style::new().fg(col))))
                .wrap(Wrap { trim: true })
                .block(Block::bordered().title(title)),
            rows[hint_idx],
        );
    }

    // footer: book refs
    let refs: Vec<Span> = if data.book_refs.is_empty() {
        vec![Span::styled("no book refs for this exercise", Style::new().fg(app.theme.muted))]
    } else {
        data.book_refs
            .iter()
            .enumerate()
            .flat_map(|(i, (chapter, _why))| {
                vec![
                    Span::styled(format!("[{}] ", i + 1), Style::new().fg(app.theme.note)),
                    Span::styled(format!("{chapter}   "), Style::new().fg(app.theme.muted)),
                ]
            })
            .collect()
    };
    f.render_widget(
        Paragraph::new(Line::from(refs)).block(Block::bordered().title(" Book refs ")),
        rows[book_idx],
    );
}

/// Parse one prose line's inline markdown into styled spans so the TUI reader
/// shows clean text instead of raw markers (mirrors the web `mdToHtml` inline
/// pass): `` `code` `` → tinted, `**bold**` → bold, `_emph_`/`*emph*` → italic,
/// `[text](url)` → just the underlined text. Unmatched markers stay literal.
/// `_` uses the GFM intraword rule (only opens at a word boundary) so
/// `snake_case` is never mangled.
fn inline_spans(text: &str, theme: &Theme) -> Vec<Span<'static>> {
    let chars: Vec<char> = text.chars().collect();
    let find = |from: usize, ch: char| (from..chars.len()).find(|&j| chars[j] == ch);
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut plain = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let take = |a: usize, b: usize| chars[a..b].iter().collect::<String>();
        // inline code `...`
        if c == '`' {
            if let Some(end) = find(i + 1, '`') {
                if end > i + 1 {
                    flush_plain(&mut plain, &mut spans);
                    spans.push(Span::styled(take(i + 1, end), Style::new().fg(theme.note)));
                    i = end + 1;
                    continue;
                }
            }
        }
        // bold **...**
        if c == '*' && chars.get(i + 1) == Some(&'*') {
            if let Some(end) = (i + 2..chars.len().saturating_sub(1))
                .find(|&j| chars[j] == '*' && chars[j + 1] == '*')
            {
                flush_plain(&mut plain, &mut spans);
                spans.push(Span::styled(take(i + 2, end), Style::new().add_modifier(Modifier::BOLD)));
                i = end + 2;
                continue;
            }
        }
        // emphasis *...* (single)
        if c == '*' {
            if let Some(end) = find(i + 1, '*') {
                if end > i + 1 {
                    flush_plain(&mut plain, &mut spans);
                    spans.push(Span::styled(take(i + 1, end), Style::new().add_modifier(Modifier::ITALIC)));
                    i = end + 1;
                    continue;
                }
            }
        }
        // emphasis _..._ — only at a word boundary (GFM intraword rule).
        if c == '_' && i.checked_sub(1).is_none_or(|p| !chars[p].is_alphanumeric()) {
            if let Some(end) = (i + 1..chars.len()).find(|&j| {
                chars[j] == '_' && chars.get(j + 1).is_none_or(|n| !n.is_alphanumeric())
            }) {
                if end > i + 1 {
                    flush_plain(&mut plain, &mut spans);
                    spans.push(Span::styled(take(i + 1, end), Style::new().add_modifier(Modifier::ITALIC)));
                    i = end + 1;
                    continue;
                }
            }
        }
        // link [text](url) → keep the text only (underlined to hint it links out).
        if c == '[' {
            if let Some(close) = find(i + 1, ']') {
                if chars.get(close + 1) == Some(&'(') {
                    if let Some(paren) = find(close + 2, ')') {
                        flush_plain(&mut plain, &mut spans);
                        spans.push(Span::styled(
                            take(i + 1, close),
                            Style::new().fg(theme.note).add_modifier(Modifier::UNDERLINED),
                        ));
                        i = paren + 1;
                        continue;
                    }
                }
            }
        }
        plain.push(c);
        i += 1;
    }
    flush_plain(&mut plain, &mut spans);
    if spans.is_empty() {
        spans.push(Span::raw(String::new()));
    }
    spans
}

fn flush_plain(plain: &mut String, spans: &mut Vec<Span<'static>>) {
    if !plain.is_empty() {
        spans.push(Span::raw(std::mem::take(plain)));
    }
}

/// Render the book reader: tab bar, a chapter list (left), and the selected
/// chapter's markdown (right), with light heading styling. `app.selected`
/// chooses the chapter. Stacks on narrow widths.
pub fn render_book(f: &mut Frame, app: &App, book: &Book) {
    let area = f.area();
    let rows = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);
    f.render_widget(tab_bar(app), rows[0]);

    let ids: Vec<&String> = book.chapters.keys().collect();
    if ids.is_empty() {
        f.render_widget(
            Paragraph::new("\n  No book content yet — run `rpro init` to seed the bundled chapters.")
                .block(Block::bordered().title(" Book ")),
            rows[1],
        );
        return;
    }
    let sel = app.selected.min(ids.len() - 1);

    let body = if App::is_narrow(area.width) {
        Layout::vertical([Constraint::Length(7), Constraint::Min(0)]).split(rows[1])
    } else {
        Layout::horizontal([Constraint::Percentage(32), Constraint::Percentage(68)]).split(rows[1])
    };

    // chapter list (left)
    let items: Vec<ListItem> = ids.iter().map(|id| ListItem::new((*id).clone())).collect();
    let list = List::new(items)
        .block(Block::bordered().title(format!(" Chapters ({}) ", ids.len())))
        .highlight_style(Style::new().fg(app.theme.accent).add_modifier(Modifier::BOLD))
        .highlight_symbol("▸ ");
    let mut state = ListState::default();
    state.select(Some(sel));
    f.render_stateful_widget(list, body[0], &mut state);

    // chapter content (right): cleaned for display (mdBook directives/hidden
    // lines stripped, un-bundled listings linked out — see rpro-book), with
    // light heading styling and code-fence lines hidden + bodies tinted.
    let chapter = &book.chapters[ids[sel]];
    let url = RustLanguage.book_ref_url(&BookRef {
        chapter: chapter.id.clone(),
        anchor: None,
        why: String::new(),
    });
    let md = chapter.display_markdown(&url);
    let mut in_code = false;
    let lines: Vec<Line> = md
        .lines()
        .filter_map(|raw| {
            let t = raw.trim_start();
            if t.starts_with("```") {
                in_code = !in_code; // swallow the fence line itself
                return None;
            }
            if in_code {
                return Some(Line::from(Span::styled(
                    format!("  {raw}"),
                    Style::new().fg(app.theme.note),
                )));
            }
            if let Some(h) = t.strip_prefix('#') {
                Some(Line::from(Span::styled(
                    h.trim_start_matches('#').trim().to_string(),
                    Style::new().fg(app.theme.accent).add_modifier(Modifier::BOLD),
                )))
            } else if let Some(b) = t.strip_prefix("> ").or_else(|| (t == ">").then_some("")) {
                // Blockquote (the Book's many Note/Warning callouts): a muted,
                // marked left rail instead of a literal `>`. ASCII-safe rail.
                let rail = if app.ascii { "| " } else { "▏ " };
                Some(Line::from(Span::styled(
                    format!("{rail}{b}"),
                    Style::new().fg(app.theme.muted).add_modifier(Modifier::ITALIC),
                )))
            } else {
                // Prose line: render inline markdown as styled spans (no raw
                // `_`/`**`/`` ` ``/`[]()` markers leaking into the reader).
                Some(Line::from(inline_spans(raw, &app.theme)))
            }
        })
        .collect();
    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((app.scroll, 0))
            .block(Block::bordered().title(format!(" {}  (PgUp/PgDn) ", ids[sel]))),
        body[1],
    );
}

/// Render the roadmap: the tab bar over a styled list parsed from a
/// checkbox-markdown string (`## Section`, `- [x]/[>]/[ ] task`, prose). Status
/// boxes get the legend's glyphs + colors; ASCII mode keeps the raw `[x]`.
pub fn render_roadmap(f: &mut Frame, app: &App, content: &str) {
    let area = f.area();
    let rows = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);
    f.render_widget(tab_bar(app), rows[0]);

    let t = &app.theme;
    let items: Vec<ListItem> = content
        .lines()
        .map(|line| {
            let s = line.trim_start();
            let task = |glyph: &str, rest: &str, style: Style| {
                let g = if app.ascii { None } else { Some(glyph) };
                let mut spans = vec![Span::raw("  ")];
                if let Some(g) = g {
                    spans.push(Span::styled(format!("{g} "), style));
                }
                spans.push(Span::styled(rest.to_string(), style));
                ListItem::new(Line::from(spans))
            };
            if let Some(h) = s.strip_prefix("## ") {
                ListItem::new(Line::from(Span::styled(
                    h.to_string(),
                    Style::new().fg(t.accent).add_modifier(Modifier::BOLD),
                )))
            } else if let Some(h) = s.strip_prefix("# ") {
                ListItem::new(Line::from(Span::styled(
                    h.to_string(),
                    Style::new().add_modifier(Modifier::BOLD),
                )))
            } else if let Some(r) = s.strip_prefix("- [x]") {
                task("✓", r.trim_start(), Style::new().fg(t.done))
            } else if let Some(r) = s.strip_prefix("- [>]") {
                task("▸", r.trim_start(), Style::new().fg(t.current).add_modifier(Modifier::BOLD))
            } else if let Some(r) = s.strip_prefix("- [ ]") {
                task("▢", r.trim_start(), Style::new().fg(t.locked))
            } else {
                ListItem::new(Line::from(Span::styled(
                    line.to_string(),
                    Style::new().fg(t.muted),
                )))
            }
        })
        .collect();
    f.render_widget(
        List::new(items).block(Block::bordered().title(" Roadmap ")),
        rows[1],
    );
}

/// A simple "coming soon" screen for tabs whose full screen isn't built yet,
/// keeping the tab bar so navigation stays consistent.
pub fn render_placeholder(f: &mut Frame, app: &App, title: &str) {
    let area = f.area();
    let rows = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);
    f.render_widget(tab_bar(app), rows[0]);
    f.render_widget(
        Paragraph::new(format!("\n  {title} — screen coming soon"))
            .block(Block::bordered().title(format!(" {title} "))),
        rows[1],
    );
}

/// Truncate to `max` chars with an ellipsis (display-only).
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let head: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{head}…")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn screen_text(width: u16, height: u16, data: &DashboardData) -> String {
        let app = App::new(Theme::dark(), true);
        let mut term = Terminal::new(TestBackend::new(width, height)).unwrap();
        term.draw(|f| render_dashboard(f, &app, data)).unwrap();
        let buf = term.backend().buffer();
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn dashboard_renders_title_tabs_and_progress() {
        let data = DashboardData {
            done: 12,
            total: 40,
            current_id: Some("ownership/01_move".into()),
            current_title: Some("Move semantics".into()),
            up_next: vec!["ownership/02_clone".into()],
            ..Default::default()
        };
        let text = screen_text(80, 20, &data);
        assert!(text.contains("Tempered Studio"), "brand title missing:\n{text}");
        assert!(text.contains("Dashboard") && text.contains("Roadmap"), "tabs missing");
        assert!(text.contains("12/40"), "gauge counts missing");
        assert!(text.contains("30%"), "gauge pct missing:\n{text}");
        assert!(text.contains("ownership/01_move"), "current id missing");
        assert!(text.contains("ownership/02_clone"), "up-next missing");
    }

    #[test]
    fn dashboard_handles_empty_state() {
        let text = screen_text(80, 20, &DashboardData::default());
        assert!(text.contains("0/0"));
        assert!(text.contains("All caught up") || text.contains("nothing in progress"));
        assert!(text.contains("rpro init")); // empty up-next hint
    }

    // Recall codes are fake non-E0 tokens on purpose: keeps rpro-tui source
    // seam-grep-clean (the gate only forbids the literal `E0xxx`).
    #[test]
    fn dashboard_shows_recall_queue_when_tracked() {
        let data = DashboardData {
            done: 3,
            total: 23,
            current_id: Some("control-flow/01".into()),
            current_title: Some("If is an expression".into()),
            up_next: vec!["control-flow/02".into()],
            due: vec!["E4321".into(), "E4399".into()],
            mastered: 1,
            tracked: 3,
        };
        let text = screen_text(80, 24, &data);
        assert!(text.contains("Recall"), "recall panel missing:\n{text}");
        assert!(text.contains("E4321") && text.contains("E4399"), "due codes missing:\n{text}");
        assert!(text.contains("1/3 mastered"), "mastery count missing:\n{text}");
    }

    #[test]
    fn dashboard_recall_hidden_until_tracked() {
        // tracked == 0 → no Recall panel (a fresh learner sees absence).
        let data = DashboardData { total: 10, up_next: vec!["a/1".into()], ..Default::default() };
        let text = screen_text(80, 24, &data);
        assert!(!text.contains("Recall"), "recall must hide at tracked=0:\n{text}");
    }

    #[test]
    fn dashboard_recall_all_mastered_message() {
        let data = DashboardData {
            total: 10,
            done: 5,
            tracked: 4,
            mastered: 4,
            due: vec![],
            ..Default::default()
        };
        let text = screen_text(80, 24, &data);
        assert!(text.contains("Recall"), "recall panel missing:\n{text}");
        assert!(text.contains("4 concepts") && text.contains("mastered"), "all-mastered msg missing:\n{text}");
    }

    fn screen_text_ex(width: u16, height: u16, data: &ExerciseViewData) -> String {
        let app = App::new(Theme::dark(), true);
        let mut term = Terminal::new(TestBackend::new(width, height)).unwrap();
        term.draw(|f| render_exercise(f, &app, data)).unwrap();
        let buf = term.backend().buffer();
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn exercise_shows_raw_output_diagnostics_and_mode() {
        // Fake non-E0 code on purpose: keeps rpro-tui source seam-grep-clean.
        use rpro_lang::{DiagLevel, Span as DiagSpan};
        let data = ExerciseViewData {
            id: "ownership/01_move".into(),
            title: "Move semantics".into(),
            raw_stderr: "error[E4321]: borrow of moved value: `s1`".into(),
            diagnostics: vec![Diagnostic {
                code: Some("E4321".into()),
                level: DiagLevel::Error,
                message: "borrow of moved value".into(),
                span: Some(DiagSpan { file: "main.rs".into(), line: 7, col: 5 }),
            }],
            assists: EditorAssists::default(), // all on = Free
            ..Default::default()
        };
        let text = screen_text_ex(90, 24, &data);
        assert!(text.contains("ownership/01_move"), "id missing:\n{text}");
        assert!(text.contains("borrow of moved"), "raw output missing");
        assert!(text.contains("[FREE]"), "mode badge missing");
        assert!(text.contains("L7:5"), "diagnostic span missing");
    }

    #[test]
    fn exercise_empty_state_keeps_raw_region() {
        let text = screen_text_ex(
            90,
            24,
            &ExerciseViewData { id: "x/1".into(), assists: EditorAssists::default(), ..Default::default() },
        );
        assert!(text.contains("no output yet"));
        assert!(text.contains("[FREE]"));
    }

    #[test]
    fn exercise_shows_hint_when_climbed() {
        // "EXXXX" is a fake non-E0 token (seam-clean); the hint text is opaque here.
        let data = ExerciseViewData {
            id: "x/1".into(),
            title: "T".into(),
            assists: EditorAssists::default(),
            hint: Some((2, 3, "Expect error EXXXX. Read the help line.".into())),
            ..Default::default()
        };
        let text = screen_text_ex(90, 24, &data);
        assert!(text.contains("Hint 2/3"), "hint title missing:\n{text}");
        assert!(text.contains("Expect error EXXXX"), "hint text missing:\n{text}");
    }

    #[test]
    fn exercise_hint_hidden_until_climbed() {
        let data =
            ExerciseViewData { id: "x/1".into(), assists: EditorAssists::default(), ..Default::default() };
        let text = screen_text_ex(90, 24, &data);
        assert!(!text.contains("Hint "), "no hint row until the learner climbs:\n{text}");
    }

    #[test]
    fn book_reader_lists_chapters_and_renders_selected_markdown() {
        use rpro_book::{Book, Chapter};
        use std::collections::BTreeMap;
        use std::path::PathBuf;
        let mut chapters = BTreeMap::new();
        chapters.insert(
            "ch03-01-variables".to_string(),
            Chapter {
                id: "ch03-01-variables".into(),
                path: PathBuf::from("a"),
                markdown: "# Variables\nLet bindings are immutable by default.".into(),
            },
        );
        chapters.insert(
            "ch04-01-ownership".to_string(),
            Chapter {
                id: "ch04-01-ownership".into(),
                path: PathBuf::from("b"),
                markdown: "# Ownership\nMove semantics.".into(),
            },
        );
        let book = Book { chapters };
        let app = App::new(Theme::dark(), true); // selected = 0 → first (ch03) chapter
        let mut term = Terminal::new(TestBackend::new(90, 20)).unwrap();
        term.draw(|f| render_book(f, &app, &book)).unwrap();
        let buf = term.backend().buffer();
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
        }
        assert!(s.contains("ch03-01-variables"), "chapter list missing:\n{s}");
        assert!(s.contains("ch04-01-ownership"), "second chapter missing");
        assert!(s.contains("Variables"), "selected heading missing");
        assert!(s.contains("immutable"), "selected content missing");
    }

    #[test]
    fn book_reader_cleans_mdbook_directives() {
        use rpro_book::{Book, Chapter};
        use std::collections::BTreeMap;
        use std::path::PathBuf;
        let mut chapters = BTreeMap::new();
        chapters.insert(
            "ch04-01-what-is-ownership".to_string(),
            Chapter {
                id: "ch04-01-what-is-ownership".into(),
                path: PathBuf::from("a"),
                markdown: "# Ownership\n> Note: borrows don't own.\n```rust\n{{#rustdoc_include ../listings/x:here}}\n```\n".into(),
            },
        );
        let book = Book { chapters };
        let app = App::new(Theme::dark(), true);
        let mut term = Terminal::new(TestBackend::new(110, 20)).unwrap();
        term.draw(|f| render_book(f, &app, &book)).unwrap();
        let buf = term.backend().buffer();
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
        }
        assert!(!s.contains("{{#"), "raw mdBook directive leaked into the TUI:\n{s}");
        assert!(!s.contains("```"), "code fence line not hidden:\n{s}");
        assert!(s.contains("Read this code listing"), "missing link-out callout:\n{s}");
        // Blockquote rendered as a marked rail, not a literal `>` (ascii mode → `|`).
        assert!(s.contains("| Note: borrows"), "blockquote not styled as a rail:\n{s}");
        assert!(!s.contains("> Note:"), "literal blockquote marker leaked:\n{s}");
    }

    #[test]
    fn roadmap_renders_sections_and_tasks() {
        let content = "## Phase 0\n- [x] seam done\n- [>] live run\n- [ ] lessons\nprose line\n";
        let app = App::new(Theme::dark(), true);
        let mut term = Terminal::new(TestBackend::new(60, 16)).unwrap();
        term.draw(|f| render_roadmap(f, &app, content)).unwrap();
        let buf = term.backend().buffer();
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
        }
        assert!(s.contains("Phase 0"), "section missing:\n{s}");
        assert!(s.contains("seam done") && s.contains("live run") && s.contains("lessons"));
        assert!(s.contains("Roadmap"), "tab/title missing");
    }

    #[test]
    fn book_content_scrolls() {
        use rpro_book::{Book, Chapter};
        use std::collections::BTreeMap;
        use std::path::PathBuf;
        let md = (0..20).map(|i| format!("line-{i:02}")).collect::<Vec<_>>().join("\n");
        let mut chapters = BTreeMap::new();
        chapters.insert(
            "ch".to_string(),
            Chapter { id: "ch".into(), path: PathBuf::from("a"), markdown: md },
        );
        let book = Book { chapters };
        let render = |scroll: u16| {
            let mut app = App::new(Theme::dark(), true);
            app.scroll = scroll;
            let mut term = Terminal::new(TestBackend::new(60, 16)).unwrap();
            term.draw(|f| render_book(f, &app, &book)).unwrap();
            let buf = term.backend().buffer();
            let mut s = String::new();
            for y in 0..buf.area.height {
                for x in 0..buf.area.width {
                    s.push_str(buf[(x, y)].symbol());
                }
            }
            s
        };
        assert!(render(0).contains("line-00"), "top line shows at scroll 0");
        let scrolled = render(8);
        assert!(!scrolled.contains("line-00"), "line-00 should scroll off");
        assert!(scrolled.contains("line-08"), "later lines show when scrolled");
    }

    #[test]
    fn inline_renders_markdown_as_spans_without_raw_markers() {
        let th = Theme::dark();
        let sp = inline_spans("the _rules_ of `String`, see **this** and [docs](http://x.io)", &th);
        let joined: String = sp.iter().map(|s| s.content.as_ref()).collect();
        // The styled content survives; the raw markers do not.
        assert!(joined.contains("rules") && joined.contains("String") && joined.contains("this") && joined.contains("docs"));
        assert!(!joined.contains('_') && !joined.contains('`') && !joined.contains('*') && !joined.contains('['),
            "raw markers leaked: {joined:?}");
        assert!(!joined.contains("http://x.io"), "link url dropped, text kept: {joined:?}");
        // The right spans carry the right styles.
        assert!(sp.iter().any(|s| s.content == "rules" && s.style.add_modifier.contains(Modifier::ITALIC)));
        assert!(sp.iter().any(|s| s.content == "this" && s.style.add_modifier.contains(Modifier::BOLD)));
        assert!(sp.iter().any(|s| s.content == "String" && s.style.fg == Some(th.note)));
        assert!(sp.iter().any(|s| s.content == "docs" && s.style.add_modifier.contains(Modifier::UNDERLINED)));
    }

    #[test]
    fn inline_preserves_snake_case_and_unmatched_markers() {
        let th = Theme::dark();
        let sp = inline_spans("call to_string and a * b here", &th);
        let joined: String = sp.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(joined, "call to_string and a * b here", "intraword _ and lone * stay literal");
        assert!(sp.iter().all(|s| !s.style.add_modifier.contains(Modifier::ITALIC)), "nothing italicised");
    }

    #[test]
    fn book_reader_styles_inline_markup() {
        use rpro_book::{Book, Chapter};
        use std::collections::BTreeMap;
        use std::path::PathBuf;
        let mut chapters = BTreeMap::new();
        chapters.insert(
            "ch".to_string(),
            Chapter {
                id: "ch".into(),
                path: PathBuf::from("a"),
                markdown: "# H\nVariables are _immutable_ by default; use `mut` to opt in.".into(),
            },
        );
        let book = Book { chapters };
        let app = App::new(Theme::dark(), true);
        let mut term = Terminal::new(TestBackend::new(90, 12)).unwrap();
        term.draw(|f| render_book(f, &app, &book)).unwrap();
        let buf = term.backend().buffer();
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
        }
        assert!(s.contains("immutable") && s.contains("mut"), "inline content present:\n{s}");
        assert!(!s.contains("_immutable_"), "literal underscore markers leaked:\n{s}");
        assert!(!s.contains("`mut`"), "literal backtick markers leaked:\n{s}");
    }
}
