"use client";

import * as SliderPrimitive from "@radix-ui/react-slider";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A range input drawn as a ruled measure: a divided track under a round bullet, so
 * it reads as a sliding cursor on a scale rather than a consumer toggle. The value
 * is a distance along a measure, which is why the track is divided and not a fill.
 *
 * The bullet is the one deliberately round control here, and rounding is the
 * meaning: a round bullet is a cursor that rides the rule, and a squared one would
 * read as a second tick. It keeps a strong contour — `border-2` — because a mark on
 * paper is inked, not hairline.
 *
 * The visible bullet is 20px; `before` expands the hit area to 44px. The touch
 * minimum applies to the target, not to the drawn control.
 */
export function Slider({
  className,
  ...props
}: ComponentProps<typeof SliderPrimitive.Root>) {
  return (
    <SliderPrimitive.Root
      className={cn(
        "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50",
        className,
      )}
      {...props}
    >
      <SliderPrimitive.Track className="relative h-1.5 w-full grow overflow-hidden rounded-sm bg-secondary">
        <SliderPrimitive.Range className="absolute h-full bg-primary" />
        <span
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 flex justify-between"
        >
          {[0, 1, 2, 3, 4].map((tick) => (
            <span key={`tick-${tick}`} className="w-px bg-subtle" />
          ))}
        </span>
      </SliderPrimitive.Track>
      <SliderPrimitive.Thumb
        className={cn(
          // A round, strongly inked bullet: the cursor rides the rule.
          "relative block size-5 cursor-grab rounded-full border-2 border-primary bg-surface",
          // The mark in the middle makes it read as a sighting dot, not a hole.
          "after:absolute after:inset-0 after:m-auto after:size-1.5 after:rounded-full",
          "after:bg-primary after:content-['']",
          "before:absolute before:-inset-[16px] before:content-['']",
          "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
          "active:cursor-grabbing disabled:pointer-events-none disabled:cursor-not-allowed",
        )}
      />
    </SliderPrimitive.Root>
  );
}
