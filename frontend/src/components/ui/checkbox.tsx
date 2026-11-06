import type { InputHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

type CheckboxProps = Omit<InputHTMLAttributes<HTMLInputElement>, "type"> & {
  /** Visible label; also the accessible name, so it must describe the effect. */
  label: string;
};

/**
 * A checkbox on the paper: a small square with a strong contour, filled with hunter
 * green when checked and ticked in the paper colour. The tick and the fill come from
 * the `checkbox-tick` class, which uses tokens rather than a baked-in white.
 *
 * The contour is `border-2` to match the radio's inked ring — an unchecked box is a
 * drawn mark, not a hairline, so a form's controls read as one family. The square
 * shape is the meaning: a box is "tick as many as apply", a circle is "choose one".
 *
 * The label wraps the input, so the whole row is a 44px target around an 18px box
 * and the accessible name comes from the visible text exactly once. The box itself
 * never grows: the drawn mark stays a mark, the row carries the touch minimum.
 */
export function Checkbox({ className, label, ...props }: CheckboxProps) {
  return (
    <label
      className={cn(
        // `min-h-11` on the wrapping label is the whole target: an `<input>` is a
        // replaced element and generates no `::before` box, so the overlay trick used
        // on `Button` would do nothing here.
        "relative flex min-h-11 w-full cursor-pointer items-center gap-3 text-sm text-ink",
        className,
      )}
    >
      <input
        type="checkbox"
        className={cn(
          "checkbox-tick size-[1.125rem] shrink-0 cursor-pointer appearance-none rounded-sm border-2 border-control bg-surface",
          "transition-colors",
          "hover:border-ink/50",
          "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
          "disabled:cursor-not-allowed disabled:opacity-50",
        )}
        {...props}
      />
      {label}
    </label>
  );
}
