import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import type { ButtonHTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * Button variants.
 *
 * `primary` is the single action a screen exists for. `secondary` is a neutral
 * alternative shown next to it. `ghost` is an inline action inside content (icon
 * buttons, table row actions). Destructive actions use `destructive` and never
 * `primary`.
 */
const buttonVariants = cva(
  "inline-flex shrink-0 items-center justify-center gap-2 rounded-sm font-medium whitespace-nowrap transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        primary: "bg-primary text-primary-foreground hover:bg-primary/90",
        secondary:
          "border border-border bg-surface text-foreground hover:bg-surface-muted",
        ghost: "text-foreground hover:bg-surface-muted",
        destructive:
          "bg-destructive text-white hover:bg-destructive/90 dark:text-background",
      },
      size: {
        sm: "h-8 px-3 text-sm",
        md: "h-10 px-4 text-sm",
        // Minimum touch target from the accessibility bar (WCAG 2.2 AA, 44x44).
        lg: "h-11 px-6 text-base",
        // Drawn at `md` height so it lines up with its neighbours; the 44px hit
        // area comes from `before`, not from a taller box.
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
