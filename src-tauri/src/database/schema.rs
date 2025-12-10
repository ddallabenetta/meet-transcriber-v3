use rusqlite::{Connection, Result};

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS meetings (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            duration_seconds INTEGER,
            audio_path TEXT,
            status TEXT NOT NULL DEFAULT 'recording'
        );

        CREATE TABLE IF NOT EXISTS transcriptions (
            id TEXT PRIMARY KEY,
            meeting_id TEXT NOT NULL,
            content TEXT NOT NULL,
            language TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (meeting_id) REFERENCES meetings(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS reports (
            id TEXT PRIMARY KEY,
            meeting_id TEXT NOT NULL,
            highlights TEXT,
            participants TEXT,
            action_items TEXT,
            raw_response TEXT,
            llm_provider TEXT,
            llm_model TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (meeting_id) REFERENCES meetings(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_transcriptions_meeting_id ON transcriptions(meeting_id);
        CREATE INDEX IF NOT EXISTS idx_reports_meeting_id ON reports(meeting_id);
        CREATE INDEX IF NOT EXISTS idx_meetings_created_at ON meetings(created_at);
        ",
    )?;

    Ok(())
}
