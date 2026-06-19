//! `rpro-storage-fs` — the filesystem implementation of the `rpro-lang`
//! [`Storage`](rpro_lang::Storage) seam, plus the typed [`Store`] the native
//! shells use directly.
//!
//! Everything the learner produces (progress, bookmarks, annotations,
//! preferences, their own code) lives as a plain JSON / TOML file under
//! `~/.rustlings-pro/`. No database; a learner can `git init` that directory
//! and version-control their notes. Not wasm-safe (uses `std::fs` + `dirs`);
//! the web surface uses a remote state-API instead.

use rpro_state::{Annotations, Bookmarks, Config, Progress};
use std::path::{Path, PathBuf};

/// Errors loading or saving state.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// I/O reading or writing.
    #[error("{kind} {path}: {source}")]
    Io {
        /// "load", "save", "mkdir" or "rename".
        kind: &'static str,
        /// File that failed.
        path: String,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// JSON parse / write failure.
    #[error("json {path}: {source}")]
    Json {
        /// File that failed.
        path: String,
        /// Underlying error.
        #[source]
        source: serde_json::Error,
    },
    /// TOML parse / write failure.
    #[error("toml {path}: {source}")]
    Toml {
        /// File that failed.
        path: String,
        /// Reason.
        #[source]
        source: TomlError,
    },
    /// `~/.rustlings-pro/` couldn't be located (no $HOME set).
    #[error("no home directory available — set $HOME")]
    NoHome,
}

/// Wraps both directions of toml's separate de/ser error types.
#[derive(Debug, thiserror::Error)]
pub enum TomlError {
    /// Parse.
    #[error(transparent)]
    De(#[from] toml::de::Error),
    /// Serialize.
    #[error(transparent)]
    Ser(#[from] toml::ser::Error),
}

/// State store rooted at `~/.rustlings-pro/`.
#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    /// Construct a store at the given root directory. Use this in tests with a
    /// tempdir; in production prefer [`Self::user`].
    #[must_use]
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Construct the user-default store at `~/.rustlings-pro/`, creating the
    /// directory if needed.
    ///
    /// # Errors
    /// `NoHome` if no home directory is available; `Io` if it can't be created.
    pub fn user() -> Result<Self, StoreError> {
        let home = dirs::home_dir().ok_or(StoreError::NoHome)?;
        let root = home.join(".rustlings-pro");
        std::fs::create_dir_all(&root).map_err(|e| StoreError::Io {
            kind: "mkdir",
            path: root.display().to_string(),
            source: e,
        })?;
        Ok(Self { root })
    }

    /// Path to the root directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn code_path(&self, id: &str) -> PathBuf {
        self.root
            .join("code")
            .join(format!("{}.rs", id.replace('/', "__")))
    }

    /// Load `progress.json` (default on missing).
    /// # Errors
    /// I/O or JSON parse.
    pub fn load_progress(&self) -> Result<Progress, StoreError> {
        load_json_or_default(&self.path("progress.json"))
    }

    /// Atomic-save `progress.json`.
    /// # Errors
    /// I/O or JSON write.
    pub fn save_progress(&self, p: &Progress) -> Result<(), StoreError> {
        save_json_atomic(&self.path("progress.json"), p)
    }

    /// Load `bookmarks.json` (default on missing).
    /// # Errors
    /// I/O or JSON parse.
    pub fn load_bookmarks(&self) -> Result<Bookmarks, StoreError> {
        load_json_or_default(&self.path("bookmarks.json"))
    }

    /// Atomic-save `bookmarks.json`.
    /// # Errors
    /// I/O or JSON write.
    pub fn save_bookmarks(&self, b: &Bookmarks) -> Result<(), StoreError> {
        save_json_atomic(&self.path("bookmarks.json"), b)
    }

    /// Load `annotations.json` (default on missing).
    /// # Errors
    /// I/O or JSON parse.
    pub fn load_annotations(&self) -> Result<Annotations, StoreError> {
        load_json_or_default(&self.path("annotations.json"))
    }

    /// Atomic-save `annotations.json`.
    /// # Errors
    /// I/O or JSON write.
    pub fn save_annotations(&self, a: &Annotations) -> Result<(), StoreError> {
        save_json_atomic(&self.path("annotations.json"), a)
    }

    /// Load `config.toml` (default on missing).
    /// # Errors
    /// I/O or TOML parse.
    pub fn load_config(&self) -> Result<Config, StoreError> {
        let p = self.path("config.toml");
        if !p.exists() {
            return Ok(Config::default());
        }
        let raw = std::fs::read_to_string(&p).map_err(|e| StoreError::Io {
            kind: "load",
            path: p.display().to_string(),
            source: e,
        })?;
        toml::from_str(&raw).map_err(|e| StoreError::Toml {
            path: p.display().to_string(),
            source: TomlError::De(e),
        })
    }

