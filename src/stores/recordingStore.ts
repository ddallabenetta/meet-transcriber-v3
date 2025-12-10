import { create } from "zustand";
import type { AudioDevice } from "@/lib/tauri-commands";
import {
  getAudioDevices,
  startRecording,
  stopRecording,
} from "@/lib/tauri-commands";

interface RecordingState {
  isRecording: boolean;
  currentMeetingId: string | null;
  elapsedSeconds: number;
  devices: AudioDevice[];
  selectedDeviceId: string | null;
  error: string | null;

  // Actions
  loadDevices: () => Promise<void>;
  setSelectedDevice: (deviceId: string | null) => void;
  start: () => Promise<void>;
  stop: () => Promise<string>;
  tick: () => void;
  reset: () => void;
}

export const useRecordingStore = create<RecordingState>((set, get) => ({
  isRecording: false,
  currentMeetingId: null,
  elapsedSeconds: 0,
  devices: [],
  selectedDeviceId: null,
  error: null,

  loadDevices: async () => {
    try {
      const devices = await getAudioDevices();
      const defaultDevice = devices.find((d) => d.is_default);
      set({
        devices,
        selectedDeviceId: defaultDevice?.id || devices[0]?.id || null,
        error: null,
      });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setSelectedDevice: (deviceId) => {
    set({ selectedDeviceId: deviceId });
  },

  start: async () => {
    try {
      const { selectedDeviceId } = get();
      const meetingId = await startRecording(selectedDeviceId || undefined);
      set({
        isRecording: true,
        currentMeetingId: meetingId,
        elapsedSeconds: 0,
        error: null,
      });
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  stop: async () => {
    try {
      const audioPath = await stopRecording();
      set({
        isRecording: false,
        currentMeetingId: null,
        error: null,
      });
      return audioPath;
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  tick: () => {
    set((state) => ({ elapsedSeconds: state.elapsedSeconds + 1 }));
  },

  reset: () => {
    set({
      isRecording: false,
      currentMeetingId: null,
      elapsedSeconds: 0,
      error: null,
    });
  },
}));
