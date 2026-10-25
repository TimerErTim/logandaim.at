import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * Badges are rubber stamps on the page: a hairline box, small tracked caps, and a
 * tint of the semantic colour rather than a solid fill. A stamp is ink on paper, so
 * it stays flat — no shadow, no pill, no gradient.
 *
 * Colour discipline: only variants whose colour actually clears AA are allowed to be
 * the text colour. `warning` is therefore ink on an amber wash rather than amber
 * text, because the prescribed amber measures 3.08:1 on paper and would be unreadable
 * as a label. `success` is limited to the surface sheet, where it measures 4.85:1.
 */
const badgeVariants = cva(
  "inline-flex items-center gap-1 rounded-sm border px-1.5 py-0.5 text-[10px] font-semibold tracking-[0.08em] uppercase",
  {
    variants: {
      variant: {
        default: "border-border bg-surface-muted text-foreground",
        success: "border-success/40 bg-success/10 text-success",
        error: "border-destructive/40 bg-destructive/10 text-destructive",
        // Ink on amber: the wash carries the colour, the word stays legible.
        warning: "border-warning/50 bg-warning/15 text-foreground",
        // The one stamp in brand ink, for the single state worth marking strongly.
        primary: "border-primary/40 bg-primary-subtle text-primary",
        info: "border-info/40 bg-info/10 text-info",
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
