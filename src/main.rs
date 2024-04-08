use std::env;
use std::fs;

fn main() {
    // 读取命令行输入
    let args: Vec<String> = env::args().collect();

    let _content = &args[1];
    let file_path = &args[2];

    // 读取文件
    let file_contents = fs::read_to_string(file_path).expect("无法读取路径为的文件。");

    println!("文件内容：\n{}", file_contents);
}
