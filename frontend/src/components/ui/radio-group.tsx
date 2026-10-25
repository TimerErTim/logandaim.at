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
 * A check circle on paper: a heavy ring drawn as an inked mark, filled with a solid
 * disc that leaves a paper gap inside the ring. The double ring is what makes it read
 * as a stamped circle rather than a generic radio.
 *
 * The ring is `border-[3px]` on a 20px circle. At 2px on 18px it read as a default
 * browser radio, not an inked one: the stroke has to be heavy enough to be seen as a
 * drawn circle at arm's length, which is the whole point of the control's look.
 */
export function RadioGroupItem({
  className,
  ...props
}: ComponentProps<typeof RadioGroupPrimitive.Item>) {
  return (
    <RadioGroupPrimitive.Item
      className={cn(
        "size-5 shrink-0 rounded-full border-[3px] border-control bg-surface",
        "transition-colors",
        "hover:border-ink/60",
        "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "data-[state=checked]:border-primary",
        className,
      )}
      {...props}
    >
      <RadioGroupPrimitive.Indicator className="flex size-full items-center justify-center">
        {/* A filled disc, held off the ring by a paper gap: the "check circle" fill. */}
        <span className="size-full rounded-full border-[3px] border-surface bg-primary" />
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
