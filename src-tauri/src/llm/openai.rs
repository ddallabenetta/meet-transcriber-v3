use super::{LlmConfig, LlmError, LlmProvider, ReportContent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct OpenAiProvider {
    config: LlmConfig,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f32,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageResponse,
}

#[derive(Deserialize)]
struct OpenAiMessageResponse {
    content: String,
}

impl OpenAiProvider {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate_report(&self, transcript: &str) -> Result<ReportContent, LlmError> {
        let api_key = self.config.api_key.clone().ok_or(LlmError::NotConfigured)?;

        let base_url = self
            .config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        let url = format!("{}/chat/completions", base_url);

        let request = OpenAiRequest {
            model: self.config.model.clone(),
            messages: vec![
                OpenAiMessage {
                    role: "system".to_string(),
                    content: self.get_system_prompt(&self.config),
                },
                OpenAiMessage {
                    role: "user".to_string(),
                    content: format!("Trascrizione della riunione:\n\n{}", transcript),
                },
            ],
            temperature: 0.3,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError(format!("Status {}: {}", status, text)));
        }

        let openai_response: OpenAiResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let content = openai_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| LlmError::ParseError("Nessuna risposta ricevuta".to_string()))?;

        self.parse_report_response(&content)
    }
}
