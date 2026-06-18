//! Status vocabulary: ASCII tokens, glyphs, styles, and the run throbber.
//!
//! The **ASCII text token is canonical** — it is load-bearing for alignment on
//! phone terminals and is the only form rendered under `ascii_only` /
//! `NO_COLOR`. The glyph is the prettier display rune; color (via [`Theme`]) is
//! always a *third* channel, never the only one — every status is also
//! distinguished by its token/glyph so it survives a monochrome terminal and
//! color-blind readers. Mirrors the shared legend in `docs/UX.md`.

use crate::theme::Theme;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;
use rpro_lang::{DiagLevel, EditorAssists};
use rpro_state::ExerciseStatus;

/// Whether to suppress non-ASCII glyphs/emoji and render the ASCII tokens only.
///
/// True unless the locale clearly advertises UTF-8 (`LC_ALL` / `LC_CTYPE` /
/// `LANG`). Conservative on purpose: a misrendered double-width glyph breaks
/// column alignment on Termux, so we only opt *in* to glyphs when UTF-8 is sure.
#[must_use]
pub fn ascii_only() -> bool {
    let utf8 = ["LC_ALL", "LC_CTYPE", "LANG"].iter().any(|k| {
        std::env::var(k).is_ok_and(|v| {
            let v = v.to_ascii_uppercase();
            v.contains("UTF-8") || v.contains("UTF8")
        })
    });
    !utf8
}

/// The canonical ASCII token for an exercise status.
#[must_use]
pub const fn status_token(s: ExerciseStatus) -> &'static str {
    match s {
        ExerciseStatus::Locked => "[ ]",
        ExerciseStatus::Current => "[>]",
        ExerciseStatus::Done => "[x]",
        ExerciseStatus::Skipped => "[~]",
    }
}

/// The display glyph for an exercise status — falls back to the ASCII token
/// when `ascii` is set.
#[must_use]
pub const fn status_glyph(s: ExerciseStatus, ascii: bool) -> &'static str {
    if ascii {
        return status_token(s);
    }
    match s {
        ExerciseStatus::Locked => "·",
        ExerciseStatus::Current => "▸",
        ExerciseStatus::Done => "✓",
        ExerciseStatus::Skipped => "»",
    }
}

/// The style for an exercise status, given the active theme.
#[must_use]
pub const fn status_style(s: ExerciseStatus, t: &Theme) -> Style {
    match s {
        ExerciseStatus::Locked => Style::new().fg(t.locked).add_modifier(Modifier::DIM),
        ExerciseStatus::Current => Style::new().fg(t.current).add_modifier(Modifier::BOLD),
        ExerciseStatus::Done => Style::new().fg(t.done),
        ExerciseStatus::Skipped => Style::new().fg(t.skipped),
    }
}

/// A ready-to-render styled glyph for an exercise status.
#[must_use]
pub fn status_cell(s: ExerciseStatus, t: &Theme, ascii: bool) -> Span<'static> {
    Span::styled(status_glyph(s, ascii).to_string(), status_style(s, t))
}

/// The canonical ASCII token for a diagnostic level.
#[must_use]
pub const fn diag_token(l: DiagLevel) -> &'static str {
    match l {
        DiagLevel::Error => "[!]",
        DiagLevel::Warning => "[w]",
        DiagLevel::Note => "[i]",
    }
}

/// The style for a diagnostic level, given the active theme.
#[must_use]
pub const fn diag_style(l: DiagLevel, t: &Theme) -> Style {
    let c = match l {
        DiagLevel::Error => t.error,
        DiagLevel::Warning => t.warn,
        DiagLevel::Note => t.note,
    };
    Style::new().fg(c)
}

/// Whether every editor assist is on — i.e. **Free mode**.
#[must_use]
pub const fn is_free(a: EditorAssists) -> bool {
    a.syntax_highlight && a.autocomplete && a.inline_diagnostics && a.format_on_save
}

/// The mode badge token: `[FREE]` (all assists on) or `[LEARN]` (any off).
#[must_use]
pub const fn mode_badge(a: EditorAssists) -> &'static str {
    if is_free(a) { "[FREE]" } else { "[LEARN]" }
}

/// An animated run spinner. The event loop advances it on each poll-timeout
/// tick and reads [`frame`](Throbber::frame); there is no ratatui motion widget,
/// so this is a frame-array `Span` pattern.
#[derive(Debug, Clone, Copy)]
pub struct Throbber {
    tick: u64,
    ascii: bool,
}

impl Throbber {
    /// A throbber that uses braille frames, or `|/-\` when `ascii` is set.
    #[must_use]
    pub const fn new(ascii: bool) -> Self {
        Self { tick: 0, ascii }
    }

    /// Advance one animation step (call on each event-loop tick).
    pub const fn advance(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    /// The current frame glyph.
    #[must_use]
    pub const fn frame(&self) -> &'static str {
        const BRAILLE: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        const ASCII: [&str; 4] = ["|", "/", "-", "\\"];
        // Divide by 2 so the animation reads at a comfortable speed at an ~80ms
        // poll tick.
        if self.ascii {
            ASCII[(self.tick / 2) as usize % ASCII.len()]
        } else {
            BRAILLE[(self.tick / 2) as usize % BRAILLE.len()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_distinct_and_ascii() {
        let all = [
            ExerciseStatus::Locked,
            ExerciseStatus::Current,
            ExerciseStatus::Done,
            ExerciseStatus::Skipped,
        ];
        let toks: Vec<_> = all.iter().map(|s| status_token(*s)).collect();
        // distinct
        for (i, a) in toks.iter().enumerate() {
            for b in &toks[i + 1..] {
                assert_ne!(a, b);
            }
            assert!(a.is_ascii());
        }
    }

    #[test]
    fn ascii_glyph_falls_back_to_token() {
        assert_eq!(
            status_glyph(ExerciseStatus::Done, true),
            status_token(ExerciseStatus::Done)
        );
        assert_eq!(status_glyph(ExerciseStatus::Done, false), "✓");
    }

    #[test]
    fn mode_badge_reflects_assists() {
        let free = EditorAssists::default();
        assert!(is_free(free));
        assert_eq!(mode_badge(free), "[FREE]");
        let learning = EditorAssists { inline_diagnostics: false, ..EditorAssists::default() };
        assert!(!is_free(learning));
        assert_eq!(mode_badge(learning), "[LEARN]");
    }

    #[test]
    fn throbber_cycles() {
        let mut t = Throbber::new(true);
        let f0 = t.frame();
        // advance through one full cycle (4 frames * 2 ticks) back to start
        for _ in 0..8 {
            t.advance();
        }
        assert_eq!(t.frame(), f0);
        // ascii frames never emit a braille rune
        assert!(t.frame().is_ascii());
    }
}
