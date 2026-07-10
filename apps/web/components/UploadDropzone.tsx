"use client";

import { cn } from "@/lib/utils";
import { Upload, AlertCircle } from "lucide-react";
import { useState, useCallback } from "react";

interface UploadDropzoneProps {
  onUpload: (file: File) => void;
  accept?: string;
  maxSize?: number;
}

const ALLOWED_EXTENSIONS = ["json", "log", "txt"];
const MAX_FILE_SIZE = 10 * 1024 * 1024;

function validateFile(file: File): string | null {
  const ext = file.name.split(".").pop()?.toLowerCase();
  if (!ext || !ALLOWED_EXTENSIONS.includes(ext)) {
    return `Invalid file type ".${ext}". Allowed: ${ALLOWED_EXTENSIONS.join(", ")}`;
  }
  if (file.size > MAX_FILE_SIZE) {
    const mb = (file.size / (1024 * 1024)).toFixed(1);
    return `File too large (${mb}MB). Maximum: ${MAX_FILE_SIZE / (1024 * 1024)}MB`;
  }
  if (file.size === 0) {
    return "File is empty";
  }
  return null;
}

export function UploadDropzone({
  onUpload,
  accept = ".json,.log,.txt",
  maxSize = MAX_FILE_SIZE,
}: UploadDropzoneProps) {
  const [dragging, setDragging] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleFile = useCallback(
    (file: File) => {
      setError(null);
      const validationError = validateFile(file);
      if (validationError) {
        setError(validationError);
        return;
      }
      onUpload(file);
    },
    [onUpload],
  );

  return (
    <div className="space-y-2">
      <div
        onDragOver={(e) => {
          e.preventDefault();
          setDragging(true);
        }}
        onDragLeave={() => setDragging(false)}
        onDrop={(e) => {
          e.preventDefault();
          setDragging(false);
          const f = e.dataTransfer.files[0];
          if (f) handleFile(f);
        }}
        className={cn(
          "border-2 border-dashed rounded-xl p-12 text-center cursor-pointer transition-all duration-300 group",
          dragging
            ? "border-accent-foreground/50 bg-accent/60 scale-[1.01]"
            : "border-border hover:border-accent-foreground/30 hover:bg-accent/30",
          error && "border-destructive/50 bg-destructive/5",
        )}
        onClick={() => {
          const input = document.createElement("input");
          input.type = "file";
          input.accept = accept;
          input.onchange = () => {
            if (input.files?.[0]) handleFile(input.files[0]);
          };
          input.click();
        }}
      >
        <div className={cn(
          "mx-auto w-14 h-14 rounded-xl flex items-center justify-center transition-all duration-300",
          dragging
            ? "bg-accent-foreground text-primary-foreground -translate-y-1"
            : "bg-secondary text-muted-foreground group-hover:bg-accent/50 group-hover:text-accent-foreground"
        )}>
          <Upload className="h-6 w-6" />
        </div>
        <p className="text-lg font-medium mt-4">Drop logs here or click to upload</p>
        <p className="text-sm text-muted-foreground mt-1">
          JSON, LOG, or TXT files (max {maxSize / (1024 * 1024)}MB)
        </p>
      </div>
      {error && (
        <div className="flex items-center gap-2 text-sm text-destructive">
          <AlertCircle className="h-4 w-4" />
          <span>{error}</span>
        </div>
      )}
    </div>
  );
}
