use std::io::{self};

use cursive::{
  theme::PaletteColor,
  utils::markup::StyledString,
  view::Nameable,
  views::{Button, Dialog, LinearLayout, TextView},
  Cursive,
};
use cursive_open_editor::{
  strategy::FindEditorStrategy, CursiveOpenEditorOptions,
};

fn main() {
  let mut siv = cursive::default();

  siv.add_layer(
    Dialog::new().title("Examplinator 9000").content(
      LinearLayout::vertical()
        .child(TextView::new("Click the button to open a text editor"))
        .child(Button::new("Edit normally", |siv| {
          let opts = CursiveOpenEditorOptions::default();
          launch_editor_and_summarize(siv, opts);
        }))
        .child(Button::new("Edit in Kate", |siv| {
          let opts = CursiveOpenEditorOptions {
            editor_strategy: FindEditorStrategy::absolute_path("/usr/bin/kate"),
            // hang the terminal until Kate is quitted
            additional_args: vec!["--block".into()],
            ..Default::default()
          };
          launch_editor_and_summarize(siv, opts);
        }))
        .child(TextView::new("(Nothing yet!)").with_name("info")),
    ),
  );

  siv.run();
}

fn launch_editor_and_summarize(
  siv: &mut Cursive,
  opts: CursiveOpenEditorOptions,
) {
  let iife: io::Result<String> = (|| {
    let res = cursive_open_editor::open_editor(siv, opts)?;
    res.status_ok()?;
    let s = res.read_to_string()?;
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
}
