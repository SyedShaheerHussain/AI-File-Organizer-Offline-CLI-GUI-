use notify::{Watcher, RecursiveMode, Result, Event};
use std::path::Path;
use std::sync::mpsc::channel;

pub fn watch_folder(path: &Path) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    println!("Watching folder: {:?}", path);

    for res in rx {
        match res {
            Ok(event) => {
                if is_create_event(&event) {
                    println!("New file detected: {:?}", event.paths);
                    // Add logic to trigger organization
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn is_create_event(event: &Event) -> bool {
    matches!(event.kind, notify::EventKind::Create(_))
}
