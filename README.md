# 📄 merge-to-docx

> 将指定目录下特定后缀的文件或文本文件合并为一个 `.docx` 文档。

---

## ✨ 功能特点

- **递归扫描**：遍历指定目录及其所有子目录。
- **按后缀过滤**：只处理指定扩展名的文件（如 `.rs`, `.py`, `.txt` 等）。
- **自动生成 Word 文档**：使用 [docx-rs](https://crates.io/crates/docx-rs) 库创建标准 `.docx` 文件。
- **路径标识**：每个文件内容前自动添加文件相对路径作为标题，便于溯源。
- **命令行友好**：支持默认参数、清晰的帮助信息和错误提示。

---

## 🚀 安装

确保已安装 [Rust 1.70+](https://www.rust-lang.org/tools/install)（本工具在 Rust 1.91 下开发测试）。

克隆并构建：

```bash
git clone https://github.com/loFei/merge2docx.git
cd merge2docx
cargo build --release
```
直接安装：
```bash
cargo install --git https://github.com/loFei/merge2docx.git
```

## 🛠️ 使用方法
```bash
Usage: merge_docx [OPTIONS]

Options:
  -i, --input-dir <INPUT_DIR>      输入目录路径 [default: .]
  -o, --output-file <OUTPUT_FILE>  输出的 .docx 文件名 [default: output.docx]
  -e, --ext <EXTENSION>            要处理的文件后缀（例如 .rs、.txt） [default: .rs]
  -h, --help                       Print help
  -V, --version                    Print version

# 示例：将src目录下所有.h .cpp结尾的文件内容合并至code.docx
merge2docx -i ./src -o code.docx --ext .h .cpp
```

