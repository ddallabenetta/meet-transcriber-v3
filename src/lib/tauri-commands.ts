import { invoke } from "@tauri-apps/api/core";

// Types
export interface AudioDevice {
  id: string;
  name: string;
  is_input: boolean;
  is_default: boolean;
}

export interface Meeting {
  id: string;
  title: string;
  created_at: string;
  duration_seconds: number | null;
  audio_path: string | null;
  status: string;
}

export interface MeetingReport {
  id: string;
  highlights: string[];
  participants: string[];
  action_items: string[];
  llm_provider: string | null;
  llm_model: string | null;
  created_at: string;
}

export interface MeetingWithTranscript {
  meeting: Meeting;
  transcript: string | null;
  report: MeetingReport | null;
}

export interface TranscriptionResult {
  text: string;
  language: string | null;
  segments: { start: number; end: number; text: string }[];
}

export interface LlmConfig {
  provider: string;
  api_key: string | null;
  base_url: string | null;
  model: string;
  system_prompt: string | null;
}

export interface ReportContent {
  highlights: string[];
  participants: string[];
  action_items: string[];
  raw_response: string;
}

export interface AppSettings {
  whisper_model: string;
  default_language: string | null;
  auto_transcribe: boolean;
  auto_generate_report: boolean;
}

export interface WhisperModel {
  id: string;
  name: string;
  size_mb: number;
  description: string;
}

// Audio commands
export async function getAudioDevices(): Promise<AudioDevice[]> {
  return invoke("get_audio_devices");
}

export async function startRecording(deviceId?: string): Promise<string> {
  return invoke("start_recording", { deviceId });
}

export async function stopRecording(): Promise<string> {
  return invoke("stop_recording");
}

export async function isRecording(): Promise<boolean> {
  return invoke("is_recording");
}

// Meeting commands
export async function createMeeting(
  title: string,
  audioPath?: string,
): Promise<Meeting> {
  return invoke("create_meeting", { title, audioPath });
}

export async function updateMeeting(
  id: string,
  title?: string,
  durationSeconds?: number,
  status?: string,
  audioPath?: string,
): Promise<void> {
  return invoke("update_meeting", { id, title, durationSeconds, status, audioPath });
}

export async function getMeetings(): Promise<Meeting[]> {
  return invoke("get_meetings");
}

export async function getMeeting(id: string): Promise<MeetingWithTranscript> {
  return invoke("get_meeting", { id });
}

export async function deleteMeeting(id: string): Promise<void> {
  return invoke("delete_meeting", { id });
}

export async function saveTranscription(
  meetingId: string,
  content: string,
  language?: string,
): Promise<string> {
  return invoke("save_transcription", { meetingId, content, language });
}

// Transcription commands
export async function transcribeMeeting(
  audioPath: string,
  modelSize?: string,
  language?: string,
): Promise<TranscriptionResult> {
  return invoke("transcribe_meeting", { audioPath, modelSize, language });
}

export async function getAvailableModels(): Promise<WhisperModel[]> {
  return invoke("get_available_models");
}

export async function startStreamingTranscription(
  audioPath: string,
  modelSize?: string,
  language?: string,
): Promise<void> {
  return invoke("start_streaming_transcription_command", {
    audioPath,
    modelSize,
    language,
  });
}

export async function stopStreamingTranscription(): Promise<void> {
  return invoke("stop_streaming_transcription_command");
}

// LLM commands
export async function generateMeetingReport(
  meetingId: string,
  transcript: string,
): Promise<ReportContent> {
  return invoke("generate_meeting_report", { meetingId, transcript });
}

export async function getLlmConfig(): Promise<LlmConfig> {
  return invoke("get_llm_config");
}

export async function saveLlmConfig(config: LlmConfig): Promise<void> {
  return invoke("save_llm_config", { config });
}

export async function getDefaultSystemPrompt(): Promise<string> {
  return invoke("get_default_system_prompt");
}

// Settings commands
export async function getAppSettings(): Promise<AppSettings> {
  return invoke("get_app_settings");
}

export async function saveAppSettings(settings: AppSettings): Promise<void> {
  return invoke("save_app_settings", { settings });
}

export async function getAppDataDir(): Promise<string> {
  return invoke("get_app_data_dir");
}
