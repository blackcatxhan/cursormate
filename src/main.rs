use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use directories::BaseDirs;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "cursor-mate")]
#[command(about = "管理 Cursor 配置文件的命令行工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 显示机器 ID 信息
    Ids,
    /// 生成随机机器 ID
    RandomIds,
    /// 删除机器 ID 文件
    Delete,
    /// 终止所有 Cursor 进程
    Kill,
  
}

fn get_machine_id_path() -> Option<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let config_dir = base_dirs.config_dir();
        let mut path = PathBuf::from(config_dir);
        path.push("cursor");
        path.push("machineid");
        Some(path)
    } else {
        None
    }
}

fn read_machine_id(file_path: &PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn show_machine_id() {
    if let Some(file_path) = get_machine_id_path() {
        println!("文件路径: {:?}", file_path);
        match read_machine_id(&file_path) {
            Ok(content) => println!("当前的 machineid 是: {}", content.trim()),
            Err(e) => println!("读取文件失败或文件不存在: {}", e),
        }
    }
}

fn delete_machine_id() {
    if let Some(file_path) = get_machine_id_path() {
        println!("确定要删除机器 ID 文件吗? [y/N]");
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            if input.trim().to_lowercase() == "y" {
                match fs::remove_file(&file_path) {
                    Ok(_) => println!("机器 ID 文件已删除"),
                    Err(e) => eprintln!("删除文件失败: {}", e)
                }
            } else {
                println!("已取消删除");
            }
        } else {
            eprintln!("读取输入失败");
        }
    }
}

fn generate_random_id() {
    if let Some(file_path) = get_machine_id_path() {
        // 生成新的UUID
        let new_machine_id = Uuid::new_v4().to_string();
        
        // 确保目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("创建目录失败: {}", e);
            });
        }
        
        // 写入新的machine id
        match fs::write(&file_path, &new_machine_id) {
            Ok(_) => println!("成功更新 machineid 为: {}", new_machine_id),
            Err(e) => eprintln!("写入文件失败: {}", e),
        }

        // 再次读取确认更新后的内容
        match read_machine_id(&file_path) {
            Ok(content) => println!("更新后的 machineid 是: {}", content.trim()),
            Err(e) => println!("读取更新后的文件失败: {}", e),
        }
    }
}

fn kill_cursor_processes() {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("taskkill")
            .args(["/F", "/IM", "cursor.exe"])
            .output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("成功终止所有 Cursor 进程");
                } else {
                    println!("没有找到运行中的 Cursor 进程");
                }
            }
            Err(e) => eprintln!("执行命令失败: {}", e),
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("pkill")
            .arg("cursor")
            .output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("成功终止所有 Cursor 进程");
                } else {
                    println!("没有找到运行中的 Cursor 进程");
                }
            }
            Err(e) => eprintln!("执行命令失败: {}", e),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ids => {
            show_machine_id();
        }
        Commands::RandomIds => {
            generate_random_id();
        }
        Commands::Delete => {
            delete_machine_id();
        }
        Commands::Kill => {
            kill_cursor_processes();
        }
      
    }
}
