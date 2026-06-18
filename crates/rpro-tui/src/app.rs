//! TUI application state — the model the event loop drives and the screens
//! render. Pure: no terminal, no I/O, so it's fully unit-testable.

use crate::status::Throbber;
use crate::theme::Theme;

/// Top-level screens, shown as tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// Progress + current exercise at a glance.
    Dashboard,
    /// The exercise view (raw compiler output + diagnostics).
    Exercise,
    /// The embedded Rust Book reader.
    Book,
    /// Roadmap / tasks.
    Roadmap,
}

impl Tab {
    /// Every tab, in display order.
    pub const ALL: [Self; 4] = [Self::Dashboard, Self::Exercise, Self::Book, Self::Roadmap];

    /// The tab's display title.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Exercise => "Exercise",
            Self::Book => "Book",
            Self::Roadmap => "Roadmap",
        }
    }

    /// The tab's position in [`Tab::ALL`].
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Dashboard => 0,
            Self::Exercise => 1,
            Self::Book => 2,
            Self::Roadmap => 3,
        }
    }

    /// The next tab, wrapping around.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::ALL[(self.index() + 1) % Self::ALL.len()]
    }

    /// The previous tab, wrapping around.
    #[must_use]
    pub const fn prev(self) -> Self {
        Self::ALL[(self.index() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

/// Below this terminal width, screens stack into a single column (phone reflow).
pub const NARROW_COLS: u16 = 50;

/// The running TUI application state.
#[derive(Debug, Clone)]
pub struct App {
    /// The active screen.
    pub tab: Tab,
    /// Resolved color theme.
    pub theme: Theme,
    /// Suppress non-ASCII glyphs/emoji (canonical tokens only).
    pub ascii: bool,
    /// Run spinner (advanced on each idle tick).
    pub throbber: Throbber,
    /// A `cargo run`/check is in flight.
    pub running: bool,
    /// The event loop should exit.
    pub should_quit: bool,
    /// Selected row in the active list.
    pub selected: usize,
    /// Length of the active list (for clamping selection).
    pub list_len: usize,
    /// Vertical scroll offset (lines) for the active scrollable pane.
    pub scroll: u16,
    /// Hint-ladder rung currently revealed on the exercise view (0 = none).
    /// Sticky to the current exercise; reset to 0 only when the exercise changes.
    pub hint_level: u8,
}

impl App {
    /// Lines moved per scroll keypress.
    const SCROLL_STEP: u16 = 3;

    /// A fresh app on the dashboard.
    #[must_use]
    pub const fn new(theme: Theme, ascii: bool) -> Self {
        Self {
            tab: Tab::Dashboard,
            theme,
            ascii,
            throbber: Throbber::new(ascii),
            running: false,
            should_quit: false,
            selected: 0,
            list_len: 0,
            scroll: 0,
            hint_level: 0,
        }
    }

    /// Move to the next tab.
    pub const fn next_tab(&mut self) {
        self.tab = self.tab.next();
        self.selected = 0;
        self.scroll = 0;
    }

    /// Move to the previous tab.
    pub const fn prev_tab(&mut self) {
        self.tab = self.tab.prev();
        self.selected = 0;
        self.scroll = 0;
    }

    /// Scroll the active pane down by `STEP` lines.
    pub const fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(Self::SCROLL_STEP);
    }

    /// Scroll the active pane up by `STEP` lines (clamped at the top).
    pub const fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(Self::SCROLL_STEP);
    }

    /// Set the active list length, clamping the selection into range.
    pub const fn set_list_len(&mut self, n: usize) {
        self.list_len = n;
        if n == 0 {
            self.selected = 0;
        } else if self.selected >= n {
            self.selected = n - 1;
        }
    }

    /// Move selection down one row (clamped at the bottom); resets scroll so a
    /// newly selected item (e.g. a book chapter) starts at the top.
    pub const fn select_next(&mut self) {
        if self.list_len > 0 && self.selected + 1 < self.list_len {
            self.selected += 1;
            self.scroll = 0;
        }
    }

    /// Move selection up one row (clamped at the top); resets scroll.
    pub const fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.scroll = 0;
        }
    }

    /// Advance the spinner one animation step (call on each idle tick).
    pub const fn tick(&mut self) {
        self.throbber.advance();
    }

    /// Request exit.
    pub const fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Whether a terminal this wide should use the stacked (phone) layout.
    #[must_use]
    pub const fn is_narrow(width: u16) -> bool {
        width < NARROW_COLS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        App::new(Theme::dark(), true)
    }

    #[test]
    fn tabs_cycle_and_wrap() {
        assert_eq!(Tab::Dashboard.next(), Tab::Exercise);
        assert_eq!(Tab::Roadmap.next(), Tab::Dashboard); // wraps
        assert_eq!(Tab::Dashboard.prev(), Tab::Roadmap); // wraps
        // ALL indices line up
        for (i, t) in Tab::ALL.iter().enumerate() {
            assert_eq!(t.index(), i);
        }
    }

    #[test]
    fn next_prev_tab_reset_selection() {
        let mut a = app();
        a.set_list_len(5);
        a.select_next();
        a.select_next();
        assert_eq!(a.selected, 2);
        a.next_tab();
        assert_eq!(a.tab, Tab::Exercise);
        assert_eq!(a.selected, 0);
    }

    #[test]
    fn selection_clamps_at_both_ends() {
        let mut a = app();
        a.set_list_len(3);
        a.select_prev(); // already at 0
        assert_eq!(a.selected, 0);
        a.select_next();
        a.select_next();
        a.select_next(); // would be 3, clamp to 2
        assert_eq!(a.selected, 2);
    }

    #[test]
    fn set_list_len_clamps_existing_selection() {
        let mut a = app();
        a.set_list_len(5);
        a.select_next();
        a.select_next();
        a.select_next(); // selected = 3
        a.set_list_len(2); // shrink
        assert_eq!(a.selected, 1);
        a.set_list_len(0);
        assert_eq!(a.selected, 0);
    }

    #[test]
    fn narrow_threshold() {
        assert!(App::is_narrow(40));
        assert!(!App::is_narrow(80));
        assert!(!App::is_narrow(NARROW_COLS));
    }

    #[test]
    fn quit_sets_flag() {
        let mut a = app();
        assert!(!a.should_quit);
        a.quit();
        assert!(a.should_quit);
    }

    #[test]
    fn scroll_moves_clamps_and_resets() {
        let mut a = app();
        assert_eq!(a.scroll, 0);
        a.scroll_up(); // clamp at top
        assert_eq!(a.scroll, 0);
        a.scroll_down();
        a.scroll_down();
        assert_eq!(a.scroll, App::SCROLL_STEP * 2);
        a.scroll_up();
        assert_eq!(a.scroll, App::SCROLL_STEP);
        a.next_tab(); // tab change resets scroll
        assert_eq!(a.scroll, 0);
        a.set_list_len(5);
        a.scroll_down();
        assert!(a.scroll > 0);
        a.select_next(); // new selection resets scroll
        assert_eq!(a.scroll, 0);
    }
}
