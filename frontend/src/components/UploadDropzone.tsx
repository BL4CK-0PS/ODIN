import { cn } from "@/lib/utils";
import { Upload } from "lucide-react";
import { useState } from "react";

export function UploadDropzone({ onUpload }: { onUpload: (file: File) => void }) {
  const [dragging, setDragging] = useState(false);
  return (
    <div
      onDragOver={(e) => { e.preventDefault(); setDragging(true); }}
      onDragLeave={() => setDragging(false)}
      onDrop={(e) => { e.preventDefault(); setDragging(false); const f = e.dataTransfer.files[0]; if (f) onUpload(f); }}
      onClick={() => { const input = document.createElement("input"); input.type = "file"; input.accept = ".json,.log,.txt"; input.onchange = () => input.files?.[0] && onUpload(input.files[0]); input.click(); }}
      className={cn("border-2 border-dashed rounded-xl p-12 text-center cursor-pointer transition-colors", dragging ? "border-primary bg-primary/5" : "border-border hover:border-primary/50")}
    >
      <Upload className="mx-auto h-10 w-10 text-muted-foreground mb-4" />
      <p className="text-lg font-medium">Drop logs here or click to upload</p>
      <p className="text-sm text-muted-foreground mt-1">JSON, LOG, or TXT files</p>
    </div>
  );
}
