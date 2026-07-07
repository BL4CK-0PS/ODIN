import { cn } from "@/lib/utils";
import { LayoutDashboard, Search, Brain, Shield, Settings, Activity, ChevronLeft } from "lucide-react";
import { navigate } from "@/lib/router";
import { useState } from "react";
import type { Page } from "@/lib/types";

const links: { href: string; label: string; icon: typeof LayoutDashboard; page: Page }[] = [
  { href: "/", label: "Dashboard", icon: LayoutDashboard, page: "dashboard" },
  { href: "/investigations", label: "Investigations", icon: Activity, page: "investigations" },
  { href: "/threat-memory", label: "Threat Memory", icon: Brain, page: "threat-memory" },
  { href: "/knowledge-explorer", label: "Knowledge Explorer", icon: Shield, page: "knowledge-explorer" },
  { href: "/search", label: "Search", icon: Search, page: "search" },
  { href: "/settings", label: "Settings", icon: Settings, page: "settings" },
];

export function Sidebar({ currentPage }: { currentPage: Page }) {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside className={`h-screen border-r border-border bg-card flex flex-col transition-all duration-200 shrink-0 ${collapsed ? "w-16" : "w-60"}`}>
      <div className="flex items-center gap-3 p-4 border-b border-border">
        <Shield className="h-6 w-6 text-primary shrink-0" />
        {!collapsed && <span className="font-bold text-sm tracking-wide">ODIN</span>}
      </div>
      <nav className="flex-1 p-2 space-y-1">
        {links.map(({ href, label, icon: Icon, page }) => {
          const active = currentPage === page || (page === "investigations" && currentPage === "investigation-detail");
          return (
            <button
              key={href}
              onClick={() => navigate(href)}
              className={`flex items-center gap-3 w-full px-3 py-2.5 rounded-lg text-sm transition-colors text-left ${active ? "bg-primary/10 text-primary font-medium" : "text-muted-foreground hover:bg-secondary hover:text-foreground"}`}
            >
              <Icon className="h-4 w-4 shrink-0" />
              {!collapsed && <span>{label}</span>}
            </button>
          );
        })}
      </nav>
      <button onClick={() => setCollapsed(!collapsed)} className="p-3 border-t border-border text-muted-foreground hover:text-foreground">
        <ChevronLeft className={`h-4 w-4 transition-transform ${collapsed && "rotate-180"}`} />
      </button>
    </aside>
  );
}
