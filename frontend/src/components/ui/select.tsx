import type { SelectHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SelectProps = SelectHTMLAttributes<HTMLSelectElement>;

/**
 * Native select, styled to match `Input`, on the same raised sheet.
 *
 * Kept native on purpose: the platform picker is the control users already know, and
 * it survives touch and one-handed use. A custom listbox would trade that for
 * styling freedom this system does not need. The chevron comes from the
 * `select-chevron` class, drawn as a mask so it inherits `currentColor` and flips
 * with the theme.
 */
export function Select({ className, ...props }: SelectProps) {
  return (
    <select
      className={cn(
        "select-chevron h-10 w-full appearance-none rounded-sm border border-control bg-elevated py-0 pr-9 pl-3 text-sm text-ink",
        "transition-colors",
        "hover:border-ink/40",
        "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-primary",
        "disabled:cursor-not-allowed disabled:bg-surface-muted disabled:text-muted",
        className,
      )}
      {...props}
    />
  );
}
