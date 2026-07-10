"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { ChevronRight } from "lucide-react";
import { cn } from "@/lib/utils";

const LABEL_MAP: Record<string, string> = {
  dashboard: "Dashboard",
  investigations: "Investigations",
  "threat-memory": "Threat Memory",
  "knowledge-explorer": "Knowledge Explorer",
  search: "Search",
  settings: "Settings",
};

export function Breadcrumbs() {
  const pathname = usePathname();

  if (pathname === "/") return null;

  const segments = pathname.split("/").filter(Boolean);

  const crumbs = segments.map((seg, i) => {
    const href = "/" + segments.slice(0, i + 1).join("/");
    const isLast = i === segments.length - 1;
    const label = LABEL_MAP[seg] || (seg.startsWith("inc-") ? seg.toUpperCase() : seg);

    return { href, label, isLast };
  });

  return (
    <nav className="flex items-center gap-1 text-xs text-muted-foreground mb-4">
      <Link
        href="/"
        className="hover:text-foreground transition-colors"
      >
        Home
      </Link>
      {crumbs.map((crumb) => (
        <span key={crumb.href} className="flex items-center gap-1">
          <ChevronRight className="h-3 w-3 opacity-40" />
          {crumb.isLast ? (
            <span className="text-foreground font-medium">{crumb.label}</span>
          ) : (
            <Link
              href={crumb.href}
              className="hover:text-foreground transition-colors"
            >
              {crumb.label}
            </Link>
          )}
        </span>
      ))}
    </nav>
  );
}
