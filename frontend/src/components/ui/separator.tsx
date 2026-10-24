"use client";

import * as SeparatorPrimitive from "@radix-ui/react-separator";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * The contour band: the standard section divider of the map language. A quiet
 * hatch strip in `--contour`, not a hairline alone. Vertical separators stay a
 * thin rule, because a hatch strip only reads as a band when it runs across.
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
