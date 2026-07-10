"use client";

import { cn } from "@/lib/utils";
import {
  LayoutDashboard,
  Search,
  Brain,
  Shield,
  Settings,
  Activity,
  Archive,
  ChevronLeft,
} from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useState } from "react";

const links = [
  { href: "/", label: "Dashboard", icon: LayoutDashboard },
  { href: "/investigations", label: "Investigations", icon: Activity },
  { href: "/threat-memory", label: "Threat Memory", icon: Brain },
  { href: "/consolidation", label: "Consolidation", icon: Archive },
  { href: "/knowledge-explorer", label: "Knowledge Explorer", icon: Shield },
  { href: "/search", label: "Search", icon: Search },
  { href: "/settings", label: "Settings", icon: Settings },
];

export function Sidebar({ onNavigate }: { onNavigate?: () => void }) {
  const pathname = usePathname();
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside className={cn(
      "h-screen border-r border-border bg-card flex flex-col transition-all duration-300 relative",
      collapsed ? "w-[60px]" : "w-60"
    )}>
      <div className={cn(
        "flex items-center border-b border-border h-14",
        collapsed ? "justify-center px-0" : "gap-3 px-5"
      )}>
        <div className="flex items-center justify-center w-8 h-8 rounded-lg bg-primary text-primary-foreground shrink-0">
          <Shield className="h-4 w-4" />
        </div>
        {!collapsed && (
          <span className="font-semibold text-sm tracking-widest uppercase">ODIN</span>
        )}
      </div>

      <nav className="flex-1 p-2 space-y-0.5">
        {links.map(({ href, label, icon: Icon }) => {
          const active = pathname === href || (href !== "/" && pathname.startsWith(href));
          return (
            <Link
              key={href}
              href={href}
              onClick={onNavigate}
              className={cn(
                "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-all duration-200 relative group",
                active
                  ? "bg-accent/60 text-accent-foreground font-medium"
                  : "text-muted-foreground hover:bg-secondary/60 hover:text-foreground"
              )}
            >
              {active && (
                <span className="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-5 bg-accent-foreground rounded-full" />
              )}
              <Icon className={cn(
                "h-4 w-4 shrink-0 transition-colors",
                active ? "text-accent-foreground" : "group-hover:text-foreground"
              )} />
              {!collapsed && <span>{label}</span>}
            </Link>
          );
        })}
      </nav>

      <button
        onClick={() => setCollapsed(!collapsed)}
        className={cn(
          "flex items-center justify-center h-11 border-t border-border text-muted-foreground hover:text-foreground hover:bg-secondary/40 transition-all",
          collapsed ? "px-0" : "px-3"
        )}
      >
        <ChevronLeft className={cn("h-4 w-4 transition-transform duration-300", collapsed && "rotate-180")} />
      </button>
    </aside>
  );
}
