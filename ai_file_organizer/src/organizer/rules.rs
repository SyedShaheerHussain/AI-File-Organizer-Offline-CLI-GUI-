use crate::models::FileInfo;
use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Result, Context};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub match_extension: Option<Vec<String>>,
    pub match_mime: Option<String>,
    pub match_name_regex: Option<String>,
    pub move_to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RulesConfig {
    pub rules: Vec<Rule>,
}

pub fn load_rules(path: &Path) -> Result<RulesConfig> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read rules file at {:?}", path))?;
    let config: RulesConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn match_rule(file: &FileInfo, rule: &Rule) -> bool {
    // Match extension
    if let Some(extensions) = &rule.match_extension {
        if !extensions.contains(&file.metadata.extension) {
            return false;
        }
    }

    // Match MIME
    if let Some(mime) = &rule.match_mime {
        if !file.metadata.mime_type.contains(mime) {
            return false;
        }
    }

    // Match Regex
    if let Some(regex_str) = &rule.match_name_regex {
        if let Ok(re) = Regex::new(regex_str) {
            if !re.is_match(&file.name) {
                return false;
            }
        }
    }

    true
}

pub fn classify_by_rules(file: &FileInfo, config: &RulesConfig) -> Option<String> {
    for rule in &config.rules {
        if match_rule(file, rule) {
            return Some(rule.move_to.clone());
        }
    }
    None
}
