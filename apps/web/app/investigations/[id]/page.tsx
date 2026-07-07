"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { TimelineView } from "@/components/TimelineView";
import { NarrativeCard } from "@/components/NarrativeCard";
import { EvidenceTable } from "@/components/EvidenceTable";
import { KnowledgeGraph } from "@/components/KnowledgeGraph";
import { SimilarityCard } from "@/components/SimilarityCard";
import { PlaybookCard } from "@/components/PlaybookCard";
import { useInvestigation } from "@/hooks/use-investigation";
import { useParams } from "next/navigation";

export default function InvestigationPage() {
  const { id } = useParams<{ id: string }>();
  const { data, isLoading, error } = useInvestigation(id);

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
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-6">
        <h1 className="text-3xl font-bold">Investigation {id}</h1>
        <Card><CardContent className="p-6 text-red-400">Failed to load investigation: {(error as Error).message}</CardContent></Card>
      </div>
    );
  }

  const incident = data?.incident as any;
  const timeline = data?.timeline as any;
  const graph = data?.graph as any;
  const memory = data?.memory as any;
  const playbooks = data?.playbooks as any;

  const incTitle = incident?.title || `Investigation ${id}`;
  const incDesc = incident?.description || "";
  const timelineEvents = timeline?.events || [];
  const graphNodes = graph?.nodes || [];
  const graphEdges = graph?.edges || [];
  const narrativeSummary = memory?.summary || incDesc;
  const narrativeConfidence = memory?.confidence ?? 0.85;
  const mitreTechniques = incident?.mitre_techniques || [];
  const evidenceList = incident?.evidence_ids?.map((eid: string) => ({ id: eid, source: eid, content_type: "Log", trust_score: 0.5 })) || [];
  const playbookSteps = playbooks?.playbooks?.[0]?.steps || ["Isolate affected systems", "Collect evidence", "Contain threat", "Remediate"];

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">{incTitle}</h1>
          <p className="text-muted-foreground mt-1">{incDesc}</p>
        </div>
      </div>

      <div className="grid grid-cols-3 gap-4">
        <NarrativeCard
          summary={narrativeSummary}
          confidence={narrativeConfidence}
          techniques={mitreTechniques}
        />
        <EvidenceTable evidence={evidenceList} />
        <PlaybookCard
          name={playbooks?.playbooks?.[0]?.name || "Response Playbook"}
          steps={playbookSteps}
        />
      </div>

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
              <TimelineView events={timelineEvents} />
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="graph">
          <KnowledgeGraph nodes={graphNodes} edges={graphEdges} />
        </TabsContent>
        <TabsContent value="similarity">
          <div className="grid grid-cols-2 gap-4">
            <SimilarityCard
              title="Ransomware — HR Dept (2026-05)"
              score={0.92}
              reasons={["Same T1059", "Same PowerShell", "Same Registry Key"]}
            />
            <SimilarityCard
              title="Phishing — Engineering (2026-04)"
              score={0.67}
              reasons={["Same T1566", "Different C2"]}
            />
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
