"use client";

import { cn } from "@/lib/utils";
import { X } from "lucide-react";
import { forwardRef, useEffect, useRef, type HTMLAttributes } from "react";

const Dialog = ({ open, onClose, children }: { open: boolean; onClose: () => void; children: React.ReactNode }) => {
  const ref = useRef<HTMLDialogElement>(null);
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    if (open) el.showModal();
    else el.close();
  }, [open]);
  return (
    <dialog ref={ref} onClose={onClose} className="rounded-xl border border-border bg-card text-foreground backdrop:bg-black/60 p-0 max-w-lg w-full">
      {children}
    </dialog>
  );
};

const DialogContent = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(({ className, children, ...props }, ref) => (
  <div ref={ref} className={cn("relative p-6", className)} {...props}>
    {children}
  </div>
));
DialogContent.displayName = "DialogContent";

const DialogHeader = ({ className, ...props }: HTMLAttributes<HTMLDivElement>) => (
  <div className={cn("flex flex-col space-y-1.5 text-center sm:text-left mb-4", className)} {...props} />
);

const DialogTitle = ({ className, ...props }: HTMLAttributes<HTMLHeadingElement>) => (
  <h2 className={cn("text-lg font-semibold leading-none tracking-tight", className)} {...props} />
);

export { Dialog, DialogContent, DialogHeader, DialogTitle };
