"use client";

import * as TabsPrimitive from "@radix-ui/react-tabs";
import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

export function Tabs({
  className,
  ...props
}: ComponentProps<typeof TabsPrimitive.Root>) {
  return (
    <TabsPrimitive.Root className={cn("space-y-4", className)} {...props} />
  );
}

/**
 * Tabs as a sheet index: the triggers sit on a hairline base, and the active one is
 * marked by a filled run of hunter green *and* a change of ink — the rule is the
 * primary signal, colour only reinforces it.
 */
export function TabsList({
  className,
  ...props
}: ComponentProps<typeof TabsPrimitive.List>) {
  return (
    <TabsPrimitive.List
      className={cn(
        "inline-flex min-h-11 items-stretch gap-1 border-b border-subtle",
        className,
      )}
      {...props}
    />
  );
}

export function TabsTrigger({
  className,
  ...props
}: ComponentProps<typeof TabsPrimitive.Trigger>) {
  return (
    <TabsPrimitive.Trigger
      className={cn(
        // 36px drawn; 44px target via a vertical overlay, since tab triggers are
        // adjacent and a horizontal one would overlap a neighbour's taps.
        "relative -mb-px inline-flex min-h-9 items-center justify-center border-b-2 border-transparent px-3",
        "before:absolute before:inset-x-0 before:-inset-y-[7px] before:content-['']",
        "text-sm font-medium text-secondary transition-colors hover:text-ink",
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
        "disabled:pointer-events-none disabled:opacity-50",
        "data-[state=active]:border-primary data-[state=active]:text-ink",
        className,
      )}
      {...props}
    />
  );
}

export function TabsContent({
  className,
  ...props
}: ComponentProps<typeof TabsPrimitive.Content>) {
  return (
    <TabsPrimitive.Content
      className={cn(
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
        className,
      )}
      {...props}
    />
  );
}
