import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type InputProps = InputHTMLAttributes<HTMLInputElement>;

/**
 * A field on paper. It sits on `--bg-elevated` — the guide's raised sheet — so it
 * reads as a place to write without needing a fill of its own. The resting contour
 * is `--border-control`, which clears the 3:1 that WCAG 2.2 §1.4.11 requires of a
 * control boundary — `--border-strong` is a 1.58:1 contour and may never bound an
 * input. On focus the contour and ring turn hunter green; on hover it darkens.
 */
export function Input({ className, ...props }: InputProps) {
  return (
    <input
      className={cn(
        "h-10 w-full rounded-sm border border-control bg-elevated px-3 text-sm text-ink",
        "transition-colors placeholder:text-muted",
        "hover:border-ink/40",
        "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-primary",
        "disabled:cursor-not-allowed disabled:bg-surface-muted disabled:text-muted",
        "aria-invalid:border-impact aria-invalid:focus-visible:outline-impact",
        className,
      )}
      {...props}
    />
  );
}
