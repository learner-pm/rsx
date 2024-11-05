use std::env;
use std::fs;
use std::fs::ReadDir;

fn transform_files(entries: ReadDir) -> Vec<String> {
    entries
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| {
                    let binding = e.path();
                    let name = binding
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default();
                    if e.path().is_file() {
                        format!("文件：{}", name)
                    } else if e.path().is_dir() {
                        format!("文件夹：{}", name)
                    } else {
                        String::new()
                    }
                })
                .filter(|entry| !entry.is_empty())
        })
        .collect()
}

pub fn list_files() {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("无法获取当前工作目录：{}", e);
            return;
        }
    };
    println!("{}", current_dir.display());
    let entries = match fs::read_dir(&current_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("无法读取目录：{}", e);
            return;
        }
    };
    let files = transform_files(entries);

    if files.is_empty() {
        println!("当前目录无其他文件");
    } else {
        //println!("档期目录下的文件：");
        for file in files {
            println!("{}", file);
        }
    }
}
