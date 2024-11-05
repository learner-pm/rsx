use std::{fs, path::Path};

pub fn search_files(query: &str, dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    search_files(query, &path);
                } else {
                    if let Some(file_name) = path.file_name() {
                        if file_name.to_string_lossy().contains(query) {
                            println!("{}", path.display())
                        }
                    }
                }
            }
        }
    }
}
