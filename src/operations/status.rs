use std::fs;
use std::path::Path;

pub fn directory_traversal(path: &Path) {
    let item: &str = path.to_str().unwrap().split('/').last().unwrap();
    if item == "rit" || item.starts_with(".") {
        return;
    }
    
    if path.is_dir() {
        println!("Found directory: {:?}", path);
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            directory_traversal(&path);
        }
    } else {
        println!("Found file: {:?}", path);
    }
}