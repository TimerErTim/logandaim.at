"use client";

import { Wertungszone } from "@/api/logandaim-api";
import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { cn } from "@/lib/cn";

/**
 * Proposal "Feldbuch & Wegmarkierung": signature elements for the visual identity,
 * prototyped so the PO can judge them in the gallery next to the current direction.
 *
 * The thesis: the competitor of this app is the paper scorecard in the quiver
 * pocket, not another app. The identity borrows from the material culture of the
 * parcours — the field book, the painted waymark on the tree, the measuring band —
 * and from nothing on any screen. Every element is justified by the domain:
 *
 * - Das Feldbuch: history is a ledger, not a dashboard.
 * - Wegmarkierung: the rotated square (diamond) marks where you are.
 * - Das Maßband: the ruler tick is the recurring graphic atom.
 * - Vom Ding: a Schuss is drawn as a hit on the ring diagram of a target face.
 *
 * This module is reachable only from `page.dev.tsx` and never ships. No token,
 * variant, or production component changes here: this is a proposal, and the PO
 * decides in the gallery (specs/product/visual-identity.md, Q-1).
 */

/**
 * Das Maßband: a divider drawn as a ruler strip. Minor ticks every 12px, major
 * ticks every 60px. Colours come from the tokens; the tick is the one ornament
 * the product has earned, because the stake-to-stake distance is the thing
 * nobody else tracks.
 */
export function TickRule({ className }: { className?: string }) {
  return (
    <div
      aria-hidden="true"
      className={cn("h-2.5 w-full", className)}
      style={{
        backgroundImage:
          "repeating-linear-gradient(to right, var(--muted-foreground) 0, var(--muted-foreground) 1px, transparent 1px, transparent 60px)," +
          "repeating-linear-gradient(to right, var(--border) 0, var(--border) 1px, transparent 1px, transparent 12px)",
        backgroundSize: "100% 100%, 100% 55%",
        backgroundPosition: "bottom, bottom",
        backgroundRepeat: "no-repeat",
      }}
    />
  );
}

/**
 * Wegmarke: the rotated square, the signature marker of the app. Real parcours
 * mark their route with painted shapes on trees; the app borrows the vocabulary.
 * Shape belongs to the app, colour belongs to the content (the Pflock master
 * data decision), so the marker never carries a status colour of its own.
 */
export function Wegmarke({ aktiv }: { aktiv?: boolean }) {
  return (
    <span
      aria-hidden="true"
      className={cn(
        "size-2.5 shrink-0 rotate-45 rounded-[2px]",
        aktiv ? "bg-primary" : "border-2 border-border",
      )}
    />
  );
}

/**
 * Das Maßband as a display, not an input: a read-only tick scale with the
 * diamond marking the measured distance. Used in history, where the value is
 * fixed and a slider would lie about being adjustable.
 */
export function Distanzband({
  distanz,
  min = 0,
  max = 80,
  einheit = "m",
  className,
}: {
  distanz: number;
  min?: number;
  max?: number;
  einheit?: string;
  className?: string;
}) {
  const anteil = Math.min(
    100,
    Math.max(0, ((distanz - min) / (max - min)) * 100),
  );

  return (
    <div
      role="img"
      aria-label={`Distanz ${distanz} ${einheit}, Skala von ${min} bis ${max} ${einheit}`}
      className={cn("w-full pt-5", className)}
    >
      <div className="relative h-2.5">
        <TickRule />
        {/* The diamond sits on the scale like a waymark on the parcours. */}
        <span
          aria-hidden="true"
          className="absolute -top-4 size-2.5 -translate-x-1/2 rotate-45 rounded-[2px] bg-primary"
          style={{ left: `${anteil}%` }}
        />
        <span
          aria-hidden="true"
          className="absolute -top-4 size-2.5 -translate-x-1/2 rounded-full border border-surface"
          style={{ left: `${anteil}%` }}
        />
      </div>
      <p className="mt-1 text-xs text-muted-foreground tabular-nums">
        <span className="font-mono font-medium text-foreground">{distanz}</span>{" "}
        {einheit}
      </p>
    </div>
  );
}

