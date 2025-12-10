import { create } from "zustand";
import type { LlmConfig, AppSettings } from "@/lib/tauri-commands";
import {
  getLlmConfig,
  saveLlmConfig,
  getAppSettings,
  saveAppSettings,
  getDefaultSystemPrompt,
} from "@/lib/tauri-commands";

interface SettingsState {
  llmConfig: LlmConfig | null;
  appSettings: AppSettings | null;
  defaultSystemPrompt: string;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadSettings: () => Promise<void>;
  updateLlmConfig: (config: LlmConfig) => Promise<void>;
  updateAppSettings: (settings: AppSettings) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  llmConfig: null,
  appSettings: null,
  defaultSystemPrompt: "",
  isLoading: false,
  error: null,

  loadSettings: async () => {
    set({ isLoading: true, error: null });
    try {
      const [llmConfig, appSettings, defaultSystemPrompt] = await Promise.all([
        getLlmConfig(),
        getAppSettings(),
        getDefaultSystemPrompt(),
      ]);
      set({
        llmConfig,
        appSettings,
        defaultSystemPrompt,
        isLoading: false,
      });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  updateLlmConfig: async (config: LlmConfig) => {
    try {
      await saveLlmConfig(config);
      set({ llmConfig: config, error: null });
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  updateAppSettings: async (settings: AppSettings) => {
    try {
      await saveAppSettings(settings);
      set({ appSettings: settings, error: null });
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },
}));
