use std::process::Command;

pub fn kill_cursor_processes() {
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

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let graceful_output = Command::new("pkill")
            .args(["-TERM", "Cursor"])
            .output();
        
        match graceful_output {
            Ok(output) => {
                if output.status.success() {
                    println!("成功终止所有 Cursor 进程");
                } else {
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

pub fn is_cursor_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq cursor.exe"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains("cursor.exe"))
            .unwrap_or(false)
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        Command::new("pgrep")
            .arg("Cursor")
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
} 