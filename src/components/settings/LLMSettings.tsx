import { useEffect, useState } from "react";
import { useSettingsStore } from "@/stores/settingsStore";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { Save, RotateCcw } from "lucide-react";

export function LLMSettings() {
  const { llmConfig, defaultSystemPrompt, loadSettings, updateLlmConfig } = useSettingsStore();
  const [formData, setFormData] = useState({
    provider: "ollama",
    api_key: "",
    base_url: "http://localhost:11434",
    model: "llama3",
    system_prompt: "",
  });

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  useEffect(() => {
    if (llmConfig) {
      setFormData({
        provider: llmConfig.provider,
        api_key: llmConfig.api_key || "",
        base_url: llmConfig.base_url || "",
        model: llmConfig.model,
        system_prompt: llmConfig.system_prompt || "",
      });
    }
  }, [llmConfig]);

  const handleSave = async () => {
    try {
      await updateLlmConfig({
        provider: formData.provider,
        api_key: formData.api_key || null,
        base_url: formData.base_url || null,
        model: formData.model,
        system_prompt: formData.system_prompt || null,
      });
      alert("Configurazione salvata!");
    } catch (e) {
      alert("Errore durante il salvataggio");
    }
  };

  const handleResetPrompt = () => {
    setFormData({ ...formData, system_prompt: "" });
  };

  const providerOptions = [
    { value: "ollama", label: "Ollama (Locale)" },
    { value: "openai", label: "OpenAI" },
    { value: "anthropic", label: "Anthropic (Claude)" },
  ];

  const modelsByProvider: Record<string, string[]> = {
    ollama: ["llama3", "llama3.1", "mistral", "gemma2"],
    openai: ["gpt-4o", "gpt-4-turbo", "gpt-3.5-turbo"],
    anthropic: ["claude-3-5-sonnet-20241022", "claude-3-opus-20240229", "claude-3-haiku-20240307"],
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Configurazione LLM</CardTitle>
        <CardDescription>
          Configura il modello di linguaggio per la generazione dei report
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Provider */}
        <div className="space-y-2">
          <label className="text-sm font-medium">Provider</label>
          <Select
            options={providerOptions}
            value={formData.provider}
            onChange={(e) => {
              const provider = e.target.value;
              setFormData({
                ...formData,
                provider,
                model: modelsByProvider[provider][0],
                base_url: provider === "ollama" ? "http://localhost:11434" : "",
              });
            }}
          />
        </div>

        {/* Model */}
        <div className="space-y-2">
          <label className="text-sm font-medium">Modello</label>
          <Select
            options={modelsByProvider[formData.provider].map(m => ({ value: m, label: m }))}
            value={formData.model}
            onChange={(e) => setFormData({ ...formData, model: e.target.value })}
          />
        </div>

        {/* API Key (per OpenAI/Anthropic) */}
        {formData.provider !== "ollama" && (
          <div className="space-y-2">
            <label className="text-sm font-medium">API Key</label>
            <Input
              type="password"
              placeholder="sk-..."
              value={formData.api_key}
              onChange={(e) => setFormData({ ...formData, api_key: e.target.value })}
            />
          </div>
        )}

        {/* Base URL */}
        <div className="space-y-2">
          <label className="text-sm font-medium">
            Base URL {formData.provider === "ollama" && "(Ollama)"}
          </label>
          <Input
            placeholder={formData.provider === "ollama" ? "http://localhost:11434" : "https://api.openai.com/v1"}
            value={formData.base_url}
            onChange={(e) => setFormData({ ...formData, base_url: e.target.value })}
          />
        </div>

        {/* System Prompt */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <label className="text-sm font-medium">Prompt di Sistema (Opzionale)</label>
            <Button
              type="button"
              variant="ghost"
              size="sm"
              onClick={handleResetPrompt}
            >
              <RotateCcw className="h-3 w-3 mr-2" />
              Reset
            </Button>
          </div>
          <Textarea
            placeholder={defaultSystemPrompt || "Lascia vuoto per usare il prompt predefinito"}
            value={formData.system_prompt}
            onChange={(e) => setFormData({ ...formData, system_prompt: e.target.value })}
            rows={8}
            className="font-mono text-xs"
          />
          <p className="text-xs text-muted-foreground">
            Personalizza le istruzioni per il modello di linguaggio
          </p>
        </div>

        <Button onClick={handleSave} className="w-full">
          <Save className="mr-2 h-4 w-4" />
          Salva Configurazione
        </Button>
      </CardContent>
    </Card>
  );
}