/**
 * Vom Ding, nicht vom Bildschirm: a Schuss is drawn as a hit on the ring
 * diagram of a target face, the same zones the archer sees on the 3D Tier.
 *
 * Zone colours in the prototype are semantic tokens standing in for the real
 * zone master data, which is not decided yet; the legend always carries the
 * zone name, so the diagram works without seeing colour (accessibility bar,
 * charter rule 5).
 */
type Treffer = {
  id: string;
  zone: Wertungszone | "Miss";
  /** Position in percent of the face diameter. */
  x: number;
  y: number;
};

const RINGE = [
  {
    zone: "Spot" as const,
    radius: 10,
    farbe: "var(--primary)",
    text: "var(--primary-foreground)",
  },
  {
    zone: "Kill" as const,
    radius: 24,
    farbe: "var(--warning)",
    text: "var(--foreground)",
  },
  {
    zone: "Body" as const,
    radius: 40,
    farbe: "var(--secondary)",
    text: "var(--muted-foreground)",
  },
];

const TREFFER: Treffer[] = [
  { id: "t1", zone: Wertungszone.Spot, x: 48, y: 46 },
  { id: "t2", zone: Wertungszone.Kill, x: 62, y: 38 },
  { id: "t3", zone: Wertungszone.Body, x: 38, y: 62 },
  { id: "t4", zone: Wertungszone.Miss, x: 78, y: 70 },
];

const ZONENFARBE: Record<Treffer["zone"], string> = {
  [Wertungszone.Spot]: "var(--primary)",
  [Wertungszone.Kill]: "var(--warning)",
  [Wertungszone.Body]: "var(--secondary)",
  [Wertungszone.Miss]: "var(--surface-muted)",
};

