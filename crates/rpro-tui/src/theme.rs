//! Color theming for the TUI.
//!
//! One [`Theme`] of named roles that every screen draws from, resolved once at
//! startup from `Config.theme` plus the environment. Three palettes: `dark`,
//! `light`, and `mono` (the `NO_COLOR` / non-color-terminal fallback, where
//! weight + glyph carry all meaning and color carries none).
//!
//! Colors use named ANSI indices as the base layer so 16-color and light/dark
//! terminals stay legible; the brand accent is the one truecolor value (it
//! degrades to the nearest ANSI on limited terminals). See `docs/UX.md`.

use ratatui::style::Color;

/// Which palette to render with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Dark terminal background.
    Dark,
    /// Light terminal background.
    Light,
    /// No color — meaning carried by glyphs + text weight only.
    Mono,
}

/// The resolved set of semantic colors every screen draws from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    /// Which palette this is.
    pub mode: Mode,
    /// Brand accent — titles, selected tab, focus border.
    pub accent: Color,
    /// Completed work — done glyph, gauge fill.
    pub done: Color,
    /// The current exercise — current glyph, cursor row.
    pub current: Color,
    /// Locked / not-yet-available rows (dim).
    pub locked: Color,
    /// Skipped rows.
    pub skipped: Color,
    /// `DiagLevel::Error`.
    pub error: Color,
    /// `DiagLevel::Warning`.
    pub warn: Color,
    /// `DiagLevel::Note`.
    pub note: Color,
    /// Keybinding hints, footers, secondary chrome.
    pub muted: Color,
}

impl Theme {
    /// The dark palette (default).
    #[must_use]
    pub const fn dark() -> Self {
        Self {
            mode: Mode::Dark,
            accent: Color::Rgb(247, 76, 0), // Ferris orange
            done: Color::Green,
            current: Color::Cyan,
            locked: Color::DarkGray,
            skipped: Color::Yellow,
            error: Color::Red,
            warn: Color::Yellow,
            note: Color::Blue,
            muted: Color::DarkGray,
        }
    }

    /// The light palette.
    #[must_use]
    pub const fn light() -> Self {
        Self {
            mode: Mode::Light,
            accent: Color::Rgb(183, 65, 14),
            done: Color::Green,
            current: Color::Blue,
            locked: Color::Gray,
            skipped: Color::Rgb(150, 120, 0),
            error: Color::Red,
            warn: Color::Yellow,
            note: Color::Blue,
            muted: Color::Gray,
        }
    }

    /// The monochrome palette — every role is `Reset`, so nothing is colored.
    /// Used when `NO_COLOR` is set or the terminal can't do color; glyph + text
    /// weight must carry all meaning.
    #[must_use]
    pub const fn mono() -> Self {
        let c = Color::Reset;
        Self {
            mode: Mode::Mono,
            accent: c,
            done: c,
            current: c,
            locked: c,
            skipped: c,
            error: c,
            warn: c,
            note: c,
            muted: c,
        }
    }

    /// Resolve from a config theme string and an explicit `no_color` flag.
    /// `no_color` wins; otherwise `"light"` → light, anything else → dark.
    #[must_use]
    pub fn resolve(config_theme: &str, no_color: bool) -> Self {
        if no_color {
            return Self::mono();
        }
        match config_theme.trim().to_ascii_lowercase().as_str() {
            "light" => Self::light(),
            _ => Self::dark(),
        }
    }

    /// Resolve from a config theme string, reading `NO_COLOR` from the
    /// environment (its mere presence disables color, per the `NO_COLOR`
    /// convention).
    #[must_use]
    pub fn from_env(config_theme: &str) -> Self {
        Self::resolve(config_theme, std::env::var_os("NO_COLOR").is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_forces_mono_regardless_of_theme() {
        let t = Theme::resolve("light", true);
        assert_eq!(t.mode, Mode::Mono);
        assert_eq!(t.accent, Color::Reset);
        assert_eq!(t.error, Color::Reset);
    }

    #[test]
    fn theme_string_selects_palette() {
        assert_eq!(Theme::resolve("light", false).mode, Mode::Light);
        assert_eq!(Theme::resolve("dark", false).mode, Mode::Dark);
        assert_eq!(Theme::resolve("LIGHT", false).mode, Mode::Light); // case-insensitive
        assert_eq!(Theme::resolve("anything-else", false).mode, Mode::Dark); // default
    }
}
