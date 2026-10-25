import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * A form label, set like a printed form's field name: small, tracked caps in the
 * secondary ink. It names the control rather than competing with the value inside
 * it, so the eye lands on the field itself.
 *
 * A real `<label>`, not a styled span: the name has to be programmatically attached
 * to its control. `Field` supplies the `htmlFor`.
 */
export function Label({ className, ...props }: ComponentProps<"label">) {
  return (
    // biome-ignore lint/a11y/noLabelWithoutControl: `htmlFor` is supplied by the caller (`Field`); the control is not a child of this component.
    <label
      className={cn(
        "text-[11px] font-semibold tracking-[0.07em] text-secondary uppercase",
        className,
      )}
      {...props}
    />
  );
}
