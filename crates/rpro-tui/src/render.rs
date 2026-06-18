//! Screen rendering. Render functions are pure over (`App` + screen data), so
//! they're snapshot-testable with `ratatui`'s `TestBackend` — no live terminal.

use crate::app::{App, Tab};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Gauge, List, ListItem, Paragraph, Tabs};
use ratatui::Frame;

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
    let rows = Layout::vertical([
        Constraint::Length(3), // tabs
        Constraint::Length(3), // gauge
        Constraint::Length(4), // current
        Constraint::Min(0),    // up next
    ])
    .split(area);

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
        rows[3],
    );
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
}
