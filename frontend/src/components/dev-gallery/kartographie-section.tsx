"use client";

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import {
  type Gipfel,
  Gradnetz,
  Hoehenprofil,
  Kartenrand,
  Kompassrose,
  Konturlinien,
  Legende,
  Massstabsleiste,
} from "@/components/dev-gallery/kartographie";

/**
 * Die kartographischen Elemente.
 *
 * This section exists to answer one question only: does the Jagdkarte identity
 * survive as an identity, without leaning on any user flow? It shows the visual
 * vocabulary — contour lines, elevation profile, compass rose, scale bar,
 * graticule, neatline, legend — as elements, with no screen, no navigation, and
 * no interaction design. How the product is used is decided elsewhere.
 *
 * Dev-only, reachable from `page.dev.tsx`.
 */

/** A relief: nested contour rings with index lines, as on a survey sheet. */
const RELIEF: Gipfel[] = [
  { id: "n1", x: 22, y: 34, radius: 4.6, ringe: 7, hoehe: "312" },
  { id: "n2", x: 51, y: 22, radius: 3.2, ringe: 6, hoehe: "284" },
  { id: "n3", x: 76, y: 42, radius: 5.6, ringe: 5, hoehe: "198" },
];

const KOTEN = [
  { id: "k1", x: 4, y: 24, label: "Punkt 1", wert: 20 },
  { id: "k2", x: 19, y: 40, label: "Punkt 2", wert: 18 },
  { id: "k3", x: 34, y: 16, label: "Punkt 3", wert: 20 },
  { id: "k4", x: 49, y: 58, label: "Punkt 4", wert: 10 },
  { id: "k5", x: 64, y: 34, label: "Punkt 5", wert: 18 },
  { id: "k6", x: 79, y: 70, label: "Punkt 6", wert: 5 },
  { id: "k7", x: 93, y: 28, label: "Punkt 7", wert: 20 },
];

const linie = (
  muster: "durch" | "gestrichelt" | "gepunktet",
  farbe: string,
) => (
  <svg width="20" height="6" aria-hidden="true">
    <line
      x1="0"
      y1="3"
      x2="20"
      y2="3"
      stroke={farbe}
      strokeWidth="1.5"
      strokeDasharray={
        muster === "gestrichelt"
          ? "4 2.5"
          : muster === "gepunktet"
            ? "1.5 2"
            : undefined
      }
    />
  </svg>
);

export function KartographieSection() {
  return (
    <ComponentSection
      id="kartographie"
      title="Kartographie — die Elemente"
      description="Die visuelle Sprache, ohne jeden Ablauf: Höhenlinien mit Indexlinien, Höhenprofil, Kompassrose, geteilte Maßstabsleiste, Gradnetz, Kartenrand und Legende. Es geht nur um das Aussehen der Elemente, nicht um Bildschirme oder Bedienwege."
    >
      <Example
        caption="Höhenlinien — geschlossene Ringe, jede fünfte als Indexlinie"
        className="block"
      >
        <div className="h-40 w-full rounded-sm border border-border bg-background p-2">
          <Konturlinien gipfel={RELIEF} label="Relief mit Höhenlinien" />
        </div>
      </Example>

      <Example
        caption="Höhenprofil — eine Messreihe als Gelände"
        className="block"
      >
        <div className="w-full rounded-sm border border-border bg-surface p-3">
          <Hoehenprofil
            koten={KOTEN}
            label="Messreihe, Punkt 1 bis 7"
            className="h-28"
          />
          <p className="mt-2 text-xs text-muted-foreground">
            Jeder Messpunkt sitzt auf einer Hilfslinie und trägt seinen Wert in
            Monospace. Die Fläche darunter ist ruhig gefüllt, nicht
            verlaufsfarbig.
          </p>
        </div>
      </Example>

      <Example caption="Kompassrose und Maßstabsleiste" className="block">
        <div className="flex flex-wrap items-end gap-6">
          <div className="space-y-1">
            <Kompassrose />
            <p className="max-w-24 text-xs text-muted-foreground">
              Achtzehn Punkte, die Nordnadel ist die einzige gefüllte Form.
            </p>
          </div>
          <div className="w-52 space-y-1">
            <Massstabsleiste teile={4} meterProTeil={10} teilung={5} />
            <p className="text-xs text-muted-foreground">
              Links unterteilt, damit sich Zwischenwerte ablesen lassen.
            </p>
          </div>
        </div>
      </Example>

      <Example
        caption="Kartenrand — Doppelrahmen mit Koordinatenzeile"
        className="block"
      >
        <Kartenrand kopf="Blatt 07/32" fuss="1:2 000 · Stand 25.10.2026">
          <div className="relative h-44 bg-background">
            <Gradnetz />
            <div className="absolute inset-0 p-2">
              <Konturlinien gipfel={RELIEF} label="Relief im Kartenrand" />
            </div>
            <div className="absolute top-2 right-2">
              <Kompassrose className="size-12" />
            </div>
            <div className="absolute bottom-2 left-2 w-28 rounded-sm border border-border bg-surface/95 p-1.5">
              <Massstabsleiste teile={3} meterProTeil={10} teilung={5} />
            </div>
          </div>
        </Kartenrand>
      </Example>

      <Example
        caption="Gradnetz — das feine Raster unter jeder Karte"
        className="block"
      >
        <div className="relative h-24 w-full overflow-hidden rounded-sm border border-border bg-background">
          <Gradnetz />
          <p className="absolute inset-0 flex items-center justify-center font-mono text-xs text-muted-foreground">
            24 px Raster · 35 % Deckkraft
          </p>
        </div>
      </Example>

      <Example
        caption="Legende — jede Bedeutung steht im Text"
        className="block"
      >
        <div className="flex flex-wrap gap-3">
          <Legende
            titel="Zeichenerklärung"
            eintraege={[
              {
                id: "a",
                probe: linie("durch", "var(--contour)"),
                text: "Höhenlinie",
              },
              {
                id: "b",
                probe: linie("durch", "var(--foreground)"),
                text: "Indexlinie (jede fünfte)",
              },
              {
                id: "c",
                probe: linie("gestrichelt", "var(--primary)"),
                text: "Weg",
              },
              {
                id: "d",
                probe: linie("gepunktet", "var(--muted-foreground)"),
                text: "Blattgrenze",
              },
            ]}
          />
          <p className="max-w-xs self-center text-xs text-muted-foreground">
            Jede Zeichnung ist Dekoration für Sehende. Jeder Wert, den sie
            trägt, steht zusätzlich als Text daneben — hier in der Legende,
            sonst in der Ansage.
          </p>
        </div>
      </Example>
    </ComponentSection>
  );
}
