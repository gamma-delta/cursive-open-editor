use std::{
  io::{self},
  path::PathBuf,
};

use tempfile::TempPath;

/// How to select the editor.
#[derive(Debug, Clone, Default)]
pub enum FindEditorStrategy {
  /// Checks, in order, the `CURSIVE_EDITOR`, `EDITOR`, and `VISUAL` environment variables.
  /// Executes the first one that is set.
  #[default]
  Envs,
  /// Execute the editor binary at the given path.
  AbsolutePath(PathBuf),
}

impl FindEditorStrategy {
  pub fn editor_path(self) -> Option<PathBuf> {
    match self {
      FindEditorStrategy::Envs => ["CURSIVE_EDITOR", "EDITOR", "VISUAL"]
        .into_iter()
        .find_map(|var_name| std::env::var_os(var_name))
        .map(|os_str| os_str.into()),
      FindEditorStrategy::AbsolutePath(it) => Some(it.to_owned()),
    }
  }
}

/// How to select which file to open.
#[derive(Debug, Clone, Default)]
pub enum EditPathStrategy {
  /// Make a temporary file and open that.
  #[default]
  MakeTmp,
  /// Open the file at the given path.
  GivePath(PathBuf),
}

/// The path an [`EditPathStrategy`] settled on.
///
/// This may contain a RAII guard for a temporary file!
pub struct EditPathStrategyOut(EditPathStrategyOutInner);

pub(crate) enum EditPathStrategyOutInner {
  GivenPath(PathBuf),
  MadeTmp(TempPath),
}

impl EditPathStrategy {
  /// Return a path to edit a file at.
  ///
  /// `MakeTmp` uses the `tempfile` crate, which uses RAII handles for
  /// its temporary files.
  /// All this is hidden inside the [`EditPathStrategyTmpRAII`].
  /// Keep it until you are through editing, then drop it.
  pub fn file_path(self) -> io::Result<EditPathStrategyOut> {
    Ok(EditPathStrategyOut(match self {
      EditPathStrategy::MakeTmp => {
        let tmp = tempfile::NamedTempFile::new()?;
        EditPathStrategyOutInner::MadeTmp(tmp.into_temp_path())
      }
      EditPathStrategy::GivePath(path) => {
        EditPathStrategyOutInner::GivenPath(path.to_owned())
      }
    }))
  }
}

impl EditPathStrategyOut {
  pub fn path(&self) -> PathBuf {
    match &self.0 {
      EditPathStrategyOutInner::GivenPath(it) => it.to_owned(),
      EditPathStrategyOutInner::MadeTmp(tmp_path) => tmp_path.to_path_buf(),
    }
  }
}
