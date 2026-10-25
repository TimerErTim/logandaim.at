import type { SelectHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SelectProps = SelectHTMLAttributes<HTMLSelectElement>;

/**
 * Native select, styled to match `Input`.
 *
 * Kept native on purpose: it works with gloves, one-handed, and the platform picker
 * is the control users already know. A custom listbox would trade that for styling
 * freedom this product does not need. The chevron comes from the `select-chevron`
 * class, drawn as a mask so it inherits `currentColor` and flips with the theme
 * instead of hardcoding a stroke colour.
 */
export function Select({ className, ...props }: SelectProps) {
  return (
    <select
      className={cn(
        "select-chevron h-10 w-full appearance-none rounded-sm border border-border bg-surface py-0 pr-9 pl-3 text-sm text-foreground",
        "transition-colors",
        "hover:border-muted-foreground/40",
        "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-ring",
        "disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      {...props}
    />
  );
}
