import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";
import { Label } from "./label";

type FieldProps = HTMLAttributes<HTMLDivElement> & {
  label: string;
  /** Helper text shown under the control when there is no error. */
  hint?: string;
  /** Validation message; replaces the hint and is announced. */
  error?: string;
  /** Marks optional fields, as a printed form does. */
  optional?: boolean;
};

/**
 * One labelled field on the form. The label sits above the control in small caps
 * with letter spacing, the way a printed form labels a line; the hint sits below in
 * the secondary ink and gives way to the error, which is announced.
 */
export function Field({
  label,
  hint,
  error,
  optional = false,
  className,
  children,
  ...props
}: FieldProps) {
  return (
    <div className={cn("space-y-1.5", className)} {...props}>
      <span className="flex items-baseline justify-between gap-2">
        <Label>{label}</Label>
        {optional ? (
          <span className="text-[11px] text-muted-foreground">optional</span>
        ) : null}
      </span>
      {children}
      {error ? (
        <p role="alert" className="text-xs text-destructive">
          {error}
        </p>
      ) : hint ? (
        <p className="text-xs text-muted-foreground">{hint}</p>
      ) : null}
    </div>
  );
}
