use crate::models::FileInfo;
use crate::organizer::rules::{RulesConfig, classify_by_rules};
use crate::organizer::ai::AIClassifier;
use anyhow::Result;

#[allow(dead_code)]
pub enum ClassificationMode {
    Rules(RulesConfig),
    AI(AIClassifier),
    Hybrid(RulesConfig, AIClassifier),
}

#[allow(dead_code)]
pub struct Classifier {
    pub mode: ClassificationMode,
}

impl Classifier {
    #[allow(dead_code)]
    pub async fn classify(&self, file: &FileInfo) -> Result<Option<String>> {
        match &self.mode {
            ClassificationMode::Rules(config) => {
                Ok(classify_by_rules(file, config))
            }
            ClassificationMode::AI(ai) => {
                Ok(Some(ai.classify(file).await?))
            }
            ClassificationMode::Hybrid(config, ai) => {
                if let Some(category) = classify_by_rules(file, config) {
                    Ok(Some(category))
                } else {
                    Ok(Some(ai.classify(file).await?))
                }
            }
        }
    }
}
