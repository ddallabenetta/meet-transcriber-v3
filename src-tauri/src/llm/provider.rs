use super::LlmError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub const DEFAULT_SYSTEM_PROMPT: &str = r#"Sei un assistente specializzato nell'analisi di trascrizioni di riunioni.
Analizza la seguente trascrizione e fornisci:

1. **Punti Salienti**: I 3-5 argomenti piu importanti discussi
2. **Partecipanti**: Le persone menzionate o che hanno partecipato
3. **Action Items**: Compiti, decisioni o azioni da intraprendere

Rispondi SOLO con un JSON valido nel seguente formato, senza altro testo:
{
  "highlights": ["punto 1", "punto 2", "punto 3"],
  "participants": ["persona 1", "persona 2"],
  "action_items": ["azione 1", "azione 2"]
}"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub system_prompt: Option<String>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            api_key: None,
            base_url: Some("http://localhost:11434".to_string()),
            model: "llama3".to_string(),
            system_prompt: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportContent {
    pub highlights: Vec<String>,
    pub participants: Vec<String>,
    pub action_items: Vec<String>,
    pub raw_response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: String,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate_report(&self, transcript: &str) -> Result<ReportContent, LlmError>;

    fn get_system_prompt(&self, config: &LlmConfig) -> String {
        config
            .system_prompt
            .clone()
            .unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string())
    }

    fn parse_report_response(&self, response: &str) -> Result<ReportContent, LlmError> {
        // Try to extract JSON from the response
        let json_str = if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                &response[start..=end]
            } else {
                response
            }
        } else {
            response
        };

        #[derive(Deserialize)]
        struct ParsedReport {
            highlights: Option<Vec<String>>,
            participants: Option<Vec<String>>,
            action_items: Option<Vec<String>>,
        }

        let parsed: ParsedReport = serde_json::from_str(json_str)
            .map_err(|e| LlmError::ParseError(format!("Errore parsing JSON: {}", e)))?;

        Ok(ReportContent {
            highlights: parsed.highlights.unwrap_or_default(),
            participants: parsed.participants.unwrap_or_default(),
            action_items: parsed.action_items.unwrap_or_default(),
            raw_response: response.to_string(),
        })
    }
}
