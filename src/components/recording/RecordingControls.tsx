import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { AudioSourceSelector } from "./AudioSourceSelector";
import { RecordingStatus } from "./RecordingStatus";
import { useRecordingStore } from "@/stores/recordingStore";
import { useMeetingsStore } from "@/stores/meetingsStore";
import { Circle, Square } from "lucide-react";
import { formatDuration } from "@/lib/utils";
import { listen } from "@tauri-apps/api/event";
import {
  startStreamingTranscription,
  stopStreamingTranscription,
  saveTranscription,
} from "@/lib/tauri-commands";

interface TranscriptionSegment {
  start: number;
  end: number;
  text: string;
}

export function RecordingControls() {
  const [meetingTitle, setMeetingTitle] = useState("");
  const [liveTranscript, setLiveTranscript] = useState<TranscriptionSegment[]>(
    [],
  );
  const [enableLiveTranscription, setEnableLiveTranscription] = useState(false);

  const { isRecording, elapsedSeconds, start, stop, tick, reset } =
    useRecordingStore();
  const { create, update } = useMeetingsStore();

  useEffect(() => {
    if (!isRecording) return;

    const interval = setInterval(() => {
      tick();
    }, 1000);

    return () => clearInterval(interval);
  }, [isRecording, tick]);

  // Listen for transcription updates
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<TranscriptionSegment[]>(
        "transcription-update",
        (event) => {
          setLiveTranscript((prev) => [...prev, ...event.payload]);
        },
      );
    };

    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const handleStart = async () => {
    if (!meetingTitle.trim()) {
      alert("Inserisci un titolo per la riunione");
      return;
    }

    try {
      await start();
      const meeting = await create(meetingTitle);
      console.log("Registrazione avviata:", meeting.id);

      // Start live transcription if enabled
      if (enableLiveTranscription) {
        const meetings = useMeetingsStore.getState().meetings;
        const latestMeeting = meetings[0];
        if (latestMeeting?.audio_path) {
          try {
            await startStreamingTranscription(
              latestMeeting.audio_path,
              "base",
              "it",
            );
            console.log("Trascrizione streaming avviata");
          } catch (e) {
            console.error("Errore avvio trascrizione streaming:", e);
          }
        }
      }

      setLiveTranscript([]);
    } catch (e) {
      console.error("Errore avvio registrazione:", e);
      alert("Errore durante l'avvio della registrazione");
    }
  };

  const handleStop = async () => {
    try {
      // Stop streaming transcription
      if (enableLiveTranscription) {
        await stopStreamingTranscription();
      }

      const audioPath = await stop();

      const meetings = useMeetingsStore.getState().meetings;
      const latestMeeting = meetings[0];

      if (latestMeeting) {
        await update(latestMeeting.id, undefined, elapsedSeconds, "recorded", audioPath);

        // Save live transcript if available
        if (liveTranscript.length > 0) {
          const fullText = liveTranscript.map((s) => s.text).join(" ");
          await saveTranscription(latestMeeting.id, fullText, "it");
          console.log("Trascrizione live salvata:", liveTranscript.length, "segmenti");
        }
      }

      reset();
      setMeetingTitle("");
      setLiveTranscript([]);
      console.log("Registrazione salvata:", audioPath);
    } catch (e) {
      console.error("Errore stop registrazione:", e);
      alert("Errore durante l'arresto della registrazione");
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Nuova Registrazione</CardTitle>
        <CardDescription>Registra una nuova riunione</CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {!isRecording ? (
          <>
            <div className="space-y-2">
              <label className="text-sm font-medium">Titolo Riunione</label>
              <Input
                placeholder="es. Meeting Team Sviluppo"
                value={meetingTitle}
                onChange={(e) => setMeetingTitle(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && handleStart()}
              />
            </div>

            <AudioSourceSelector />

            <div className="flex items-center justify-between p-4 rounded-lg border">
              <div>
                <p className="font-medium">Trascrizione in Tempo Reale</p>
                <p className="text-sm text-muted-foreground">
                  Trascrivi mentre registri (richiede più risorse)
                </p>
              </div>
              <input
                type="checkbox"
                className="h-5 w-5"
                checked={enableLiveTranscription}
                onChange={(e) => setEnableLiveTranscription(e.target.checked)}
              />
            </div>

            <Button
              onClick={handleStart}
              className="w-full"
              size="lg"
              disabled={!meetingTitle.trim()}
            >
              <Circle className="mr-2 h-5 w-5 fill-current" />
              Avvia Registrazione
            </Button>
          </>
        ) : (
          <>
            <RecordingStatus
              title={meetingTitle}
              duration={formatDuration(elapsedSeconds)}
            />

            {enableLiveTranscription && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium">Trascrizione Live</h3>
                <div className="p-4 rounded-lg bg-muted/50 max-h-48 overflow-y-auto">
                  {liveTranscript.length === 0 ? (
                    <p className="text-sm text-muted-foreground">
                      In attesa di audio da trascrivere...
                    </p>
                  ) : (
                    <p className="text-sm whitespace-pre-wrap leading-relaxed">
                      {liveTranscript.map((s) => s.text).join(" ")}
                    </p>
                  )}
                </div>
              </div>
            )}

            <Button
              onClick={handleStop}
              variant="destructive"
              className="w-full"
              size="lg"
            >
              <Square className="mr-2 h-5 w-5" />
              Ferma Registrazione
            </Button>
          </>
        )}
      </CardContent>
    </Card>
  );
}
