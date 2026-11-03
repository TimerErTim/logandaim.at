"use client";

import { Check } from "lucide-react";

import { cn } from "@/lib/cn";
import { PflockMarker } from "./pflock-swatch";

/**
 * The club's own vocabulary of stake colours. These hex values are *data*, exactly
 * like the value stored on a Pflock: the vocabulary is what the staff recognise on
 * the range, not a palette the design system owns. They are therefore never tokens,
 * and they only ever reach the DOM through `--pflock-farbe`.
 */
export const PFLOCK_FARBEN = [
  { name: "Weiß", farbe: "#ffffff" },
  { name: "Blau", farbe: "#1b4f9c" },
  { name: "Rot", farbe: "#c0392b" },
  { name: "Gelb", farbe: "#e8b81c" },
  { name: "Grün", farbe: "#2e7d32" },
  { name: "Schwarz", farbe: "#1a1a1a" },
  { name: "Orange", farbe: "#d9731a" },
] as const;

/** The colour a newly created Parkour starts with. */
export const PFLOCK_FARBE_STANDARD = "#c0392b";

/** `<input type="color">` takes only a full `#rrggbb`, so it never gets anything else. */
const HEX = /^#[0-9a-f]{6}$/;

type PflockFarbeFeldProps = {
  /** Visible field name; the `<legend>` names the group. */
  label: string;
  /** Stored value, a lowercase `#rrggbb`. */
  value: string;
  onValueChange: (farbe: string) => void;
  disabled?: boolean;
  className?: string;
};

/**
 * Picks a Pflock's colour: the club's vocabulary as swatches, plus the platform's
 * colour picker for anything outside it.
 *
 * The buttons are a group of choices, not one control, so this is a `fieldset` with its
 * own `<legend>` rather than a `Field`: `Field` attaches a `<label>` to a single
 * control, and a label pointing at a `fieldset` is the unattached-caption failure it
 * exists to prevent. The legend copies `Field`'s label row (18px tall, tracked caps,
 * secondary ink), so the picker starts its controls on the same line as a neighbouring
 * field in a `FormRow`.
 *
 * A swatch alone is not a name, so every option carries its German colour name as its
 * accessible name, and the chosen one is marked by a heavier `primary` contour, an ink
 * check on a paper chip, and `aria-pressed` — never by colour alone. The stored value
 * stays readable in mono beside them, because the colour is data a person has to
 * compare and quote.
 */
export function PflockFarbeFeld({
  label,
  value,
  onValueChange,
  disabled = false,
  className,
}: PflockFarbeFeldProps) {
  const gewaehlt = value.toLowerCase();
  const gueltig = HEX.test(gewaehlt);

  return (
    <fieldset className={cn("min-w-0 space-y-1.5", className)}>
      <legend className="flex min-h-[18px] items-center text-[11px] font-semibold tracking-[0.07em] text-secondary uppercase">
        {label}
      </legend>
      <div className="space-y-2">
        <div className="flex flex-wrap items-center gap-2">
          {PFLOCK_FARBEN.map((eintrag) => {
            const istGewaehlt = eintrag.farbe === gewaehlt;
            return (
              <button
                key={eintrag.farbe}
                type="button"
                aria-pressed={istGewaehlt}
                disabled={disabled}
                onClick={() => onValueChange(eintrag.farbe)}
                // Drawn at the 44px touch minimum, so no `::before` overlay is needed.
                className={cn(
                  "relative inline-flex size-11 items-center justify-center rounded-sm border bg-[var(--pflock-farbe)]",
                  "transition-colors",
                  "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
                  "disabled:cursor-not-allowed disabled:opacity-50",
                  istGewaehlt
                    ? "border-primary"
                    : "border-control hover:border-ink/60",
                )}
                // The documented inline style that names data, not a token.
                style={
                  { "--pflock-farbe": eintrag.farbe } as React.CSSProperties
                }
              >
                <span className="sr-only">{eintrag.name}</span>
                {istGewaehlt ? (
                  // A paper chip with an ink check: the mark sits on paper, not on the
                  // swatch, so it survives every colour, pale or saturated alike.
                  <span
                    aria-hidden="true"
                    className="flex size-5 items-center justify-center rounded-sm border border-subtle bg-surface text-primary"
                  >
                    <Check className="size-3" />
                  </span>
                ) : null}
              </button>
            );
          })}
        </div>

        <div className="flex flex-wrap items-center gap-4">
          <PflockMarker farbe={gewaehlt} />
          <span className="flex items-center gap-2">
            <input
              type="color"
              value={gueltig ? gewaehlt : "#000000"}
              onChange={(event) =>
                onValueChange(event.target.value.toLowerCase())
              }
              disabled={disabled}
              aria-label="Eigene Farbe"
              className={cn(
                // A replaced element, so it is drawn at 44px rather than grown by an
                // overlay; the contour is the control contour, like every field.
                "h-11 w-16 cursor-pointer rounded-sm border border-control bg-elevated p-1",
                "transition-colors hover:border-ink/40",
                "focus-visible:border-primary focus-visible:outline-2 focus-visible:outline-offset-0 focus-visible:outline-primary",
                "disabled:cursor-not-allowed disabled:bg-surface-muted",
                "[&::-webkit-color-swatch-wrapper]:p-0",
                "[&::-webkit-color-swatch]:rounded-sm [&::-webkit-color-swatch]:border-0",
              )}
            />
            <span className="text-xs text-secondary">Eigene Farbe</span>
          </span>
        </div>
      </div>
    </fieldset>
  );
}
