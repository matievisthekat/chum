use crate::lib::cli::Context;

// The exit code and message to return to the shell
pub type Result = std::result::Result<i32, (i32, String)>;
pub type Handler = Box<dyn Fn(&Context) -> Result>;
pub type Arg = (String, bool);

pub struct Command {
  pub name: String,
  pub description: String,
  pub examples: Vec<String>,
  pub alllowed_args: Vec<Arg>,
  pub allowed_flags: Vec<String>,
  pub handler: Handler,
}

impl Command {
  pub fn new(
    name: String,
    description: String,
    examples: Vec<String>,
    alllowed_args: Vec<Arg>,
    allowed_flags: Vec<String>,
    handler: Handler,
  ) -> Command {
    Command {
      name: name,
      description: description,
      examples: examples,
      alllowed_args: alllowed_args,
      allowed_flags: allowed_flags,
      handler: handler,
    }
  }
}
