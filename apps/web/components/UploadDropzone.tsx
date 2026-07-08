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
const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB

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
          "border-2 border-dashed rounded-xl p-12 text-center cursor-pointer transition-colors",
          dragging
            ? "border-primary bg-primary/5"
            : "border-border hover:border-primary/50",
          error && "border-red-400/50 bg-red-500/5",
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
        <Upload className="mx-auto h-10 w-10 text-muted-foreground mb-4" />
        <p className="text-lg font-medium">Drop logs here or click to upload</p>
        <p className="text-sm text-muted-foreground mt-1">
          JSON, LOG, or TXT files (max {maxSize / (1024 * 1024)}MB)
        </p>
      </div>
      {error && (
        <div className="flex items-center gap-2 text-sm text-red-400">
          <AlertCircle className="h-4 w-4" />
          <span>{error}</span>
        </div>
      )}
    </div>
  );
}
