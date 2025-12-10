import { useEffect } from "react";
import { useMeetingsStore } from "@/stores/meetingsStore";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { formatDate, formatDuration } from "@/lib/utils";
import { FileAudio, Trash2, Eye } from "lucide-react";

interface MeetingListProps {
  onSelectMeeting: (id: string) => void;
}

export function MeetingList({ onSelectMeeting }: MeetingListProps) {
  const { meetings, loadMeetings, remove, isLoading } = useMeetingsStore();

  useEffect(() => {
    loadMeetings();
  }, [loadMeetings]);

  const getStatusBadge = (status: string) => {
    const statusMap: Record<string, { label: string; className: string }> = {
      recording: { label: "Registrazione", className: "bg-destructive text-destructive-foreground" },
      recorded: { label: "Registrata", className: "bg-secondary text-secondary-foreground" },
      transcribing: { label: "Trascrizione", className: "bg-primary text-primary-foreground" },
      transcribed: { label: "Trascritta", className: "bg-accent text-accent-foreground" },
      completed: { label: "Completata", className: "bg-green-500 text-white" },
    };

    const { label, className } = statusMap[status] || statusMap.recorded;
    return (
      <span className={`text-xs px-2 py-1 rounded-full ${className}`}>
        {label}
      </span>
    );
  };

  const handleDelete = async (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (confirm("Sei sicuro di voler eliminare questa riunione?")) {
      try {
        await remove(id);
      } catch (e) {
        alert("Errore durante l'eliminazione");
      }
    }
  };

  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Storico Riunioni</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-center text-muted-foreground py-8">Caricamento...</p>
        </CardContent>
      </Card>
    );
  }

  if (meetings.length === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Storico Riunioni</CardTitle>
          <CardDescription>Le tue riunioni registrate appariranno qui</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-center text-muted-foreground py-8">
            Nessuna riunione registrata
          </p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Storico Riunioni</CardTitle>
        <CardDescription>{meetings.length} riunioni registrate</CardDescription>
      </CardHeader>
      <CardContent className="space-y-2">
        {meetings.map((meeting) => (
          <div
            key={meeting.id}
            className="flex items-center gap-3 p-3 rounded-lg border bg-card hover:bg-accent/50 transition-colors cursor-pointer"
            onClick={() => onSelectMeeting(meeting.id)}
          >
            <FileAudio className="h-10 w-10 text-muted-foreground flex-shrink-0" />

            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-1">
                <h3 className="font-medium truncate">{meeting.title}</h3>
                {getStatusBadge(meeting.status)}
              </div>
              <div className="flex items-center gap-3 text-sm text-muted-foreground">
                <span>{formatDate(meeting.created_at)}</span>
                {meeting.duration_seconds && (
                  <span>• {formatDuration(meeting.duration_seconds)}</span>
                )}
              </div>
            </div>

            <div className="flex items-center gap-2">
              <Button
                size="icon"
                variant="ghost"
                onClick={() => onSelectMeeting(meeting.id)}
              >
                <Eye className="h-4 w-4" />
              </Button>
              <Button
                size="icon"
                variant="ghost"
                onClick={(e) => handleDelete(meeting.id, e)}
              >
                <Trash2 className="h-4 w-4 text-destructive" />
              </Button>
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
