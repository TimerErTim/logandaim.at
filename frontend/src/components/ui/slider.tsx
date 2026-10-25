"use client";

import * as SliderPrimitive from "@radix-ui/react-slider";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * Range slider, drawn as a scale bar: a squared track with contour hairlines at
 * the quarter points and a compact square knob, so it reads as a measuring
 * instrument rather than a consumer toggle.
 *
 * The visible knob is 20px; `before` expands the hit area to 44px. The touch
 * minimum applies to the target, not to the drawn control, so a glove-friendly
 * slider does not have to look like a dumbbell.
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
        {/* Quarter hairlines: the scale is divided, not just filled. */}
        <span
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 flex justify-between"
        >
          {[0, 1, 2, 3, 4].map((teilung) => (
            <span
              key={`teilung-${teilung}`}
              className="w-px bg-contour"
              style={{ opacity: 0.8 }}
            />
          ))}
        </span>
      </SliderPrimitive.Track>
      <SliderPrimitive.Thumb className="relative block size-5 cursor-grab rounded-sm border-2 border-primary bg-surface before:absolute before:-inset-3.5 before:content-[''] focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:pointer-events-none disabled:cursor-not-allowed" />
    </SliderPrimitive.Root>
  );
}
