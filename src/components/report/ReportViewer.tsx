import type { MeetingReport } from "@/lib/tauri-commands";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Lightbulb, Users, CheckSquare } from "lucide-react";

interface ReportViewerProps {
  report: MeetingReport;
}

export function ReportViewer({ report }: ReportViewerProps) {
  return (
    <div className="space-y-6">
      {/* Punti Salienti */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Lightbulb className="h-5 w-5 text-yellow-500" />
            Punti Salienti
          </CardTitle>
        </CardHeader>
        <CardContent>
          {report.highlights.length > 0 ? (
            <ul className="space-y-2">
              {report.highlights.map((highlight, index) => (
                <li key={index} className="flex items-start gap-2">
                  <span className="text-muted-foreground mt-1">•</span>
                  <span>{highlight}</span>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-muted-foreground text-sm">
              Nessun punto saliente identificato
            </p>
          )}
        </CardContent>
      </Card>

      {/* Partecipanti */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Users className="h-5 w-5 text-blue-500" />
            Partecipanti
          </CardTitle>
        </CardHeader>
        <CardContent>
          {report.participants.length > 0 ? (
            <div className="flex flex-wrap gap-2">
              {report.participants.map((participant, index) => (
                <span
                  key={index}
                  className="px-3 py-1 bg-secondary rounded-full text-sm"
                >
                  {participant}
                </span>
              ))}
            </div>
          ) : (
            <p className="text-muted-foreground text-sm">
              Nessun partecipante identificato
            </p>
          )}
        </CardContent>
      </Card>

      {/* Action Items */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <CheckSquare className="h-5 w-5 text-green-500" />
            Action Items
          </CardTitle>
        </CardHeader>
        <CardContent>
          {report.action_items.length > 0 ? (
            <ul className="space-y-3">
              {report.action_items.map((item, index) => (
                <li
                  key={index}
                  className="flex items-start gap-3 p-3 rounded-lg bg-muted/50"
                >
                  <input type="checkbox" className="mt-1 h-4 w-4" />
                  <span>{item}</span>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-muted-foreground text-sm">
              Nessuna azione identificata
            </p>
          )}
        </CardContent>
      </Card>

      {/* Info Report */}
      {report.llm_provider && (
        <div className="text-xs text-muted-foreground text-center">
          Report generato con {report.llm_provider} ({report.llm_model})
        </div>
      )}
    </div>
  );
}
