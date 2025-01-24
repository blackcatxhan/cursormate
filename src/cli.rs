use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cursor-mate")]
#[command(about = "The command line tool to manage the Cursor configuration file", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 显示当前 Telemetry IDs 信息
    Ids,
    /// 生成随机 Telemetry IDs
    RandomIds,
    /// 删除配置文件
    Delete,
    /// 终止所有 Cursor 进程
    Kill,
} 
