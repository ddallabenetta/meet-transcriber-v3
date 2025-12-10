mod audio;
mod commands;
mod database;
mod llm;
mod transcription;

use audio::AudioRecorder;
use database::Database;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Database,
    pub recorder: Mutex<AudioRecorder>,
    pub app_data_dir: PathBuf,
    pub current_meeting_id: Mutex<Option<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Setup logging in debug mode
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Impossibile ottenere la directory dati app");

            // Initialize database
            let db =
                Database::new(app_data_dir.clone()).expect("Impossibile inizializzare il database");

            // Initialize app state
            let state = AppState {
                db,
                recorder: Mutex::new(AudioRecorder::new()),
                app_data_dir,
                current_meeting_id: Mutex::new(None),
            };

            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Audio commands
            commands::audio::get_audio_devices,
            commands::audio::start_recording,
            commands::audio::stop_recording,
            commands::audio::is_recording,
            // Meeting commands
            commands::meetings::create_meeting,
            commands::meetings::update_meeting,
            commands::meetings::get_meetings,
            commands::meetings::get_meeting,
            commands::meetings::delete_meeting,
            commands::meetings::save_transcription,
            // Transcription commands
            commands::transcription::transcribe_meeting,
            commands::transcription::get_available_models,
            commands::transcription::start_streaming_transcription_command,
            commands::transcription::stop_streaming_transcription_command,
            // LLM commands
            commands::llm::generate_meeting_report,
            commands::llm::get_llm_config,
            commands::llm::save_llm_config,
            commands::llm::get_default_system_prompt,
            // Settings commands
            commands::settings::get_app_settings,
            commands::settings::save_app_settings,
            commands::settings::get_app_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("Errore durante l'esecuzione dell'applicazione Tauri");
}
