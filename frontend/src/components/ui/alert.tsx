import { cva, type VariantProps } from "class-variance-authority";
import type { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * A note in the margin. The variant is carried by a coloured left rule and a tinted
 * wash, not by a full coloured block: this is a logbook, and a message is an
 * annotation on the page rather than a banner shouting across it.
 *
 * **The body text is always ink, never the semantic colour.** The prescribed amber
 * (#D97706) measures 3.08:1 on paper, so it is used as a rule, a wash, and an icon —
 * never for words. Reading colour on a warning would be a contrast failure; the word
 * itself is what carries the meaning anyway.
 *
 * `error` is announced to assistive technology; every variant also expects an icon,
 * so meaning never rests on the tint alone.
 */
const alertVariants = cva(
  "flex items-start gap-3 rounded-sm border border-l-2 px-4 py-3 text-sm text-ink",
  {
    variants: {
      variant: {
        info: "border-subtle border-l-info bg-info/10",
        // `bg-warning/12` keeps the wash light enough that ink on it clears AA.
        success: "border-success/30 border-l-success bg-success/10",
        warning: "border-warning/40 border-l-warning bg-warning/12",
        error: "border-impact/30 border-l-impact bg-impact/10",
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

const iconColour: Record<NonNullable<AlertProps["variant"]>, string> = {
  info: "text-info",
  success: "text-success",
  warning: "text-warning",
  error: "text-impact",
};

export function Alert({
  className,
  variant = "info",
  icon,
  children,
  ...props
}: AlertProps) {
  const tone = variant ?? "info";

  return (
    <div
      // Errors are announced to assistive technology, not only shown.
      role={tone === "error" ? "alert" : "status"}
      className={cn(alertVariants({ variant: tone }), className)}
      {...props}
    >
      {icon ? (
        <span className={cn("mt-0.5 shrink-0", iconColour[tone])}>{icon}</span>
      ) : null}
      <div className="min-w-0 flex-1">{children}</div>
    </div>
  );
}

export { alertVariants };
