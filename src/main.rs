mod operations;
use crate::operations::add::run_add;
use crate::operations::init::run_init;
use rit::Command;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command: Command = Command::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    match command.command.as_str() {
        "init" => run_init().unwrap_or_else(|err| {
            eprintln!("Problem initializing repository: {err}");
            process::exit(1)
        }),
        "add" => run_add(&command.optional_arg).unwrap_or_else(|err| {
            eprintln!("Problem adding files: {err}");
            process::exit(1)
        }),
        "status" => (),
        "commit" => (),
        "log" => (),
        "help" => (),
        _ => {
            eprintln!("Command not found: type ./rit help for more information");
        }
    }
}
