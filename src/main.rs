mod lib;
use lib::{cli, command_manager};

fn main() {
    const FLAG_IDENTIFIER: &str = ".";

    let mut cli = cli::Cli::new(FLAG_IDENTIFIER, "0.0.0-alpha");

    cli.register_command(command_manager::Command::new(
        "foo".to_string(),
        "Test command".to_string(),
        vec!["bar .baz".to_string()],
        vec![("File to run".to_string(), true)],
        vec!["baz".to_string()],
        Box::new(|ctx| {
            ctx.display.info("Running foo".to_string());
            Ok(0)
        }),
    ));

    cli.register_flag(
        "version",
        Box::new(|ctx| {
            println!("{}", ctx.cli.version);
            return Ok(0);
        }),
    );

    cli.run();
}
