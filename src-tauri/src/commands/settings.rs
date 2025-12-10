use crate::AppState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub whisper_model: String,
    pub default_language: Option<String>,
    pub auto_transcribe: bool,
    pub auto_generate_report: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            whisper_model: "base".to_string(),
            default_language: Some("it".to_string()),
            auto_transcribe: false,
            auto_generate_report: false,
        }
    }
}

#[tauri::command]
pub fn get_app_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    let get_setting = |key: &str| -> Option<String> {
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .ok()
    };

    Ok(AppSettings {
        whisper_model: get_setting("whisper_model").unwrap_or_else(|| "base".to_string()),
        default_language: get_setting("default_language"),
        auto_transcribe: get_setting("auto_transcribe")
            .map(|v| v == "true")
            .unwrap_or(false),
        auto_generate_report: get_setting("auto_generate_report")
            .map(|v| v == "true")
            .unwrap_or(false),
    })
}

#[tauri::command]
pub fn save_app_settings(state: State<'_, AppState>, settings: AppSettings) -> Result<(), String> {
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

    upsert("whisper_model", &settings.whisper_model)?;

    if let Some(lang) = &settings.default_language {
        upsert("default_language", lang)?;
    }

    upsert(
        "auto_transcribe",
        if settings.auto_transcribe {
            "true"
        } else {
            "false"
        },
    )?;
    upsert(
        "auto_generate_report",
        if settings.auto_generate_report {
            "true"
        } else {
            "false"
        },
    )?;

    Ok(())
}

#[tauri::command]
pub fn get_app_data_dir(state: State<'_, AppState>) -> String {
    state.app_data_dir.to_string_lossy().to_string()
}
