use crate::transcription::{
    start_streaming_transcription, stop_streaming_transcription, transcribe_audio,
    TranscriptionResult,
};
use crate::AppState;
use std::path::PathBuf;
use tauri::{State, Window};

#[tauri::command]
pub async fn transcribe_meeting(
    state: State<'_, AppState>,
    audio_path: String,
    model_size: Option<String>,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    let sidecar_path = state.app_data_dir.join("transcriber");

    // Check if sidecar exists, otherwise use Python directly for development
    let sidecar = if sidecar_path.exists() {
        sidecar_path
    } else {
        // Fallback to python script for development
        PathBuf::from("python")
    };

    let audio = PathBuf::from(&audio_path);
    let model = model_size.unwrap_or_else(|| "base".to_string());

    transcribe_audio(&sidecar, &audio, &model, language)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_streaming_transcription_command(
    window: Window,
    audio_path: String,
    model_size: Option<String>,
    language: Option<String>,
) -> Result<(), String> {
    let model = model_size.unwrap_or_else(|| "base".to_string());

    start_streaming_transcription(window, &audio_path, &model, language)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_streaming_transcription_command() -> Result<(), String> {
    stop_streaming_transcription()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_available_models() -> Vec<WhisperModel> {
    vec![
        WhisperModel {
            id: "tiny".to_string(),
            name: "Tiny".to_string(),
            size_mb: 75,
            description: "Veloce, meno accurato".to_string(),
        },
        WhisperModel {
            id: "base".to_string(),
            name: "Base".to_string(),
            size_mb: 142,
            description: "Bilanciato".to_string(),
        },
        WhisperModel {
            id: "small".to_string(),
            name: "Small".to_string(),
            size_mb: 466,
            description: "Buona accuratezza".to_string(),
        },
        WhisperModel {
            id: "medium".to_string(),
            name: "Medium".to_string(),
            size_mb: 1500,
            description: "Alta accuratezza".to_string(),
        },
        WhisperModel {
            id: "large-v3".to_string(),
            name: "Large V3".to_string(),
            size_mb: 3000,
            description: "Massima accuratezza".to_string(),
        },
    ]
}

#[derive(serde::Serialize)]
pub struct WhisperModel {
    pub id: String,
    pub name: String,
    pub size_mb: u32,
    pub description: String,
}
