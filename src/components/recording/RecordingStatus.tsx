import { Circle } from "lucide-react";

interface RecordingStatusProps {
  title: string;
  duration: string;
}

export function RecordingStatus({ title, duration }: RecordingStatusProps) {
  return (
    <div className="flex flex-col items-center justify-center py-8 space-y-4">
      <div className="relative">
        <Circle className="h-20 w-20 text-destructive fill-destructive animate-pulse" />
        <div className="absolute inset-0 flex items-center justify-center">
          <Circle className="h-16 w-16 text-background fill-background" />
        </div>
      </div>

      <div className="text-center space-y-2">
        <h3 className="text-2xl font-bold">{title}</h3>
        <p className="text-3xl font-mono tabular-nums">{duration}</p>
        <p className="text-sm text-muted-foreground">Registrazione in corso...</p>
      </div>
    </div>
  );
}