    /// Atomic-save `config.toml`.
    /// # Errors
    /// I/O or TOML write.
    pub fn save_config(&self, c: &Config) -> Result<(), StoreError> {
        let p = self.path("config.toml");
        let s = toml::to_string_pretty(c).map_err(|e| StoreError::Toml {
            path: p.display().to_string(),
            source: TomlError::Ser(e),
        })?;
        write_atomic(&p, s.as_bytes())
    }
}

/// The `rpro-lang` Storage seam: serialized (JSON-string) progress + the
/// learner's code, for the async core. The typed [`Store`] API above is what
/// the native shells use directly today.
#[async_trait::async_trait(?Send)]
impl rpro_lang::Storage for Store {
    async fn load_progress(&self) -> Result<String, rpro_lang::StoreError> {
        let p = self.path("progress.json");
        if !p.exists() {
            return Ok("{}".to_string());
        }
        std::fs::read_to_string(&p).map_err(|e| rpro_lang::StoreError::Io(e.to_string()))
    }

    async fn save_progress(&self, json: &str) -> Result<(), rpro_lang::StoreError> {
        write_atomic(&self.path("progress.json"), json.as_bytes())
            .map_err(|e| rpro_lang::StoreError::Io(e.to_string()))
    }

    async fn load_code(&self, ex: &rpro_lang::ExerciseId) -> Result<String, rpro_lang::StoreError> {
        let p = self.code_path(&ex.0);
        if !p.exists() {
            return Err(rpro_lang::StoreError::NotFound(ex.0.clone()));
        }
        std::fs::read_to_string(&p).map_err(|e| rpro_lang::StoreError::Io(e.to_string()))
    }

    async fn save_code(
        &self,
        ex: &rpro_lang::ExerciseId,
        src: &str,
    ) -> Result<(), rpro_lang::StoreError> {
        write_atomic(&self.code_path(&ex.0), src.as_bytes())
            .map_err(|e| rpro_lang::StoreError::Io(e.to_string()))
    }
}

fn load_json_or_default<T: Default + serde::de::DeserializeOwned>(
    path: &Path,
) -> Result<T, StoreError> {
    if !path.exists() {
        return Ok(T::default());
    }
    let raw = std::fs::read_to_string(path).map_err(|e| StoreError::Io {
        kind: "load",
        path: path.display().to_string(),
        source: e,
    })?;
    serde_json::from_str(&raw).map_err(|e| StoreError::Json {
        path: path.display().to_string(),
        source: e,
    })
}

fn save_json_atomic<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), StoreError> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| StoreError::Json {
        path: path.display().to_string(),
        source: e,
    })?;
    write_atomic(path, &bytes)
}

/// Write `bytes` to `path` via tempfile + rename so a reader sees the old or
/// new file, never partial.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), StoreError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent).map_err(|e| StoreError::Io {
        kind: "mkdir",
        path: parent.display().to_string(),
        source: e,
    })?;
    let tmp = parent.join(format!(
        ".{}.tmp",
        path.file_name()
            .map_or_else(|| "rpro".to_string(), |s| s.to_string_lossy().to_string())
    ));
    std::fs::write(&tmp, bytes).map_err(|e| StoreError::Io {
        kind: "save",
        path: tmp.display().to_string(),
        source: e,
    })?;
    std::fs::rename(&tmp, path).map_err(|e| StoreError::Io {
        kind: "rename",
        path: path.display().to_string(),
        source: e,
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Store, TomlError};
    use rpro_state::{Bookmarks, Config, ExerciseStatus, Progress};

    #[test]
    fn load_progress_returns_default_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::at(dir.path());
        assert!(s.load_progress().unwrap().entries.is_empty());
    }

    #[test]
    fn save_then_load_progress_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::at(dir.path());
        let mut p = Progress::default();
        p.set_current("ownership/01");
        p.set_done("ownership/02");
        s.save_progress(&p).unwrap();
        let loaded = s.load_progress().unwrap();
        assert_eq!(loaded.entries.len(), 2);
        assert_eq!(loaded.entries["ownership/02"].status, ExerciseStatus::Done);
    }

    #[test]
    fn save_then_load_bookmarks() {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::at(dir.path());
        let mut b = Bookmarks::default();
        b.add("ch04-01", Some("rules".into()), Some("ownership rules".into()));
        s.save_bookmarks(&b).unwrap();
        assert_eq!(s.load_bookmarks().unwrap().items.len(), 1);
    }

    #[test]
    fn config_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::at(dir.path());
        let c = Config { theme: "light".into(), editor: "helix".into(), ..Config::default() };
        s.save_config(&c).unwrap();
        let loaded = s.load_config().unwrap();
        assert_eq!(loaded.theme, "light");
        assert_eq!(loaded.editor, "helix");
    }

    #[test]
    fn toml_error_is_constructible() {
        // keep the variant referenced so it isn't dead behind cfg(test)
        let _ = TomlError::De;
    }
}
