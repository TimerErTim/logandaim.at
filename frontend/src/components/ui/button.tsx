import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import type { ButtonHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * Buttons, in the order the logbook uses them.
 *
 * `primary` is the one action a screen exists for: solid hunter green, paper-white
 * label. `secondary` is the quiet alternative next to it — a hairline that only
 * becomes present on hover, exactly as a printed form's second button behaves.
 * `ghost` is an inline action inside content. `destructive` is a hairline in the
 * failure red, never a solid red block: deleting a record is not the point of the
 * page.
 *
 * Corners are `rounded-sm` (4px). Nothing here is a pill; the paper is angular and
 * the shape is not decoration.
 */
const buttonVariants = cva(
  [
    // `relative` is part of the base because every size but `lg` reaches the touch
    // minimum through a `before` overlay, never by drawing taller than its row.
    "relative inline-flex shrink-0 items-center justify-center gap-2 rounded-sm",
    "text-sm font-medium whitespace-nowrap transition-colors",
    "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
    "disabled:cursor-not-allowed disabled:opacity-50",
    "[&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  ].join(" "),
  {
    variants: {
      variant: {
        primary:
          "bg-primary text-primary-foreground hover:bg-primary-hover active:bg-primary-hover",
        secondary:
          "border border-strong bg-elevated text-ink hover:border-ink/40 hover:bg-surface-muted",
        ghost: "text-ink hover:bg-surface-muted",
        destructive:
          "border border-impact/40 bg-transparent text-impact hover:border-impact hover:bg-impact/10",
      },
      size: {
        // The touch minimum is the *hit area*, never the drawn box. Two traps make a
        // naive `-inset-y` overlay a lie:
        //   1. `inset` resolves against the padding box, so a 1px border silently
        //      shrinks the pseudo box by 2px.
        //   2. `inset-y` alone leaves `left/right: auto`, so the box collapses to
        //      zero width in a flex container and nothing is hit-testable at all.
        // Hence `inset-x-0` plus an inset sized for a *contoured* control; the
        // horizontal inset stays 0 so a button never steals a neighbour's taps.
        sm: "h-8 px-2.5 text-[13px] before:absolute before:inset-x-0 before:-inset-y-[8px] before:content-['']",
        md: "h-10 px-4 before:absolute before:inset-x-0 before:-inset-y-[4px] before:content-['']",
        // The full 44px touch minimum, drawn as such.
        lg: "h-11 px-6 text-base",
        // Drawn at `md` size so it aligns with its neighbours; the hit area grows on
        // both axes, since an icon is not wide enough to reach 44 on its own.
        icon: "size-10 before:absolute before:-inset-[5px] before:content-['']",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  },
);

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> &
  VariantProps<typeof buttonVariants> & {
    /** Render the variant classes on a child element, for links. */
    asChild?: boolean;
  };

export function Button({
  className,
  variant = "primary",
  size = "md",
  type = "button",
  asChild = false,
  ...props
}: ButtonProps) {
  const Component = asChild ? Slot : "button";

  return (
    <Component
      {...(asChild ? {} : { type })}
      className={cn(buttonVariants({ variant, size }), className)}
      {...props}
    />
  );
}

export { buttonVariants };
