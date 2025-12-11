import { create } from "zustand";
import type {
  Meeting,
  MeetingWithTranscript,
  ReportContent,
} from "@/lib/tauri-commands";
import {
  getMeetings,
  getMeeting,
  createMeeting,
  updateMeeting,
  deleteMeeting,
  saveTranscription,
  transcribeMeeting,
  generateMeetingReport,
} from "@/lib/tauri-commands";

interface MeetingsState {
  meetings: Meeting[];
  currentMeeting: MeetingWithTranscript | null;
  isLoading: boolean;
  isTranscribing: boolean;
  isGeneratingReport: boolean;
  error: string | null;

  // Actions
  loadMeetings: () => Promise<void>;
  loadMeeting: (id: string) => Promise<void>;
  create: (title: string, audioPath?: string) => Promise<Meeting>;
  update: (
    id: string,
    title?: string,
    durationSeconds?: number,
    status?: string,
    audioPath?: string,
  ) => Promise<void>;
  remove: (id: string) => Promise<void>;
  transcribe: (
    meetingId: string,
    audioPath: string,
    modelSize?: string,
    language?: string,
  ) => Promise<string>;
  generateReport: (
    meetingId: string,
    transcript: string,
  ) => Promise<ReportContent>;
  clearCurrent: () => void;
}

export const useMeetingsStore = create<MeetingsState>((set, get) => ({
  meetings: [],
  currentMeeting: null,
  isLoading: false,
  isTranscribing: false,
  isGeneratingReport: false,
  error: null,

  loadMeetings: async () => {
    set({ isLoading: true, error: null });
    try {
      const meetings = await getMeetings();
      set({ meetings, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  loadMeeting: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      const meeting = await getMeeting(id);
      set({ currentMeeting: meeting, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  create: async (title: string, audioPath?: string) => {
    try {
      const meeting = await createMeeting(title, audioPath);
      set((state) => ({
        meetings: [meeting, ...state.meetings],
        error: null,
      }));
      return meeting;
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  update: async (
    id: string,
    title?: string,
    durationSeconds?: number,
    status?: string,
    audioPath?: string,
  ) => {
    try {
      await updateMeeting(id, title, durationSeconds, status, audioPath);
      // Reload meetings to get updated data
      await get().loadMeetings();
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  remove: async (id: string) => {
    try {
      await deleteMeeting(id);
      set((state) => ({
        meetings: state.meetings.filter((m) => m.id !== id),
        currentMeeting:
          state.currentMeeting?.meeting.id === id ? null : state.currentMeeting,
        error: null,
      }));
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  transcribe: async (
    meetingId: string,
    audioPath: string,
    modelSize?: string,
    language?: string,
  ) => {
    set({ isTranscribing: true, error: null });
    try {
      const result = await transcribeMeeting(audioPath, modelSize, language);
      await saveTranscription(
        meetingId,
        result.text,
        result.language || undefined,
      );

      // Reload current meeting to get updated transcript
      await get().loadMeeting(meetingId);

      set({ isTranscribing: false });
      return result.text;
    } catch (e) {
      set({ error: String(e), isTranscribing: false });
      throw e;
    }
  },

  generateReport: async (meetingId: string, transcript: string) => {
    set({ isGeneratingReport: true, error: null });
    try {
      const report = await generateMeetingReport(meetingId, transcript);

      // Reload current meeting to get updated report
      await get().loadMeeting(meetingId);

      set({ isGeneratingReport: false });
      return report;
    } catch (e) {
      set({ error: String(e), isGeneratingReport: false });
      throw e;
    }
  },

  clearCurrent: () => {
    set({ currentMeeting: null });
  },
}));
