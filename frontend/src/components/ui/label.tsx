import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * A form label, set like a printed form's field name: small, tracked caps in the
 * secondary ink. It names the control rather than competing with the value inside
 * it, so the eye lands on the field itself.
 */
export function Label({
  className,
  ...props
}: HTMLAttributes<HTMLSpanElement>) {
  return (
    <span
      className={cn(
        "text-[11px] font-semibold tracking-[0.07em] text-secondary uppercase",
        className,
      )}
      {...props}
    />
  );
}
