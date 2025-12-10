use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{LlmConfig, LlmError, LlmProvider, ReportContent};

pub struct OllamaProvider {
    config: LlmConfig,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate_report(&self, transcript: &str) -> Result<ReportContent, LlmError> {
        let base_url = self.config.base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());

        let url = format!("{}/api/generate", base_url);

        let request = OllamaRequest {
            model: self.config.model.clone(),
            prompt: format!("Trascrizione della riunione:\n\n{}", transcript),
            system: self.get_system_prompt(&self.config),
            stream: false,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError(format!("Status {}: {}", status, text)));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        self.parse_report_response(&ollama_response.response)
    }
}
