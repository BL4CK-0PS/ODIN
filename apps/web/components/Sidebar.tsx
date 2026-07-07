"use client";

import { cn } from "@/lib/utils";
import {
  LayoutDashboard,
  Search,
  Brain,
  Shield,
  Settings,
  Activity,
  ChevronLeft,
} from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useState } from "react";

const links = [
  { href: "/", label: "Dashboard", icon: LayoutDashboard },
  { href: "/investigations", label: "Investigations", icon: Activity },
  { href: "/threat-memory", label: "Threat Memory", icon: Brain },
  { href: "/knowledge-explorer", label: "Knowledge Explorer", icon: Shield },
  { href: "/search", label: "Search", icon: Search },
  { href: "/settings", label: "Settings", icon: Settings },
];

export function Sidebar() {
  const pathname = usePathname();
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside className={cn(
      "h-screen border-r border-border bg-card flex flex-col transition-all duration-200",
      collapsed ? "w-16" : "w-60"
    )}>
      <div className="flex items-center gap-3 p-4 border-b border-border">
        <Shield className="h-6 w-6 text-primary shrink-0" />
        {!collapsed && <span className="font-bold text-sm tracking-wide">ODIN</span>}
      </div>

      <nav className="flex-1 p-2 space-y-1">
        {links.map(({ href, label, icon: Icon }) => {
          const active = pathname === href || (href !== "/" && pathname.startsWith(href));
          return (
            <Link
              key={href}
              href={href}
              className={cn(
                "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-colors",
                active
                  ? "bg-primary/10 text-primary font-medium"
                  : "text-muted-foreground hover:bg-secondary hover:text-foreground"
              )}
            >
              <Icon className="h-4 w-4 shrink-0" />
              {!collapsed && <span>{label}</span>}
            </Link>
          );
        })}
      </nav>

      <button
        onClick={() => setCollapsed(!collapsed)}
        className="p-3 border-t border-border text-muted-foreground hover:text-foreground"
      >
        <ChevronLeft className={cn("h-4 w-4 transition-transform", collapsed && "rotate-180")} />
      </button>
    </aside>
  );
}
