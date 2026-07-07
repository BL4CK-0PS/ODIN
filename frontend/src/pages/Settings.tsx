import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Settings, Shield, Bell } from "lucide-react";

export function SettingsPage() {
  return (
    <div className="space-y-6 max-w-2xl">
      <div className="flex items-center gap-3"><Settings className="h-8 w-8 text-primary" /><div><h1 className="text-3xl font-bold">Settings</h1><p className="text-muted-foreground mt-1">Configure ODIN connections and preferences</p></div></div>
      <Card>
        <CardHeader><div className="flex items-center gap-2"><Shield className="h-5 w-5 text-primary" /><CardTitle>API Connection</CardTitle></div></CardHeader>
        <CardContent className="space-y-4">
          <div><label className="text-sm font-medium mb-1 block">API URL</label><div className="flex gap-2"><Input defaultValue="http://localhost:3000/api/v1" /><Button variant="secondary">Save</Button></div></div>
          <div className="flex items-center gap-2 text-sm"><Badge variant="outline" className="text-green-400">Connected</Badge><span className="text-muted-foreground">v0.1.0</span></div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader><div className="flex items-center gap-2"><Bell className="h-5 w-5 text-primary" /><CardTitle>Notifications</CardTitle></div></CardHeader>
        <CardContent><p className="text-sm text-muted-foreground">Notification preferences coming soon.</p></CardContent>
      </Card>
    </div>
  );
}
