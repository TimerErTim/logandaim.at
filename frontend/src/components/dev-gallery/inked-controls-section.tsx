"use client";

import { useState } from "react";

import {
  ComponentSection,
  Example,
} from "@/components/dev-gallery/component-section";
import { Checkbox } from "@/components/ui/checkbox";
import { RadioGroup, RadioGroupOption } from "@/components/ui/radio-group";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";

/**
 * The inked control family: every control that is drawn rather than typed is set
 * with a strong contour, so a form reads as marks made on paper rather than as a
 * browser's default chrome.
 *
 * The slider bullet and the radio are the two round members; roundness is reserved
 * for the controls where the shape carries meaning (a cursor on a rule, one choice
 * among several).
 */
export function InkedControlsSection() {
  const [distanz, setDistanz] = useState(42);
  const [scale, setScale] = useState("beta");

  return (
    <ComponentSection
      id="inked"
      index="15"
      title="Markierte Bedienelemente"
      description="Alles, was gezeichnet statt getippt wird, trägt eine kräftige Kontur: ein Häkchen ist eine gemalte Marke, kein Haarlinienkästchen. Rund bleibt nur, wo die Form die Bedeutung trägt."
    >
      <Example
        caption="Slider — runde Kugel mit kräftiger Kontur auf geteilter Skala"
        className="block"
      >
        <div className="w-full space-y-2">
          <Slider
            value={[distanz]}
            onValueChange={([next]) => setDistanz(next ?? distanz)}
            min={10}
            max={80}
            step={1}
            aria-label="Abstand"
          />
          <div className="flex items-baseline justify-between font-mono text-xs text-secondary tabular-nums">
            <span>10</span>
            <span className="text-sm font-semibold text-ink">{distanz} m</span>
            <span>80</span>
          </div>
        </div>
      </Example>

      <Example
        caption="Prüfkreis — eine von mehreren Optionen"
        className="block"
      >
        <RadioGroup
          value={scale}
          onValueChange={setScale}
          aria-label="Auswahl"
          className="gap-0"
        >
          <RadioGroupOption
            value="alpha"
            label="Alpha — gewählt wird gefüllt"
          />
          <RadioGroupOption value="beta" label="Beta" />
          <RadioGroupOption value="gamma" label="Gamma" />
        </RadioGroup>
      </Example>

      <Example
        caption="Häkchen — eckig, weil Mehrfachauswahl"
        className="block"
      >
        <div className="space-y-1">
          <Checkbox label="Erster Eintrag" defaultChecked />
          <Checkbox label="Zweiter Eintrag" />
          <Checkbox label="Deaktiviert" disabled />
        </div>
      </Example>

      <Example caption="Schalter — die Ausnahme, weil die Form einen Weg beschreibt">
        <div className="flex min-h-11 items-center gap-3">
          <Switch aria-labelledby="gallery-inked-switch" defaultChecked />
          <span id="gallery-inked-switch" className="text-sm text-ink">
            Details anzeigen
          </span>
        </div>
        <div className="flex min-h-11 items-center gap-3">
          <Switch aria-labelledby="gallery-inked-switch-off" />
          <span id="gallery-inked-switch-off" className="text-sm text-ink">
            Aus
          </span>
        </div>
      </Example>
    </ComponentSection>
  );
}
