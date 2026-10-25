import type { HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * A paper sheet: a hairline plus a whisper of shadow, never a heavy border or an
 * elevation card. `papier` supplies the shadow; dark mode sets it to `none`, so the
 * hairline alone separates the sheet.
 */
export function Card({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "papier rounded-md border border-border bg-surface",
        className,
      )}
      {...props}
    />
  );
}

/**
 * The sheet header: content first, then a quiet ruled line along its bottom edge,
 * like the printed rule under a heading in a logbook.
 */
export function CardHeader({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={cn("p-6 pb-4", className)} {...props}>
      <div className="space-y-1">{children}</div>
      <div
        aria-hidden="true"
        className="kontur-band-quiet -mx-6 -mb-4 mt-4 h-px"
      />
    </div>
  );
}

export function CardTitle({
  className,
  ...props
}: HTMLAttributes<HTMLHeadingElement>) {
  return (
    <h2
      className={cn(
        "font-serif text-lg font-semibold tracking-tight",
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
    <p className={cn("text-sm text-muted-foreground", className)} {...props} />
  );
}

export function CardContent({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return <div className={cn("p-6 pt-0", className)} {...props} />;
}

export function CardFooter({
  className,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("flex items-center gap-3 p-6 pt-0", className)}
      {...props}
    />
  );
}
