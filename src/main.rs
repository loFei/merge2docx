use clap::Parser;
use docx_rs::{Docx, Paragraph, Run};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 输入目录路径
    #[arg(short = 'i', long, default_value = ".")]
    input_dir: String,

    /// 输出的 .docx 文件名
    #[arg(short = 'o', long, default_value = "output.docx")]
    output_file: String,

    /// 要处理的文件后缀（例如 .rs、.txt）
    #[arg(short = 'e', long = "ext", default_value = ".rs")]
    extension: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let input_dir = Path::new(&cli.input_dir);
    let output_file = &cli.output_file;
    let extension = cli.extension.trim_start_matches('.');

    if !input_dir.exists() {
        eprintln!("输入目录不存在: {}", input_dir.display());
        std::process::exit(1);
    }

    let mut docx = Docx::new();

    // 递归遍历目录
    for entry in WalkDir::new(input_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            if ext == extension {
                // 读取文件内容
                let content = fs::read_to_string(path)
                    .unwrap_or_else(|_| format!("无法读取文件: {}", path.display()));

                // 添加文件路径作为标题
                let relative_path = path.strip_prefix(input_dir).unwrap_or(path);
                docx = docx.add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(format!("File: {}", relative_path.display()))
                            .bold(),
                    ),
                );

                // 添加文件内容（逐行处理，避免段落过长）
                for line in content.lines() {
                    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
                }

                // 空一行分隔
                docx = docx.add_paragraph(Paragraph::new());
            }
        }
    }

    // 写入 .docx 文件
    let path = std::path::Path::new(&output_file);
    let file = std::fs::File::create(path).unwrap();
    docx.build().pack(file)?;
    println!("成功生成文档: {}", output_file);

    Ok(())
}
