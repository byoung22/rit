use std::env;
use std::fs;
use std::path::Path;

pub mod init {
    use super::*;
    pub fn init_repo() -> std::io::Result<()> {
        let git_dir = format!("{}/.rit", env::current_dir()?.display());
        if !Path::new(&git_dir).exists() {
            fs::create_dir_all(format!("{}/objects", git_dir))?;
            fs::create_dir_all(format!("{}/refs/heads", git_dir))?;
            fs::write(format!("{}/HEAD", git_dir), b"ref: refs/heads/main")?;
            println!("Initialized empty Rit repository in {}", git_dir);
        } else {
            println!("Rit repository already exists at {}", git_dir);
        }
        Ok(())
    }
}
