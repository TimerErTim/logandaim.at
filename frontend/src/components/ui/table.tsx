import type { ComponentProps, HTMLAttributes } from "react";

import { cn } from "@/lib/cn";

/**
 * A ruled ledger. Rows are separated by hairlines, never by zebra fills: the paper
 * already has a tone, and banding it twice fights the texture. The header is a
 * ruled strip in tracked small caps; the numbers align in tabular figures so a
 * column of scores reads down in a straight line.
 */
export function Table({
  className,
  ...props
}: HTMLAttributes<HTMLTableElement>) {
  return (
    <div className="overflow-x-auto rounded-md border border-subtle bg-surface">
      <table
        className={cn("w-full border-collapse text-sm", className)}
        {...props}
      />
    </div>
  );
}

/** The column headings: a ruled strip, tracked caps, secondary ink. */
export function TableHead({
  className,
  ...props
}: HTMLAttributes<HTMLTableSectionElement>) {
  return (
    <thead
      className={cn(
        "sticky top-0 z-10 border-b border-subtle bg-surface-muted text-left",
        "text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase",
        className,
      )}
      {...props}
    />
  );
}

export function TableBody({
  className,
  ...props
}: HTMLAttributes<HTMLTableSectionElement>) {
  return (
    <tbody className={cn("divide-y divide-subtle", className)} {...props} />
  );
}

// `ComponentProps` rather than `HTMLAttributes`: a sortable row needs its `ref`, and
// React 19 passes it as a plain prop.
export function TableRow({ className, ...props }: ComponentProps<"tr">) {
  return (
    <tr
      className={cn("transition-colors hover:bg-surface-muted/50", className)}
      {...props}
    />
  );
}

/**
 * A header cell. `scope="col"` is the default because that is the common case; a row
 * header passes `scope="row"` and gets the UI face back, since a row's label is
 * content rather than a column name.
 */
export function TableHeader({ className, ...props }: ComponentProps<"th">) {
  return (
    <th
      scope="col"
      className={cn("px-3 py-2 font-semibold", className)}
      {...props}
    />
  );
}

export function TableCell({ className, ...props }: ComponentProps<"td">) {
  return (
    <td className={cn("px-3 py-2.5 align-middle", className)} {...props} />
  );
}

/**
 * A row that reads as a form line: label on the left, value on the right, joined by
 * a dotted leader. Used for the metric pairs a logbook carries everywhere
 * (Distanz · 42 m, Wurfzahl · 6) instead of stacking them as cards.
 */
export function TableKeyValue({
  labels,
  className,
}: {
  labels: { id: string; label: string; value: string }[];
  className?: string;
}) {
  return (
    <dl className={cn("space-y-1.5 text-sm", className)}>
      {labels.map((entry) => (
        <div key={entry.id} className="flex items-baseline gap-2">
          <dt className="shrink-0 text-secondary">{entry.label}</dt>
          <span
            aria-hidden="true"
            className="leader mb-1 h-2 min-w-4 flex-1 opacity-60"
          />
          <dd className="shrink-0 font-medium tabular-nums">{entry.value}</dd>
        </div>
      ))}
    </dl>
  );
}
