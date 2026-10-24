import { Search } from "lucide-react";
import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type SearchInputProps = InputHTMLAttributes<HTMLInputElement>;

export function SearchInput({ className, ...props }: SearchInputProps) {
  return (
    <div className="relative w-full max-w-xs">
      <Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
      <input
        type="search"
        className={cn(
          "h-10 w-full rounded-md border border-border bg-surface pr-3 pl-9 text-sm text-foreground transition-colors placeholder:text-muted-foreground focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50",
          className,
        )}
        {...props}
      />
    </div>
  );
}
