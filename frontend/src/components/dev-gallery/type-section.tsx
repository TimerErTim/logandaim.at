"use client";

import { Example } from "@/components/dev-gallery/component-section";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

/**
 * Typography of the logbook: two faces, two jobs.
 *
 * Lora is the book face and appears only in titles; Inter is the work face and
 * carries every control, table, and figure. The section shows them together on real
 * product copy, because a pair that reads well in a specimen can still fall apart in
 * a dense table. Both are installed in `layout.tsx`, so nothing extra is fetched.
 */
export function TypeSection() {
  return (
    <section id="type" aria-labelledby="type-heading" className="space-y-3">
      <header className="space-y-1">
        <h2
          id="type-heading"
          className="font-serif text-lg font-semibold tracking-tight"
        >
          Schrift
        </h2>
        <p className="max-w-prose text-sm text-muted-foreground">
          Lora trägt die Überschriften, Inter die Arbeit. Der Buchcharakter lebt
          in den Titeln; alles, was gelesen und bedient wird, bleibt nüchtern.
        </p>
      </header>

      <div className="flex flex-wrap items-center gap-2">
        <Badge variant="outline">Lora · Titel</Badge>
        <Badge variant="outline">Inter · Schnittstelle</Badge>
        <Badge variant="outline">Geist Mono · Maschinenwerte</Badge>
      </div>

      <Example caption="Überschrift und Fließtext">
        <div className="w-full space-y-1">
          <p className="font-serif text-2xl font-semibold tracking-tight">
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

      <Example caption="Tabelle und Bedienelemente">
        <div className="w-full space-y-3">
          <table className="w-full border-collapse text-sm">
            <thead className="border-b border-border text-left text-[11px] tracking-[0.08em] text-muted-foreground uppercase">
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
            <span className="font-mono text-xs text-muted-foreground tabular-nums">
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
    </section>
  );
}

const SAMPLE_ROWS = [
  { pfeil: "1", zone: "Kill", punkte: "1" },
  { pfeil: "2", zone: "Body", punkte: "5" },
  { pfeil: "3", zone: "Miss", punkte: "0" },
];
