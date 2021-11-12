use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Display {
  colour_choice: ColorChoice,
}

impl Display {
  pub fn new(choice: ColorChoice) -> Display {
    Display {
      colour_choice: choice,
    }
  }

  #[allow(dead_code)]
  pub fn success(&self, msg: String) {
    self.writeln(Color::Green, "\u{2714} ".to_owned() + &msg);
  }

  #[allow(dead_code)]
  pub fn warn(&self, msg: String) {
    self.writeln(Color::Yellow, "\u{26A0} ".to_owned() + &msg);
  }

  #[allow(dead_code)]
  pub fn error(&mut self, msg: String) {
    self.writeln(Color::Red, "\u{2718} ".to_owned() + &msg);
  }

  #[allow(dead_code)]
  pub fn info(&self, msg: String) {
    self.writeln(Color::Blue, "\u{2139} ".to_owned() + &msg);
  }

  pub fn writeln(&self, colour: Color, msg: String) {
    let mut stdout = StandardStream::stdout(self.colour_choice);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(colour)));
    let res = writeln!(stdout, "{}", format!("{}", msg));

    match res {
      Ok(_) => {}
      Err(_) => {
        println!("{}", msg);
      }
    }
  }
}
