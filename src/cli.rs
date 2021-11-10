use std::env;

struct Cli {
  version: String,
  flag_identifier: String,
  known_commands: Vec<String>,
  command: String,
  flags: Vec<String>,
  args: Vec<String>,
}

impl Cli {
  fn new(flag_identifier: &str, version: &str) -> Self {
    let cli = Cli {
      version: version.to_string(),
      flag_identifier: flag_identifier.to_string(),
      known_commands: vec![],
      command: "".to_string(),
      flags: vec![],
      args: vec![],
    };

    cli.parse()
  }

  fn parse(mut self) -> Self {
    let all_args: Vec<String> = env::args().skip(1).collect();
    let prefix = self.flag_identifier.as_str();
    let mut args = vec![];

    for arg in all_args {
      if arg.starts_with(prefix) {
        let flag = arg.replace(prefix, "");
        self.flags.push(flag);
      } else {
        args.push(arg);
      }
    }

    if args.len() > 0 {
      self.command = args[0].clone();
      self.args = args.clone()[1..];
    }

    self
  }
}
