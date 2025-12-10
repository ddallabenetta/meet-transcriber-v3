import { useEffect, useState } from "react";
import { useSettingsStore } from "@/stores/settingsStore";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { getAvailableModels } from "@/lib/tauri-commands";
import type { WhisperModel } from "@/lib/tauri-commands";
import { Save } from "lucide-react";

export function AudioSettings() {
  const { appSettings, loadSettings, updateAppSettings } = useSettingsStore();
  const [models, setModels] = useState<WhisperModel[]>([]);
  const [formData, setFormData] = useState({
    whisper_model: "base",
    default_language: "it",
    auto_transcribe: false,
    auto_generate_report: false,
  });

  useEffect(() => {
    loadSettings();
    loadModels();
  }, [loadSettings]);

  useEffect(() => {
    if (appSettings) {
      setFormData({
        whisper_model: appSettings.whisper_model,
        default_language: appSettings.default_language || "it",
        auto_transcribe: appSettings.auto_transcribe,
        auto_generate_report: appSettings.auto_generate_report,
      });
    }
  }, [appSettings]);

  const loadModels = async () => {
    const availableModels = await getAvailableModels();
    setModels(availableModels);
  };

  const handleSave = async () => {
    try {
      await updateAppSettings({
        whisper_model: formData.whisper_model,
        default_language: formData.default_language || null,
        auto_transcribe: formData.auto_transcribe,
        auto_generate_report: formData.auto_generate_report,
      });
      alert("Impostazioni salvate!");
    } catch (e) {
      alert("Errore durante il salvataggio");
    }
  };

  const languageOptions = [
    { value: "it", label: "Italiano" },
    { value: "en", label: "Inglese" },
    { value: "es", label: "Spagnolo" },
    { value: "fr", label: "Francese" },
    { value: "de", label: "Tedesco" },
    { value: "auto", label: "Auto-detect" },
  ];

  return (
    <Card>
      <CardHeader>
        <CardTitle>Impostazioni Audio e Trascrizione</CardTitle>
        <CardDescription>
          Configura le opzioni per la registrazione e trascrizione
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Whisper Model */}
        <div className="space-y-2">
          <label className="text-sm font-medium">Modello Whisper</label>
          <Select
            options={models.map((m) => ({
              value: m.id,
              label: `${m.name} - ${m.description} (~${m.size_mb}MB)`,
            }))}
            value={formData.whisper_model}
            onChange={(e) =>
              setFormData({ ...formData, whisper_model: e.target.value })
            }
          />
          <p className="text-xs text-muted-foreground">
            Modelli più grandi sono più accurati ma richiedono più tempo
          </p>
        </div>

        {/* Default Language */}
        <div className="space-y-2">
          <label className="text-sm font-medium">Lingua Predefinita</label>
          <Select
            options={languageOptions}
            value={formData.default_language}
            onChange={(e) =>
              setFormData({ ...formData, default_language: e.target.value })
            }
          />
        </div>

        {/* Auto Transcribe */}
        <div className="flex items-center justify-between p-4 rounded-lg border">
          <div>
            <p className="font-medium">Trascrizione Automatica</p>
            <p className="text-sm text-muted-foreground">
              Avvia automaticamente la trascrizione al termine della
              registrazione
            </p>
          </div>
          <input
            type="checkbox"
            className="h-5 w-5"
            checked={formData.auto_transcribe}
            onChange={(e) =>
              setFormData({ ...formData, auto_transcribe: e.target.checked })
            }
          />
        </div>

        {/* Auto Generate Report */}
        <div className="flex items-center justify-between p-4 rounded-lg border">
          <div>
            <p className="font-medium">Generazione Report Automatica</p>
            <p className="text-sm text-muted-foreground">
              Genera automaticamente il report dopo la trascrizione
            </p>
          </div>
          <input
            type="checkbox"
            className="h-5 w-5"
            checked={formData.auto_generate_report}
            onChange={(e) =>
              setFormData({
                ...formData,
                auto_generate_report: e.target.checked,
              })
            }
          />
        </div>

        <Button onClick={handleSave} className="w-full">
          <Save className="mr-2 h-4 w-4" />
          Salva Impostazioni
        </Button>
      </CardContent>
    </Card>
  );
}
