use std::collections::HashMap;
use std::env;

use crate::lib::{command_manager, display};

pub struct Context<'lt> {
  pub cli: &'lt Cli,
}

pub struct Cli {
  pub version: String,
  pub commands: HashMap<String, command_manager::Command>,
  pub args: Vec<String>,
  pub flags: Vec<String>,
  wanted_command: String,
  flag_handlers: HashMap<String, command_manager::Handler>,
}

impl Cli {
  pub fn new(flag_identifier: &str, version: &str) -> Self {
    let all_args: Vec<String> = env::args().skip(1).collect();
    let prefix = flag_identifier;
    let mut args = vec![];
    let mut pure_args = vec![];
    let mut flags = vec![];
    let mut wanted_command = String::new();

    for arg in all_args {
      if arg.starts_with(prefix) {
        let flag = arg.replace(prefix, "");
        flags.push(flag);
      } else {
        args.push(arg);
      }
    }

    if args.len() > 0 {
      wanted_command = args[0].clone();
      pure_args = args.clone()[1..].to_vec();
    }

    Cli {
      version: version.to_string(),
      wanted_command: wanted_command,
      commands: HashMap::new(),
      flags: flags,
      flag_handlers: HashMap::new(),
      args: pure_args,
    }
  }

  pub fn run(&self) {
    let command = self.commands.get(&self.wanted_command);
    let exit: command_manager::Result;
    let context = Context { cli: self.clone() };

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
        display::error(message);
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
}
