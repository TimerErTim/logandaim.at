import type { SelectHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SelectProps = SelectHTMLAttributes<HTMLSelectElement>;

/**
 * Native select, styled to match `Input`.
 *
 * Kept native on purpose: it works with gloves, one-handed, and the platform
 * picker is the accessible control archers already know. A custom listbox would
 * trade that for styling freedom this product does not need.
 */
export function Select({ className, ...props }: SelectProps) {
  return (
    <select
      className={cn(
        "h-10 w-full rounded-md border border-border bg-surface px-3 text-sm text-foreground transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50",
        className,
      )}
      {...props}
    />
  );
}
