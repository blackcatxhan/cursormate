use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use directories::BaseDirs;
use clap::{Parser, Subcommand};
use std::process::Command;
use serde_json::Value;
use serde_json::json;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Parser)]
#[command(name = "cursor-mate")]
#[command(about = "管理 Cursor 配置文件的命令行工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 显示当前 Telemetry IDs 信息
    Ids,
    /// 生成随机 Telemetry IDs
    RandomIds,
    /// 删除配置文件
    Delete,
    /// 终止所有 Cursor 进程
    Kill,
}

/// 获取 storage.json 文件路径
fn get_storage_path() -> Option<PathBuf> {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut path = PathBuf::new();
        
        #[cfg(target_os = "windows")]
        {
            path.push(base_dirs.data_dir());
            path.push("Cursor");
        }
        
        #[cfg(target_os = "macos")]
        {
            path.push(base_dirs.home_dir());
            path.push("Library");
            path.push("Application Support");
            path.push("Cursor");
        }
        
        #[cfg(target_os = "linux")]
        {
            path.push(base_dirs.config_dir());
            path.push("Cursor");
        }
        
        path.push("User");
        path.push("globalStorage");
        path.push("storage.json");
        
        Some(path)
    } else {
        None
    }
}

/// 检查并设置文件权限
fn set_file_permissions(_path: &PathBuf) -> std::io::Result<()> {
    #[cfg(not(target_os = "windows"))]
    {
        let metadata = fs::metadata(_path)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o644); // 设置读写权限
        fs::set_permissions(_path, perms)?;
     
    }
    
    #[cfg(unix)]  // 同时处理 Linux 和 macOS
    {
        let metadata = fs::metadata(_path)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o644);  // 设置用户读写，组和其他用户只读权限
        fs::set_permissions(_path, perms)?;
        Ok(())
    }
    Ok(())
}

/// 更新 storage.json 中的 ID
fn update_storage_ids() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_storage_path().ok_or("无法获取配置文件路径")?;
    
    if !file_path.exists() {
        return Err("配置文件不存在".into());
    }
    
    // 设置文件权限
    set_file_permissions(&file_path)?;
    
    // 读取现有文件
    let content = fs::read_to_string(&file_path)?;
    let mut json: Value = serde_json::from_str(&content)?;
    
    // 生成新的 UUID
    let machine_id = Uuid::new_v4().to_string();
    let mac_machine_id = Uuid::new_v4().to_string();
    let dev_device_id = Uuid::new_v4().to_string();
    
    // 更新 telemetry IDs
    if let Value::Object(ref mut map) = json {
        map.insert("telemetry.macMachineId".to_string(), json!(mac_machine_id));
        map.insert("telemetry.sqmId".to_string(), json!(machine_id));
        map.insert("telemetry.machineId".to_string(), json!(machine_id));
        map.insert("telemetry.devDeviceId".to_string(), json!(dev_device_id));
    }
    
    // 写入文件
    fs::write(&file_path, serde_json::to_string_pretty(&json)?)?;
    
    println!("已更新 Telemetry IDs:");
    println!("Mac Machine ID: {}", mac_machine_id);
    println!("Machine ID: {}", machine_id);
    println!("Dev Device ID: {}", dev_device_id);
    
    Ok(())
}

/// 读取并显示当前的 Telemetry IDs
fn show_telemetry_ids() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_storage_path().ok_or("无法获取配置文件路径")?;
    
    if !file_path.exists() {
        return Err("配置文件不存在".into());
    }
    
    let content = fs::read_to_string(&file_path)?;
    let json: Value = serde_json::from_str(&content)?;
    
    println!("当前 Telemetry IDs:");
    println!("文件路径: {:?}", file_path);
    
    if let Value::Object(ref map) = json {
        if let Some(value) = map.get("telemetry.macMachineId") {
            println!("Mac Machine ID: {}", value);
        }
        if let Some(value) = map.get("telemetry.machineId") {
            println!("Machine ID: {}", value);
        }
        if let Some(value) = map.get("telemetry.devDeviceId") {
            println!("Dev Device ID: {}", value);
        }
    }
    
    Ok(())
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
        // 首先尝试优雅地终止进程
        let graceful_output = Command::new("pkill")
            .args(["-TERM", "Cursor"])
            .output();
        
        match graceful_output {
            Ok(output) => {
                if output.status.success() {
                    println!("成功终止所有 Cursor 进程");
                } else {
                    // 如果优雅终止失败，再尝试强制终止
                    let force_output = Command::new("pkill")
                        .args(["-KILL", "Cursor"])
                        .output();
                    
                    match force_output {
                        Ok(output) => {
                            if output.status.success() {
                                println!("成功强制终止所有 Cursor 进程");
                            } else {
                                println!("没有找到运行中的 Cursor 进程");
                            }
                        }
                        Err(e) => eprintln!("执行命令失败: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("执行命令失败: {}", e),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 首先尝试优雅地终止进程
        let graceful_output = Command::new("pkill")
            .args(["-TERM", "Cursor"])
            .output();
        
        match graceful_output {
            Ok(output) => {
                if output.status.success() {
                    println!("成功终止所有 Cursor 进程");
                } else {
                    // 如果优雅终止失��，再尝试强制终止
                    let force_output = Command::new("pkill")
                        .args(["-KILL", "Cursor"])
                        .output();
                    
                    match force_output {
                        Ok(output) => {
                            if output.status.success() {
                                println!("成功强制终止所有 Cursor 进程");
                            } else {
                                println!("没有找到运行中的 Cursor 进程");
                            }
                        }
                        Err(e) => eprintln!("执行命令失败: {}", e),
                    }
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
            if let Err(e) = show_telemetry_ids() {
                eprintln!("读取 ID 失败: {}", e);
            }
        }
        Commands::RandomIds => {
            if let Err(e) = update_storage_ids() {
                eprintln!("更新 ID 失败: {}", e);
            }
        }
        Commands::Delete => {
            if let Some(file_path) = get_storage_path() {
                println!("确定要删除配置���件吗? [y/N]");
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    if input.trim().to_lowercase() == "y" {
                        match fs::remove_file(&file_path) {
                            Ok(_) => println!("配置文件已删除"),
                            Err(e) => eprintln!("删除文件失败: {}", e)
                        }
                    } else {
                        println!("已取消删除");
                    }
                }
            }
        }
        Commands::Kill => {
            kill_cursor_processes();
        }
    }
}
