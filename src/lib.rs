use sha256::digest;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl GitObject {
    pub fn get_sha(&self) -> std::io::Result<String> {
        match self {
            GitObject::Blob(blob) => blob.calculate_sha(),
            GitObject::Tree(tree) => tree.calculate_sha(),
            GitObject::Commit(commit) => commit.calculate_sha(),
        }
    }
}

pub trait SerializeObject {
    fn serialize_data(&self) -> std::io::Result<(String, Vec<u8>)>;

    // Calculate the SHA-256 hash of the object and writes the binary to the objects directory
    fn calculate_sha(&self) -> std::io::Result<String> {
        // SHA 256 code from hashing the header + the object bytes
        let (obj_type, bytes) = self.serialize_data()?;
        let mut header_and_bytes = format!("{} {}\0", obj_type, bytes.len())
            .as_bytes()
            .to_vec();
        header_and_bytes.extend(&bytes);
        let sha = digest(&header_and_bytes);

        // // Write the binary data to the ./objects directory
        // let dir = format!("./objects/{}", &sha[..2]);
        // let file_path = format!("{}/{}", dir, &sha[2..]);

        // // Create the directory if it doesn't exist
        // fs::create_dir_all(&dir)?;

        // // Write the binary data to the file
        // let mut file = File::create(&file_path)?;
        // file.write_all(&header_and_bytes)?;

        return Ok(sha);
    }
}

#[derive(Debug)]
pub struct Blob {
    pub data: Vec<u8>,
}

impl Blob {
    pub fn new(path: &Path) -> std::io::Result<Blob> {
        let data = fs::read(path)?;
        return Ok(Blob { data });
    }
}

impl SerializeObject for Blob {
    fn serialize_data(&self) -> std::io::Result<(String, Vec<u8>)> {
        return Ok(("blob".to_string(), self.data.clone()));
    }
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

impl SerializeObject for Tree {
    fn serialize_data(&self) -> std::io::Result<(String, Vec<u8>)> {
        let mut out = Vec::new();
        for entry in &self.entries {
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
}

#[derive(Debug)]
pub struct Commit {
    pub tree: String, // SHA-256 hash of the tree object
    pub parent: Option<String>,
    pub author: String,
    pub message: String,
    pub timestamp: u64,
}

impl SerializeObject for Commit {
    fn serialize_data(&self) -> std::io::Result<(String, Vec<u8>)> {
        let mut out = Vec::new();

        // tree <tree SHA-1>
        out.extend_from_slice(format!("tree {}\n", self.tree).as_bytes());

        // parent <parent commit SHA-1>   # (optional, for non-initial commits)
        if let Some(ref parent) = self.parent {
            out.extend_from_slice(format!("parent {}\n", parent).as_bytes());
        }

        // author <name> <email> <timestamp> <timezone>
        out.extend_from_slice(format!("author {} {}\n", self.author, self.timestamp).as_bytes());

        // committer <name> <email> <timestamp> <timezone>
        out.extend_from_slice(format!("committer {} {}\n", self.author, self.timestamp).as_bytes());
        out.extend_from_slice(b"\n");

        // <commit message>
        out.extend_from_slice(self.message.as_bytes());

        return Ok(("commit".to_string(), out));
    }
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
