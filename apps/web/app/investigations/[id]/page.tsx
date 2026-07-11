"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { TimelineView } from "@/components/TimelineView";
import { NarrativeCard } from "@/components/NarrativeCard";
import { EvidenceTable } from "@/components/EvidenceTable";
import { KnowledgeGraph } from "@/components/KnowledgeGraph";
import { SimilarityCard } from "@/components/SimilarityCard";
import { SimilarityReason } from "@/components/SimilarityReason";
import { PlaybookCard } from "@/components/PlaybookCard";
import { TrustScoreChart } from "@/components/charts/TrustScoreChart";
import { useInvestigation } from "@/hooks/use-investigation";
import { useSearchSimilar } from "@/hooks/use-search";
import { useToast } from "@/hooks/use-toast";
import { api } from "@/lib/api";
import { useParams } from "next/navigation";
import { useState } from "react";
import { ThumbsUp, ThumbsDown, ArrowRight } from "lucide-react";
import { useQueryClient } from "@tanstack/react-query";
import type { RankedResult, Evidence } from "@/lib/types";

const STATUS_TRANSITIONS: Record<string, string[]> = {
  New: ["Investigating", "Closed"],
  Investigating: ["Contained", "Eradicated", "Closed"],
  Contained: ["Eradicated", "Recovered", "Closed"],
  Eradicated: ["Recovered", "Closed"],
  Recovered: ["Closed"],
  Closed: [],
};

const STATUS_COLORS: Record<string, string> = {
  New: "bg-blue-500/10 text-blue-500 border-blue-500/20",
  Investigating: "bg-amber-500/10 text-amber-500 border-amber-500/20",
  Contained: "bg-orange-500/10 text-orange-500 border-orange-500/20",
  Eradicated: "bg-red-500/10 text-red-500 border-red-500/20",
  Recovered: "bg-green-500/10 text-green-500 border-green-500/20",
  Closed: "bg-gray-500/10 text-gray-500 border-gray-500/20",
};

interface TimelineViewEvent {
  id: string;
  source: string;
  type: string;
  collected_at: string;
}

