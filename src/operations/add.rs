use core::panic;
use rit::{Blob, SerializeObject};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

struct Index {
    // Stored in the format: <path>: (<mode>, <sha>)
    entries: HashMap<String, (String, String)>,
}

impl Index {
    // Build the index from the index file stored in the format: <path> <mode> <sha>\n
    fn build_from_file() -> Self {
        let mut entries: HashMap<String, (String, String)> = HashMap::new();
        let index = fs::read_to_string(".rit/index").unwrap();
        for line in index.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            entries.insert(
                parts[0].to_string(),
                (parts[1].to_string(), parts[2].to_string()),
            );
        }

        return Index { entries };
    }

    fn directory_traversal(&mut self, path: &Path) {
        let item: &str = path.to_str().unwrap().split('/').last().unwrap();
        if item == "rit" || item.starts_with(".") {
            panic!("Invalid file");
        }

        if path.is_dir() {
            // DEBUG: println!("Found directory: {:?}", path);
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                self.directory_traversal(&path);
            }
        } else {
            let blob = Blob::new(path).unwrap();
            let sha = blob.calculate_sha().unwrap();
            let mode = "100644".to_string(); // default file mode
            let path_str = path.to_str().unwrap().to_string();
            self.entries.insert(path_str, (mode, sha));
        }
    }

    fn add_single_file(&mut self, path: &Path) {
        let item: &str = path.to_str().unwrap().split('/').last().unwrap();
        if item == "rit" || item.starts_with(".") {
            panic!("Invalid file");
        }

        let blob = Blob::new(path).unwrap();
        let sha = blob.calculate_sha().unwrap();
        let mode = "100644".to_string(); // default file mode
        let path_str = path.to_str().unwrap().to_string();
        self.entries.insert(path_str, (mode, sha));
    }
}

pub fn run_add(path: &String) -> std::io::Result<()> {
    // Read the index file
    let mut index = Index::build_from_file();

    if path == "." {
        // Traverse the root directory for changes
        index.directory_traversal(Path::new("./"));
    } else {
        // Add the specified file
        let path = Path::new(path);
        if path.is_dir() {
            index.directory_traversal(path);
        } else {
            index.add_single_file(path);
        }
    }

    // Write the updated index file
    let mut index_file = fs::File::create(".rit/index")?;
    for (path, (mode, sha)) in index.entries.iter() {
        let line = format!("{} {} {}\n", path, mode, sha);
        index_file.write_all(line.as_bytes())?;
    }

    Ok(())
}
