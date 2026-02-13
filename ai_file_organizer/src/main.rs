mod cli;
mod config;
mod constants;
mod gui;
mod models;
mod organizer;

use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            println!("Scanning path: {}", path);
            let files = organizer::scanner::scan(Path::new(&path))?;
            println!("Found {} files", files.len());
            for mut file in files {
                let _ = organizer::metadata::extract_metadata(&mut file);
                println!("- {} ({})", file.name, file.metadata.mime_type);
            }
        }
        Commands::Organize { path, ai, rules, dry_run } => {
            println!("Organizing path: {}", path);
            let organizer = organizer::Organizer::new(dry_run, ai, rules);
            organizer.process(Path::new(&path)).await?;
        }
        Commands::Duplicates { path, remove, dry_run } => {
            println!("Checking for duplicates in: {}", path);
            let mut files = organizer::scanner::scan(Path::new(&path))?;
            let duplicates = organizer::duplicates::find_duplicates(&mut files)?;
            
            if remove {
                organizer::duplicates::remove_duplicates(&duplicates, dry_run)?;
            } else {
                for (hash, set) in duplicates {
                    println!("Hash {}: {} duplicates", hash, set.len());
                    for f in set {
                        println!("  - {:?}", f.path);
                    }
                }
            }
        }
        Commands::Undo { path } => {
            println!("Undoing changes in: {}", path);
            organizer::undo::undo(Path::new(&path))?;
        }
        Commands::Watch { path } => {
            println!("Watching path: {}", path);
            organizer::watcher::watch_folder(Path::new(&path))?;
        }
        Commands::Gui => {
            if let Err(e) = gui::run_gui() {
                eprintln!("GUI Error: {}", e);
            }
        }
    }

    Ok(())
}
