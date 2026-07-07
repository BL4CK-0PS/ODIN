import { cn } from "@/lib/utils";
import type { ReactNode } from "react";

export function ScrollArea({ children, className }: { children?: ReactNode; className?: string }) {
  return <div className={cn("relative overflow-auto", className)}>{children}</div>;
}
