use crate::audio::{list_audio_devices, AudioDevice};
use crate::AppState;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_audio_devices() -> Vec<AudioDevice> {
    list_audio_devices()
}

#[tauri::command]
pub async fn start_recording(
    state: State<'_, AppState>,
    device_id: Option<String>,
) -> Result<String, String> {
    let meeting_id = Uuid::new_v4().to_string();

    // Create recordings directory
    let recordings_dir = state.app_data_dir.join("recordings");
    std::fs::create_dir_all(&recordings_dir).map_err(|e| e.to_string())?;

    let audio_path = recordings_dir.join(format!("{}.wav", meeting_id));

    let mut recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    recorder
        .start_recording(audio_path, device_id)
        .map_err(|e| e.to_string())?;

    // Store current meeting id
    let mut current = state.current_meeting_id.lock().map_err(|e| e.to_string())?;
    *current = Some(meeting_id.clone());

    Ok(meeting_id)
}

#[tauri::command]
pub async fn stop_recording(state: State<'_, AppState>) -> Result<String, String> {
    let mut recorder = state.recorder.lock().map_err(|e| e.to_string())?;
    let audio_path = recorder.stop_recording().map_err(|e| e.to_string())?;

    let mut current = state.current_meeting_id.lock().map_err(|e| e.to_string())?;
    *current = None;

    Ok(audio_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn is_recording(state: State<'_, AppState>) -> bool {
    state
        .recorder
        .lock()
        .map(|r| r.is_recording())
        .unwrap_or(false)
}
