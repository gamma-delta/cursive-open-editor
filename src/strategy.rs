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
  /// Helper constructor for `AbsolutePath`.
  pub fn absolute_path<P: Into<PathBuf>>(p: P) -> Self {
    Self::AbsolutePath(p.into())
  }

  /// Turn it into a real path.
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

/// The path an [`EditPathStrategy`] settled on.
///
/// This may contain a RAII guard for a temporary file!
/// Dropping it before you read from the file
/// (for example, if you save the path but not the file)
/// may invalidate the file.
pub struct EditPathStrategyOut(EditPathStrategyOutInner);

pub(crate) enum EditPathStrategyOutInner {
  GivenPath(PathBuf),
  MadeTmp(TempPath),
}

impl EditPathStrategyOut {
  /// Get the path edited.
  pub fn path(&self) -> PathBuf {
    match &self.0 {
      EditPathStrategyOutInner::GivenPath(it) => it.to_owned(),
      EditPathStrategyOutInner::MadeTmp(tmp_path) => tmp_path.to_path_buf(),
    }
  }

  /// If this was previously a temporary file, make the file persistent.
  /// This removes any RAII this may have; you'll have to clean up the
  /// file yourself.
  pub fn persist(&mut self) -> io::Result<()> {
    let EditPathStrategyOutInner::MadeTmp(_) = &self.0 else {
      // nothing to do
      return Ok(());
    };
    // force-get
    let EditPathStrategyOutInner::MadeTmp(it) = std::mem::replace(
      &mut self.0,
      EditPathStrategyOutInner::GivenPath("slhgdsf".into()),
    ) else {
      unreachable!()
    };
    match it.keep() {
      Ok(path) => {
        self.0 = EditPathStrategyOutInner::GivenPath(path);
        Ok(())
      }
      Err(ono) => {
        // put it back!
        self.0 = EditPathStrategyOutInner::MadeTmp(ono.path);
        Err(ono.error)
      }
    }
  }
}
