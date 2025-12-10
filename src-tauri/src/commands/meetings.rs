use crate::AppState;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub duration_seconds: Option<i64>,
    pub audio_path: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingWithTranscript {
    pub meeting: Meeting,
    pub transcript: Option<String>,
    pub report: Option<MeetingReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingReport {
    pub id: String,
    pub highlights: Vec<String>,
    pub participants: Vec<String>,
    pub action_items: Vec<String>,
    pub llm_provider: Option<String>,
    pub llm_model: Option<String>,
    pub created_at: String,
}

#[tauri::command]
pub fn create_meeting(
    state: State<'_, AppState>,
    title: String,
    audio_path: Option<String>,
) -> Result<Meeting, String> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO meetings (id, title, created_at, audio_path, status) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, title, created_at, audio_path, "recording"],
    ).map_err(|e| e.to_string())?;

    Ok(Meeting {
        id,
        title,
        created_at,
        duration_seconds: None,
        audio_path,
        status: "recording".to_string(),
    })
}

#[tauri::command]
pub fn update_meeting(
    state: State<'_, AppState>,
    id: String,
    title: Option<String>,
    duration_seconds: Option<i64>,
    status: Option<String>,
) -> Result<(), String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    if let Some(t) = title {
        conn.execute(
            "UPDATE meetings SET title = ?1 WHERE id = ?2",
            params![t, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(d) = duration_seconds {
        conn.execute(
            "UPDATE meetings SET duration_seconds = ?1 WHERE id = ?2",
            params![d, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(s) = status {
        conn.execute(
            "UPDATE meetings SET status = ?1 WHERE id = ?2",
            params![s, id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_meetings(state: State<'_, AppState>) -> Result<Vec<Meeting>, String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, title, created_at, duration_seconds, audio_path, status FROM meetings ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let meetings = stmt
        .query_map([], |row| {
            Ok(Meeting {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                duration_seconds: row.get(3)?,
                audio_path: row.get(4)?,
                status: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|m| m.ok())
        .collect();

    Ok(meetings)
}

#[tauri::command]
pub fn get_meeting(
    state: State<'_, AppState>,
    id: String,
) -> Result<MeetingWithTranscript, String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    // Get meeting
    let meeting: Meeting = conn
        .query_row(
            "SELECT id, title, created_at, duration_seconds, audio_path, status FROM meetings WHERE id = ?1",
            params![id],
            |row| {
                Ok(Meeting {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    duration_seconds: row.get(3)?,
                    audio_path: row.get(4)?,
                    status: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    // Get transcript
    let transcript: Option<String> = conn
        .query_row(
            "SELECT content FROM transcriptions WHERE meeting_id = ?1 ORDER BY created_at DESC LIMIT 1",
            params![id],
            |row| row.get(0),
        )
        .ok();

    // Get report
    let report: Option<MeetingReport> = conn
        .query_row(
            "SELECT id, highlights, participants, action_items, llm_provider, llm_model, created_at FROM reports WHERE meeting_id = ?1 ORDER BY created_at DESC LIMIT 1",
            params![id],
            |row| {
                let highlights_json: String = row.get(1)?;
                let participants_json: String = row.get(2)?;
                let action_items_json: String = row.get(3)?;

                Ok(MeetingReport {
                    id: row.get(0)?,
                    highlights: serde_json::from_str(&highlights_json).unwrap_or_default(),
                    participants: serde_json::from_str(&participants_json).unwrap_or_default(),
                    action_items: serde_json::from_str(&action_items_json).unwrap_or_default(),
                    llm_provider: row.get(4)?,
                    llm_model: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .ok();

    Ok(MeetingWithTranscript {
        meeting,
        transcript,
        report,
    })
}

#[tauri::command]
pub fn delete_meeting(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    // Get audio path to delete file
    let audio_path: Option<String> = conn
        .query_row(
            "SELECT audio_path FROM meetings WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .ok();

    // Delete from database (cascades to transcriptions and reports)
    conn.execute("DELETE FROM meetings WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;

    // Delete audio file if exists
    if let Some(path) = audio_path {
        let _ = std::fs::remove_file(path);
    }

    Ok(())
}

#[tauri::command]
pub fn save_transcription(
    state: State<'_, AppState>,
    meeting_id: String,
    content: String,
    language: Option<String>,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    let conn = state.db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO transcriptions (id, meeting_id, content, language, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, meeting_id, content, language, created_at],
    ).map_err(|e| e.to_string())?;

    // Update meeting status
    conn.execute(
        "UPDATE meetings SET status = 'transcribed' WHERE id = ?1",
        params![meeting_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(id)
}
