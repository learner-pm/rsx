mod help;
mod list;
mod read;
mod search;
mod util;
mod version;

use crate::help::print_help;
use crate::list::list_files;
use read::read_content;
use search::search_files;
use std::process;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use util::progress::ProgressSpinner;
use version::print_version;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "list" | "ls" => list_files(),
        "help" | "h" => print_help(),
        "version" | "v" => print_version(),
        "search" => {
            if args.len() != 3 {
                println!("用法: cli search [文件名]");
                process::exit(1);
            }
            let query = &args[2];
            let current_dir = std::env::current_dir().expect("无法获取当前目录");
            let file_count = Arc::new(AtomicUsize::new(0));

            let loading = ProgressSpinner::new("正在查询中", 0);

            let results = search_files(query, &current_dir, &file_count, &loading);

            loading.finish("查询结束".to_string());

            if results.is_empty() {
                println!("没有匹配到 {} 的文件", query)
            } else {
                loading.update_message(format!("查询结束，总共有{}个文件", results.len()));
                //println!("", results.len());
                for path in results {
                    println!("{}", path.display())
                }
            }
        }
        "read" | "cat" => {
            if args.len() < 3 {
                eprintln!("错误: 'read' 命令需要一个文件名作为参数");
                process::exit(1);
            }
            let file_name = &args[2];
            read_content(file_name);
        }
        _ => {
            // 如果是未知命令，打印错误并退出程序
            eprintln!("错误: 未知的命令 '{}'。使用 --help 获取帮助", command);
            process::exit(1); // 非零退出码表示错误
        }
    }
}
