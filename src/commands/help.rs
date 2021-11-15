use crate::lib::command_manager::Command;

pub fn command_info() -> Command {
  Command {
    name: "help".to_string(),
    description: "Display help information for a command".to_string(),
    examples: vec!["".to_string(), "compress".to_string()],
    allowed_args: vec![("The command to get help for".to_string(), false)],
    allowed_flags: vec![],
    handler: Box::new(|ctx| match ctx.cli.args.first() {
      Some(arg) => {
        let command = ctx.cli.commands.get(arg);
        match command {
          Some(command) => {
            println!("{}", command.description);
            println!("");
            println!("Examples:");
            for example in &command.examples {
              println!("{}", example);
            }
            return Ok(0);
          }
          None => {
            return Err((1, format!("No command found with name {}", arg)));
          }
        }
      }
      None => {
        println!(
          "Commands: {}",
          ctx
            .cli
            .commands
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ")
        );
        Ok(0)
      }
    }),
  }
}
