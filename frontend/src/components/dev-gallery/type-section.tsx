"use client";

import { Example } from "@/components/dev-gallery/component-section";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

/**
 * Typography of the logbook: two faces, two jobs.
 *
 * Lora is the book face and appears only in titles; Inter is the work face and
 * carries every control, table, and figure; Geist Mono is reserved for values a
 * machine produced. The specimen shows them together on real product copy, because
 * a pairing that reads well alone can still fall apart in a dense table.
 *
 * Both families are loaded in `layout.tsx` via `next/font`, so nothing extra is
 * fetched at runtime and there is no flash of a fallback face.
 */
export function TypeSection() {
  return (
    <section
      id="type"
      aria-labelledby="type-heading"
      className="scroll-mt-6 space-y-4"
    >
      <header className="flex items-baseline gap-4 pb-3">
        <span
          aria-hidden="true"
          className="font-mono text-xs text-muted-foreground tabular-nums"
        >
          02
        </span>
        <div className="min-w-0 flex-1 space-y-1">
          <h2
            id="type-heading"
            className="font-serif text-lg font-semibold tracking-tight"
          >
            Schrift
          </h2>
          <p className="max-w-prose text-sm text-muted-foreground">
            Lora trägt die Überschriften, Inter die Arbeit. Der Buchcharakter
            lebt in den Titeln; alles, was gelesen und bedient wird, bleibt
            nüchtern.
          </p>
        </div>
      </header>

      <div aria-hidden="true" className="rule-ticks mb-4 h-px w-full" />

      <div className="grid gap-4 lg:grid-cols-2">
        <div className="light space-y-5 rounded-md border border-border bg-background p-4 text-foreground">
          <TypeSpecimen />
        </div>
        <div className="dark space-y-5 rounded-md border border-border bg-background p-4 text-foreground">
          <TypeSpecimen />
        </div>
      </div>
    </section>
  );
}

function TypeSpecimen() {
  return (
    <>
      <div className="flex flex-wrap items-center gap-2">
        <Badge variant="outline">Lora · Titel</Badge>
        <Badge variant="outline">Inter · Schnittstelle</Badge>
        <Badge variant="outline">Geist Mono · Werte</Badge>
      </div>

      <Example caption="Überschrift und Fließtext" className="block">
        <div className="w-full space-y-2">
          <p className="font-serif text-2xl leading-tight font-semibold tracking-tight">
            Neuer Eintrag — Revier Süd
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

      <Example caption="Tabelle und Bedienelemente" className="block">
        <div className="w-full space-y-3">
          <table className="w-full border-collapse text-sm">
            <thead className="border-b border-border text-left text-[10px] tracking-[0.09em] text-muted-foreground uppercase">
              <tr>
                <th scope="col" className="py-1.5 font-semibold">
                  Pfeil
                </th>
                <th scope="col" className="py-1.5 font-semibold">
                  Zone
                </th>
                <th scope="col" className="py-1.5 text-right font-semibold">
                  Punkte
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-border">
              {SAMPLE_ROWS.map((row) => (
                <tr key={row.pfeil}>
                  <td className="py-1.5 font-mono text-xs tabular-nums">
                    {row.pfeil}
                  </td>
                  <td className="py-1.5">{row.zone}</td>
                  <td className="py-1.5 text-right font-medium tabular-nums">
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
            <span className="font-mono text-xs text-muted-foreground tabular-nums">
              0712 · 284 / 320
            </span>
          </div>
        </div>
      </Example>

      <Example caption="Zeichen" className="block">
        <p className="w-full text-sm">
          ÄÖÜäöüß · 0123456789 · € · → · ✓ · — »Wertungszone«
        </p>
      </Example>
    </>
  );
}

const SAMPLE_ROWS = [
  { pfeil: "1", zone: "Kill", punkte: "1" },
  { pfeil: "2", zone: "Body", punkte: "5" },
  { pfeil: "3", zone: "Miss", punkte: "0" },
];
