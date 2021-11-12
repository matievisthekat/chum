use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(dead_code)]
pub fn success(msg: &str) {
  writeln(Color::Green, &("\u{2714} ".to_owned() + msg));
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
  writeln(Color::Yellow, &("\u{26A0} ".to_owned() + msg));
}

#[allow(dead_code)]
pub fn error(msg: &str) {
  writeln(Color::Red, &("\u{2718} ".to_owned() + msg));
}

#[allow(dead_code)]
pub fn info(msg: &str) {
  writeln(Color::Blue, &("\u{2139} ".to_owned() + msg));
}

pub fn writeln(colour: Color, msg: &str) {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  let _ = stdout.set_color(ColorSpec::new().set_fg(Some(colour)));
  let res = writeln!(stdout, "{}", format!("{}", msg));

  match res {
    Ok(_) => {}
    Err(_) => {
      println!("{}", msg);
    }
  }
}
