use std::fs;
use std::path::PathBuf;
use directories::BaseDirs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn get_storage_path() -> Option<PathBuf> {
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

pub fn set_file_permissions(_path: &PathBuf) -> std::io::Result<()> {
    #[cfg(not(target_os = "windows"))]
    {
        let metadata = fs::metadata(_path)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o644);
        fs::set_permissions(_path, perms)?;
    }
    
    Ok(())
}

pub fn delete_config_file() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(file_path) = get_storage_path() {
        println!("Are you sure you want to delete the configuration file?? [y/N]");
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            if input.trim().to_lowercase() == "y" {
                fs::remove_file(&file_path)?;
                println!("Configuration file deleted");
            } else {
                println!("Undeleted");
            }
        }
    }
    Ok(())
} 
