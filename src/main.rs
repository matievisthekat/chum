mod cli;

fn main() {
    const FLAG_IDENTIFIER: &str = ".";

    let cli = cli::Cli::new(FLAG_IDENTIFIER, "0.0.0-alpha");
}
