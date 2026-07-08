"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { useInvestigations } from "@/hooks/use-investigation";
import Link from "next/link";

const sevColor: Record<string, string> = {
  Critical: "text-red-400 border-red-400/30",
  High: "text-orange-400 border-orange-400/30",
  Medium: "text-yellow-400 border-yellow-400/30",
  Low: "text-green-400 border-green-400/30",
};

export default function InvestigationsPage() {
  const { data: incidents, isLoading, error } = useInvestigations();

  if (isLoading) {
    return (
      <div className="space-y-6">
        <h1 className="text-3xl font-bold">Investigations</h1>
        <div className="grid gap-4">
          {[1, 2, 3].map((i) => (
            <Card key={i}>
              <CardHeader className="flex-row items-start justify-between space-y-0">
                <div className="space-y-2">
                  <Skeleton className="h-5 w-64" />
                  <Skeleton className="h-3 w-24" />
                </div>
                <Skeleton className="h-5 w-16" />
              </CardHeader>
              <CardContent className="flex gap-2">
                <Skeleton className="h-5 w-20" />
                <Skeleton className="h-5 w-14" />
                <Skeleton className="h-5 w-14" />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-6">
        <h1 className="text-3xl font-bold">Investigations</h1>
        <Card><CardContent className="p-6 text-red-400">Failed to load investigations: {(error as Error).message}</CardContent></Card>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Investigations</h1>
      <div className="grid gap-4">
        {(incidents?.length ?? 0) === 0 ? (
          <Card><CardContent className="p-6 text-muted-foreground">No investigations found. Upload one from the Dashboard.</CardContent></Card>
        ) : (
          incidents?.map((inc) => (
            <Link key={inc.id} href={`/investigations/${inc.id}`}>
              <Card className="cursor-pointer transition-colors hover:border-primary/50">
                <CardHeader className="flex-row items-start justify-between space-y-0">
                  <div>
                    <CardTitle className="text-base">{inc.title}</CardTitle>
                    <p className="text-xs text-muted-foreground mt-1">{inc.created_at ? new Date(inc.created_at).toLocaleDateString() : ""}</p>
                  </div>
                  <Badge variant="outline" className={sevColor[inc.severity] || ""}>
                    {inc.severity}
                  </Badge>
                </CardHeader>
                <CardContent className="flex gap-2 flex-wrap">
                  <Badge variant="secondary">{inc.status}</Badge>
                  {(inc.mitre_techniques || []).map((t) => (
                    <Badge key={t} variant="outline">{t}</Badge>
                  ))}
                </CardContent>
              </Card>
            </Link>
          ))
        )}
      </div>
    </div>
  );
}
