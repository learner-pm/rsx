pub const HELP_TEXT: [&str; 5] = [
    "\x1b[36mhelp, h\x1b[0m   显示帮助信息",
    "\x1b[36mlist, ls\x1b[0m   列出当前目录下的文件和文件夹",
    "\x1b[36msearch [文件名], s [文件名]\x1b[0m   从当前目录下查询目标文件",
    "\x1b[36mread [文件名], cat [文件名]\x1b[0m   读取文件内容或列出文件夹中的文件",
    "\x1b[36mversion, v\x1b[0m   版本号",
];

pub fn print_help() {
    println!("");
    println!("\x1b[32mOptions:\x1b[0m");
    for line in HELP_TEXT.iter() {
        println!(" {}", line);
    }
    println!("");
}
