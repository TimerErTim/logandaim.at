"use client";

import * as ProgressPrimitive from "@radix-ui/react-progress";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A measured band. The track is a ruled measure — hairlines at every fifth — and
 * the value is a filled run of hunter green along it, so a partial value can be read
 * against the ticks rather than guessed. The bar stays flat: no gradient, no gloss.
 *
 * It is not a pill. The guide reserves round shapes for things whose roundness means
 * something, and a progress track does not: `rounded-sm` keeps it a printed measure.
 */
export function Progress({
  className,
  value = 0,
  ...props
}: ComponentProps<typeof ProgressPrimitive.Root>) {
  const clamped = Math.min(100, Math.max(0, value ?? 0));

  return (
    <ProgressPrimitive.Root
      value={clamped}
      className={cn(
        "relative h-2 w-full overflow-hidden rounded-sm bg-surface-muted",
        className,
      )}
      {...props}
    >
      <ProgressPrimitive.Indicator
        className="h-full bg-primary transition-transform"
        style={{ transform: `translateX(-${100 - clamped}%)` }}
      />
      <span
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 flex justify-between"
      >
        {[0, 1, 2, 3, 4].map((ticks) => (
          <span key={`tick-${ticks}`} className="w-px bg-contour" />
        ))}
      </span>
    </ProgressPrimitive.Root>
  );
}
