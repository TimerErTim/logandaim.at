"use client";

import type { ReactNode } from "react";

import { cn } from "@/lib/cn";

/**
 * Kartographische Bausteine.
 *
 * The map language only earns its name if it does real cartography instead of
 * drawing map *icons*. These are the genuine instruments of a topographic sheet:
 * contour lines with index contours, an elevation profile, a compass rose, a
 * subdivided scale bar, a graticule, and a neatline.
 *
 * Everything below is built from tokens (`var(--…)`) and carries no palette of
 * its own. All drawing is decoration for sighted users; every figure that carries
 * a value is repeated in text, so no reading depends on the drawing.
 */

/* ------------------------------------------------------------------ */
/* Contour lines                                                        */
/* ------------------------------------------------------------------ */

/** A peak on the sheet: a hill in the terrain, i.e. a strong target or round. */
export type Gipfel = {
  id: string;
  /** Position in percent of the sheet. */
  x: number;
  y: number;
  /** Radius of the innermost contour, in percent of the sheet width. */
  radius: number;
  /** Number of contour rings drawn around this peak. */
  ringe: number;
  /** Value printed on the index contour, for example "284". */
  hoehe: string;
};

/**
 * A closed contour ring. The radius is perturbed by two deterministic harmonics so
 * rings read as surveyed terrain rather than as concentric circles; the same seed
 * reproduces the same shape every render.
 */
function konturPfad(
  cx: number,
  cy: number,
  r: number,
  seed: number,
  schritte = 72,
): string {
  const punkte: string[] = [];
  for (let i = 0; i < schritte; i += 1) {
    const a = (i / schritte) * Math.PI * 2;
    const wogen =
      1 +
      0.14 * Math.sin(a * 3 + seed) +
      0.08 * Math.sin(a * 5 + seed * 1.7) +
      0.04 * Math.sin(a * 8 + seed * 2.3);
    punkte.push(
      `${(cx + Math.cos(a) * r * wogen).toFixed(2)},${(cy + Math.sin(a) * r * wogen).toFixed(2)}`,
    );
  }
  return `M${punkte.join("L")}Z`;
}

/**
 * The terrain of a sheet. `Gipfel` become nested contour rings; every fifth ring
 * is an index contour drawn heavier and labelled in mono, which is how real
 * topographic sheets stay readable at a glance.
 */
export function Konturlinien({
  gipfel,
  className,
  label = "Höhenlinien",
}: {
  gipfel: Gipfel[];
  className?: string;
  label?: string;
}) {
  return (
    <svg
      viewBox="0 0 100 62"
      role="img"
      aria-label={`${label}: ${gipfel.map((g) => g.hoehe).join(", ")}`}
      preserveAspectRatio="none"
      className={cn("h-full w-full", className)}
    >
      {gipfel.map((berg) => {
        const ringe = Array.from({ length: berg.ringe }, (_, i) => {
          const index = i + 1;
          const radius = berg.radius * (1 + i * 0.62);
          return {
            key: `${berg.id}-${index}`,
            pfad: konturPfad(
              berg.x,
              berg.y,
              radius,
              index * 1.31 + berg.x * 0.07,
            ),
            // Every fifth ring is an index contour, the cartographer's emphasis.
            index: index % 5 === 0,
            hoehe: berg.hoehe,
          };
        });

        return (
          <g key={berg.id}>
            {ringe.map((ring) => (
              <path
                key={ring.key}
                d={ring.pfad}
                fill="none"
                stroke="var(--contour)"
                strokeWidth={ring.index ? 0.45 : 0.2}
                vectorEffect="non-scaling-stroke"
              />
            ))}
            <circle cx={berg.x} cy={berg.y} r="0.55" fill="var(--foreground)" />
            <text
              x={berg.x + 1.4}
              y={berg.y + 0.9}
              fontSize="1.7"
              fontWeight="700"
              fill="var(--foreground)"
              className="font-mono"
            >
              {berg.hoehe}
            </text>
          </g>
        );
      })}
    </svg>
  );
}

/* ------------------------------------------------------------------ */
/* Graticule and sheet furniture                                        */
/* ------------------------------------------------------------------ */

