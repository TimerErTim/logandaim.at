import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * A note in the margin. The variant is carried by a coloured left rule and a tinted
 * wash, not by a full coloured block: this is a logbook, and a message is an
 * annotation on the page rather than a banner shouting across it.
 *
 * `error` is announced to assistive technology; every variant also expects an icon,
 * so the meaning never rests on the tint alone.
 */
const alertVariants = cva(
  "flex items-start gap-3 rounded-sm border border-l-2 px-4 py-3 text-sm",
  {
    variants: {
      variant: {
        info: "border-border border-l-muted-foreground bg-surface-muted/60 text-foreground",
        success:
          "border-success/30 border-l-success bg-success/10 text-foreground",
        error:
          "border-destructive/30 border-l-destructive bg-destructive/10 text-foreground",
      },
    },
    defaultVariants: {
      variant: "info",
    },
  },
);

type AlertProps = HTMLAttributes<HTMLDivElement> &
  VariantProps<typeof alertVariants> & {
    /** Leading icon; gives the variant a second channel besides colour. */
    icon?: ReactNode;
  };

export function Alert({
  className,
  variant = "info",
  icon,
  children,
  ...props
}: AlertProps) {
  return (
    <div
      // Errors are announced to assistive technology, not only shown.
      role={variant === "error" ? "alert" : "status"}
      className={cn(alertVariants({ variant }), className)}
      {...props}
    >
      {icon ? (
        <span
          className={cn(
            "mt-0.5 shrink-0",
            variant === "error" && "text-destructive",
            variant === "success" && "text-success",
            variant === "info" && "text-muted-foreground",
          )}
        >
          {icon}
        </span>
      ) : null}
      <div className="min-w-0 flex-1">{children}</div>
    </div>
  );
}

export { alertVariants };
