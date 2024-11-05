use std::{
    fs::{self, File},
    io::Error,
    io::{self, Read},
    path::Path,
};

pub fn read_content(file_name: &str) {
    let path = Path::new(file_name);
    if path.is_file() {
        match read_file_content(file_name) {
            Ok(content) => println!("{}", content),
            Err(e) => file_err(e),
        }
    } else if path.is_dir() {
        match list_files_in_directory(file_name) {
            Ok(files) => {
                if files.is_empty() {
                    println!("-")
                } else {
                    for file in files {
                        println!("{}", file)
                    }
                }
            }
            Err(e) => file_err(e),
        }
    }
}

fn read_file_content(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn list_files_in_directory(dir_name: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(dir_name)?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        files.push(file_name);
    }
    Ok(files)
}

fn file_err(e: Error) {
    eprintln!("读取文件失败：{}", e)
}
