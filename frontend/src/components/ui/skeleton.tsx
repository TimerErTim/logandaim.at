import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/** Placeholder for content that is loading; pair with `aria-busy` on the region. */
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
