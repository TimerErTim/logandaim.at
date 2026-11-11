"use client";

import { Check } from "lucide-react";
import type { CSSProperties } from "react";
import {
  PFLOCK_FARBE_STANDARD,
  PFLOCK_FARBEN,
} from "@/components/stammdaten/pflock-farbe-feld";
export function FreieFarbeFeld({
  farben,
  value,
  onChange,
  disabled = false,
  label = "Farbe",
}: {
  farben: string[];
  value: string | null;
  onChange: (farbe: string) => void;
  disabled?: boolean;
  label?: string;
}) {
  const options = [
    ...new Set(
      [...PFLOCK_FARBEN.map(({ farbe }) => farbe), ...farben].map((farbe) =>
        farbe.toLowerCase(),
      ),
    ),
  ];
  const current = value ?? PFLOCK_FARBE_STANDARD;

  return (
    <fieldset className="min-w-0 space-y-1.5">
      <legend className="min-h-[18px] text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
        {label}
      </legend>
      <div className="flex flex-wrap items-center gap-2">
        {options.map((farbe) => {
          const selected = value?.toLowerCase() === farbe;
          const name = PFLOCK_FARBEN.find(
            (entry) => entry.farbe === farbe,
          )?.name;
          return (
            <button
              key={farbe}
              type="button"
              aria-label={`${name ?? "Eigene Farbe"} ${farbe}`}
              aria-pressed={selected}
              disabled={disabled}
              onClick={() => onChange(farbe)}
              className={`relative inline-flex size-11 items-center justify-center rounded-sm border bg-[var(--pflock-farbe)] focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary disabled:cursor-not-allowed disabled:opacity-50 ${selected ? "border-primary" : "border-control hover:border-ink/60"}`}
              style={{ "--pflock-farbe": farbe } as CSSProperties}
            >
              {selected ? (
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
        <label className="flex items-center gap-2">
          <input
            type="color"
            value={current}
            onChange={(event) => onChange(event.target.value.toLowerCase())}
            disabled={disabled}
            aria-label="Eigene Farbe auswählen"
            className="h-11 w-16 cursor-pointer rounded-sm border border-control bg-elevated p-1 focus-visible:outline-2 focus-visible:outline-primary disabled:cursor-not-allowed disabled:bg-surface-muted [&::-webkit-color-swatch-wrapper]:p-0 [&::-webkit-color-swatch]:rounded-sm [&::-webkit-color-swatch]:border-0"
          />
          <span className="text-xs text-secondary">Eigene Farbe</span>
        </label>
      </div>
    </fieldset>
  );
}
