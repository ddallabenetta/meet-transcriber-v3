pub mod anthropic;
pub mod ollama;
pub mod openai;
pub mod provider;

pub use anthropic::AnthropicProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAiProvider;
pub use provider::{LlmConfig, LlmProvider, ReportContent};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Errore di rete: {0}")]
    NetworkError(String),
    #[error("Errore API: {0}")]
    ApiError(String),
    #[error("Errore di parsing: {0}")]
    ParseError(String),
    #[error("Provider non configurato")]
    NotConfigured,
}

pub async fn generate_report(
    config: &LlmConfig,
    transcript: &str,
) -> Result<ReportContent, LlmError> {
    match config.provider.as_str() {
        "openai" => {
            let provider = OpenAiProvider::new(config.clone());
            provider.generate_report(transcript).await
        }
        "anthropic" => {
            let provider = AnthropicProvider::new(config.clone());
            provider.generate_report(transcript).await
        }
        "ollama" => {
            let provider = OllamaProvider::new(config.clone());
            provider.generate_report(transcript).await
        }
        _ => Err(LlmError::NotConfigured),
    }
}
