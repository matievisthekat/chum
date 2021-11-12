mod lib;
use lib::{cli, display};

fn main() {
    const FLAG_IDENTIFIER: &str = ".";

    let mut cli = cli::Cli::new(FLAG_IDENTIFIER, "0.0.0-alpha");

    cli.register_command(
        "foo",
        Box::new(|_| {
            println!("Bar");
            return Ok(0);
        }),
    );

    cli.register_command(
        "bar",
        Box::new(|_| {
            display::info("test");
            return Ok(0);
        }),
    );

    cli.register_flag(
        "version",
        Box::new(|ctx| {
            println!("{}", ctx.cli.version);
            return Ok(0);
        }),
    );

    cli.run();
}
