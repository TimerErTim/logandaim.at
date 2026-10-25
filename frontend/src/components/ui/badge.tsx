import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * Badges are rubber stamps on the page: a hairline box, small tracked caps, and a
 * tint of the semantic colour rather than a solid fill. A stamp is ink on paper, so
 * it stays flat — no shadow, no pill, no gradient.
 *
 * `success`, `error`, and `warning` carry meaning; they must never be the only
 * carrier of it (the text says the same thing), per the accessibility bar.
 */
const badgeVariants = cva(
  "inline-flex items-center gap-1 rounded-sm border px-1.5 py-0.5 text-[10px] font-semibold tracking-[0.08em] uppercase",
  {
    variants: {
      variant: {
        default: "border-border bg-surface-muted text-foreground",
        success: "border-success/40 bg-success/10 text-success",
        error: "border-destructive/40 bg-destructive/10 text-destructive",
        warning: "border-warning/40 bg-warning/10 text-warning",
        // A stamp in the brand ink: used for the one state worth marking strongly.
        primary: "border-primary/40 bg-primary/10 text-primary",
        outline: "border-border bg-transparent text-foreground",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  },
);

type BadgeProps = HTMLAttributes<HTMLSpanElement> &
  VariantProps<typeof badgeVariants>;

export function Badge({
  className,
  variant = "default",
  ...props
}: BadgeProps) {
  return (
    <span className={cn(badgeVariants({ variant }), className)} {...props} />
  );
}

export { badgeVariants };
