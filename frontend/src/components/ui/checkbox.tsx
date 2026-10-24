import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type CheckboxProps = Omit<InputHTMLAttributes<HTMLInputElement>, "type"> & {
  /** Visible label; also the accessible name, so it must describe the effect. */
  label: string;
};

export function Checkbox({ className, label, ...props }: CheckboxProps) {
  return (
    // The label wraps the input: the accessible name comes from the visible text
    // once, and the whole row is a 44px touch target around a 16px box.
    <label
      className={cn(
        "flex min-h-11 cursor-pointer items-center gap-3 text-sm text-foreground",
        className,
      )}
    >
      <input
        type="checkbox"
        className="size-4 shrink-0 rounded border border-border accent-primary focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
        {...props}
      />
      {label}
    </label>
  );
}
