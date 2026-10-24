"use client";

import Link from "next/link";
import { useState } from "react";

import { GallerySections } from "@/components/dev-gallery/sections";
import { TypeSection } from "@/components/dev-gallery/type-section";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";

/**
 * Component gallery and visual identity review surface.
 *
 * Exists only under `next dev`: the file is `page.dev.tsx`, and `next.config.ts`
 * adds `dev.tsx` to `pageExtensions` for the development phase only. `next build`
 * does not see it, so nothing here ships.
 *
 * The workflow it serves: change a token in `globals.css`, this page reloads, the
 * PO looks at both themes, and the decision is written into
 * `.cursor/rules/design-system.mdc` and `specs/product/visual-identity.md`.
 */
export default function DevGalleryPage() {
  const [showTouchTargets, setShowTouchTargets] = useState(false);

  return (
    <div className="mx-auto max-w-6xl space-y-6 p-6">
      <SetBreadcrumbLabel label="Design-Galerie" />

      <header className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">
          Design-Galerie
        </h1>
        <p className="max-w-prose text-sm text-muted-foreground">
          Jede Komponente in jeder Variante, links hell und rechts dunkel. Diese
          Seite existiert nur im Dev-Server. Farben und Radien kommen aus{" "}
          <code className="font-mono text-xs">globals.css</code>; jede Änderung
          erscheint hier nach dem Hot-Reload.
        </p>
        <Alert variant="info">
          Nicht im statischen Export enthalten: die Route wird über{" "}
          <code className="font-mono text-xs">pageExtensions</code> nur in der
          Dev-Phase aufgelöst.
        </Alert>
      </header>

      <div className="flex flex-wrap items-center gap-3">
        <label className="flex min-h-11 cursor-pointer items-center gap-3 text-sm">
          <input
            type="checkbox"
            checked={showTouchTargets}
            onChange={(event) => setShowTouchTargets(event.target.checked)}
            className="size-4 accent-primary"
          />
          44px-Touch-Ziele markieren
        </label>
        <Button variant="ghost" asChild>
          <Link href="/">Zur App</Link>
        </Button>
      </div>

      <div className="space-y-10">
        <TypeSection />
        <GallerySections showTouchTargets={showTouchTargets} />
      </div>
    </div>
  );
}
