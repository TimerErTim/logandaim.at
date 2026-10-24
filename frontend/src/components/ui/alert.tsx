import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/lib/cn";

const alertVariants = cva(
  "flex items-start gap-3 rounded-lg border px-4 py-3 text-sm",
  {
    variants: {
      variant: {
        info: "border-border bg-surface-muted text-foreground",
        success: "border-success/30 bg-success/10 text-foreground",
        error: "border-destructive/30 bg-destructive/10 text-foreground",
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
      {icon ? <span className="mt-0.5 shrink-0">{icon}</span> : null}
      <div className="min-w-0 flex-1">{children}</div>
    </div>
  );
}

export { alertVariants };
