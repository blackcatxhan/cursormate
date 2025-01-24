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
                eprintln!("Read ID failed: {}", e);
            }
        }
        Commands::RandomIds => {

            // 无论 Cursor 进程是否存在，都尝试关闭进程
            process::kill_cursor_processes();
            
            // 等待 3 秒确保进程完全关闭
            thread::sleep(Duration::from_secs(3));

        
            if let Err(e) = telemetry::update_storage_ids() {
                eprintln!("Update ID failed: {}", e);
            }
        }
        Commands::Delete => {
            if let Err(e) = config::delete_config_file() {
                eprintln!("Failed to delete the configuration file: {}", e);
            }
        }
        Commands::Kill => {
            process::kill_cursor_processes();
        }
    }
}
