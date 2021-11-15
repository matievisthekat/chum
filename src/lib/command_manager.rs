use crate::lib::cli::Context;

// The exit code and message to return to the shell
pub type Result = std::result::Result<i32, (i32, String)>;
pub type Handler = Box<dyn Fn(&Context) -> Result>;
pub type Arg = (String, bool);

pub struct Command {
  pub name: String,
  pub description: String,
  pub examples: Vec<String>,
  pub allowed_args: Vec<Arg>,
  pub allowed_flags: Vec<String>,
  pub handler: Handler,
}
