use crate::llm::{generate_report, LlmConfig, ReportContent};
use crate::AppState;
use chrono::Utc;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn generate_meeting_report(
    state: State<'_, AppState>,
    meeting_id: String,
    transcript: String,
) -> Result<ReportContent, String> {
    // Get LLM config from settings
    let config = get_llm_config_internal(&state)?;

    // Generate report
    let report = generate_report(&config, &transcript)
        .await
        .map_err(|e| e.to_string())?;

    // Save to database
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    let highlights_json = serde_json::to_string(&report.highlights).unwrap_or_default();
    let participants_json = serde_json::to_string(&report.participants).unwrap_or_default();
    let action_items_json = serde_json::to_string(&report.action_items).unwrap_or_default();

    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO reports (id, meeting_id, highlights, participants, action_items, raw_response, llm_provider, llm_model, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            id,
            meeting_id,
            highlights_json,
            participants_json,
            action_items_json,
            report.raw_response,
            config.provider,
            config.model,
            created_at
        ],
    ).map_err(|e| e.to_string())?;

    // Update meeting status
    conn.execute(
        "UPDATE meetings SET status = 'completed' WHERE id = ?1",
        params![meeting_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(report)
}

#[tauri::command]
pub fn get_llm_config(state: State<'_, AppState>) -> Result<LlmConfig, String> {
    get_llm_config_internal(&state)
}

fn get_llm_config_internal(state: &State<'_, AppState>) -> Result<LlmConfig, String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    let get_setting = |key: &str| -> Option<String> {
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .ok()
    };

    Ok(LlmConfig {
        provider: get_setting("llm_provider").unwrap_or_else(|| "ollama".to_string()),
        api_key: get_setting("llm_api_key"),
        base_url: get_setting("llm_base_url"),
        model: get_setting("llm_model").unwrap_or_else(|| "llama3".to_string()),
        system_prompt: get_setting("llm_system_prompt"),
    })
}

#[tauri::command]
pub fn save_llm_config(state: State<'_, AppState>, config: LlmConfig) -> Result<(), String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    let upsert = |key: &str, value: &str| -> Result<(), String> {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2",
            params![key, value],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    };

    upsert("llm_provider", &config.provider)?;
    upsert("llm_model", &config.model)?;

    if let Some(api_key) = &config.api_key {
        upsert("llm_api_key", api_key)?;
    }

    if let Some(base_url) = &config.base_url {
        upsert("llm_base_url", base_url)?;
    }

    if let Some(system_prompt) = &config.system_prompt {
        upsert("llm_system_prompt", system_prompt)?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_default_system_prompt() -> String {
    crate::llm::provider::DEFAULT_SYSTEM_PROMPT.to_string()
}
