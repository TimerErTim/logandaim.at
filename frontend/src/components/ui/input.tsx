import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type InputProps = InputHTMLAttributes<HTMLInputElement>;

/**
 * A flat field with a fine contour. The paper is already the surface, so the input
 * does not need a fill of its own beyond it — the hairline is the field. On focus
 * the contour turns hunter green and a soft ring appears behind it; on hover the
 * line darkens slightly, so the control announces itself before it is touched.
 */
export function Input({ className, ...props }: InputProps) {
  return (
    <input
      className={cn(
        "h-10 w-full rounded-sm border border-border bg-surface px-3 text-sm text-foreground",
        "transition-colors placeholder:text-muted-foreground",
        "hover:border-muted-foreground/40",
        "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-ring",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "aria-invalid:border-destructive aria-invalid:focus-visible:outline-destructive",
        className,
      )}
      {...props}
    />
  );
}
