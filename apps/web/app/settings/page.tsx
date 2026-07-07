"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { useSettingsStore } from "@/stores/settings";
import { Settings, Shield, Bell } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { useState } from "react";

export default function SettingsPage() {
  const apiUrl = useSettingsStore((s) => s.apiUrl);
  const setApiUrl = useSettingsStore((s) => s.setApiUrl);
  const [localUrl, setLocalUrl] = useState(apiUrl);

  const { data: health } = useQuery({
    queryKey: ["health"],
    queryFn: async () => {
      const res = await fetch(`${apiUrl}/system/health`);
      const json = await res.json();
      return json;
    },
    refetchInterval: 30_000,
  });

  const handleSave = () => {
    setApiUrl(localUrl);
  };

  return (
    <div className="space-y-6 max-w-2xl">
      <div className="flex items-center gap-3">
        <Settings className="h-8 w-8 text-primary" />
        <div>
          <h1 className="text-3xl font-bold">Settings</h1>
          <p className="text-muted-foreground mt-1">Configure ODIN connections and preferences</p>
        </div>
      </div>

      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <Shield className="h-5 w-5 text-primary" />
            <CardTitle>API Connection</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-1 block">API URL</label>
            <div className="flex gap-2">
              <Input value={localUrl} onChange={(e) => setLocalUrl(e.target.value)} />
              <Button variant="secondary" onClick={handleSave}>Save</Button>
            </div>
          </div>
          <div className="flex items-center gap-2 text-sm">
            {health?.success ? (
              <Badge variant="outline" className="text-green-400">Connected</Badge>
            ) : (
              <Badge variant="outline" className="text-red-400">Disconnected</Badge>
            )}
            <span className="text-muted-foreground">{(health as any)?.data?.version ? `v${(health as any).data.version}` : ""}</span>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <Bell className="h-5 w-5 text-primary" />
            <CardTitle>Notifications</CardTitle>
          </div>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">Notification preferences coming soon.</p>
        </CardContent>
      </Card>
    </div>
  );
}
