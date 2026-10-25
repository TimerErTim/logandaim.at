"use client";

import type { ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * Datenbilder — the logbook's data vocabulary.
 *
 * The identity says data is drawn like millimetre paper: thin geometric lines on a
 * faint grid, flat, with no fills or gradients. These are the pieces a bow log
 * actually needs: the target face with its hit, a measurement row on graph paper, a
 * ruled scale for distances, and a stamp legend.
 *
 * Everything below is drawn from tokens (`var(--…)`) and carries no palette of its
 * own. Every drawing is decoration for sighted users; each value it shows is
 * repeated in text, so no reading depends on the drawing.
 */

/* ------------------------------------------------------------------ */
/* Millimetre grid                                                      */
/* ------------------------------------------------------------------ */

/**
 * The graph-paper ground: a fine 8px grid with a heavier line every fifth step,
 * exactly how a sheet of millimetre paper is printed. Sits behind a plot.
 */
export function Millimetergrid({ className }: { className?: string }) {
  return (
    <div
      aria-hidden="true"
      className={cn("pointer-events-none absolute inset-0", className)}
      style={{
        backgroundImage: [
          "linear-gradient(to right, var(--contour) 1px, transparent 1px)",
          "linear-gradient(to bottom, var(--contour) 1px, transparent 1px)",
          "linear-gradient(to right, var(--border) 1px, transparent 1px)",
          "linear-gradient(to bottom, var(--border) 1px, transparent 1px)",
        ].join(", "),
        backgroundSize: "8px 8px, 8px 8px, 40px 40px, 40px 40px",
        opacity: 0.5,
      }}
    />
  );
}

/* ------------------------------------------------------------------ */
/* Target face                                                          */
/* ------------------------------------------------------------------ */

export type Treffer = {
  id: string;
  /** Where the arrow landed, in percent of the target face. */
  x: number;
  y: number;
  /** The Wertungszone the hit falls in, written out beside the drawing. */
  zone: string;
};

/**
 * The target face: concentric scoring rings on millimetre paper with the hits
 * plotted. A miss is drawn outside the rings rather than as a badge, because that is
 * what actually happened. Rings are thin strokes — no colour wash — and the legend
 * repeats every hit with its zone.
 */
export function Zielscheibe({
  treffer,
  className,
  label = "Trefferbild",
}: {
  treffer: Treffer[];
  className?: string;
  label?: string;
}) {
  const ringe = [
    { r: 46, zone: "4" },
    { r: 36, zone: "3" },
    { r: 26, zone: "2" },
    { r: 16, zone: "1" },
  ];

  return (
    <svg
      viewBox="0 0 100 100"
      role="img"
      aria-label={`${label}: ${treffer.map((t) => `${t.id} ${t.zone}`).join(", ")}`}
      className={cn("aspect-square w-full max-w-48", className)}
    >
      {/* Crosshair, as printed on a scoring card. */}
      <line
        x1="50"
        y1="2"
        x2="50"
        y2="98"
        stroke="var(--contour)"
        strokeWidth="0.4"
        strokeDasharray="2 2"
      />
      <line
        x1="2"
        y1="50"
        x2="98"
        y2="50"
        stroke="var(--contour)"
        strokeWidth="0.4"
        strokeDasharray="2 2"
      />

      {ringe.map((ring) => (
        <circle
          key={`ring-${ring.zone}`}
          cx="50"
          cy="50"
          r={ring.r}
          fill="none"
          stroke="var(--border)"
          strokeWidth="0.5"
        />
      ))}

      {/* The inner ten: the only filled mark, so the centre reads at a glance. */}
      <circle cx="50" cy="50" r="5" fill="var(--primary)" />

      {treffer.map((hit) => (
        <g key={hit.id}>
          <circle
            cx={hit.x}
            cy={hit.y}
            r="3.2"
            fill="var(--surface)"
            stroke="var(--destructive)"
            strokeWidth="1.1"
          />
          <line
            x1={hit.x - 2.2}
            y1={hit.y}
            x2={hit.x + 2.2}
            y2={hit.y}
            stroke="var(--destructive)"
            strokeWidth="0.9"
          />
          <line
            x1={hit.x}
            y1={hit.y - 2.2}
            x2={hit.x}
            y2={hit.y + 2.2}
            stroke="var(--destructive)"
            strokeWidth="0.9"
          />
        </g>
      ))}
    </svg>
  );
}

/* ------------------------------------------------------------------ */
/* Measurement row                                                      */
/* ------------------------------------------------------------------ */

export type Messpunkt = {
  id: string;
  /** Position along the series in percent. */
  x: number;
  /** Height in the drawing, 0–100. */
  y: number;
  label: string;
  wert: number;
};

/**
 * A measurement series on graph paper: a thin line through the points, a hairline
 * dropping to the baseline, and the value in mono at each point. Drawn like a
 * pencil plot in a notebook, not like a dashboard chart.
 */
export function Messreihe({
  punkte,
  className,
  label,
}: {
  punkte: Messpunkt[];
  className?: string;
  label: string;
}) {
  const pfad = punkte.map((p) => `${p.x},${100 - p.y}`).join(" ");

  return (
    <div className={cn("relative", className)}>
      <Millimetergrid />
      <svg
        viewBox="0 0 100 100"
        role="img"
        aria-label={`${label}: ${punkte.map((p) => `${p.label} ${p.wert}`).join(", ")}`}
        preserveAspectRatio="none"
        className="relative h-28 w-full"
      >
        <polyline
          points={pfad}
          fill="none"
          stroke="var(--foreground)"
          strokeWidth="1.2"
          vectorEffect="non-scaling-stroke"
        />
        {punkte.map((punkt) => (
          <g key={punkt.id}>
            <line
              x1={punkt.x}
              y1={100 - punkt.y}
              x2={punkt.x}
              y2="100"
              stroke="var(--contour)"
              strokeWidth="0.4"
              strokeDasharray="1.5 1.5"
              vectorEffect="non-scaling-stroke"
            />
            <circle
              cx={punkt.x}
              cy={100 - punkt.y}
              r="1.1"
              fill="var(--surface)"
              stroke="var(--foreground)"
              strokeWidth="0.7"
              vectorEffect="non-scaling-stroke"
            />
            <text
              x={punkt.x}
              y={100 - punkt.y - 3}
              textAnchor="middle"
              fontSize="2.6"
              fontWeight="700"
              fill="var(--foreground)"
              className="font-mono"
            >
              {punkt.wert}
            </text>
          </g>
        ))}
      </svg>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* Ruled scale                                                          */
/* ------------------------------------------------------------------ */

/**
 * A ruled scale for a distance: alternating filled and empty segments with the metre
 * rule written out underneath ("ein Strich = 10 m"). The rule is text, so it reads
 * without the drawing.
 */
export function Skalenleiste({
  teile = 4,
  meterProTeil = 10,
  className,
}: {
  teile?: number;
  meterProTeil?: number;
  className?: string;
}) {
  const stuecke = Array.from({ length: teile }, (_, i) => ({
    key: `teil-${i + 1}`,
    gefuellt: i % 2 === 0,
  }));

  return (
    <div className={cn("space-y-1", className)}>
      <div className="flex h-2.5 items-stretch border border-foreground">
        {stuecke.map((stueck, i) => (
          <div
            key={stueck.key}
            className={cn(
              "flex-1",
              stueck.gefuellt ? "bg-foreground" : "bg-transparent",
              i > 0 ? "border-l border-foreground" : "",
            )}
          />
        ))}
      </div>
      <div className="flex justify-between font-mono text-[10px] text-muted-foreground">
        <span>0</span>
        <span>ein Strich = {meterProTeil} m</span>
        <span>{teile * meterProTeil} m</span>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* Stamp legend                                                         */
/* ------------------------------------------------------------------ */

export type StempelEintrag = {
  id: string;
  probe: ReactNode;
  text: string;
};

/**
 * A legend for the drawings: every mark with its meaning in words. A drawing is
 * commentary; this box is where its reading actually lives.
 */
export function Stempelledende({
  titel,
  eintraege,
  className,
}: {
  titel: string;
  eintraege: StempelEintrag[];
  className?: string;
}) {
  return (
    <div
      className={cn(
        "rounded-sm border border-border bg-surface p-3",
        className,
      )}
    >
      <p className="mb-2 text-[11px] font-semibold tracking-[0.08em] text-muted-foreground uppercase">
        {titel}
      </p>
      <ul className="space-y-1.5">
        {eintraege.map((eintrag) => (
          <li key={eintrag.id} className="flex items-center gap-2 text-xs">
            <span aria-hidden="true" className="shrink-0">
              {eintrag.probe}
            </span>
            <span>{eintrag.text}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
