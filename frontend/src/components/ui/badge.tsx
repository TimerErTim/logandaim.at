import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * Badge variants. `success`, `error`, and `warning` carry meaning; they must never
 * be the only carrier of it (the text or an icon says the same thing), per the
 * accessibility bar.
 */
const badgeVariants = cva(
  "inline-flex items-center gap-1 rounded-md border px-2.5 py-0.5 text-xs font-medium",
  {
    variants: {
      variant: {
        default: "border-border bg-surface-muted text-foreground",
        success: "border-success/30 bg-success/10 text-success",
        error: "border-destructive/30 bg-destructive/10 text-destructive",
        warning: "border-warning/30 bg-warning/10 text-warning",
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
