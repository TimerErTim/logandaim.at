"use client";

import * as SwitchPrimitive from "@radix-ui/react-switch";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A switch is allowed to be a pill: the shape carries "either end of a track", which
 * a square cannot say. Its contour is `border-2` so the track is an inked mark like
 * the radio ring and the checkbox box — the whole control family has weight.
 *
 * The thumb is a paper disc with a strong contour rather than a shadow cast onto the
 * track, so it stays flat like everything else and reads as a sliding seal.
 */
export function Switch({
  className,
  ...props
}: ComponentProps<typeof SwitchPrimitive.Root>) {
  return (
    <SwitchPrimitive.Root
      className={cn(
        "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-control",
        "transition-colors",
        "data-[state=checked]:border-primary data-[state=checked]:bg-primary",
        "data-[state=unchecked]:bg-surface-muted",
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
        "disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      {...props}
    >
      <SwitchPrimitive.Thumb
        className={cn(
          "pointer-events-none block size-4 rounded-full border-2 border-control bg-surface",
          "data-[state=checked]:border-primary",
          "transition-transform",
          "data-[state=checked]:translate-x-[1.375rem] data-[state=unchecked]:translate-x-0.5",
        )}
      />
    </SwitchPrimitive.Root>
  );
}
