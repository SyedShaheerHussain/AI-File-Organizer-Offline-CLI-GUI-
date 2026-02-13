use crate::models::FileInfo;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

pub struct AIClassifier {
    api_key: String,
}

impl AIClassifier {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn classify(&self, file: &FileInfo) -> Result<String> {
        let client = reqwest::Client::new();
        let prompt = format!(
            "Classify the following file into one of these categories: Work, Personal, Finance, Media, Code, Screenshots, Notes, Archives. 
            File Name: {}
            MIME Type: {}
            Metadata: {:?}
            Only return the category name.",
            file.name, file.metadata.mime_type, file.metadata.extra
        );

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&OpenAIRequest {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: prompt,
                }],
            })
            .send()
            .await?;

        let res_body: OpenAIResponse = response.json().await?;
        let category = res_body.choices.first()
            .ok_or_else(|| anyhow!("No response from AI"))?
            .message.content.trim().to_string();

        Ok(category)
    }
}
