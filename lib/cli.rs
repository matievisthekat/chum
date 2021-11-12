use std::collections::HashMap;
use std::env;

type Handler = Box<dyn Fn(&Context) -> Result>;

// The exit code and message to return to the shell
type Result = std::result::Result<i32, (i32, String)>;

pub struct Context {
  pub cli: &'static Cli,
  pub args: Vec<String>,
}

pub struct Cli {
  pub version: String,
  flag_identifier: String,
  command_handlers: HashMap<String, Handler>,
  command: String,
  flag_handlers: HashMap<String, Handler>,
  flags: Vec<String>,
  args: Vec<String>,
}

impl Cli {
  pub fn new(flag_identifier: &str, version: &str) -> Self {
    let mut cli = Cli {
      version: version.to_string(),
      flag_identifier: flag_identifier.to_string(),
      command_handlers: HashMap::new(),
      command: "".to_string(),
      flag_handlers: HashMap::new(),
      flags: vec![],
      args: vec![],
    };

    cli.parse();

    cli
  }

  pub fn run(&self) {
    let command_handler = self.command_handlers.get(&self.command);
    let context = Context {
      cli: self.clone(),
      args: self.args.clone(),
    };
    let exit: Result;

    if self.command.is_empty() && self.flags.len() > 0 {
      let flag_handler = self.flag_handlers.get(&self.flags[0]);
      exit = self.match_flag_handler(flag_handler, &context);
    } else {
      exit = self.match_command_handler(command_handler, &context);
    }

    match exit {
      Ok(code) => std::process::exit(code),
      Err((code, message)) => {
        eprintln!("{}", message);
        std::process::exit(code)
      }
    }
  }

  pub fn register_command(&mut self, command: &str, handler: Handler) {
    self.command_handlers.insert(command.to_string(), handler);
  }

  pub fn register_flag(&mut self, flag: &str, handler: Handler) {
    self.flag_handlers.insert(flag.to_string(), handler);
  }

  fn match_command_handler(&self, command_handler: Option<&Handler>, context: &Context) -> Result {
    match command_handler {
      Some(handler) => return handler(context),
      None => {
        if self.command.is_empty() {
          return Err((127, "No command specified".to_string()));
        } else {
          return Err((127, format!("Unknown command: {}", self.command)));
        }
      }
    };
  }

  fn match_flag_handler(&self, flag_handler: Option<&Handler>, context: &Context) -> Result {
    match flag_handler {
      Some(handler) => return handler(context),
      None => return Err((1, format!("Unknown flag: {}", self.flags[0]))),
    }
  }

  fn parse(&mut self) -> &Self {
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
      self.args = args.clone()[1..].to_vec();
    }

    self
  }
}
