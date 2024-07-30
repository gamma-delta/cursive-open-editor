pub mod strategy;

use std::{
  ffi::OsString,
  fs::File,
  io::{self},
  path::PathBuf,
  process::{Command, ExitStatus},
};

use cursive::Cursive;
use strategy::{EditPathStrategy, EditPathStrategyOut, FindEditorStrategy};

pub struct CursiveOpenEditorOptions {
  pub editor_strategy: FindEditorStrategy,
  pub edit_path_strategy: EditPathStrategy,
  pub additional_args: Vec<OsString>,
}

impl Default for CursiveOpenEditorOptions {
  fn default() -> Self {
    Self {
      editor_strategy: FindEditorStrategy::default(),
      edit_path_strategy: EditPathStrategy::default(),
      additional_args: Vec::new(),
    }
  }
}

pub fn open_editor(
  siv: &mut Cursive,
  options: CursiveOpenEditorOptions,
) -> io::Result<EditorOpened> {
  let editor_path = options.editor_strategy.editor_path().ok_or_else(|| {
    io::Error::new(
      io::ErrorKind::NotFound,
      "could not find an editor to launch",
    )
  })?;
  let edit_path = options.edit_path_strategy.file_path()?;

  let dump = siv.dump();
  siv.clear();

  let mut cmd_builder = Command::new(editor_path);
  cmd_builder.args(&options.additional_args);
  cmd_builder.arg(&edit_path.path());

  // run the editor!
  let mut kid = cmd_builder.spawn()?;
  let status = kid.wait()?;

  siv.restore(dump);
  Ok(EditorOpened {
    status,
    edited_path: edit_path,
  })
}

/// Information about the editor ran.
pub struct EditorOpened {
  pub status: ExitStatus,
  pub edited_path: EditPathStrategyOut,
}

impl EditorOpened {
  /// Returns an [`io::Error`] if the [`ExitStatus`] of the editor program
  /// wasn't a success.
  ///
  /// Note that the file could still have been successfully edited!
  /// This is just a helper to use with the `?` operator, or whatever.
  pub fn status_ok(&self) -> io::Result<()> {
    if self.status.success() {
      Ok(())
    } else {
      Err(io::Error::other(format!("non-OK exit: {}", &self.status)))
    }
  }

  /// Open the edited file as a read-only [`File`].
  pub fn edited_file(&self) -> io::Result<File> {
    let path = self.edited_path.path();
    File::open(&path)
  }

  /// The path that was edited.
  pub fn edited_path(&self) -> PathBuf {
    self.edited_path.path()
  }
}
