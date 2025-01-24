use serde_json::Value;
use uuid::Uuid;
use std::fs;
use serde_json::json;
use crate::config::{get_storage_path, set_file_permissions};

pub fn generate_64_hex() -> String {
    let uuid1 = Uuid::new_v4().to_string().replace("-", "");
    let uuid2 = Uuid::new_v4().to_string().replace("-", "");
    format!("{}{}", uuid1, uuid2)[..64].to_string()
}

pub fn update_storage_ids() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_storage_path().ok_or("Unable to get the configuration file path")?;
    
    if !file_path.exists() {
        return Err("The configuration file does not exist".into());
    }
    
    set_file_permissions(&file_path)?;
    
    let content = fs::read_to_string(&file_path)?;
    let mut json: Value = serde_json::from_str(&content)?;
    
    let machine_id = generate_64_hex();
    let mac_machine_id = generate_64_hex();
    let dev_device_id = Uuid::new_v4().to_string();
    
    if let Value::Object(ref mut map) = json {
        map.insert("telemetry.macMachineId".to_string(), json!(mac_machine_id));
        map.insert("telemetry.sqmId".to_string(), json!(machine_id));
        map.insert("telemetry.machineId".to_string(), json!(machine_id));
        map.insert("telemetry.devDeviceId".to_string(), json!(dev_device_id));
    }
    
    fs::write(&file_path, serde_json::to_string_pretty(&json)?)?;
    
    println!("Updated Telemetry IDs:");
    println!("Mac Machine ID: {}", mac_machine_id);
    println!("Machine ID: {}", machine_id);
    println!("Dev Device ID: {}", dev_device_id);
    
    Ok(())
}

pub fn show_telemetry_ids() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_storage_path().ok_or("Unable to get the configuration file path")?;
    
    if !file_path.exists() {
        return Err("The configuration file does not exist".into());
    }
    
    let content = fs::read_to_string(&file_path)?;
    let json: Value = serde_json::from_str(&content)?;
    
    println!("Current Telemetry IDs:");
    println!("File Path: {:?}", file_path);
    
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
