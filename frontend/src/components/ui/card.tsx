import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * A sheet of paper. A hairline and a whisper of shadow lift it off the desk; in
 * dark mode the shadow is `none` and the hairline does the work. Nothing heavier:
 * the guide is explicit that there are no heavy borders and no leather.
 */
export function Card({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "paper rounded-md border border-border bg-surface",
        className,
      )}
      {...props}
    />
  );
}

/**
 * The sheet's title block: title and optional description, then a faint ruled line
 * across the full width. The rule is a hairline with ticks, like the printed fill
 * line of a form — one pixel, not a band.
 */
export function CardHeader({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={cn("px-6 pt-5 pb-4", className)} {...props}>
      <div className="space-y-1">{children}</div>
      <div
        aria-hidden="true"
        className="rule-ticks-quiet -mx-6 mt-4 h-px"
        style={{ transform: "translateY(0.5px)" }}
      />
    </div>
  );
}

/**
 * The title is the logbook's hand: serif, tight, and the only place the historical
 * character appears. It is deliberately one step larger than body text but never
 * a display size — this is a tool, not a magazine.
 */
export function CardTitle({
  className,
  ...props
}: HTMLAttributes<HTMLHeadingElement>) {
  return (
    <h2
      className={cn(
        "font-serif text-[17px] leading-snug font-semibold tracking-tight",
        className,
      )}
      {...props}
    />
  );
}

export function CardDescription({
  className,
  ...props
}: HTMLAttributes<HTMLParagraphElement>) {
  return (
    <p
      className={cn("text-[13px] text-muted-foreground", className)}
      {...props}
    />
  );
}

export function CardContent({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return <div className={cn("px-6 pt-5 pb-6", className)} {...props} />;
}

export function CardFooter({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "flex items-center gap-2 border-t border-border px-6 py-3",
        className,
      )}
      {...props}
    />
  );
}
