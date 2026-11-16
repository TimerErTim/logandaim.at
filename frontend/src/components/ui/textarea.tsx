import { type ComponentPropsWithRef, useEffect, useRef } from "react";

import { cn } from "@/lib/cn";

type TextareaProps = ComponentPropsWithRef<"textarea"> & {
  autoGrow?: boolean;
};

/**
 * A note field on the paper: same raised sheet and resting contour as `Input`,
 * with a 44px minimum height and optional content-driven growth.
 */
export function Textarea({
  className,
  rows,
  autoGrow = true,
  onChange,
  ref,
  value,
  defaultValue,
  ...props
}: TextareaProps) {
  const internalRef = useRef<HTMLTextAreaElement>(null);

  const grow = useRef((element: HTMLTextAreaElement) => {
    element.style.height = "auto";
    element.style.height = `${element.scrollHeight}px`;
  });

  useEffect(() => {
    if (!autoGrow || !internalRef.current) return;
    const nextValue = value ?? defaultValue;
    if (typeof nextValue === "string") internalRef.current.value = nextValue;
    grow.current(internalRef.current);
  }, [autoGrow, defaultValue, value]);

  return (
    <textarea
      ref={(element) => {
        internalRef.current = element;
        if (typeof ref === "function") ref(element);
        else if (ref) ref.current = element;
      }}
      rows={rows ?? 1}
      value={value}
      defaultValue={defaultValue}
      onChange={(event) => {
        onChange?.(event);
        if (autoGrow) grow.current(event.currentTarget);
      }}
      className={cn(
        "min-h-11 w-full rounded-sm border border-control bg-elevated px-3 py-2 text-sm text-ink",
        "transition-colors placeholder:text-muted",
        "hover:border-ink/40",
        "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-primary",
        "disabled:cursor-not-allowed disabled:bg-surface-muted disabled:text-muted",
        "aria-invalid:border-impact aria-invalid:focus-visible:outline-impact",
        "resize-none",
        className,
      )}
      {...props}
    />
  );
}
