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

            // Try to close the process regardless of whether the Cursor process exists
            process::kill_cursor_processes();
            
            // Wait 3 seconds to ensure the process is completely shut down
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
