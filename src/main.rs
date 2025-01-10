mod operations;
use rit::Command;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command: Command = Command::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    match command.command.as_str() {
        "init" => operations::init::init_repo().unwrap_or_else(|err| {
            eprintln!("Problem initializing repository: {err}");
            process::exit(1)
        }),
        "status" => (),
        "add" => (),
        "commit" => (),
        "log" => (),
        "help" => (),
        _ => {
            eprintln!("Command not found: type ./rit help for more information");
        }
    }

    // use sha256::digest;
    // use std::env;
    // use std::fs;
    // use std::io::{self, Write};
    // use std::path::Path;
    // let args: Vec<String> = env::args().collect();
    // let command = args[1].clone();

    // // 1. Read the file content
    // let file_path = "./text.txt"; // Replace with your file path
    // let content = fs::read(file_path)?;

    // // 2. Construct the blob format
    // let blob_header = format!("blob {}\0", content.len());
    // let mut blob_data = blob_header.into_bytes();
    // blob_data.extend(&content);

    // // 3. Compute SHA-1 hash
    // let hash_str = digest(&blob_data);

    // // 4. Store the blob in .git/objects
    // let object_dir = format!(".rit/objects/{}/", &hash_str[0..2]);
    // let object_path = format!("{}{}", object_dir, &hash_str[2..]);

    // // Create directories if they don't exist
    // fs::create_dir_all(&object_dir)?;

    // // Write the blob as a compressed object
    // let mut file = fs::File::create(object_path)?;
    // file.write_all(&blob_data)?;

    // println!("Blob stored as: {}", hash_str);
    // Ok(())
}
