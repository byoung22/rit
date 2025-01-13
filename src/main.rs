mod operations;
use crate::operations::init::init_repo;
use crate::operations::status::directory_traversal;
use rit::Command;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command: Command = Command::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    match command.command.as_str() {
        "init" => init_repo().unwrap_or_else(|err| {
            eprintln!("Problem initializing repository: {err}");
            process::exit(1)
        }),
        "add" => (),
        "status" => directory_traversal(Path::new("./")),
        "commit" => (),
        "log" => (),
        "help" => (),
        _ => {
            eprintln!("Command not found: type ./rit help for more information");
        }
    }
}
