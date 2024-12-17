mod cli;
mod config;
mod telemetry;
mod process;

use cli::{Cli, Commands};
use clap::Parser;
use std::thread;
use std::time::Duration;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ids => {
            if let Err(e) = telemetry::show_telemetry_ids() {
                eprintln!("读取 ID 失败: {}", e);
            }
        }
        Commands::RandomIds => {
            if process::is_cursor_running() {
                println!("检测到 Cursor 正在运行，尝试关闭进程...");
                process::kill_cursor_processes();
                
                thread::sleep(Duration::from_secs(3));
                
                if process::is_cursor_running() {
                    eprintln!("无法完全关闭 Cursor 进程，请手动关闭后再试");
                    return;
                }
            }
            
            if let Err(e) = telemetry::update_storage_ids() {
                eprintln!("更新 ID 失败: {}", e);
            }
        }
        Commands::Delete => {
            if let Err(e) = config::delete_config_file() {
                eprintln!("删除配置文件失败: {}", e);
            }
        }
        Commands::Kill => {
            process::kill_cursor_processes();
        }
    }
}
