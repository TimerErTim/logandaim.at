import { cn } from "@/lib/cn";

/**
 * A Pflock's colour is *caller data*, not a design token: the club stores a
 * `#rrggbb` hex value once and the same colour must look the same on the admin page,
 * on the public Parkour list, and in a captured Runde snapshot.
 *
 * The swatch therefore receives the value through the `--pflock-farbe` custom property
 * and paints it with `bg-[var(--pflock-farbe)]`. That is the one permitted inline style
 * in this codebase that is not a token name — the design rule documents it, and the
 * marker always carries the value as text beside it, so colour is never the only
 * signal.
 */
export function PflockSwatch({
  farbe,
  className,
}: {
  farbe: string;
  className?: string;
}) {
  return (
    <span
      aria-hidden="true"
      style={{ "--pflock-farbe": farbe } as React.CSSProperties}
      className={cn(
        "inline-block size-3 shrink-0 rounded-sm border border-control bg-[var(--pflock-farbe)]",
        className,
      )}
    />
  );
}

/**
 * The swatch plus the stored hex value, the pair that carries the meaning. The value
 * is a machine value, so it is set in mono: the colour itself may be anything, and
 * the text is what a reader can actually compare and quote.
 */
export function PflockMarker({ farbe }: { farbe: string }) {
  return (
    <span className="inline-flex items-center gap-2">
      <PflockSwatch farbe={farbe} />
      <span className="font-mono text-xs text-secondary">{farbe}</span>
    </span>
  );
}
