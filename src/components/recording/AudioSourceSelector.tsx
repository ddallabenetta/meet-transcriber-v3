import { useEffect } from "react";
import { Select } from "@/components/ui/select";
import { useRecordingStore } from "@/stores/recordingStore";
import { Mic } from "lucide-react";

export function AudioSourceSelector() {
  const { devices, selectedDeviceId, setSelectedDevice, loadDevices } =
    useRecordingStore();

  useEffect(() => {
    loadDevices();
  }, [loadDevices]);

  const options = devices.map((device) => ({
    value: device.id,
    label: `${device.name}${device.is_default ? " (predefinito)" : ""}`,
  }));

  if (devices.length === 0) {
    return (
      <div className="flex items-center gap-2 text-sm text-muted-foreground">
        <Mic className="h-4 w-4" />
        <span>Nessun dispositivo audio trovato</span>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <label className="text-sm font-medium flex items-center gap-2">
        <Mic className="h-4 w-4" />
        Sorgente Audio
      </label>
      <Select
        options={options}
        value={selectedDeviceId || ""}
        onChange={(e) => setSelectedDevice(e.target.value)}
      />
    </div>
  );
}
