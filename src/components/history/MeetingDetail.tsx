import { useEffect, useState } from "react";
import { useMeetingsStore } from "@/stores/meetingsStore";
import { useSettingsStore } from "@/stores/settingsStore";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { TranscriptViewer } from "./TranscriptViewer";
import { ReportViewer } from "../report/ReportViewer";
import { formatDate, formatDuration } from "@/lib/utils";
import { ArrowLeft, FileText, Sparkles, Loader2, Volume2 } from "lucide-react";
import { convertFileSrc } from "@tauri-apps/api/core";

interface MeetingDetailProps {
  meetingId: string;
  onBack: () => void;
}

export function MeetingDetail({ meetingId, onBack }: MeetingDetailProps) {
  const {
    currentMeeting,
    loadMeeting,
    transcribe,
    generateReport,
    isTranscribing,
    isGeneratingReport,
  } = useMeetingsStore();
  const { appSettings } = useSettingsStore();
  const [activeTab, setActiveTab] = useState<"transcript" | "report">(
    "transcript",
  );

  useEffect(() => {
    loadMeeting(meetingId);
  }, [meetingId, loadMeeting]);

  const handleTranscribe = async () => {
    if (!currentMeeting?.meeting.audio_path) return;

    try {
      const modelSize = appSettings?.whisper_model || "base";
      const language = appSettings?.default_language || "it";

      await transcribe(
        meetingId,
        currentMeeting.meeting.audio_path,
        modelSize,
        language,
      );
    } catch (e) {
      alert("Errore durante la trascrizione");
    }
  };

  const handleGenerateReport = async () => {
    if (!currentMeeting?.transcript) return;

    try {
      await generateReport(meetingId, currentMeeting.transcript);
      setActiveTab("report");
    } catch (e) {
      alert("Errore durante la generazione del report");
    }
  };

  if (!currentMeeting) {
    return (
      <Card>
        <CardContent className="py-8">
          <p className="text-center text-muted-foreground">Caricamento...</p>
        </CardContent>
      </Card>
    );
  }

  const { meeting, transcript, report } = currentMeeting;

  return (
    <div className="space-y-4">
      <Button variant="ghost" onClick={onBack}>
        <ArrowLeft className="mr-2 h-4 w-4" />
        Torna allo storico
      </Button>

      <Card>
        <CardHeader>
          <div className="flex items-start justify-between">
            <div>
              <CardTitle className="text-2xl">{meeting.title}</CardTitle>
              <CardDescription>
                {formatDate(meeting.created_at)}
                {meeting.duration_seconds &&
                  ` • ${formatDuration(meeting.duration_seconds)}`}
              </CardDescription>
            </div>
            <span className="text-xs px-3 py-1 rounded-full bg-secondary">
              {meeting.status}
            </span>
          </div>
        </CardHeader>

        <CardContent className="space-y-4">
          {/* Audio Player */}
          {meeting.audio_path && (
            <div className="rounded-lg border p-4 bg-muted/50">
              <div className="flex items-center gap-3 mb-2">
                <Volume2 className="h-4 w-4 text-muted-foreground" />
                <span className="text-sm font-medium">Registrazione Audio</span>
              </div>
              <audio
                controls
                className="w-full"
                src={convertFileSrc(meeting.audio_path)}
              >
                Il tuo browser non supporta la riproduzione audio.
              </audio>
            </div>
          )}

          {!transcript && (
            <div className="text-center py-8 space-y-4">
              <FileText className="h-12 w-12 mx-auto text-muted-foreground" />
              <div>
                <p className="text-muted-foreground mb-4">
                  Questa riunione non è ancora stata trascritta
                </p>
                <Button onClick={handleTranscribe} disabled={isTranscribing}>
                  {isTranscribing ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Trascrizione in corso...
                    </>
                  ) : (
                    <>
                      <FileText className="mr-2 h-4 w-4" />
                      Avvia Trascrizione
                    </>
                  )}
                </Button>
              </div>
            </div>
          )}

          {transcript && (
            <>
              <div className="flex items-center gap-2 border-b">
                <button
                  className={`px-4 py-2 font-medium transition-colors ${
                    activeTab === "transcript"
                      ? "border-b-2 border-primary text-foreground"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                  onClick={() => setActiveTab("transcript")}
                >
                  Trascrizione
                </button>
                <button
                  className={`px-4 py-2 font-medium transition-colors ${
                    activeTab === "report"
                      ? "border-b-2 border-primary text-foreground"
                      : "text-muted-foreground hover:text-foreground"
                  }`}
                  onClick={() => setActiveTab("report")}
                >
                  Report
                </button>
              </div>

              {activeTab === "transcript" && (
                <div className="space-y-4">
                  <TranscriptViewer transcript={transcript} />

                  {!report && (
                    <div className="text-center pt-4">
                      <Button
                        onClick={handleGenerateReport}
                        disabled={isGeneratingReport}
                      >
                        {isGeneratingReport ? (
                          <>
                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            Generazione report in corso...
                          </>
                        ) : (
                          <>
                            <Sparkles className="mr-2 h-4 w-4" />
                            Genera Report con AI
                          </>
                        )}
                      </Button>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "report" && (
                <div>
                  {report ? (
                    <ReportViewer report={report} />
                  ) : (
                    <div className="text-center py-8 space-y-4">
                      <Sparkles className="h-12 w-12 mx-auto text-muted-foreground" />
                      <div>
                        <p className="text-muted-foreground mb-4">
                          Nessun report disponibile
                        </p>
                        <Button
                          onClick={handleGenerateReport}
                          disabled={isGeneratingReport}
                        >
                          {isGeneratingReport ? (
                            <>
                              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                              Generazione in corso...
                            </>
                          ) : (
                            <>
                              <Sparkles className="mr-2 h-4 w-4" />
                              Genera Report
                            </>
                          )}
                        </Button>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
