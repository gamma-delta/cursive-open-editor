use std::{
  io::{self, Read, Seek},
  path::PathBuf,
};

use cursive::{
  theme::PaletteColor,
  utils::markup::StyledString,
  view::Nameable,
  views::{Button, Dialog, LinearLayout, ListView, TextView},
};
use cursive_open_editor::{
  strategy::EditPathStrategy, CursiveOpenEditorOptions,
};

fn dotfile_paths() -> Vec<PathBuf> {
  [
    ".bashrc",
    ".gitconfig",
    ".zshrc",
    ".doesnotexistexamplerc",
    ".cargo/config.toml",
    ".config/fish/config.fish",
    ".config/helix/config.toml",
    ".config/rustfmt/rustfmt.toml",
  ]
  .into_iter()
  .map(|p| {
    let path = dirs::home_dir().unwrap();
    path.join(p)
  })
  .collect()
}

fn main() {
  let mut siv = cursive::default();

  siv.add_layer(
    Dialog::new()
      .title("Dotfiles Editor")
      .content(
        LinearLayout::vertical()
          .child(TextView::new(
            "Select a dotfile to edit it.\n\
           This does actually edit the dotfile! Be careful!",
          ))
          .child({
            let mut list = ListView::new();
            list.add_delimiter();

            for file_path in dotfile_paths().into_iter() {
              let file_display = file_path.display().to_string();
              if matches!(std::fs::exists(&file_path), Ok(true)) {
                list.add_child(&file_display, edit_button(file_path))
              } else {
                list.add_child(
                  &file_display,
                  Button::new("not found", |_| {}).with_enabled(false),
                )
              }
            }

            list
          }),
      )
      .button("Finish", |siv| siv.quit()),
  );

  siv.run();
}

fn edit_button(file_path: PathBuf) -> Button {
  Button::new("Edit", move |siv| {
    let opts = CursiveOpenEditorOptions {
      edit_path_strategy: EditPathStrategy::GivePath(file_path.to_owned()),
      ..Default::default()
    };
    let iife: io::Result<()> = (|| {
      let res = cursive_open_editor::open_editor(siv, opts)?;
      res.status_ok()?;
      Ok(())
    })();
    if let Err(ono) = iife {
      siv.add_layer(Dialog::info(ono.to_string()));
    }
  })
}
