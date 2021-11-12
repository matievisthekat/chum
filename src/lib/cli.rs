use std::collections::HashMap;
use std::env;
use termcolor::ColorChoice;

use crate::lib::{command_manager, display::Display};

pub struct Context<'lt> {
  pub cli: &'lt Cli,
  pub args: Vec<String>,
  pub display: Display,
}

pub struct Cli {
  pub version: String,
  flag_identifier: String,
  wanted_command: String,
  commands: HashMap<String, command_manager::Command>,
  flags: Vec<String>,
  flag_handlers: HashMap<String, command_manager::Handler>,
  args: Vec<String>,
}

impl Cli {
  pub fn new(flag_identifier: &str, version: &str) -> Self {
    let mut cli = Cli {
      version: version.to_string(),
      flag_identifier: flag_identifier.to_string(),
      wanted_command: "".to_string(),
      commands: HashMap::new(),
      flags: vec![],
      flag_handlers: HashMap::new(),
      args: vec![],
    };

    cli.parse();

    cli
  }

  pub fn run(&self) {
    let command = self.commands.get(&self.wanted_command);
    let exit: command_manager::Result;
    let display = Display::new(ColorChoice::Auto);
    let context = Context {
      cli: self.clone(),
      args: self.args.clone(),
      display: display,
    };

    // if no command supplied, and at least one flag supplied
    if self.wanted_command.is_empty() && self.flags.len() > 0 {
      let flag_handler = self.flag_handlers.get(&self.flags[0]);
      exit = self.match_flag_handler(flag_handler, &context);
    } else {
      exit = self.run_command(command, &context);
    }

    match exit {
      Ok(code) => std::process::exit(code),
      Err((code, message)) => {
        eprintln!("{}", message);
        std::process::exit(code)
      }
    }
  }

  pub fn register_command(&mut self, command: command_manager::Command) {
    self.commands.insert(command.name.clone(), command);
  }

  pub fn register_flag(&mut self, flag: &str, handler: command_manager::Handler) {
    self.flag_handlers.insert(flag.to_string(), handler);
  }

  fn run_command(
    &self,
    command: Option<&command_manager::Command>,
    context: &Context,
  ) -> command_manager::Result {
    match command {
      Some(command) => {
        let handler = &command.handler;
        return handler(context);
      }
      None => {
        if self.wanted_command.is_empty() {
          return Err((127, "No command specified".to_string()));
        } else {
          return Err((127, format!("Unknown command: {}", self.wanted_command)));
        }
      }
    };
  }

  fn match_flag_handler(
    &self,
    flag_handler: Option<&command_manager::Handler>,
    context: &Context,
  ) -> command_manager::Result {
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
      self.wanted_command = args[0].clone();
      self.args = args.clone()[1..].to_vec();
    }

    self
  }
}
