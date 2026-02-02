

use std::path::PathBuf;

use walkdir::WalkDir;
pub fn scan_walkdir(path: String) -> Vec<PathBuf>{
    let mut entry_vec = Vec::new();
    for entry in WalkDir::new(path){
       match entry {
           Ok(e) => entry_vec.push(e.path().to_path_buf()),
           Err(_) => {}
       }
    }
    entry_vec
}