use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[allow(dead_code)]
pub fn success(msg: String) {
  writeln(Color::Green, "\u{2714} ".to_owned() + &msg);
}

#[allow(dead_code)]
pub fn warn(msg: String) {
  writeln(Color::Yellow, "\u{26A0} ".to_owned() + &msg);
}

#[allow(dead_code)]
pub fn error(msg: String) {
  writeln(Color::Red, "\u{2718} ".to_owned() + &msg);
}

#[allow(dead_code)]
pub fn info(msg: String) {
  writeln(Color::Blue, "\u{2139} ".to_owned() + &msg);
}

pub fn writeln(colour: Color, msg: String) {
  let mut stdout = StandardStream::stdout(ColorChoice::Auto);
  let _ = stdout.set_color(ColorSpec::new().set_fg(Some(colour)));
  let res = writeln!(stdout, "{}", format!("{}", msg));

  match res {
    Ok(_) => {}
    Err(_) => {
      println!("{}", msg);
    }
  }

  match stdout.reset() {
    Ok(_) => {}
    Err(e) => {
      eprintln!("Failed to reset stdout after colour change: {}", e);
    }
  }
}
