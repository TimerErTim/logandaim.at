"use client";

import { IBM_Plex_Sans, Nunito_Sans, Source_Sans_3 } from "next/font/google";
import { useState } from "react";

import { Example } from "@/components/dev-gallery/component-section";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

/**
 * Font candidates. `next/font` needs a module-level call, and this module is only
 * reachable from `page.dev.tsx`, so the production build never emits these
 * families. The variables carry a `-candidate-` segment so they cannot collide with
 * the `@theme` keys that map `--font-plex` and friends to the family.
 *
 * `preload: false` keeps a candidate from being fetched until its stack is applied.
 */
const plex = IBM_Plex_Sans({
  subsets: ["latin"],
  weight: ["400", "500", "600"],
  variable: "--font-candidate-plex",
  preload: false,
});

const source = Source_Sans_3({
  subsets: ["latin"],
  weight: ["400", "500", "600"],
  variable: "--font-candidate-source",
  preload: false,
});

const nunito = Nunito_Sans({
  subsets: ["latin"],
  weight: ["400", "500", "600"],
  variable: "--font-candidate-nunito",
  preload: false,
});

/**
 * Typeface comparison for the PO.
 *
 * Every candidate is shown at UI size, at table size, and with real product copy,
 * because a typeface that reads well in a specimen can still be wrong in a dense
 * table. The candidates are installed with `next/font`, so this renders the real
 * files, not a Google Fonts link.
 *
 * Once one is chosen, the others are removed and the choice moves into
 * `layout.tsx` (the `next/font` call) and `globals.css` (`--font-sans`), documented
 * in the design rule.
 */
const CANDIDATES = [
  {
    id: "geist",
    name: "Geist",
    family: "var(--font-geist-sans)",
    note: "Aktuell installiert. Geometrisch, sehr sauber, leicht technisch.",
    tone: "neutral" as const,
  },
  {
    id: "plex",
    name: "IBM Plex Sans",
    family: "var(--font-candidate-plex)",
    note: "Sachlich, offene Formen, gut in Tabellen. Werkzeug-Charakter.",
    tone: "neutral" as const,
  },
  {
    id: "source",
    name: "Source Sans 3",
    family: "var(--font-candidate-source)",
    note: "Ruhig, kompakt, hohe x-Höhe. Sehr gut lesbar bei kleinen Graden.",
    tone: "neutral" as const,
  },
  {
    id: "nunito",
    name: "Nunito Sans",
    family: "var(--font-candidate-nunito)",
    note: "Weicher, freundlicher. Wärmer als die anderen, weniger technisch.",
    tone: "warm" as const,
  },
];

const SAMPLE_ROWS = [
  { pfeil: "1", zone: "Kill", punkte: "1" },
  { pfeil: "2", zone: "Body", punkte: "5" },
  { pfeil: "3", zone: "Miss", punkte: "0" },
];

export function TypeSection() {
  const [candidate, setCandidate] = useState(CANDIDATES[0]?.id ?? "geist");
  const active =
    CANDIDATES.find((entry) => entry.id === candidate) ?? CANDIDATES[0];

  return (
    <section
      id="type"
      aria-labelledby="type-heading"
      className={`${plex.variable} ${source.variable} ${nunito.variable} space-y-3`}
    >
      <header className="space-y-1">
        <h2
          id="type-heading"
          className="text-base font-semibold tracking-tight"
        >
          Schrift
        </h2>
        <p className="max-w-prose text-sm text-muted-foreground">
          Kandidaten in echter Produktsprache, bei UI-Größe und in der Tabelle.
          Umschalten, um denselben Inhalt in einer anderen Schrift zu sehen.
        </p>
      </header>

      <div className="flex flex-wrap items-center gap-2">
        {CANDIDATES.map((entry) => (
          <Button
            key={entry.id}
            size="sm"
            variant={entry.id === active.id ? "primary" : "secondary"}
            aria-pressed={entry.id === active.id}
            onClick={() => setCandidate(entry.id)}
          >
            {entry.name}
          </Button>
        ))}
        <Badge variant="outline">{active.name}</Badge>
      </div>

      {/*
        The candidate variable is resolved on this element, not through a `@theme`
        alias: an alias on `:root` would substitute the (undefined) variable at the
        root and inherit the invalid value.
      */}
      <div style={{ fontFamily: active.family }} className="space-y-4">
        <p className="max-w-prose text-sm text-muted-foreground">
          {active.note}
        </p>

        <Example caption="Überschrift und Fließtext">
          <div className="w-full space-y-1">
            <p className="text-2xl font-semibold tracking-tight">
              Schuss erfassen
            </p>
            <p className="text-sm">
              Jeden Pfeil an einem Ziel registrieren. Der erste Treffer und der
              Mehrschuss werden daraus berechnet.
            </p>
            <p className="text-sm text-muted-foreground">
              Nebentext: 32 Ziele, 4 Pflöcke, Distanz 42 Schritte.
            </p>
          </div>
        </Example>

        <Example caption="Tabelle und Bedienelemente">
          <div className="w-full space-y-3">
            <table className="w-full border-collapse text-sm">
              <thead className="text-left text-xs uppercase tracking-wide text-muted-foreground">
                <tr>
                  <th className="py-1 font-semibold">Pfeil</th>
                  <th className="py-1 font-semibold">Zone</th>
                  <th className="py-1 text-right font-semibold">Punkte</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {SAMPLE_ROWS.map((row) => (
                  <tr key={row.pfeil}>
                    <td className="py-1.5 tabular-nums">{row.pfeil}</td>
                    <td className="py-1.5">{row.zone}</td>
                    <td className="py-1.5 text-right tabular-nums">
                      {row.punkte}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            <div className="flex flex-wrap items-center gap-2">
              <Button size="sm">Schuss erfassen</Button>
              <Button size="sm" variant="secondary">
                Abbrechen
              </Button>
              <span className="text-xs text-muted-foreground tabular-nums">
                0712 · 284 / 320
              </span>
            </div>
          </div>
        </Example>

        <Example caption="Zeichen">
          <p className="text-sm">
            ÄÖÜäöüß · 0123456789 · € · → · ✓ · — »Wertungszone«
          </p>
        </Example>
      </div>
    </section>
  );
}
