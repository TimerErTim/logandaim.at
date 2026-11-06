"use client";

import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

export const TooltipProvider = TooltipPrimitive.Provider;
export const Tooltip = TooltipPrimitive.Root;
export const TooltipTrigger = TooltipPrimitive.Trigger;

/**
 * A tooltip is a pencil annotation: inverted ink on a small sheet, one line, no
 * arrow. It explains a control, so it must never be the only place an essential
 * instruction exists.
 */
export function TooltipContent({
  className,
  sideOffset = 6,
  ...props
}: ComponentProps<typeof TooltipPrimitive.Content>) {
  return (
    <TooltipPrimitive.Portal>
      <TooltipPrimitive.Content
        sideOffset={sideOffset}
        className={cn(
          "paper-overlay z-50 max-w-64 rounded-sm bg-ink px-2 py-1 text-xs text-canvas",
          className,
        )}
        {...props}
      />
    </TooltipPrimitive.Portal>
  );
}
