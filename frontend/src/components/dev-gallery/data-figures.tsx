"use client";

import type { ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * The drawing vocabulary of the identity: data is drawn like millimetre paper —
 * thin geometric lines on a faint grid, flat, with no fills or gradients.
 *
 * The pieces are deliberately generic (a grid, a ring plot, a series plot, a scale
 * bar, a legend) because they demonstrate *how* data is drawn, not *what* any one
 * screen shows. Everything is token-driven and carries no palette of its own. Each
 * drawing's values are repeated in text or in `aria-label`, so no reading depends
 * on the picture.
 */

/* ------------------------------------------------------------------ */
/* Millimetre grid                                                      */
/* ------------------------------------------------------------------ */

/**
 * The graph-paper ground: a fine 8px grid with a heavier line every fifth step,
 * exactly how a sheet of millimetre paper is printed. Sits behind a plot.
 */
export function GridPaper({ className }: { className?: string }) {
  return (
    <div
      aria-hidden="true"
      className={cn("pointer-events-none absolute inset-0", className)}
      style={{
        backgroundImage: [
          "linear-gradient(to right, var(--border-subtle) 1px, transparent 1px)",
          "linear-gradient(to bottom, var(--border-subtle) 1px, transparent 1px)",
          "linear-gradient(to right, var(--border-strong) 1px, transparent 1px)",
          "linear-gradient(to bottom, var(--border-strong) 1px, transparent 1px)",
        ].join(", "),
        backgroundSize: "8px 8px, 8px 8px, 40px 40px, 40px 40px",
        opacity: 0.5,
      }}
    />
  );
}

/* ------------------------------------------------------------------ */
/* Ring plot                                                            */
/* ------------------------------------------------------------------ */

export type RingMark = {
  id: string;
  /** Position in percent of the plot area. */
  x: number;
  y: number;
  /** Written out beside the drawing, so the label never rests on position alone. */
  label: string;
};

/**
 * Concentric rings with marks plotted on them: the thinnest possible way to show a
 * set of measurements against a centre. Rings are hairline strokes with no colour
 * wash; the inner ring is the only filled mark, so the centre reads at a glance.
 * Every mark is repeated in the accessible label.
 */
export function RingPlot({
  marks,
  className,
  label = "Ring plot",
}: {
  marks: RingMark[];
  className?: string;
  label?: string;
}) {
  const rings = [46, 36, 26, 16];

  return (
    <svg
      viewBox="0 0 100 100"
      role="img"
      aria-label={`${label}: ${marks.map((mark) => `${mark.id} ${mark.label}`).join(", ")}`}
      className={cn("aspect-square w-full max-w-48", className)}
    >
      {/* Crosshair, as printed on ruled paper. */}
      <line
        x1="50"
        y1="2"
        x2="50"
        y2="98"
        stroke="var(--border-subtle)"
        strokeWidth="0.4"
        strokeDasharray="2 2"
      />
      <line
        x1="2"
        y1="50"
        x2="98"
        y2="50"
        stroke="var(--border-subtle)"
        strokeWidth="0.4"
        strokeDasharray="2 2"
      />

      {rings.map((radius) => (
        <circle
          key={`ring-${radius}`}
          cx="50"
          cy="50"
          r={radius}
          fill="none"
          stroke="var(--border-strong)"
          strokeWidth="0.5"
        />
      ))}

      {/* The centre: the only filled mark, so it reads at a glance. */}
      <circle cx="50" cy="50" r="5" fill="var(--color-primary)" />

      {marks.map((mark) => (
        <g key={mark.id}>
          <circle
            cx={mark.x}
            cy={mark.y}
            r="3.2"
            fill="var(--bg-surface)"
            stroke="var(--semantic-impact)"
            strokeWidth="1.1"
          />
          <line
            x1={mark.x - 2.2}
            y1={mark.y}
            x2={mark.x + 2.2}
            y2={mark.y}
            stroke="var(--semantic-impact)"
            strokeWidth="0.9"
          />
          <line
            x1={mark.x}
            y1={mark.y - 2.2}
            x2={mark.x}
            y2={mark.y + 2.2}
            stroke="var(--semantic-impact)"
            strokeWidth="0.9"
          />
        </g>
      ))}
    </svg>
  );
}

/* ------------------------------------------------------------------ */
/* Series plot                                                          */
/* ------------------------------------------------------------------ */

export type SeriesPoint = {
  id: string;
  /** Position along the series in percent. */
  x: number;
  /** Height in the drawing, 0–100. */
  y: number;
  label: string;
  value: number;
};

/**
 * A series on graph paper: a thin line through the points, a hairline dropping to the
 * baseline, and the value in mono at each point. Drawn like a pencil plot in a
 * notebook, not like a dashboard chart.
 */
export function SeriesPlot({
  points,
  className,
  label,
}: {
  points: SeriesPoint[];
  className?: string;
  label: string;
}) {
  const path = points.map((point) => `${point.x},${100 - point.y}`).join(" ");

  return (
    <div className={cn("relative", className)}>
      <GridPaper />
      <svg
        viewBox="0 0 100 100"
        role="img"
        aria-label={`${label}: ${points.map((point) => `${point.label} ${point.value}`).join(", ")}`}
        preserveAspectRatio="none"
        className="relative h-28 w-full"
      >
        <polyline
          points={path}
          fill="none"
          stroke="var(--text-primary)"
          strokeWidth="1.2"
          vectorEffect="non-scaling-stroke"
        />
        {points.map((point) => (
          <g key={point.id}>
            <line
              x1={point.x}
              y1={100 - point.y}
              x2={point.x}
              y2="100"
              stroke="var(--border-subtle)"
              strokeWidth="0.4"
              strokeDasharray="1.5 1.5"
              vectorEffect="non-scaling-stroke"
            />
            <circle
              cx={point.x}
              cy={100 - point.y}
              r="1.1"
              fill="var(--bg-surface)"
              stroke="var(--text-primary)"
              strokeWidth="0.7"
              vectorEffect="non-scaling-stroke"
            />
            <text
              x={point.x}
              y={100 - point.y - 3}
              textAnchor="middle"
              fontSize="2.6"
              fontWeight="700"
              fill="var(--text-primary)"
              className="font-mono"
            >
              {point.value}
            </text>
          </g>
        ))}
      </svg>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* Scale bar                                                            */
/* ------------------------------------------------------------------ */

/**
 * A ruled scale: alternating filled and empty segments with the rule written out
 * underneath. The rule is text, so it reads without the drawing.
 */
export function ScaleBar({
  segments = 4,
  unitsPerSegment = 10,
  unit = "units",
  className,
}: {
  segments?: number;
  unitsPerSegment?: number;
  unit?: string;
  className?: string;
}) {
  const pieces = Array.from({ length: segments }, (_, index) => ({
    key: `segment-${index + 1}`,
    filled: index % 2 === 0,
  }));

  return (
    <div className={cn("space-y-1", className)}>
      <div className="flex h-2.5 items-stretch border border-ink">
        {pieces.map((piece, index) => (
          <div
            key={piece.key}
            className={cn(
              "flex-1",
              piece.filled ? "bg-ink" : "bg-transparent",
              index > 0 ? "border-l border-ink" : "",
            )}
          />
        ))}
      </div>
      <div className="flex justify-between font-mono text-[10px] text-secondary">
        <span>0</span>
        <span>
          one stroke = {unitsPerSegment} {unit}
        </span>
        <span>
          {segments * unitsPerSegment} {unit}
        </span>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* Legend                                                               */
/* ------------------------------------------------------------------ */

export type LegendEntry = {
  id: string;
  sample: ReactNode;
  text: string;
};

/**
 * A legend for the drawings: every mark with its meaning in words. A drawing is
 * commentary; this box is where its reading actually lives.
 */
export function Legend({
  title,
  entries,
  className,
}: {
  title: string;
  entries: LegendEntry[];
  className?: string;
}) {
  return (
    <div
      className={cn(
        "rounded-sm border border-subtle bg-surface p-3",
        className,
      )}
    >
      <p className="mb-2 text-[11px] font-semibold tracking-[0.08em] text-secondary uppercase">
        {title}
      </p>
      <ul className="space-y-1.5">
        {entries.map((entry) => (
          <li key={entry.id} className="flex items-center gap-2 text-xs">
            <span aria-hidden="true" className="shrink-0">
              {entry.sample}
            </span>
            <span>{entry.text}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
