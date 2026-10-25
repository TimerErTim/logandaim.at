"use client";

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import {
  Messreihe,
  Millimetergrid,
  Skalenleiste,
  Stempelledende,
  Zielscheibe,
} from "@/components/dev-gallery/datenbilder";

/**
 * Datenbilder — the data vocabulary of the logbook.
 *
 * This section exists to answer one question: does the identity survive as an
 * identity without leaning on any user flow? It shows the drawing vocabulary only —
 * the target face with its hit, a measurement series on graph paper, a ruled scale,
 * and a stamp legend. No screens, no navigation, no interaction design. How the
 * product is used is decided elsewhere.
 *
 * Dev-only, reachable from `page.dev.tsx`.
 */
export function DatenbilderSection() {
  return (
    <ComponentSection
      id="datenbilder"
      index="04"
      title="Datenbilder"
      description="Wie Daten gezeichnet werden: dünne geometrische Linien auf Millimeterpapier, flach und ohne Verläufe. Das Trefferbild, eine Messreihe, ein Maßstab und die Legende — nur das Aussehen der Elemente, keine Bildschirme oder Bedienwege."
    >
      <Example caption="Trefferbild — Ringe als dünne Striche, Treffer eingezeichnet">
        <div className="flex flex-wrap items-start gap-4">
          <div className="relative w-40 shrink-0">
            <Millimetergrid />
            <Zielscheibe
              treffer={[
                { id: "Pfeil 1", x: 47, y: 45, zone: "Kill" },
                { id: "Pfeil 2", x: 57, y: 52, zone: "Body" },
                { id: "Pfeil 3", x: 76, y: 33, zone: "Miss" },
              ]}
              className="relative"
            />
          </div>
          <p className="max-w-xs text-xs text-muted-foreground">
            Die Zehn ist die einzige gefüllte Fläche, damit die Mitte sofort
            lesbar ist. Jeder Treffer steht zusätzlich als Text daneben; die
            Zeichnung trägt keine Bedeutung allein.
          </p>
        </div>
      </Example>

      <Example
        caption="Messreihe — eine Serie als Bleistiftlinie auf Millimeterpapier"
        className="block"
      >
        <div className="w-full rounded-sm border border-border bg-surface p-3">
          <Messreihe
            punkte={[
              { id: "m1", x: 4, y: 24, label: "Pfeil 1", wert: 20 },
              { id: "m2", x: 19, y: 40, label: "Pfeil 2", wert: 18 },
              { id: "m3", x: 34, y: 16, label: "Pfeil 3", wert: 20 },
              { id: "m4", x: 49, y: 58, label: "Pfeil 4", wert: 10 },
              { id: "m5", x: 64, y: 34, label: "Pfeil 5", wert: 18 },
              { id: "m6", x: 79, y: 70, label: "Pfeil 6", wert: 5 },
              { id: "m7", x: 93, y: 28, label: "Pfeil 7", wert: 20 },
            ]}
            label="Messreihe, Pfeil 1 bis 7"
          />
          <p className="mt-2 text-xs text-muted-foreground">
            Jeder Punkt sitzt auf einer Hilfslinie und trägt seinen Wert in
            Mono. Kein Balken, kein Verlauf, kein Gitter mit Hintergrundfarbe.
          </p>
        </div>
      </Example>

      <Example
        caption="Maßstab — der Wert steht als Regel daneben"
        className="block"
      >
        <div className="w-64 space-y-2">
          <div className="flex items-baseline gap-2">
            <span className="text-xs text-muted-foreground">Distanz</span>
            <span className="text-sm font-semibold tabular-nums">42 m</span>
          </div>
          <Skalenleiste teile={4} meterProTeil={10} />
        </div>
      </Example>

      <Example caption="Legende — jede Zeichnung wird im Text erklärt">
        <Stempelledende
          titel="Zeichenerklärung"
          eintraege={[
            {
              id: "a",
              probe: (
                <svg width="18" height="12" aria-hidden="true">
                  <circle
                    cx="9"
                    cy="6"
                    r="5"
                    fill="none"
                    stroke="var(--border)"
                    strokeWidth="1"
                  />
                </svg>
              ),
              text: "Wertungsring",
            },
            {
              id: "b",
              probe: (
                <svg width="18" height="12" aria-hidden="true">
                  <circle cx="9" cy="6" r="3" fill="var(--primary)" />
                </svg>
              ),
              text: "Zehner — die einzige gefüllte Form",
            },
            {
              id: "c",
              probe: (
                <svg width="18" height="12" aria-hidden="true">
                  <circle
                    cx="9"
                    cy="6"
                    r="3.2"
                    fill="none"
                    stroke="var(--destructive)"
                    strokeWidth="1.2"
                  />
                </svg>
              ),
              text: "Treffer",
            },
          ]}
        />
      </Example>
    </ComponentSection>
  );
}