/** The fine graticule behind a sheet, with edge labels in mono. */
export function Gradnetz({ className }: { className?: string }) {
  return (
    <div
      aria-hidden="true"
      className={cn("pointer-events-none absolute inset-0", className)}
      style={{
        backgroundImage:
          "linear-gradient(to right, var(--contour) 1px, transparent 1px), linear-gradient(to bottom, var(--contour) 1px, transparent 1px)",
        backgroundSize: "24px 24px",
        opacity: 0.35,
      }}
    />
  );
}

/**
 * The neatline: the framed edge of a map sheet. A double border with mono
 * coordinates along the top and left, exactly how a printed sheet is trimmed.
 */
export function Kartenrand({
  kopf,
  fuss,
  children,
  className,
}: {
  /** Sheet reference, for example "Blatt 07/32". */
  kopf: string;
  /** Scale and revision line, for example "1:2 000 · Stand 25.10.2026". */
  fuss: string;
  children: ReactNode;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "rounded-md border border-border bg-surface p-1.5",
        className,
      )}
    >
      <div className="flex items-center justify-between px-1.5 pb-1 font-mono text-[10px] tracking-wide text-muted-foreground">
        <span>{kopf}</span>
        <span aria-hidden="true">N ↑</span>
      </div>
      <div className="relative overflow-hidden rounded-sm border border-border">
        {children}
      </div>
      <div className="flex items-center justify-between px-1.5 pt-1 font-mono text-[10px] tracking-wide text-muted-foreground">
        <span>{fuss}</span>
        <span aria-hidden="true">logandaim.at</span>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* Scale bar                                                            */
/* ------------------------------------------------------------------ */

/**
 * A true cartographic scale bar: alternating filled and empty segments, with the
 * leftmost segment subdivided into five so a distance can be read to the
 * subdivision. The metre rule is written out underneath, as on a printed sheet.
 */
