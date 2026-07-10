"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { useSettingsStore } from "@/stores/settings";
import { useToast } from "@/hooks/use-toast";
import { Settings, Shield, Sun, Moon, Save } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { useState, useCallback } from "react";

function isValidUrl(str: string) {
  try {
    const url = new URL(str);
    return url.protocol === "http:" || url.protocol === "https:";
  } catch {
    return false;
  }
}

export default function SettingsPage() {
  const apiUrl = useSettingsStore((s) => s.apiUrl);
  const setApiUrl = useSettingsStore((s) => s.setApiUrl);
  const theme = useSettingsStore((s) => s.theme);
  const setTheme = useSettingsStore((s) => s.setTheme);
  const [localUrl, setLocalUrl] = useState(apiUrl);
  const [urlError, setUrlError] = useState<string | null>(null);
  const { toast } = useToast();

  const { data: health, isError: healthError } = useQuery({
    queryKey: ["health", apiUrl],
    queryFn: async () => {
      const res = await fetch(`${apiUrl}/system/health`, {
        signal: AbortSignal.timeout(5000),
      });
      const json = await res.json();
      return json;
    },
    refetchInterval: 30_000,
    retry: 1,
  });

  const handleSave = useCallback(() => {
    const trimmed = localUrl.trim().replace(/\/+$/, "");
    if (!trimmed) {
      setUrlError("API URL is required");
      return;
    }
    if (!isValidUrl(trimmed)) {
      setUrlError("Must be a valid HTTP or HTTPS URL");
      return;
    }
    setUrlError(null);
    setApiUrl(trimmed);
    toast({
      title: "API URL updated",
      description: trimmed,
      variant: "success",
    });
  }, [localUrl, setApiUrl, toast]);

  const handleUrlChange = useCallback((value: string) => {
    setLocalUrl(value);
    setUrlError(null);
  }, []);

  const version = health?.data?.version as string | undefined;

  return (
    <div className="space-y-6 max-w-2xl">
      <div className="flex items-center gap-3">
        <div className="flex items-center justify-center w-10 h-10 rounded-xl bg-accent/30">
          <Settings className="h-5 w-5 text-accent-foreground" />
        </div>
        <div>
          <h1 className="text-3xl font-bold">Settings</h1>
          <p className="text-muted-foreground mt-1">Configure ODIN connections and preferences</p>
        </div>
      </div>

      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <div className="flex items-center justify-center w-8 h-8 rounded-lg bg-accent/30">
              <Shield className="h-4 w-4 text-accent-foreground" />
            </div>
            <CardTitle>API Connection</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-1 block" htmlFor="api-url">
              API URL
            </label>
            <div className="flex gap-2">
              <div className="flex-1 space-y-1">
                <Input
                  id="api-url"
                  value={localUrl}
                  onChange={(e) => handleUrlChange(e.target.value)}
                  placeholder="http://localhost:3001/api/v1"
                  className={urlError ? "border-destructive" : ""}
                />
                {urlError && (
                  <p className="text-xs text-destructive">{urlError}</p>
                )}
              </div>
              <Button variant="secondary" onClick={handleSave}>
                <Save className="h-4 w-4 mr-1" /> Save
              </Button>
            </div>
          </div>
          <div className="flex items-center gap-2 text-sm">
            {healthError ? (
              <Badge variant="outline" className="text-red-400">Disconnected</Badge>
            ) : health?.success ? (
              <Badge variant="outline" className="text-green-400">Connected</Badge>
            ) : (
              <Badge variant="outline" className="text-yellow-400">Checking...</Badge>
            )}
            {version && <span className="text-muted-foreground">v{version}</span>}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <div className="flex items-center justify-center w-8 h-8 rounded-lg bg-accent/30">
              {theme === "dark" ? (
                <Moon className="h-4 w-4 text-accent-foreground" />
              ) : (
                <Sun className="h-4 w-4 text-accent-foreground" />
              )}
            </div>
            <CardTitle>Appearance</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium">Theme</p>
              <p className="text-xs text-muted-foreground">
                Toggle between dark and light mode
              </p>
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
