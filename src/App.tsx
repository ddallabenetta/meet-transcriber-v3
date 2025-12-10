import { useState } from "react";
import { RecordingControls } from "./components/recording/RecordingControls";
import { MeetingList } from "./components/history/MeetingList";
import { MeetingDetail } from "./components/history/MeetingDetail";
import { LLMSettings } from "./components/settings/LLMSettings";
import { AudioSettings } from "./components/settings/AudioSettings";
import { Mic, History, Settings } from "lucide-react";

type View = "recording" | "history" | "settings";

function App() {
  const [currentView, setCurrentView] = useState<View>("recording");
  const [selectedMeetingId, setSelectedMeetingId] = useState<string | null>(
    null,
  );

  const handleSelectMeeting = (id: string) => {
    setSelectedMeetingId(id);
  };

  const handleBackToList = () => {
    setSelectedMeetingId(null);
  };

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-2xl font-bold">Meet Transcriber</h1>
          <p className="text-sm text-muted-foreground">
            Registra, trascrivi e analizza le tue riunioni
          </p>
        </div>
      </header>

      {/* Navigation */}
      <nav className="border-b">
        <div className="container mx-auto px-4">
          <div className="flex gap-1">
            <button
              onClick={() => {
                setCurrentView("recording");
                setSelectedMeetingId(null);
              }}
              className={`flex items-center gap-2 px-4 py-3 font-medium transition-colors ${
                currentView === "recording"
                  ? "border-b-2 border-primary text-foreground"
                  : "text-muted-foreground hover:text-foreground"
              }`}
            >
              <Mic className="h-4 w-4" />
              Registrazione
            </button>
            <button
              onClick={() => {
                setCurrentView("history");
                setSelectedMeetingId(null);
              }}
              className={`flex items-center gap-2 px-4 py-3 font-medium transition-colors ${
                currentView === "history"
                  ? "border-b-2 border-primary text-foreground"
                  : "text-muted-foreground hover:text-foreground"
              }`}
            >
              <History className="h-4 w-4" />
              Storico
            </button>
            <button
              onClick={() => {
                setCurrentView("settings");
                setSelectedMeetingId(null);
              }}
              className={`flex items-center gap-2 px-4 py-3 font-medium transition-colors ${
                currentView === "settings"
                  ? "border-b-2 border-primary text-foreground"
                  : "text-muted-foreground hover:text-foreground"
              }`}
            >
              <Settings className="h-4 w-4" />
              Impostazioni
            </button>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-6">
        {currentView === "recording" && (
          <div className="max-w-2xl mx-auto">
            <RecordingControls />
          </div>
        )}

        {currentView === "history" && (
          <div className="max-w-4xl mx-auto">
            {selectedMeetingId ? (
              <MeetingDetail
                meetingId={selectedMeetingId}
                onBack={handleBackToList}
              />
            ) : (
              <MeetingList onSelectMeeting={handleSelectMeeting} />
            )}
          </div>
        )}

        {currentView === "settings" && (
          <div className="max-w-4xl mx-auto space-y-6">
            <LLMSettings />
            <AudioSettings />
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="border-t mt-12">
        <div className="container mx-auto px-4 py-4 text-center text-sm text-muted-foreground">
          Meet Transcriber v0.1.0 - Trascrizione locale e report AI
        </div>
      </footer>
    </div>
  );
}

export default App;
