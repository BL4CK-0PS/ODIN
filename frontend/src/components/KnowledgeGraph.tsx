import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Brain, ArrowRight } from "lucide-react";

interface Node { id: string; type: string; label: string }
interface Edge { source: string; target: string; label: string }

export function KnowledgeGraph({ nodes, edges }: { nodes: Node[]; edges: Edge[] }) {
  return (
    <Card>
      <CardHeader><div className="flex items-center gap-2"><Brain className="h-5 w-5 text-primary" /><CardTitle className="text-lg">Knowledge Graph</CardTitle></div></CardHeader>
      <CardContent>
        <ScrollArea className="max-h-96">
          <div className="space-y-3">{nodes.map((node) => (
            <div key={node.id}>
              <div className="flex items-center gap-2 p-2 rounded-lg bg-secondary/50 text-sm">
                <div className="w-2 h-2 rounded-full bg-primary" />
                <span className="font-medium">{node.label}</span>
                <span className="text-xs text-muted-foreground ml-auto">{node.type}</span>
              </div>
              {edges.filter((e) => e.source === node.id).map((edge) => {
                const target = nodes.find((n) => n.id === edge.target);
                return (
                  <div key={`${edge.source}-${edge.target}`} className="ml-4 mt-1 flex items-center gap-2 text-xs text-muted-foreground">
                    <ArrowRight className="h-3 w-3" /><span>{edge.label}</span><span className="font-medium">{target?.label || edge.target}</span>
                  </div>
                );
              })}
            </div>
          ))}</div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}
