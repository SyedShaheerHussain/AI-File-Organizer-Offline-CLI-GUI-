use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan folder for files and show statistics
    Scan {
        /// Path to scan
        path: String,
    },
    /// Organize files based on rules or AI
    Organize {
        /// Path to organize
        path: String,

        /// Use AI classification
        #[arg(long)]
        ai: bool,

        /// Path to rules file
        #[arg(long)]
        rules: Option<String>,

        /// Dry run mode
        #[arg(long)]
        dry_run: bool,
    },
    /// Find and manage duplicate files
    Duplicates {
        /// Path to check
        path: String,

        /// Remove duplicates automatically
        #[arg(long)]
        remove: bool,

        /// Dry run mode
        #[arg(long)]
        dry_run: bool,
    },
    /// Undo latest organization
    Undo {
        /// Root path
        path: String,
    },
    /// Watch folder for new files and organize them
    Watch {
        /// Path to watch
        path: String,
    },
    /// Launch GUI
    Gui,
}
