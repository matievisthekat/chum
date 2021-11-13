mod lib;
use lib::{cli, command_manager, compression};
use std::fs::File;

fn main() {
    const FLAG_IDENTIFIER: &str = ",";

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

    cli.register_command(command_manager::Command::new(
        "compress".to_string(),
        "Test compression".to_string(),
        vec!["./in ./out".to_string()],
        vec![
            ("File to compress".to_string(), true),
            ("Output file".to_string(), true),
        ],
        vec![],
        Box::new(|ctx| {
            let input = compression::get_file_as_binary(&ctx.args[0].to_string());
            let compressed = compression::Compress::new(&input);
            println!("{:?}", compressed);
            Ok(0)
        }),
    ));

    cli.register_command(command_manager::Command::new(
        "decompress".to_string(),
        "Test decompression".to_string(),
        vec!["./in ./out".to_string()],
        vec![
            ("File to decompress".to_string(), true),
            ("Output file".to_string(), true),
        ],
        vec![],
        Box::new(|ctx| {
            let input = File::open(&ctx.args[0].to_string()).unwrap();
            let output = compression::Decompress::new(Box::new(input));
            println!("{:?}", output);
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
