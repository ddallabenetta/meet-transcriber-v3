pub mod sidecar;

pub use sidecar::{
    start_streaming_transcription, stop_streaming_transcription, transcribe_audio,
    TranscriptionResult,
};