export function Massstabsleiste({
  teile = 4,
  meterProTeil = 10,
  teilung = 5,
  className,
}: {
  teile?: number;
  meterProTeil?: number;
  /** Subdivisions of the leftmost segment. */
  teilung?: number;
  className?: string;
}) {
  const rechts = Array.from({ length: teile }, (_, i) => ({
    key: `teil-${i + 1}`,
    gefuellt: i % 2 === 0,
  }));

  return (
    <div className={cn("space-y-1", className)}>
      <div className="flex h-2.5 items-stretch border border-foreground">
        <div className="flex w-10 shrink-0">
          {Array.from({ length: teilung }, (_, i) => ({
            key: `teilung-${i + 1}`,
            gefuellt: i % 2 === 0,
          })).map((stueck, i) => (
            <div
              key={stueck.key}
              className={cn(
                "flex-1",
                stueck.gefuellt ? "bg-foreground" : "bg-transparent",
                i === teilung - 1 ? "border-r border-foreground" : "",
              )}
            />
          ))}
        </div>
        {rechts.map((stueck) => (
          <div
            key={stueck.key}
            className={cn(
              "flex-1 border-l border-foreground",
              stueck.gefuellt ? "bg-foreground" : "bg-transparent",
            )}
          />
        ))}
      </div>
      <div className="flex justify-between font-mono text-[10px] text-muted-foreground">
        <span>{meterProTeil / teilung} m</span>
        <span>0</span>
        <span>{teile * meterProTeil} m</span>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/* Compass rose                                                         */
/* ------------------------------------------------------------------ */

/**
 * An eight-point compass rose. Points that carry a meaning are labelled; the rest
 * are decoration. Used as the sheet's orientation mark and as the frame for
 * directional accuracy.
 */
export function Kompassrose({
  className,
  label = "Kompassrose",
}: {
  className?: string;
  label?: string;
}) {
  const strahlen = Array.from({ length: 8 }, (_, i) => {
    const winkel = (i * Math.PI) / 4 - Math.PI / 2;
    const lang = i % 2 === 0 ? 15 : 9;
    return {
      key: `strahl-${i}`,
      x1: 20 + Math.cos(winkel) * 4,
      y1: 20 + Math.sin(winkel) * 4,
      x2: 20 + Math.cos(winkel) * lang,
      y2: 20 + Math.sin(winkel) * lang,
      haupt: i % 2 === 0,
    };
  });

  return (
    <svg
      viewBox="0 0 40 40"
      role="img"
      aria-label={label}
      className={cn("size-16", className)}
    >
      <circle
        cx="20"
        cy="20"
        r="17"
        fill="none"
        stroke="var(--border)"
        strokeWidth="0.6"
      />
      <circle
        cx="20"
        cy="20"
        r="11"
        fill="none"
        stroke="var(--border)"
        strokeWidth="0.4"
        strokeDasharray="2 2"
      />
      {strahlen.map((strahl) => (
        <line
          key={strahl.key}
          x1={strahl.x1}
          y1={strahl.y1}
          x2={strahl.x2}
          y2={strahl.y2}
          stroke={strahl.haupt ? "var(--foreground)" : "var(--contour)"}
          strokeWidth={strahl.haupt ? 1.1 : 0.6}
        />
      ))}
      {/* The north needle, filled, the only solid shape on the rose. */}
      <path d="M20 6 L23.5 20 L20 18 L16.5 20 Z" fill="var(--primary)" />
      <text
        x="20"
        y="34.5"
        textAnchor="middle"
        fontSize="5"
        fontWeight="700"
        fill="var(--foreground)"
        className="font-mono"
      >
        N
      </text>
    </svg>
  );
}

/* ------------------------------------------------------------------ */
/* Elevation profile                                                    */
/* ------------------------------------------------------------------ */

export type Kote = {
  id: string;
  /** Position along the route in percent. */
  x: number;
  /** Height in the drawing, 0–100. */
  y: number;
  label: string;
  wert: number;
};

/**
 * The elevation profile: a route's height drawn as terrain, with every measured
 * point marked and labelled. Used for score progression along a Runde, where the
 * height is the arrow's performance and the route is the sequence of Ziele.
 */
export function Hoehenprofil({
  koten,
  className,
  label,
}: {
  koten: Kote[];
  className?: string;
  label: string;
}) {
  const punkte = koten.map((kote) => `${kote.x},${100 - kote.y}`).join(" ");
  const flaeche = `0,100 ${punkte} 100,100`;

  return (
    <svg
      viewBox="0 0 100 100"
      role="img"
      aria-label={`${label}: ${koten.map((k) => `${k.label} ${k.wert}`).join(", ")}`}
      preserveAspectRatio="none"
      className={cn("h-32 w-full", className)}
    >
      <polygon points={flaeche} fill="var(--surface-muted)" />
      <polyline
        points={punkte}
        fill="none"
        stroke="var(--primary)"
        strokeWidth="1.4"
        vectorEffect="non-scaling-stroke"
      />
      {koten.map((kote) => (
        <g key={kote.id}>
          <line
            x1={kote.x}
            y1={100 - kote.y}
            x2={kote.x}
            y2="100"
            stroke="var(--contour)"
            strokeWidth="0.4"
            strokeDasharray="1.5 1.5"
            vectorEffect="non-scaling-stroke"
          />
          <circle
            cx={kote.x}
            cy={100 - kote.y}
            r="1.1"
            fill="var(--primary)"
            stroke="var(--surface)"
            strokeWidth="0.5"
            vectorEffect="non-scaling-stroke"
          />
          <text
            x={kote.x}
            y={100 - kote.y - 3}
            textAnchor="middle"
            fontSize="2.6"
            fontWeight="700"
            fill="var(--foreground)"
            className="font-mono"
          >
            {kote.wert}
          </text>
        </g>
      ))}
    </svg>
  );
}

/* ------------------------------------------------------------------ */
/* Legend                                                               */
/* ------------------------------------------------------------------ */

export type LegendenEintrag = {
  id: string;
  probe: ReactNode;
  text: string;
};

/** The legend box: bordered, titled, and always readable without the drawing. */
export function Legende({
  titel,
  eintraege,
  className,
}: {
  titel: string;
  eintraege: LegendenEintrag[];
  className?: string;
}) {
  return (
    <div
      className={cn(
        "rounded-sm border border-border bg-surface p-3",
        className,
      )}
    >
      <p className="mb-2 font-mono text-[10px] font-bold tracking-[0.14em] text-muted-foreground uppercase">
        {titel}
      </p>
      <ul className="space-y-1.5">
        {eintraege.map((eintrag) => (
          <li key={eintrag.id} className="flex items-center gap-2 text-xs">
            <span aria-hidden="true" className="flex w-5 justify-center">
              {eintrag.probe}
            </span>
            <span className="text-foreground">{eintrag.text}</span>
          </li>
        ))}
      </ul>
    </div>
  );
}