function RingDiagramm() {
  const trefferText = TREFFER.map((t) => t.zone).join(", ");
  return (
    <div className="flex items-start gap-6">
      <svg
        role="img"
        aria-label={`Zielscheibe mit ${TREFFER.length} Treffern: ${trefferText}`}
        viewBox="0 0 100 100"
        className="size-36 shrink-0"
      >
        {/* The face outline; outside it is Miss. */}
        <circle
          cx="50"
          cy="50"
          r="48"
          fill="var(--surface)"
          stroke="var(--border)"
          strokeWidth="1"
        />
        {RINGE.map((ring) => (
          <circle
            key={ring.zone}
            cx="50"
            cy="50"
            r={ring.radius}
            fill={ring.farbe}
            stroke="var(--border)"
            strokeWidth="0.75"
          />
        ))}
        {/* Ring labels stay readable inside their ring. */}
        {RINGE.map((ring) => (
          <text
            key={ring.zone}
            x="50"
            y={50 - ring.radius + 8}
            textAnchor="middle"
            fontSize="7"
            fontWeight="600"
            fill={ring.text}
          >
            {ring.zone}
          </text>
        ))}
        {TREFFER.map((treffer) => (
          <circle
            key={treffer.id}
            cx={treffer.x}
            cy={treffer.y}
            r={treffer.zone === Wertungszone.Miss ? 2.5 : 3.5}
            fill={
              treffer.zone === Wertungszone.Miss
                ? "var(--destructive)"
                : "var(--foreground)"
            }
            stroke="var(--surface)"
            strokeWidth="1"
          />
        ))}
      </svg>
      <ul className="space-y-1.5 text-xs">
        {(["Spot", "Kill", "Body", "Miss"] as const).map((zone) => (
          <li key={zone} className="flex items-center gap-2">
            <span
              aria-hidden="true"
              className="size-3 rounded-sm border border-border"
              style={{ background: ZONENFARBE[zone] }}
            />
            <span className="font-medium text-foreground">{zone}</span>
            <span className="text-muted-foreground tabular-nums">
              {TREFFER.filter((t) => t.zone === zone).length}×
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}

/**
 * Das Feldbuch: the history of a Schütze as a ledger, not a dashboard.
 * Dated, ruled, tabular; machine values in the mono face; entries separated by
 * the Maßband. No KPI tiles, no floating cards.
 */
const RUNDEN = [
  {
    id: "r1",
    datum: "12.10.2026",
    parcours: "Parcours Großraming",
    ziele: [
      { id: "z7", ziel: 7, zone: "Kill", punkte: "18", distanz: 42 },
      { id: "z8", ziel: 8, zone: "Spot", punkte: "20", distanz: 28 },
      { id: "z9", ziel: 9, zone: "Miss", punkte: "0", distanz: 55 },
    ],
  },
  {
    id: "r2",
    datum: "05.10.2026",
    parcours: "Parcours Großraming",
    ziele: [
      { id: "z1", ziel: 1, zone: "Body", punkte: "10", distanz: 33 },
      { id: "z2", ziel: 2, zone: "Kill", punkte: "18", distanz: 47 },
    ],
  },
  {
    id: "r3",
    datum: "28.09.2026",
    parcours: "Parcours Steyrtal",
    ziele: [{ id: "z4", ziel: 4, zone: "Spot", punkte: "20", distanz: 51 }],
  },
];

/** Ledger entries separated by the Maßband, the way a field book reads. */
function FeldbuchEintrag() {
  return (
    <div className="w-full space-y-4">
      {RUNDEN.map((runde, index) => (
        <article key={runde.id} className="w-full space-y-2">
          {index > 0 ? <TickRule className="mb-4" /> : null}
          <header className="flex items-baseline justify-between gap-2">
            <h3 className="text-sm font-semibold">
              Runde <span className="text-muted-foreground">·</span>{" "}
              {runde.parcours}
            </h3>
            <span className="font-mono text-xs text-muted-foreground">
              {runde.datum}
            </span>
          </header>
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader>Ziel</TableHeader>
                <TableHeader>Zone</TableHeader>
                <TableHeader className="text-right">Punkte</TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {runde.ziele.map((ziel) => (
                <TableRow key={ziel.id}>
                  <TableCell>
                    Ziel <span className="tabular-nums">{ziel.ziel}</span>
                  </TableCell>
                  <TableCell>
                    <span className="tabular-nums">{ziel.distanz} m</span>
                    <span className="text-muted-foreground"> · </span>
                    {ziel.zone}
                  </TableCell>
                  <TableCell className="text-right font-mono tabular-nums">
                    {ziel.punkte}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </article>
      ))}
    </div>
  );
}

/** An archived proposal kept for reference; it does not govern any token. */
export function FeldbuchSection() {
  return (
    <ComponentSection
      id="feldbuch"
      title="Feldbuch & Wegmarkierung (Archiv)"
      description="Archivierter Vorschlag, überholt vom Bogenschuss-Logbuch: die Historie als Feldbuch, die Raute als Wegmarke, das Maßband als Maßstab, der Treffer auf der Ringscheibe. Nur zur Einordnung — die Skizzen speisen keine Tokens."
    >
      <Example
        caption="Wegmarke — Kennzeichnung der aktiven Stelle"
        className="block"
      >
        <div className="flex items-center gap-3">
          <Wegmarke aktiv />
          <span className="text-sm">An der aktiven Stelle</span>
        </div>
        <div className="mt-3 flex items-center gap-3">
          <Wegmarke />
          <span className="text-sm">An jeder anderen Stelle</span>
        </div>
      </Example>

      <Example
        caption="Maßband — Trennlinie und Distanzanzeige"
        className="block"
      >
        <div className="w-full space-y-3">
          <p className="text-sm">Abschnitt zwischen zwei Einträgen.</p>
          <TickRule />
          <p className="text-sm">Der nächste Abschnitt beginnt.</p>
          <Distanzband distanz={42} />
          <Distanzband distanz={55} />
        </div>
      </Example>

      <Example
        caption="Ringscheibe — der Schuss am Tier, nicht als Badge"
        className="block"
      >
        <RingDiagramm />
      </Example>

      <Example caption="Feldbuch — die Historie als Eintrag" className="block">
        <FeldbuchEintrag />
      </Example>
    </ComponentSection>
  );
}
