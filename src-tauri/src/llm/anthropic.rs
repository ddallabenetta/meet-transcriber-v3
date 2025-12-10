use super::{LlmConfig, LlmError, LlmProvider, ReportContent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct AnthropicProvider {
    config: LlmConfig,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

impl AnthropicProvider {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn generate_report(&self, transcript: &str) -> Result<ReportContent, LlmError> {
        let api_key = self.config.api_key.clone().ok_or(LlmError::NotConfigured)?;

        let base_url = self
            .config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://api.anthropic.com/v1".to_string());

        let url = format!("{}/messages", base_url);

        let request = AnthropicRequest {
            model: self.config.model.clone(),
            max_tokens: 4096,
            system: self.get_system_prompt(&self.config),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: format!("Trascrizione della riunione:\n\n{}", transcript),
            }],
        };

        let response = self
            .client
            .post(&url)
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
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

        let anthropic_response: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let content = anthropic_response
            .content
            .first()
            .map(|c| c.text.clone())
            .ok_or_else(|| LlmError::ParseError("Nessuna risposta ricevuta".to_string()))?;

        self.parse_report_response(&content)
    }
}
