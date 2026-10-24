import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";
import { Label } from "./label";

type FieldProps = HTMLAttributes<HTMLDivElement> & {
  label: string;
  /** Helper text shown under the control when there is no error. */
  hint?: string;
  /** Validation message; replaces the hint and is announced. */
  error?: string;
};

export function Field({
  label,
  hint,
  error,
  className,
  children,
  ...props
}: FieldProps) {
  return (
    <div className={cn("space-y-1.5", className)} {...props}>
      <Label>{label}</Label>
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
