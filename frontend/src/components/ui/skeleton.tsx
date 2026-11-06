import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * A placeholder that keeps the shape of what is coming, so the page does not jump
 * when it arrives. It pulses on `--surface-muted` — a faint printed block, not a
 * shimmer — and is hidden from the accessibility tree; the region around it carries
 * `aria-busy`.
 */
export function Skeleton({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      aria-hidden="true"
      className={cn("animate-pulse rounded-sm bg-surface-muted", className)}
      {...props}
    />
  );
}
