"use client";

import * as SeparatorPrimitive from "@radix-ui/react-separator";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * The ruled line: the standard section divider. A tick rule in `--contour`, like
 * the printed ruling between two blocks of a logbook page. Vertical separators stay
 * a thin hairline, because a tick rule only reads as a rule when it runs across.
 */
export function Separator({
  className,
  orientation = "horizontal",
  decorative = true,
  ...props
}: ComponentProps<typeof SeparatorPrimitive.Root>) {
  return (
    <SeparatorPrimitive.Root
      decorative={decorative}
      orientation={orientation}
      className={cn(
        orientation === "horizontal"
          ? "kontur-band h-1.5 w-full"
          : "h-full w-px bg-border",
        className,
      )}
      {...props}
    />
  );
}
