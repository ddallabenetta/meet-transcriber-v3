import { Button } from "@/components/ui/button";
import { Copy, Check } from "lucide-react";
import { useState } from "react";

interface TranscriptViewerProps {
  transcript: string;
}

export function TranscriptViewer({ transcript }: TranscriptViewerProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(transcript);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold">Trascrizione</h3>
        <Button variant="outline" size="sm" onClick={handleCopy}>
          {copied ? (
            <>
              <Check className="mr-2 h-4 w-4" />
              Copiato!
            </>
          ) : (
            <>
              <Copy className="mr-2 h-4 w-4" />
              Copia
            </>
          )}
        </Button>
      </div>

      <div className="p-4 rounded-lg bg-muted/50 max-h-96 overflow-y-auto">
        <p className="text-sm whitespace-pre-wrap leading-relaxed">
          {transcript}
        </p>
      </div>
    </div>
  );
}
