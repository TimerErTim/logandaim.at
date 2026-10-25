"use client";

import * as ProgressPrimitive from "@radix-ui/react-progress";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A measured band: the track is a ruled scale, the value a filled run along it.
 * Hairlines above the track mark the quarter points, the way a printed scale is
 * divided, so a value can be read against them instead of floating freely.
 *
 * The bar stays pill-shaped because here the shape is the meaning (a continuous
 * quantity along a track); everything it is drawn with comes from tokens.
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
        "relative h-2 w-full overflow-hidden rounded-full bg-surface-muted",
        className,
      )}
      {...props}
    >
      <ProgressPrimitive.Indicator
        className="h-full bg-primary transition-transform"
        style={{ transform: `translateX(-${100 - clamped}%)` }}
      />
      {/* Ruled quarter marks over the track, drawn from the contour hairline. */}
      <span
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 flex justify-between"
      >
        {[0, 1, 2, 3, 4].map((teilung) => (
          <span
            key={`teilung-${teilung}`}
            className="w-px bg-contour"
            style={{ opacity: 0.7 }}
          />
        ))}
      </span>
    </ProgressPrimitive.Root>
  );
}
