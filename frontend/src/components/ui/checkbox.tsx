import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type CheckboxProps = Omit<InputHTMLAttributes<HTMLInputElement>, "type"> & {
  /** Visible label; also the accessible name, so it must describe the effect. */
  label: string;
};

/**
 * A checkbox on the paper: a small square with a hairline, filled with hunter green
 * when checked and ticked in the paper colour. The tick and the fill come from the
 * `checkbox-tick` class, which uses tokens rather than a baked-in white.
 *
 * The label wraps the input, so the whole row is a 44px target around a 16px box
 * and the accessible name comes from the visible text exactly once.
 */
export function Checkbox({ className, label, ...props }: CheckboxProps) {
  return (
    <label
      className={cn(
        "flex min-h-11 cursor-pointer items-center gap-3 text-sm text-ink",
        className,
      )}
    >
      <input
        type="checkbox"
        className={cn(
          "checkbox-tick size-4 shrink-0 cursor-pointer appearance-none rounded-sm border border-subtle bg-surface",
          "transition-colors",
          "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
          "disabled:cursor-not-allowed disabled:opacity-50",
        )}
        {...props}
      />
      {label}
    </label>
  );
}
