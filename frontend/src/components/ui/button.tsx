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
    "inline-flex shrink-0 items-center justify-center gap-2 rounded-sm",
    "text-sm font-medium whitespace-nowrap transition-colors",
    "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring",
    "disabled:cursor-not-allowed disabled:opacity-50",
    "[&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  ].join(" "),
  {
    variants: {
      variant: {
        primary:
          "bg-primary text-primary-foreground hover:bg-primary/90 active:bg-primary",
        secondary:
          "border border-border bg-surface text-foreground hover:border-muted-foreground/40 hover:bg-surface-muted",
        ghost: "text-foreground hover:bg-surface-muted",
        destructive:
          "border border-destructive/40 bg-transparent text-destructive hover:border-destructive hover:bg-destructive/10",
      },
      size: {
        sm: "h-8 px-2.5 text-[13px]",
        md: "h-10 px-4",
        // The full 44px touch minimum, drawn as such.
        lg: "h-11 px-6 text-base",
        // Drawn at `md` width so it aligns with its neighbours; the 44px hit area
        // comes from the `before` overlay rather than a larger drawn box.
        icon: "relative size-10 before:absolute before:-inset-0.5 before:content-['']",
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
