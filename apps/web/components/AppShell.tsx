"use client";

import { useState, useEffect, type ReactNode } from "react";
import { Sidebar } from "@/components/Sidebar";
import { Breadcrumbs } from "@/components/Breadcrumbs";
import { Menu, X } from "lucide-react";
import { cn } from "@/lib/utils";
import { usePathname } from "next/navigation";

export function AppShell({ children }: { children: ReactNode }) {
  const [mobileOpen, setMobileOpen] = useState(false);
  const pathname = usePathname();

  // close mobile nav on route change
  useEffect(() => {
    setMobileOpen(false);
  }, [pathname]);

  return (
    <div className="flex h-screen overflow-hidden">
      {/* mobile hamburger */}
      <button
        onClick={() => setMobileOpen(!mobileOpen)}
        className="fixed top-4 left-4 z-50 flex items-center justify-center w-9 h-9 rounded-lg bg-card border border-border shadow-medium lg:hidden"
        aria-label="Toggle navigation"
      >
        {mobileOpen ? <X className="h-4 w-4" /> : <Menu className="h-4 w-4" />}
      </button>

      {/* mobile backdrop */}
      {mobileOpen && (
        <div
          className="fixed inset-0 z-30 bg-black/50 backdrop-blur-sm lg:hidden"
          onClick={() => setMobileOpen(false)}
        />
      )}

      {/* sidebar - desktop always visible, mobile slide-over */}
      <div className={cn(
        "fixed inset-y-0 left-0 z-40 lg:relative lg:z-auto transition-transform duration-300 lg:translate-x-0",
        mobileOpen ? "translate-x-0" : "-translate-x-full"
      )}>
        <Sidebar onNavigate={() => setMobileOpen(false)} />
      </div>

      {/* main content */}
      <main className="flex-1 overflow-auto">
        <div className="p-6 pt-16 lg:pt-6 page-enter">
          <Breadcrumbs />
          {children}
        </div>
      </main>
    </div>
  );
}
