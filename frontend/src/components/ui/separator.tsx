"use client";

import * as SeparatorPrimitive from "@radix-ui/react-separator";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * The section divider: a hairline with small ticks, like the printed rule between
 * two blocks of a logbook page. Vertical separators stay a plain hairline, because a
 * tick rule only reads as a rule when it runs across.
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
          ? "rule-ticks h-px w-full"
          : "h-full w-px bg-subtle",
        className,
      )}
      {...props}
    />
  );
}
