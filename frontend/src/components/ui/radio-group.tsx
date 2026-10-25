"use client";

import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";
import { type ComponentProps, useId } from "react";

import { cn } from "@/lib/cn";

export function RadioGroup({
  className,
  ...props
}: ComponentProps<typeof RadioGroupPrimitive.Root>) {
  return (
    <RadioGroupPrimitive.Root
      className={cn("grid gap-1", className)}
      {...props}
    />
  );
}

/**
 * A check circle on paper: a ring drawn with a strong contour, as an inked mark
 * rather than a hairline, and filled with ink — a solid disc inside the ring, not a
 * small dot swimming in a large hole. The double ring is what makes it read as a
 * stamped circle rather than a generic radio.
 *
 * Roundness is the meaning here, so it is one of the few round controls; the ring
 * never goes hairline, because an inked mark has weight.
 */
export function RadioGroupItem({
  className,
  ...props
}: ComponentProps<typeof RadioGroupPrimitive.Item>) {
  return (
    <RadioGroupPrimitive.Item
      className={cn(
        // A drawn circle: strong contour at rest so it is an inked mark, not a hairline.
        "size-[1.125rem] shrink-0 rounded-full border-2 border-control bg-surface",
        "transition-colors",
        "hover:border-ink/50",
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "data-[state=checked]:border-primary",
        className,
      )}
      {...props}
    >
      <RadioGroupPrimitive.Indicator className="flex size-full items-center justify-center">
        {/* A filled disc, ringed by the contour: the "check circle" fill. */}
        <span className="size-full rounded-full border-2 border-surface bg-primary" />
      </RadioGroupPrimitive.Indicator>
    </RadioGroupPrimitive.Item>
  );
}

/**
 * Row with a 44px touch target.
 *
 * The item is a `button`, which is labelable, so `htmlFor` on the row names it
 * without relying on implicit nesting.
 */
export function RadioGroupOption({
  value,
  label,
}: {
  value: string;
  label: string;
}) {
  const id = useId();

  return (
    <div className="flex min-h-11 items-center gap-3">
      <RadioGroupItem id={id} value={value} />
      <label htmlFor={id} className="cursor-pointer text-sm text-ink">
        {label}
      </label>
    </div>
  );
}
