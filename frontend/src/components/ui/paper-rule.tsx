"use client";

import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

/**
 * Paper furniture: the quiet marks that separate blocks on a page of a logbook
 * without becoming borders or ornaments.
 *
 * A logbook does not divide its pages with boxes and cards — it divides them with
 * ruling, hatch, a tear line, and a printed caption. These components are that
 * vocabulary, and they are deliberately bland: a reader should notice the content
 * change, not the mark that announced it.
 *
 * None of them is interactive, so none of them needs a touch target or an accessible
 * name. They are `aria-hidden` by default and take a `decorative={false}` only when
 * the mark carries meaning the text does not.
 */

/**
 * The hachure wall: diagonal `///////` strokes that rest the eye between two blocks.
 * This is the default way to separate major sections — a wall of strokes reads as a
 * change of subject without claiming to be a border around anything.
 *
 * `dense` doubles the stroke weight for a break that should hold more weight, e.g.
 * between the end of a page and its footer.
 */
export function HatchBand({
  className,
  dense = false,
  height = "md",
  ...props
}: ComponentProps<"div"> & {
  dense?: boolean;
  /** `sm` is a breath between blocks, `md` between sections, `lg` before a footer. */
  height?: "sm" | "md" | "lg";
}) {
  const heightClass = { sm: "h-4", md: "h-6", lg: "h-10" }[height];

  return (
    <div
      aria-hidden="true"
      className={cn(
        dense ? "hatch-dense" : "hatch",
        heightClass,
        "w-full",
        className,
      )}
      {...props}
    />
  );
}

/**
 * The tear line: a perforation drawn as short dashes, the way a bound page is
 * marked so it can be separated. Use it between a record and its annex, never as a
 * plain section rule — that is what `Separator` is for.
 */
export function Perforation({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      aria-hidden="true"
      className={cn("perforation h-px w-full", className)}
      {...props}
    />
  );
}

/**
 * The margin note: a captioned block that sits beside or under content the way a
 * handwritten remark sits in the margin of a page. It is a rule, a label in tracked
 * caps, and quiet body text — never a tinted box, so it never competes with an
 * `Alert`, which is reserved for something the reader must act on.
 */
export function MarginNote({
  label,
  children,
  className,
}: {
  label: string;
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <aside
      className={cn(
        "border-l border-subtle pl-3 text-xs text-secondary",
        className,
      )}
    >
      <p className="font-semibold tracking-[0.09em] uppercase">{label}</p>
      <div className="mt-1 text-sm text-ink">{children}</div>
    </aside>
  );
}

/**
 * The sheet number: the printed plate number in the margin, the way every page of a
 * bound book carries one. Mono and tabular so numbers line up column-wise down a
 * page. Decorative by default, since the surrounding heading usually says the same.
 */
export function SheetNumber({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <span
      className={cn(
        "font-mono text-[11px] text-secondary tabular-nums",
        className,
      )}
    >
      {children}
    </span>
  );
}

/**
 * The register line: a wide, low-contrast line of words in tracked caps, used as a
 * page's running head — the line a ledger prints across the top of every spread.
 *
 * It is deliberately not a divider. `Separator` separates; this one orients.
 */
export function RegisterLine({
  items,
  className,
}: {
  items: readonly string[];
  className?: string;
}) {
  return (
    <p
      className={cn(
        "flex flex-wrap items-center gap-x-3 gap-y-1 text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase",
        className,
      )}
    >
      {items.map((item, index) => (
        <span key={item} className="flex items-center gap-3">
          {index > 0 ? <span aria-hidden="true">·</span> : null}
          {item}
        </span>
      ))}
    </p>
  );
}

/**
 * The ruler edge: a graduated scale along the left of a block, so the block states
 * its own measure instead of borrowing an icon that means "measure". Decorative.
 */
export function RulerEdge({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      aria-hidden="true"
      className={cn("ruler-edge w-2 self-stretch", className)}
      {...props}
    />
  );
}

/**
 * A sheet with a turned-down corner: the page remembers it was read. Wraps content
 * that has been seen before rather than marking it new.
 *
 * The fold is decorative; never use it as the only sign of state.
 */
export function DogEaredSheet({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      className={cn("dog-ear relative overflow-hidden", className)}
      {...props}
    />
  );
}

/**
 * The stamp mark: a hairline box with tracked caps, rotated a degree or two, so it
 * reads as a stamp pressed onto the sheet rather than a badge in a layout.
 *
 * A real `Badge` already carries status; this one is for a *record-level* mark — a
 * verdict, a period, an archive note — and it is intentionally plainer, because it
 * annotates a document rather than a row in a table.
 */
export function StampMark({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <span
      className={cn(
        "inline-flex -rotate-1 items-center border border-strong px-1.5 py-0.5",
        "font-mono text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase",
        className,
      )}
    >
      {children}
    </span>
  );
}
