import { Search } from "lucide-react";
import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SearchInputProps = InputHTMLAttributes<HTMLInputElement>;

/**
 * A search field with its icon inside the contour. Same raised sheet and resting
 * contour as `Input`, with the leading icon inset far enough that typed text never
 * runs under it.
 */
export function SearchInput({ className, ...props }: SearchInputProps) {
  return (
    <div className="relative w-full max-w-xs">
      <Search
        aria-hidden="true"
        className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-secondary"
      />
      <input
        type="search"
        className={cn(
          "h-10 w-full rounded-sm border border-control bg-elevated pr-3 pl-9 text-sm text-ink",
          "transition-colors placeholder:text-muted",
          "hover:border-ink/40",
          "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-primary",
          "disabled:cursor-not-allowed disabled:bg-surface-muted disabled:text-muted",
          className,
        )}
        {...props}
      />
    </div>
  );
}
