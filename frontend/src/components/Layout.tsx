import type { ReactNode } from "react";
import { Sidebar } from "./Sidebar";
import type { Page } from "@/lib/types";

export function Layout({ page, children }: { page: Page; children: ReactNode }) {
  return (
    <div className="flex h-screen">
      <Sidebar currentPage={page} />
      <main className="flex-1 overflow-auto p-6">{children}</main>
    </div>
  );
}
