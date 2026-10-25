"use client";

import * as ProgressPrimitive from "@radix-ui/react-progress";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A measured band: the track is a ruled scale, the value a filled run along it.
 * Small hairlines at the quarter points divide the track like a printed ruler, so
 * a value can be read against the ticks instead of floating freely.
 *
 * The bar keeps its pill shape because here the shape is the meaning (a continuous
 * quantity along a track); every colour it is drawn with comes from tokens.
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
      {/* Ruled quarter marks: a measured scale, matched to the grid style. */}
      <span
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 flex justify-between"
      >
        {[0, 1, 2, 3, 4].map((teilung) => (
          <span
            key={`teilung-${teilung}`}
            className="w-px bg-contour"
            style={{ opacity: 0.9 }}
          />
        ))}
      </span>
    </ProgressPrimitive.Root>
  );
}
