"use client";

import Link from "next/link";
import { useState } from "react";
import { touchTargetOutline } from "@/components/dev-gallery/component-section";
import { DataFiguresSection } from "@/components/dev-gallery/data-figures-section";
import { InkedControlsSection } from "@/components/dev-gallery/inked-controls-section";
import { PaperFurnitureSection } from "@/components/dev-gallery/paper-furniture-section";
import {
  AlertSection,
  BadgeSection,
  ButtonSection,
  CardSection,
  DataSection,
  FormSection,
  MiscSection,
  OverlaySection,
} from "@/components/dev-gallery/sections";
import {
  CheckpointSliderSection,
  SortableTableSection,
} from "@/components/dev-gallery/sortable-table-section";
import { TokenSwatches } from "@/components/dev-gallery/token-swatches";
import { TypeSection } from "@/components/dev-gallery/type-section";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";

/**
 * The design gallery: the visual identity reviewed as a logbook of itself.
 *
 * Exists only under `next dev`. The file is `page.dev.tsx`, and `next.config.ts`
 * adds `dev.tsx` to `pageExtensions` for the development phase only, so `next build`
 * never sees it and nothing here ships.
 *
 * The workflow it serves: change a token in `globals.css`, this page reloads, and
 * both themes are judged side by side on the same specimen — no switching, no
 * memory. Decisions are then written into `.cursor/rules/design-system.mdc` and
 * `specs/product/visual-identity.md`.
 */

/** The plate index, in the order the system is read: foundation, then parts. */
const PLATES = [
  { id: "tokens", index: "01", label: "Tokens" },
  { id: "type", index: "02", label: "Schrift" },
  { id: "sortable-table", index: "03", label: "Sortierbare Tabelle" },
  { id: "figures", index: "04", label: "Datenbilder" },
  { id: "checkpoint-slider", index: "05", label: "Bereichs-Slider" },
  { id: "button", index: "06", label: "Button" },
  { id: "badge", index: "07", label: "Badge" },
  { id: "alert", index: "08", label: "Alert" },
  { id: "card", index: "09", label: "Card" },
  { id: "form", index: "10", label: "Formular" },
  { id: "data", index: "11", label: "Tabelle & Maß" },
  { id: "overlay", index: "12", label: "Überlagerung" },
  { id: "misc", index: "13", label: "Kleinteile" },
  { id: "furniture", index: "14", label: "Papier-Möbel" },
  { id: "inked", index: "15", label: "Markierte Bedienelemente" },
] as const;

export default function DevGalleryPage() {
  const [showTouchTargets, setShowTouchTargets] = useState(false);

  return (
    <div className="min-h-full bg-canvas">
      <SetBreadcrumbLabel label="Design-Galerie" />

      <div className="mx-auto max-w-6xl px-6 py-10">
        {/* Title block: a logbook's cover page. */}
        <header className="space-y-4">
          <div className="flex flex-wrap items-center gap-2">
            <Badge variant="primary">Sleek Paperstyle</Badge>
            <Badge variant="outline">Nur im Dev-Server</Badge>
            <span className="font-mono text-xs text-secondary">15 Tafeln</span>
          </div>

          <div className="space-y-2">
            <h1 className="font-serif text-3xl leading-tight font-semibold tracking-tight text-balance">
              Gestaltungs-Galerie
            </h1>
            <p className="max-w-prose text-sm text-secondary">
              Die visuelle Identität als Musterbuch. Jede Tafel zeigt dieselben
              Bausteine zweimal: links auf warmem Papier, rechts im Lampenlicht.
              Farben, Radien und Schatten kommen ausschließlich aus{" "}
              <code className="font-mono text-xs text-ink">globals.css</code>.
            </p>
          </div>

          <Separator />
        </header>

        {/* Index: a printed table of contents, not a nav bar. */}
        <nav aria-label="Tafeln" className="py-6">
          <ol className="grid gap-x-8 gap-y-1.5 sm:grid-cols-2 lg:grid-cols-3">
            {PLATES.map((plate) => (
              <li key={plate.id}>
                <a
                  href={`#${plate.id}`}
                  className="group flex items-baseline gap-2 rounded-sm py-0.5 text-sm transition-colors hover:text-primary focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary"
                >
                  <span className="font-mono text-[11px] text-secondary tabular-nums">
                    {plate.index}
                  </span>
                  <span>{plate.label}</span>
                  <span
                    aria-hidden="true"
                    className="leader mb-1 h-2 flex-1 opacity-50"
                  />
                </a>
              </li>
            ))}
          </ol>
        </nav>

        <Separator />

        {/* Toolbar: the one control that is about the gallery, not the system. */}
        <div className="flex flex-wrap items-center justify-between gap-3 py-4">
          <label className="flex min-h-11 cursor-pointer items-center gap-3 text-sm">
            <input
              type="checkbox"
              checked={showTouchTargets}
              onChange={(event) => setShowTouchTargets(event.target.checked)}
              className="checkbox-tick size-4 appearance-none rounded-sm border border-subtle bg-surface"
            />
            44 px Touch-Ziele markieren
          </label>
          <Button variant="ghost" size="sm" asChild>
            <Link href="/">Zur App</Link>
          </Button>
        </div>

        <div className="space-y-14">
          <TokenSwatches />
          <TypeSection />
          <SortableTableSection />
          <DataFiguresSection />
          <CheckpointSliderSection />
          <div className={showTouchTargets ? touchTargetOutline : undefined}>
            <div className="space-y-14">
              <ButtonSection />
              <BadgeSection />
              <AlertSection />
              <CardSection />
              <FormSection />
              <DataSection />
              <OverlaySection />
              <MiscSection />
              <PaperFurnitureSection />
              <InkedControlsSection />
            </div>
          </div>
        </div>

        {/* Colophon: the page signs itself off, as a printed specimen sheet does. */}
        <footer className="mt-16 border-t border-subtle pt-4">
          <p className="flex flex-wrap items-center gap-x-3 gap-y-1 font-mono text-[11px] text-secondary">
            <span>Lora · Inter · Geist Mono</span>
            <span aria-hidden="true">·</span>
            <span>WCAG 2.2 AA</span>
            <span aria-hidden="true">·</span>
            <span>44 px Touch</span>
          </p>
        </footer>
      </div>
    </div>
  );
}
