"use client";

import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * Segmented control. Used for picking a Wertungszone: every option is a real
 * button with text and colour, so the selection never depends on colour alone.
 */
const toggleGroupItemVariants = cva(
  "inline-flex min-h-11 items-center justify-center gap-2 rounded-sm px-3 text-sm font-medium transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-surface-muted text-foreground hover:bg-border data-[state=on]:bg-primary data-[state=on]:text-primary-foreground",
        outline:
          "border border-border text-foreground hover:bg-surface-muted data-[state=on]:border-primary data-[state=on]:text-primary",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  },
);

export function ToggleGroup({
  className,
  ...props
}: ComponentProps<typeof ToggleGroupPrimitive.Root>) {
  return (
    <ToggleGroupPrimitive.Root
      className={cn(
        "flex w-fit flex-wrap items-center gap-1 rounded-sm",
        className,
      )}
      {...props}
    />
  );
}

export function ToggleGroupItem({
  className,
  variant = "default",
  ...props
}: ComponentProps<typeof ToggleGroupPrimitive.Item> &
  VariantProps<typeof toggleGroupItemVariants>) {
  return (
    <ToggleGroupPrimitive.Item
      className={cn(toggleGroupItemVariants({ variant }), className)}
      {...props}
    />
  );
}
