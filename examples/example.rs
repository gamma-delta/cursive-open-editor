use std::io::{self, Read, Seek};

use cursive::{
  theme::PaletteColor,
  utils::markup::StyledString,
  view::Nameable,
  views::{Button, Dialog, LinearLayout, TextView},
};
use cursive_open_editor::CursiveOpenEditorOptions;

fn main() {
  let mut siv = cursive::default();

  siv.add_layer(
    Dialog::new().title("Examplinator 9000").content(
      LinearLayout::vertical()
        .child(TextView::new("Click the button to open a text editor"))
        .child(Button::new("Click me!", |siv| {
          let opts = CursiveOpenEditorOptions::default();
          let iife: io::Result<String> = (|| {
            let res = cursive_open_editor::open_editor(siv, opts)?;
            res.status_ok()?;
            let mut s = String::new();
            let mut f = res.edited_file()?;
            f.seek(io::SeekFrom::Start(0))?;
            f.read_to_string(&mut s)?;
            Ok(s)
          })();

          siv.call_on_name("info", |v: &mut TextView| match iife {
            Ok(s) => {
              v.set_content(format!(
                "You typed {} words!",
                s.split_whitespace().count()
              ));
            }
            Err(ono) => {
              v.set_content(StyledString::styled(
                format!("Something went wrong! {}", ono),
                PaletteColor::Highlight,
              ));
            }
          });
        }))
        .child(TextView::new("(Nothing yet!)").with_name("info")),
    ),
  );

  siv.run();
}
