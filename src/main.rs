use std::collections::HashMap;
use std::env;

fn main() {
    const FLAG_IDENTIFIER: &str = "--";
    let args: Vec<String> = env::args().skip(1).collect();
    let command = args[0].clone().to_lowercase();

    let flag_args: Vec<String> = args
        .iter()
        .filter(|arg| arg.starts_with(FLAG_IDENTIFIER))
        .map(|arg| arg.to_string())
        .collect();

    let mut flags = HashMap::new();
    for flag in flag_args {
        let mut flag_name = flag.clone();
        for _ in FLAG_IDENTIFIER.chars() {
            flag_name.remove(0);
        }
        flags.insert(flag_name, true);
    }
}
