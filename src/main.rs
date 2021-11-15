mod commands;
mod lib;
use lib::cli::Cli;

fn main() {
    const FLAG_IDENTIFIER: &str = ",";

    let mut cli = Cli::new(FLAG_IDENTIFIER, "0.0.0-alpha");

    cli.register_command(commands::help::command_info());

    cli.register_flag(
        "version",
        Box::new(|ctx| {
            println!("{}", ctx.cli.version);
            return Ok(0);
        }),
    );

    cli.run();
}
