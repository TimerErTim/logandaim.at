"use client";

import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A segmented control: a printed strip of cells sharing one contour, where exactly
 * one is marked as chosen. Used for picking a single option from a small set, so
 * every option is a real button with text, and the selection is carried by the fill
 * *and* by weight, never by colour alone.
 *
 * The cells share their borders rather than sitting apart: `-ml-px` collapses the
 * seam so the strip reads as one ruled block on the paper.
 */
const toggleGroupItemVariants = cva(
  [
    "relative -ml-px inline-flex min-h-11 items-center justify-center gap-2 px-3.5",
    "border border-border text-sm font-medium text-muted-foreground",
    "transition-colors first:ml-0 first:rounded-l-sm last:rounded-r-sm",
    "hover:bg-surface-muted hover:text-foreground",
    "focus-visible:z-10 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring",
    "disabled:pointer-events-none disabled:opacity-50",
    "data-[state=on]:z-10 data-[state=on]:border-primary data-[state=on]:bg-primary data-[state=on]:font-semibold data-[state=on]:text-primary-foreground",
  ].join(" "),
);

export function ToggleGroup({
  className,
  ...props
}: ComponentProps<typeof ToggleGroupPrimitive.Root>) {
  return (
    <ToggleGroupPrimitive.Root
      className={cn("flex w-fit flex-wrap items-center", className)}
      {...props}
    />
  );
}

export function ToggleGroupItem({
  className,
  ...props
}: ComponentProps<typeof ToggleGroupPrimitive.Item> &
  VariantProps<typeof toggleGroupItemVariants>) {
  return (
    <ToggleGroupPrimitive.Item
      className={cn(toggleGroupItemVariants(), className)}
      {...props}
    />
  );
}