function FeedbackSection({ incidentId }: { incidentId: string }) {
  const [rating, setRating] = useState<1 | -1 | null>(null);
  const [feedback, setFeedback] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const { toast } = useToast();

  const handleSubmit = async () => {
    setSubmitting(true);
    try {
      await api.postFeedback(incidentId, feedback, rating === 1 ? 1 : 0);
      setSubmitted(true);
      toast({ title: "Feedback submitted", variant: "success" });
    } catch (err) {
      toast({
        title: "Failed to submit feedback",
        description: err instanceof Error ? err.message : undefined,
        variant: "error",
      });
    } finally {
      setSubmitting(false);
    }
  };

  if (submitted) {
    return (
      <Card>
        <CardContent className="p-4 text-sm text-green-400">
          Thank you for your feedback. This helps improve threat memory accuracy.
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">Was this analysis helpful?</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex gap-2">
          <Button
            variant={rating === 1 ? "default" : "outline"}
            size="sm"
            onClick={() => setRating(rating === 1 ? null : 1)}
          >
            <ThumbsUp className="h-4 w-4 mr-1" /> Yes
          </Button>
          <Button
            variant={rating === -1 ? "default" : "outline"}
            size="sm"
            onClick={() => setRating(rating === -1 ? null : -1)}
          >
            <ThumbsDown className="h-4 w-4 mr-1" /> No
          </Button>
        </div>
        {rating && (
          <div className="space-y-2">
            <Textarea
              placeholder="Additional feedback..."
              value={feedback}
              onChange={(e) => setFeedback(e.target.value)}
              className="text-sm"
              maxLength={5000}
            />
            <div className="flex items-center justify-between">
              <span className="text-xs text-muted-foreground">
                {feedback.length}/5000
              </span>
              <Button size="sm" onClick={handleSubmit} disabled={submitting}>
                {submitting ? "Submitting..." : "Submit"}
              </Button>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

export default function InvestigationPage() {
  const { id } = useParams<{ id: string }>();
  const { data, isLoading, error } = useInvestigation(id);
  const { data: similarityData, isLoading: isSimLoading } = useSearchSimilar(id);
  const queryClient = useQueryClient();
  const { toast } = useToast();
  const [updatingStatus, setUpdatingStatus] = useState(false);

  if (isLoading) {
    return (
      <div className="space-y-6">
        <Skeleton className="h-9 w-96" />
        <Skeleton className="h-4 w-64" />
        <div className="grid grid-cols-3 gap-4 mt-6">
          <Skeleton className="h-48" />
          <Skeleton className="h-48" />
          <Skeleton className="h-48" />
        </div>
        <Skeleton className="h-64" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-6">
        <h1 className="text-3xl font-bold">Investigation {id}</h1>
        <Card>
          <CardContent className="p-6 text-red-400">
            Failed to load investigation: {(error as Error).message}
          </CardContent>
        </Card>
      </div>
    );
  }

  const incident = data?.incident;
  const timeline = data?.timeline as { incident_id: string; events: unknown[] } | undefined;
  const memory = data?.memory;
  const graph = data?.graph as { nodes: unknown[]; edges: unknown[] } | undefined;
  const playbooks = data?.playbooks as { incident_id: string; playbooks: unknown[] } | undefined;

  const incTitle = incident?.title || `Investigation ${id}`;
  const incDesc = incident?.description || "";
  const narrativeSummary = memory?.summary || incDesc;
  const narrativeConfidence = memory?.confidence ?? 0.85;
  const mitreTechniques: string[] = incident?.mitre_techniques || [];
  const timelineEvents: unknown[] = timeline?.events || [];

  const evidenceList: Evidence[] = timelineEvents.map((ev) => {
    const e = ev as Record<string, unknown>;
    return {
      id: e.id as string,
      source: e.source as string,
      content_type: e.type as Evidence["content_type"],
      trust_score: (e.trust_score as number) ?? 0.9,
      content: (e.content as string) || "",
      incident_id: id,
      collected_at: (e.collected_at as string) || "",
    };
  });

  const playbookList = (playbooks?.playbooks || []) as { name: string; steps: string[] }[];
  const primaryPlaybook = playbookList[0] || { name: "Response Playbook", steps: ["Isolate affected systems", "Collect evidence", "Contain threat", "Remediate"] };

  const results: RankedResult[] = (similarityData || []).filter(r => r.memory?.incident_id !== id);

  const currentStatus = incident?.status || "New";
  const allowedTransitions = STATUS_TRANSITIONS[currentStatus] || [];

  const handleStatusChange = async (newStatus: string) => {
    setUpdatingStatus(true);
    try {
      await api.updateStatus(id, newStatus);
      queryClient.invalidateQueries({ queryKey: ["investigation", id] });
      toast({ title: `Status updated to ${newStatus}`, variant: "success" });
    } catch (err) {
      toast({
        title: "Failed to update status",
        description: err instanceof Error ? err.message : undefined,
        variant: "error",
      });
    } finally {
      setUpdatingStatus(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-start justify-between">
        <div>
          <h1 className="text-3xl font-bold">{incTitle}</h1>
          <p className="text-muted-foreground mt-1">{incDesc}</p>
        </div>
        <div className="flex items-center gap-2">
          <Badge variant="outline" className={STATUS_COLORS[currentStatus]}>
            {currentStatus}
          </Badge>
          {allowedTransitions.length > 0 && (
            <div className="flex gap-1">
              {allowedTransitions.map((status) => (
                <Button
                  key={status}
                  variant="outline"
                  size="sm"
                  onClick={() => handleStatusChange(status)}
                  disabled={updatingStatus}
                  className="text-xs"
                >
                  {status}
                  <ArrowRight className="h-3 w-3 ml-1" />
                </Button>
              ))}
            </div>
          )}
          <Button
            variant="outline"
            size="sm"
            onClick={async () => {
              try {
                const html = await api.getReportHtml(id);
                const blob = new Blob([html], { type: "text/html" });
                const url = URL.createObjectURL(blob);
                window.open(url, "_blank");
              } catch (err) {
                toast({
                  title: "Failed to generate report",
                  description: err instanceof Error ? err.message : undefined,
                  variant: "error",
                });
              }
            }}
          >
            Generate Report
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4">
        <NarrativeCard
          summary={narrativeSummary}
          confidence={narrativeConfidence}
          techniques={mitreTechniques}
        />
        <EvidenceTable evidence={evidenceList} />
        <div className="space-y-4">
          {playbookList.map((pb, idx) => (
            <PlaybookCard
              key={idx}
              name={pb.name}
              steps={pb.steps}
            />
          ))}
          {playbookList.length === 0 && (
            <PlaybookCard
              name={primaryPlaybook.name}
              steps={primaryPlaybook.steps}
            />
          )}
          <FeedbackSection incidentId={id} />
        </div>
      </div>

      <TrustScoreChart evidence={evidenceList} />

      <Tabs defaultValue="timeline">
        <TabsList>
          <TabsTrigger value="timeline">Timeline</TabsTrigger>
          <TabsTrigger value="graph">Knowledge Graph</TabsTrigger>
          <TabsTrigger value="similarity">Similar Incidents</TabsTrigger>
        </TabsList>
        <TabsContent value="timeline">
          <Card>
            <CardHeader><CardTitle>Event Timeline</CardTitle></CardHeader>
            <CardContent className="h-96">
              <TimelineView events={timelineEvents as TimelineViewEvent[]} />
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="graph">
          <KnowledgeGraph nodes={(graph?.nodes || []) as any} edges={(graph?.edges || []) as any} />
        </TabsContent>
        <TabsContent value="similarity">
          {isSimLoading ? (
            <div className="grid grid-cols-2 gap-4">
              <Skeleton className="h-24" />
              <Skeleton className="h-24" />
            </div>
          ) : results.length === 0 ? (
            <Card>
              <CardContent className="p-6 text-muted-foreground">
                No other similar investigations found in memory.
              </CardContent>
            </Card>
          ) : (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                {results.map((r, i) => (
                  <div key={i} className="space-y-2">
                    <SimilarityCard
                      title={String((r.memory?.context as Record<string, unknown>)?.title || r.memory?.summary || `Incident ${r.memory?.incident_id}`)}
                      score={r.score?.overall ?? 0}
                      reasons={r.reasons || []}
                    />
                    {r.reasons && r.reasons.length > 0 && (
                      <SimilarityReason
                        reasons={r.reasons.map((reason: string) => ({
                          label: reason,
                          matched: true,
                        }))}
                      />
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
}
