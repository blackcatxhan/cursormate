use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cursor-mate")]
#[command(about = "Command line tool for managing Cursor configuration files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display current Telemetry IDs information
    Ids,
    /// Generate random Telemetry IDs
    RandomIds,
    /// Deleting a Profile
    Delete,
    /// Terminate all Cursor processes
    Kill,
} 
