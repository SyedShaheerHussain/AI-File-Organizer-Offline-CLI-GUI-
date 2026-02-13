pub mod archives;
pub mod scanner;
pub mod renamer;
pub mod classifier;
pub mod rules;
pub mod ai;
pub mod duplicates;
pub mod metadata;
pub mod undo;
pub mod utils;
pub mod watcher;

use std::path::Path;
use anyhow::Result;

pub struct Organizer {
    pub dry_run: bool,
    pub use_ai: bool,
    pub custom_rules: Option<String>,
}

impl Organizer {
    pub fn new(dry_run: bool, use_ai: bool, custom_rules: Option<String>) -> Self {
        Self { dry_run, use_ai, custom_rules }
    }

    pub async fn process(&self, path: &Path) -> Result<()> {
        log::info!("Starting organization of {:?}", path);
        
        // Step 1: Scan
        let mut files = scanner::scan(path)?;
        log::info!("Found {} files", files.len());

        let renamer = renamer::Renamer::new(path.to_path_buf(), self.dry_run);
        
        // Load rules if any
        let rules_path = if let Some(custom) = &self.custom_rules {
            Path::new(custom).to_path_buf()
        } else {
            path.join("config/default_rules.json")
        };

        let rules_config = if rules_path.exists() {
            Some(rules::load_rules(&rules_path)?)
        } else {
            None
        };

        // Initialize AI if needed
        let ai_classifier = if self.use_ai {
            // Get API key from env for now
            if let Ok(key) = std::env::var("OPENAI_API_KEY") {
                Some(ai::AIClassifier::new(key))
            } else {
                log::warn!("AI enabled but OPENAI_API_KEY not set");
                None
            }
        } else {
            None
        };

        for file in &mut files {
            // Step 2: Extract Metadata
            let _ = metadata::extract_metadata(file);

            // Step 3: Classify
            let mut category = "Unknown".to_string();

            // Try rules first
            if let Some(config) = &rules_config {
                if let Some(cat) = rules::classify_by_rules(file, config) {
                    category = cat;
                }
            }

            // Try AI if rules didn't work and AI is available
            if category == "Unknown" {
                if let Some(ai) = &ai_classifier {
                    if let Ok(ai_cat) = ai.classify(file).await {
                        category = ai_cat;
                    }
                }
            }

            // Step 4: Rename/Move
            let dest = renamer.apply(file, &category)?;
            println!("Processed {:?} -> {:?}", file.path, dest);
        }

        Ok(())
    }
}
