"use client";

import { useEffect, useRef, useState } from "react";
import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import {
  AA_NORMAL_TEXT,
  contrastRatio,
} from "@/components/dev-gallery/contrast";
import { Badge } from "@/components/ui/badge";

/** Tokens worth showing as a swatch, in the order the design rule documents them. */
const COLOUR_TOKENS = [
  "bg-canvas",
  "bg-surface",
  "bg-elevated",
  "bg-surface-muted",
  "border-subtle",
  "border-strong",
  "text-primary",
  "text-secondary",
  "text-muted",
  "color-primary",
  "color-primary-hover",
  "color-primary-subtle",
  "primary-foreground",
  "semantic-impact",
  "semantic-warning",
  "semantic-success",
  "semantic-info",
] as const;

/**
 * Pairs the design system promises are readable. Every entry must clear AA. The
 * prescribed muted ink, amber, and strong contour do not clear AA as text, so they
 * are deliberately absent here: the rulebook restricts them to placeholders, marks,
 * and contours instead of pretending they pass.
 */
const CONTRAST_PAIRS = [
  { fg: "text-primary", bg: "bg-canvas", label: "Text / Seite" },
  { fg: "text-primary", bg: "bg-surface", label: "Text / Karte" },
  { fg: "text-secondary", bg: "bg-surface", label: "Nebentext / Karte" },
  { fg: "text-secondary", bg: "bg-surface-muted", label: "Nebentext / Fläche" },
  {
    fg: "primary-foreground",
    bg: "color-primary",
    label: "Primärtext / Primär",
  },
  { fg: "color-primary", bg: "bg-surface", label: "Marke / Karte" },
  { fg: "semantic-impact", bg: "bg-surface", label: "Einschlag / Karte" },
  { fg: "semantic-success", bg: "bg-surface", label: "Erfolg / Karte" },
  { fg: "semantic-info", bg: "bg-surface", label: "Info / Karte" },
] as const;

type Measurement = {
  label: string;
  fg: string;
  bg: string;
  ratio: number | null;
};

function readVar(element: Element, token: string): string {
  return getComputedStyle(element).getPropertyValue(`--${token}`).trim();
}

/**
 * Swatches plus live contrast checks for one theme.
 *
 * Reads the computed custom properties after mount, so the numbers always describe
 * what the browser actually paints. Values are re-read when the theme frame is
 * re-rendered, which is enough for the palette workflow.
 */
export function TokenSwatches() {
  return (
    <ComponentSection
      id="tokens"
      index="01"
      title="Tokens"
      description="Alle Farbwerte kommen aus globals.css. Die Kontrastwerte werden aus den gerenderten CSS-Variablen gemessen (WCAG 2.2 AA, normaler Text 4.5:1)."
    >
      <SwatchGrid />
    </ComponentSection>
  );
}

function SwatchGrid() {
  const probeRef = useRef<HTMLDivElement>(null);
  const [measurements, setMeasurements] = useState<Measurement[]>([]);

  useEffect(() => {
    const probe = probeRef.current;
    if (!probe) {
      return;
    }

    const next = CONTRAST_PAIRS.map(({ fg, bg, label }) => {
      const foreground = readVar(probe, fg);
      const background = readVar(probe, bg);
      return {
        label,
        fg: foreground,
        bg: background,
        ratio: contrastRatio(foreground, background),
      };
    });

    setMeasurements(next);
  }, []);

  return (
    <div ref={probeRef} className="space-y-4">
      <Example caption="Palette">
        {COLOUR_TOKENS.map((token) => (
          <div key={token} className="w-28 space-y-1">
            <div
              className="h-12 rounded-sm border border-subtle"
              style={{ background: `var(--${token})` }}
            />
            <code className="block font-mono text-[10px] break-all text-secondary">
              --{token}
            </code>
          </div>
        ))}
      </Example>

      <div className="space-y-1.5">
        <p className="text-xs font-medium text-secondary">Kontrast</p>
        <ul className="space-y-1">
          {measurements.map((entry) => {
            const passes =
              entry.ratio !== null && entry.ratio >= AA_NORMAL_TEXT;
            return (
              <li
                key={entry.label}
                className="flex items-center justify-between gap-2 text-xs"
              >
                <span>{entry.label}</span>
                <span className="flex items-center gap-2">
                  <code className="font-mono text-secondary">
                    {entry.ratio === null ? "–" : `${entry.ratio.toFixed(2)}:1`}
                  </code>
                  <Badge variant={passes ? "success" : "error"}>
                    {entry.ratio === null ? "?" : passes ? "AA" : "unter AA"}
                  </Badge>
                </span>
              </li>
            );
          })}
        </ul>
        {measurements.length === 0 ? (
          <p className="text-xs text-secondary">Wird gemessen…</p>
        ) : null}
      </div>
    </div>
  );
}
