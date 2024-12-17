// Windows-specific imports
#[cfg(target_os = "windows")]
use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::TerminateProcess,
    tlhelp32::{
        CreateToolhelp32Snapshot, Process32First, Process32Next,
        TH32CS_SNAPPROCESS, PROCESSENTRY32
    },
    winnt::PROCESS_TERMINATE,
};

// Unix-specific imports (macOS and Linux)
#[cfg(any(target_os = "macos", target_os = "linux"))]
use {
    libc::{kill, SIGKILL, SIGTERM},
    std::io::BufReader,
    std::process::Stdio,
    std::process::Command,
    std::io::BufRead,
};


#[cfg(target_os = "windows")]
fn find_pids(process_name: &str) -> Result<Vec<u32>, String> {

    let mut pids = Vec::new();

    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err("Failed to create snapshot".to_string());
    }

    let mut entry: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;


    if unsafe { Process32First(snapshot, &mut entry) } == 0 {
        unsafe { CloseHandle(snapshot) };
        return Err("Failed to get first process".to_string());
    }

    loop {
        let mut name_vec = Vec::new();
        for &c in entry.szExeFile.iter(){
             if c == 0{
                 break;
             }
            name_vec.push(c as u8);
        }
        let process_exe_name = String::from_utf8_lossy(&name_vec);

        if process_exe_name.to_lowercase() == process_name.to_lowercase(){
            pids.push(entry.th32ProcessID);
        }


        if unsafe { Process32Next(snapshot, &mut entry) } == 0 {
            break;
        }
    }
    unsafe { CloseHandle(snapshot) };

    Ok(pids)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn find_pids(process_name: &str) -> Result<Vec<u32>, String> {
    let output = Command::new("ps")
        .args(&["-ax", "-o", "pid,comm"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute ps command: {}", e))?;
    if !output.status.success() {
         return Err(format!("ps command failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let reader = BufReader::new(output.stdout.as_slice());
    let mut pids = Vec::new();

    for line in reader.lines(){
        let line = line.map_err(|e| format!("Failed to read line from ps output: {}", e))?;
         let parts: Vec<&str> = line.trim().split_whitespace().collect();
         if parts.len() < 2{
              continue;
         }

         let pid_str = parts[0];
         let current_process_name = parts[1];

          if current_process_name.to_lowercase() == process_name.to_lowercase() {
                let pid = pid_str.parse::<u32>().map_err(|e|format!("Invalid pid {} from ps output: {}",pid_str, e))?;
                pids.push(pid);
          }

    }
    Ok(pids)
}


#[cfg(target_os = "windows")]
fn kill_process(pid: u32) -> Result<(), ()>{
    use winapi::um::processthreadsapi::OpenProcess;
    let handle = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
    if handle.is_null() {
        return Err(());
    }
    let result = unsafe { TerminateProcess(handle, 1) };
    unsafe { CloseHandle(handle) };
    if result == 0{
      return Err(());
    }
    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn kill_process(pid: u32) -> Result<(), ()>{
    let result = unsafe { kill(pid as i32, SIGKILL) };
    if result != 0{
        return Err(());
    }
    Ok(())
}

pub fn kill_cursor_processes() {
    #[cfg(target_os = "windows")]
    {
        if let Ok(pids) = find_pids("cursor.exe") {
            if pids.is_empty() {
                println!("没有找到运行中的 Cursor 进程");
                return;
            }

            let mut success = false;
            for pid in pids {
                match kill_process(pid) {
                    Ok(()) => {
                        println!("成功终止 Cursor 进程 (PID: {})", pid);
                        success = true;
                    }
                    Err(()) => eprintln!("无法终止 Cursor 进程 (PID: {})", pid),
                }
            }

            if success {
                println!("成功终止所有 Cursor 进程");
            }
        } else {
            eprintln!("查找 Cursor 进程失败");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        if let Ok(pids) = find_pids("Cursor").or_else(|_| find_pids("cursor")) {
            if pids.is_empty() {
                println!("没有找到运行中的 Cursor 进程");
                return;
            }

            let mut success = false;
            for pid in &pids {
                match unsafe { kill(*pid as i32, SIGTERM) } {
                    0 => {
                        println!("成功终止 Cursor 进程 (PID: {})", pid);
                        success = true;
                    }
                    _ => {
                        match kill_process(*pid) {
                            Ok(()) => {
                                println!("成功强制终止 Cursor 进程 (PID: {})", pid);
                                success = true;
                            }
                            Err(()) => eprintln!("无法终止 Cursor 进程 (PID: {})", pid),
                        }
                    }
                }
            }

            if success {
                println!("成功终止所有 Cursor 进程");
            }
        } else {
            eprintln!("查找 Cursor 进程失败");
        }
    }
}
