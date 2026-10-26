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
        "relative size-5 shrink-0 rounded-full border-[3px] border-control bg-surface",
        // 20 + 24 = 44 hit area on a button element; the drawn circle stays small
        // because the ink weight, not the size, is what makes it read as a mark.
        "before:absolute before:-inset-[16px] before:content-['']",
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
        <span className="pointer-events-none size-full rounded-full border-[3px] border-surface bg-primary" />
      </RadioGroupPrimitive.Indicator>
    </RadioGroupPrimitive.Item>
  );
}

/**
 * Row with a 44px touch target.
 *
 * The whole row is the label, so the target is the row rather than the 20px circle
 * plus its text: a label of 17px tall next to a small diagram is not a touch target,
 * whatever the control's own box measures. The item is a `button`, which is
 * labelable, so pointing `htmlFor` at it works without relying on implicit nesting.
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
      {/* The text is part of the target: a 17px label beside a 20px circle leaves
          the row short of the minimum even when the circle's own overlay passes. */}
      <label
        htmlFor={id}
        className="flex min-h-11 cursor-pointer items-center text-sm text-ink"
      >
        {label}
      </label>
    </div>
  );
}
