import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { TimelineView } from "@/components/TimelineView";
import { NarrativeCard } from "@/components/NarrativeCard";
import { EvidenceTable } from "@/components/EvidenceTable";
import { KnowledgeGraph } from "@/components/KnowledgeGraph";
import { SimilarityCard } from "@/components/SimilarityCard";
import { PlaybookCard } from "@/components/PlaybookCard";
import { Tabs } from "@/components/ui/tabs";

const timeline = [
  { id: "e1", source: "Sysmon Event ID 1 — Process Creation", type: "Log", collected_at: "2026-07-06T10:00:00Z" },
  { id: "e2", source: "Network Connection — 192.168.1.100:443", type: "NetworkCapture", collected_at: "2026-07-06T10:05:00Z" },
  { id: "e3", source: "Registry Key Modification — HKLM\\SOFTWARE\\Malware", type: "FileSystemArtifact", collected_at: "2026-07-06T10:12:00Z" },
];
const evidence = [
  { id: "e1", source: "Process Creation Log", content_type: "Log", trust_score: 0.94 },
  { id: "e2", source: "C2 Network Traffic", content_type: "NetworkCapture", trust_score: 0.88 },
  { id: "e3", source: "Registry Artifact", content_type: "FileSystemArtifact", trust_score: 0.97 },
];
const nodes = [
  { id: "n1", type: "Process", label: "powershell.exe" },
  { id: "n2", type: "Network", label: "192.168.1.100:443" },
  { id: "n3", type: "Registry", label: "HKLM\\SOFTWARE\\Malware" },
];
const edges = [
  { source: "n1", target: "n2", label: "connected_to" },
  { source: "n1", target: "n3", label: "modified" },
];

export function InvestigationDetail({ id }: { id: string }) {
  return (
    <div className="space-y-6">
      <div><h1 className="text-3xl font-bold">Investigation {id}</h1><p className="text-muted-foreground mt-1">Ransomware Outbreak — Finance Department</p></div>
      <div className="grid grid-cols-3 gap-4">
        <NarrativeCard summary="Initial compromise via phishing email. PowerShell process launched C2 connection to external IP." confidence={0.91} techniques={["T1059", "T1486", "T1547"]} />
        <EvidenceTable evidence={evidence} />
        <PlaybookCard name="Ransomware Response" steps={["Isolate affected systems", "Block C2 domains", "Collect memory dumps", "Identify encryption scope"]} />
      </div>
      <Tabs tabs={[
        { label: "Timeline", value: "timeline", content: <Card><CardHeader><CardTitle>Event Timeline</CardTitle></CardHeader><CardContent className="h-96"><TimelineView events={timeline} /></CardContent></Card> },
        { label: "Knowledge Graph", value: "graph", content: <KnowledgeGraph nodes={nodes} edges={edges} /> },
        { label: "Similar Incidents", value: "similarity", content: <div className="grid grid-cols-2 gap-4"><SimilarityCard title="Ransomware — HR Dept (2026-05)" score={0.92} reasons={["Same T1059", "Same PowerShell", "Same Registry Key"]} /><SimilarityCard title="Phishing — Engineering (2026-04)" score={0.67} reasons={["Same T1566", "Different C2"]} /></div> },
      ]} />
    </div>
  );
}
