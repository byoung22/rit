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
    pub tree: String, // SHA-1 hash of the tree object
    pub parent: Option<String>,
    pub author: String,
    pub message: String,
    pub timestamp: u64,
}

pub struct Command {
    pub command: String,
    pub optionalArg: String,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("Enter a command, type ./rit help for more information");
        }

        let command = args[1].clone();
        let mut optionalArg = String::from("");

        if command == "add" || command == "commit" {
            if args.len() < 3 {
                return Err("Enter a file name");
            }
            optionalArg = args[2].clone();
        }

        return Ok(Command {
            command,
            optionalArg,
        });
    }
}
