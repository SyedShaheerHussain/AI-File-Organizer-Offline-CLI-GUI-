use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub openai_api_key: Option<String>,
    pub ignore_list: Vec<String>,
    pub default_output_pattern: String,
    pub thread_count: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            openai_api_key: None,
            ignore_list: vec![".git".to_string(), "node_modules".to_string()],
            default_output_pattern: "{category}/{year}/{month}".to_string(),
            thread_count: num_cpus::get(),
        }
    }
}

#[allow(dead_code)]
pub fn load_config() -> AppConfig {
    // In a real app, load from disk
    AppConfig::default()
}
