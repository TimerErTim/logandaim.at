import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * Badge variants, styled as a rubber stamp on the page: a hairline box, small
 * tracked caps, and a tint of the semantic colour rather than a solid fill. A stamp
 * is ink on paper, so it stays flat — no shadow, no pill.
 *
 * `success`, `error`, and `warning` carry meaning; they must never be the only
 * carrier of it (the text or an icon says the same thing), per the accessibility
 * bar.
 */
const badgeVariants = cva(
  "inline-flex items-center gap-1 rounded-sm border px-1.5 py-0.5 text-[11px] font-semibold tracking-[0.06em] uppercase",
  {
    variants: {
      variant: {
        default: "border-border bg-surface-muted text-foreground",
        success: "border-success/40 bg-success/10 text-success",
        error: "border-destructive/40 bg-destructive/10 text-destructive",
        warning: "border-warning/40 bg-warning/10 text-warning",
        outline: "border-border text-foreground",
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
