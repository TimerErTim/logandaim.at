"use client";

import { useState } from "react";

import { DemoPage } from "@/components/dev-gallery/demo-page";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { cn } from "@/lib/cn";

/**
 * The composed plate: every primitive on one screen, arranged the way a real page
 * arranges them. It is the robustness test for the design system — the parts can
 * each pass and the page still fail on density, hierarchy, or neighbouring contrast.
 *
 * Not drawn twice side by side: a page needs width to be judged, so the theme is
 * switched instead. Both themes are complete scopes, so the switch is truthful.
 */
export function ComposedPageSection() {
  const [theme, setTheme] = useState<"light" | "dark">("light");

  return (
    <section
      id="composed"
      aria-labelledby="composed-heading"
      className="scroll-mt-6"
    >
      <header className="flex items-baseline gap-4 pb-3">
        <span
          aria-hidden="true"
          className="font-mono text-xs text-secondary tabular-nums"
        >
          16
        </span>
        <div className="min-w-0 flex-1 space-y-1">
          <h2
            id="composed-heading"
            className="font-serif text-lg font-semibold tracking-tight"
          >
            Zusammengesetzte Seite
          </h2>
          <p className="max-w-prose text-sm text-secondary">
            Eine ganze Seite aus denselben Bausteinen: Laufkopf, Titelblock,
            Filterzeile, Register, Kennzahlen, Zeichnung, Liniensystem, Formular
            und Fußblock. Geprüft wird das Zusammenspiel — Dichte, Rangfolge und
            Nachbarschaft, nicht der einzelne Baustein.
          </p>
        </div>
      </header>

      <div aria-hidden="true" className="rule-ticks mb-4 h-px w-full" />

      <div className="mb-3 flex flex-wrap items-center justify-between gap-3">
        <ToggleGroup
          type="single"
          value={theme}
          onValueChange={(next) => {
            if (next) {
              setTheme(next as "light" | "dark");
            }
          }}
          aria-label="Theme der Seite"
        >
          <ToggleGroupItem value="light">Papier</ToggleGroupItem>
          <ToggleGroupItem value="dark">Lampe</ToggleGroupItem>
        </ToggleGroup>
        <code className="font-mono text-[10px] text-secondary">
          .{theme} · ganze Seite
        </code>
      </div>

      <div
        className={cn(
          theme,
          "paper overflow-hidden rounded-md border border-subtle bg-canvas text-ink",
        )}
      >
        <DemoPage />
      </div>
    </section>
  );
}
