"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { useSettingsStore } from "@/stores/settings";
import { Settings, Shield, Sun, Moon } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { useState } from "react";

export default function SettingsPage() {
  const apiUrl = useSettingsStore((s) => s.apiUrl);
  const setApiUrl = useSettingsStore((s) => s.setApiUrl);
  const theme = useSettingsStore((s) => s.theme);
  const setTheme = useSettingsStore((s) => s.setTheme);
  const [localUrl, setLocalUrl] = useState(apiUrl);

  const { data: health } = useQuery({
    queryKey: ["health", apiUrl],
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

  const version = health?.data?.version as string | undefined;

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
            {version && <span className="text-muted-foreground">v{version}</span>}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            {theme === "dark" ? <Moon className="h-5 w-5 text-primary" /> : <Sun className="h-5 w-5 text-primary" />}
            <CardTitle>Appearance</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium">Theme</p>
              <p className="text-xs text-muted-foreground">Toggle between dark and light mode</p>
            </div>
            <Button
              variant="outline"
              onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            >
              {theme === "dark" ? (
                <><Sun className="h-4 w-4 mr-2" /> Light Mode</>
              ) : (
                <><Moon className="h-4 w-4 mr-2" /> Dark Mode</>
              )}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
