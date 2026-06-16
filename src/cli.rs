use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(short, long, help = "Start in daemon mode")]
    pub daemon: bool,
    #[arg(long, help = "Data directory override")]
    pub data_directory: Option<PathBuf>,
    #[command(subcommand)]
    pub action: Option<Command>
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Migrations {
        #[command(subcommand)]
        action: MigrationsCommand
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum MigrationsCommand {
    Apply,
    RollBack,
    Pending,
    History
}