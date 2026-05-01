//! User preferences. Stored as TOML for hand-editability.

use serde::{Deserialize, Serialize};

/// User preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Color theme — currently `"dark"` or `"light"`. The TUI
    /// auto-detects but this overrides.
    #[serde(default = "default_theme")]
    pub theme: String,
    /// External editor command (`"$EDITOR"` shells out, otherwise
    /// the literal binary). The TUI's "open file in editor" key
    /// uses this. Default is `nano` because Rustlings-Pro is
    /// beginner-leaning.
    #[serde(default = "default_editor")]
    pub editor: String,
    /// Whether `rpro hint` opens book references in the in-app
    /// reader (true) or in the system browser (false).
    #[serde(default = "default_in_app_book")]
    pub in_app_book: bool,
}

fn default_theme() -> String {
    "dark".into()
}

fn default_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| "nano".into())
}

const fn default_in_app_book() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            editor: default_editor(),
            in_app_book: default_in_app_book(),
        }
    }
}
