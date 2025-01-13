use sha256::digest;
use std::fs;
use std::path::Path;

pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

#[derive(Debug)]
pub struct Blob {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub sha: String, // This references either a blob or another tree
}

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

#[derive(Debug)]
pub struct Commit {
    pub tree: String, // SHA-256 hash of the tree object
    pub parent: Option<String>,
    pub author: String,
    pub message: String,
    pub timestamp: u64,
}

pub struct Command {
    pub command: String,
    pub optional_arg: String,
}

// Parses arguments
impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("Enter a command, type ./rit help for more information");
        }

        let command = args[1].clone();
        let mut optional_arg = String::from("");

        if command == "add" || command == "commit" {
            if args.len() < 3 {
                return Err("Enter a file name");
            }
            optional_arg = args[2].clone();
        }

        return Ok(Command {
            command,
            optional_arg,
        });
    }
}

// Stores the object in the ./objects directory as binary
pub fn store_object(obj: &GitObject) -> std::io::Result<String> {
    // SHA 256 code from hashing the header + the object bytes
    let (obj_type, bytes) = serialize_object(obj)?;
    let mut header_and_bytes = format!("{} {}\0", obj_type, bytes.len())
        .as_bytes()
        .to_vec();
    header_and_bytes.extend(&bytes);
    let sha = digest(&header_and_bytes);

    // Write binary to ./objects directory in folders based on the first two characters of the SHA
    let object_path = format!("./.rit/objects/{}/{}", &sha[0..2], &sha[2..]);
    if !Path::new(&object_path).exists() {
        fs::create_dir_all(format!("./.rit/objects/{}", &sha[0..2]))?;
        fs::write(&object_path, header_and_bytes)?;
    }
    return Ok(sha);
}

// Uses different hashing algorithms based on the object type
pub fn serialize_object(obj: &GitObject) -> std::io::Result<(String, Vec<u8>)> {
    match obj {
        // Blobs are stored in the format: blob <size>\0<data>
        GitObject::Blob(blob) => {
            let data = blob.data.clone();
            return Ok(("blob".to_string(), data));
        }
        // Trees are stored in the format: <file-mode> <filename>\0<SHA of blob or tree>
        GitObject::Tree(tree) => {
            let mut out = Vec::new();
            for entry in &tree.entries {
                // mode (ASCII)
                out.extend_from_slice(entry.mode.as_bytes());
                // space
                out.push(b' ');

                // name (ASCII)
                out.extend_from_slice(entry.name.as_bytes());
                // null terminator
                out.push(0);

                // SHA-256 hash (32 bytes)
                let hash_bytes = hex::decode(&entry.sha).unwrap();
                out.extend_from_slice(&hash_bytes);
            }
            return Ok(("tree".to_string(), out));
        }
        // commit <size>\0
        // tree <tree SHA-1>
        // parent <parent commit SHA-1>   # (optional, for non-initial commits)
        // author <name> <email> <timestamp> <timezone>
        // committer <name> <email> <timestamp> <timezone>
        // <commit message>
        GitObject::Commit(commit) => {
            let mut out = Vec::new();
            out.extend_from_slice(format!("tree {}\n", commit.tree).as_bytes());
            if let Some(ref parent) = commit.parent {
                out.extend_from_slice(format!("parent {}\n", parent).as_bytes());
            }
            out.extend_from_slice(
                format!("author {} {}\n", commit.author, commit.timestamp).as_bytes(),
            );
            out.extend_from_slice(
                format!("committer {} {}\n", commit.author, commit.timestamp).as_bytes(),
            );
            out.extend_from_slice(b"\n");
            out.extend_from_slice(commit.message.as_bytes());
            return Ok(("commit".to_string(), out));
        }
    }
}
