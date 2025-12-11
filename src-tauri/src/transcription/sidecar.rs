use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Child, Stdio};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Window};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[derive(Error, Debug)]
pub enum TranscriptionError {
    #[error("Errore avvio sidecar: {0}")]
    SidecarStartError(String),
    #[error("Errore comunicazione: {0}")]
    CommunicationError(String),
    #[error("Errore trascrizione: {0}")]
    TranscriptionFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub language: Option<String>,
    pub segments: Vec<TranscriptionSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Serialize)]
struct TranscriptionRequest {
    command: String,
    audio_path: String,
    model_size: String,
    language: Option<String>,
}

#[derive(Deserialize)]
struct SidecarResponse {
    success: bool,
    error: Option<String>,
    result: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct StreamingUpdate {
    #[serde(rename = "type")]
    update_type: String,
    segments: Vec<TranscriptionSegment>,
}

// Sidecar process globale per streaming
use once_cell::sync::Lazy;
static STREAMING_PROCESS: Lazy<Arc<Mutex<Option<tokio::process::Child>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub async fn transcribe_audio(
    sidecar_path: &Path,
    audio_path: &Path,
    model_size: &str,
    language: Option<String>,
) -> Result<TranscriptionResult, TranscriptionError> {
    // Usa il Python del venv
    let current_dir = std::env::current_dir()
        .map_err(|e| TranscriptionError::SidecarStartError(e.to_string()))?;

    let python_executable = current_dir
        .join("python")
        .join("venv")
        .join("bin")
        .join("python3");

    let mut child = Command::new(&python_executable)
        .arg(sidecar_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| TranscriptionError::SidecarStartError(e.to_string()))?;

    let stdin = child.stdin.as_mut().ok_or_else(|| {
        TranscriptionError::CommunicationError("Impossibile accedere a stdin".to_string())
    })?;

    let request = TranscriptionRequest {
        command: "transcribe".to_string(),
        audio_path: audio_path.to_string_lossy().to_string(),
        model_size: model_size.to_string(),
        language,
    };

    let request_json = serde_json::to_string(&request)
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;

    stdin
        .write_all(request_json.as_bytes())
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;
    stdin
        .write_all(b"\n")
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;
    stdin
        .flush()
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;

    drop(child.stdin.take());

    let stdout = child.stdout.take().ok_or_else(|| {
        TranscriptionError::CommunicationError("Impossibile accedere a stdout".to_string())
    })?;

    let mut reader = BufReader::new(stdout);
    let mut response_line = String::new();

    reader
        .read_line(&mut response_line)
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;

    let response: SidecarResponse = serde_json::from_str(&response_line)
        .map_err(|e| TranscriptionError::CommunicationError(format!("Parsing risposta: {}", e)))?;

    if response.success {
        let result: TranscriptionResult = serde_json::from_value(response.result.unwrap())
            .map_err(|e| {
                TranscriptionError::CommunicationError(format!("Parsing result: {}", e))
            })?;
        Ok(result)
    } else {
        Err(TranscriptionError::TranscriptionFailed(
            response
                .error
                .unwrap_or_else(|| "Errore sconosciuto".to_string()),
        ))
    }
}

pub async fn start_streaming_transcription(
    window: Window,
    audio_path: &str,
    model_size: &str,
    language: Option<String>,
) -> Result<(), TranscriptionError> {
    // Trova il percorso dello script Python e del venv
    let current_dir = std::env::current_dir()
        .map_err(|e| TranscriptionError::SidecarStartError(e.to_string()))?;

    let python_script = current_dir.join("python").join("src").join("main.py");

    let python_executable = current_dir
        .join("python")
        .join("venv")
        .join("bin")
        .join("python3");

    let mut child = Command::new(&python_executable)
        .arg(&python_script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| TranscriptionError::SidecarStartError(e.to_string()))?;

    let mut stdin = child.stdin.take().ok_or_else(|| {
        TranscriptionError::CommunicationError("Impossibile accedere a stdin".to_string())
    })?;

    let stdout = child.stdout.take().ok_or_else(|| {
        TranscriptionError::CommunicationError("Impossibile accedere a stdout".to_string())
    })?;

    // Store process reference PRIMA di usarlo
    {
        let mut process_guard = STREAMING_PROCESS.lock().unwrap();
        *process_guard = Some(child);
    }

    // Invia comando start_streaming
    let request = TranscriptionRequest {
        command: "start_streaming".to_string(),
        audio_path: audio_path.to_string(),
        model_size: model_size.to_string(),
        language,
    };

    let request_json = serde_json::to_string(&request)
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;

    stdin
        .write_all(request_json.as_bytes())
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;
    stdin
        .write_all(b"\n")
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;
    stdin
        .flush()
        .await
        .map_err(|e| TranscriptionError::CommunicationError(e.to_string()))?;

    // Spawn task per leggere gli update
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // Parse response
                    if let Ok(update) = serde_json::from_str::<StreamingUpdate>(&line) {
                        if update.update_type == "streaming_update" {
                            // Emit event to frontend
                            let _ = window.emit("transcription-update", update.segments);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading streaming output: {}", e);
                    break;
                }
            }
        }
    });

    Ok(())
}

pub async fn stop_streaming_transcription() -> Result<(), TranscriptionError> {
    let child_opt = {
        let mut process_guard = STREAMING_PROCESS.lock().unwrap();
        process_guard.take()
    };

    if let Some(mut child) = child_opt {
        // Invia comando stop al processo
        if let Some(mut stdin) = child.stdin.take() {
            let request = TranscriptionRequest {
                command: "stop_streaming".to_string(),
                audio_path: String::new(),
                model_size: String::new(),
                language: None,
            };

            if let Ok(request_json) = serde_json::to_string(&request) {
                let _ = stdin.write_all(request_json.as_bytes()).await;
                let _ = stdin.write_all(b"\n").await;
                let _ = stdin.flush().await;
            }
        }

        // Termina il processo
        let _ = child.kill().await;
    }

    Ok(())
}
