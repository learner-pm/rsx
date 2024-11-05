pub const HELP_TEXT: [&str; 4] = [
    "【help, h】 显示帮助信息",
    "【list, ls】 列出当前目录下的文件和文件夹",
    "【read [文件名], cat [文件名]】 读取文件内容或列出文件夹中的文件",
    "【verbose, v】 打开详细日志",
];

pub fn print_help() {
    for line in HELP_TEXT.iter() {
        println!("{}", line);
    }
}
