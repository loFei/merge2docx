use clap::Parser;
use docx_rs::{Docx, Paragraph, Run};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'i', long, default_value = ".")]
    input_dir: String,

    #[arg(short = 'o', long, default_value = "output.docx")]
    output_file: String,

    #[arg(long = "ext", default_value = "rs", num_args = 1..)]
    extensions: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let input_dir = Path::new(&cli.input_dir);
    let output_file = &cli.output_file;

    let extensions: HashSet<String> = cli
        .extensions
        .into_iter()
        .map(|s| s.trim_start_matches('.').to_lowercase())
        .collect();

    if !input_dir.exists() {
        eprintln!("输入目录不存在: {}", input_dir.display());
        std::process::exit(1);
    }

    // 扫描所有匹配文件
    let mut files_to_process = Vec::new();
    for entry in WalkDir::new(input_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            if extensions.contains(&ext) {
                files_to_process.push(path.to_path_buf());
            }
        }
    }

    let total = files_to_process.len();
    if total == 0 {
        println!("未找到匹配扩展名的文件。");
        return Ok(());
    }

    println!("共找到 {} 个文件，正在生成文档...", total);

    // === 设置双行进度显示 ===
    let multi = MultiProgress::new();

    // 第一行：当前文件信息（纯文本，无进度条）
    let file_pb = multi.add(ProgressBar::new_spinner());
    file_pb.set_style(ProgressStyle::default_bar().template("{msg}").unwrap());
    file_pb.set_message("准备中...");

    // 第二行：真正的进度条
    let progress_pb = multi.add(ProgressBar::new(total as u64));
    progress_pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:50}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );

    let mut docx = Docx::new();

    for path in &files_to_process {
        let relative_path = path.strip_prefix(input_dir).unwrap_or(path);
        let display_path = relative_path.display().to_string();

        // 更新第一行：当前文件
        file_pb.set_message(format!("正在处理: {}", display_path));

        // 读取并添加内容
        let content = fs::read_to_string(path)
            .unwrap_or_else(|_| format!("无法读取文件: {}", path.display()));

        docx = docx.add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text(format!("File: {}", relative_path.display()))
                    .bold(),
            ),
        );

        for line in content.lines() {
            docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
        }
        docx = docx.add_paragraph(Paragraph::new());

        // 更新进度
        progress_pb.inc(1);
    }

    // 处理完成
    file_pb.finish_with_message("处理完成！");
    progress_pb.finish_with_message("✅ 文件处理完毕");

    println!("正在生成 .docx 文件 '{}'，请稍候...", output_file);

    // === 启动 loading 动画 ===
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let handle = thread::spawn(move || {
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut i = 0;
        while r.load(Ordering::Relaxed) {
            print!("\r{} 正在写入文档... ", spinner[i % spinner.len()]);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            i += 1;
            thread::sleep(Duration::from_millis(100));
        }
        // 清除 loading 行
        print!("\r{: <50}\r", ""); // 用空格覆盖，然后回车
        std::io::Write::flush(&mut std::io::stdout()).ok();
    });
    // 写入文件
    let file = std::fs::File::create(output_file)?;
    docx.build().pack(file)?;

    // === 停止 loading 动画 ===
    running.store(false, Ordering::Relaxed);
    handle.join().unwrap(); // 等待动画线程结束

    println!("\n成功生成文档: {}", output_file);

    Ok(())
}

