import type { TextareaHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement>;

/**
 * A note field on the paper: same raised sheet and resting contour as `Input`,
 * taller and top-aligned.
 */
export function Textarea({ className, ...props }: TextareaProps) {
  return (
    <textarea
      className={cn(
        "min-h-20 w-full resize-y rounded-sm border border-border-strong bg-surface-elevated px-3 py-2 text-sm text-foreground",
        "transition-colors placeholder:text-placeholder",
        "hover:border-muted-foreground/60",
        "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-ring",
        "disabled:cursor-not-allowed disabled:bg-surface-muted disabled:text-placeholder",
        "aria-invalid:border-destructive aria-invalid:focus-visible:outline-destructive",
        className,
      )}
      {...props}
    />
  );
}
