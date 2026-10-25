"use client";

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import {
  GridPaper,
  Legend,
  RingPlot,
  ScaleBar,
  SeriesPlot,
} from "@/components/dev-gallery/data-figures";

/**
 * How data is drawn: thin geometric lines on millimetre paper, flat, with no
 * gradients or fills. Shows the drawing vocabulary as elements only — a ring plot, a
 * series plot, a scale bar, and a legend — so the visual language can be judged on
 * its own, without screens or flows.
 */
export function DataFiguresSection() {
  return (
    <ComponentSection
      id="figures"
      index="04"
      title="Datenbilder"
      description="Dünne geometrische Linien auf Millimeterpapier, flach und ohne Verläufe: Ringdiagramm, Messreihe, Maßstab und Legende. Nur das Aussehen der Elemente — keine Bildschirme und keine Abläufe."
    >
      <Example caption="Ringdiagramm — konzentrische Haarlinien mit eingezeichneten Marken">
        <div className="flex flex-wrap items-start gap-4">
          <div className="relative w-40 shrink-0">
            <GridPaper />
            <RingPlot
              marks={[
                { id: "1", x: 47, y: 45, label: "inner" },
                { id: "2", x: 57, y: 52, label: "middle" },
                { id: "3", x: 76, y: 33, label: "outer" },
              ]}
              className="relative"
            />
          </div>
          <p className="max-w-xs text-xs text-muted-foreground">
            Die Mitte ist die einzige gefüllte Fläche, damit sie sofort lesbar
            ist. Jede Marke steht zusätzlich als Text daneben; die Zeichnung
            trägt keine Bedeutung allein.
          </p>
        </div>
      </Example>

      <Example
        caption="Messreihe — eine Bleistiftlinie auf Millimeterpapier"
        className="block"
      >
        <div className="w-full rounded-sm border border-border bg-surface p-3">
          <SeriesPlot
            points={[
              { id: "p1", x: 4, y: 24, label: "1", value: 20 },
              { id: "p2", x: 19, y: 40, label: "2", value: 18 },
              { id: "p3", x: 34, y: 16, label: "3", value: 20 },
              { id: "p4", x: 49, y: 58, label: "4", value: 10 },
              { id: "p5", x: 64, y: 34, label: "5", value: 18 },
              { id: "p6", x: 79, y: 70, label: "6", value: 5 },
              { id: "p7", x: 93, y: 28, label: "7", value: 20 },
            ]}
            label="Messreihe, Punkte 1 bis 7"
          />
          <p className="mt-2 text-xs text-muted-foreground">
            Jeder Punkt sitzt auf einer Hilfslinie und trägt seinen Wert in
            Mono. Keine Balken, kein Verlauf, kein gefülltes Gitter.
          </p>
        </div>
      </Example>

      <Example
        caption="Maßstab — der Wert steht als Regel daneben"
        className="block"
      >
        <div className="w-64 space-y-2">
          <div className="flex items-baseline gap-2">
            <span className="text-xs text-muted-foreground">Messwert</span>
            <span className="text-sm font-semibold tabular-nums">42 m</span>
          </div>
          <ScaleBar segments={4} unitsPerSegment={10} unit="units" />
        </div>
      </Example>

      <Example caption="Legende — jede Zeichnung wird im Text erklärt">
        <Legend
          title="Zeichenerklärung"
          entries={[
            {
              id: "a",
              sample: (
                <svg width="18" height="12" aria-hidden="true">
                  <circle
                    cx="9"
                    cy="6"
                    r="5"
                    fill="none"
                    stroke="var(--border-strong)"
                    strokeWidth="1"
                  />
                </svg>
              ),
              text: "Rasterlinie",
            },
            {
              id: "b",
              sample: (
                <svg width="18" height="12" aria-hidden="true">
                  <circle cx="9" cy="6" r="3" fill="var(--color-primary)" />
                </svg>
              ),
              text: "Bezugspunkt — die einzige gefüllte Form",
            },
            {
              id: "c",
              sample: (
                <svg width="18" height="12" aria-hidden="true">
                  <circle
                    cx="9"
                    cy="6"
                    r="3.2"
                    fill="none"
                    stroke="var(--semantic-impact)"
                    strokeWidth="1.2"
                  />
                </svg>
              ),
              text: "Hervorgehobene Marke",
            },
          ]}
        />
      </Example>
    </ComponentSection>
  );
}
