"use client";

import * as SliderPrimitive from "@radix-ui/react-slider";
import { useId } from "react";

import { cn } from "@/lib/cn";

/**
 * A named checkpoint on the track. `farbe` is a colour supplied by the caller, not
 * a design token, so it is rendered as given.
 */
export type Checkpoint = {
  id: string;
  /** Position on the value scale. */
  wert: number;
  /** Caller-supplied band colour, for example `#3f6b8a`. */
  farbe: string;
  /** Name shown in the legend; colour is never the only signal. */
  label: string;
};

type CheckpointSliderProps = {
  checkpoints: Checkpoint[];
  value: number;
  onValueChange: (next: number) => void;
  min: number;
  max: number;
  step?: number;
  /** Accessible name of the slider. */
  label: string;
  /** Unit appended to the read-out and to `aria-valuetext`. */
  einheit?: string;
  disabled?: boolean;
  className?: string;
};

/**
 * Continuous slider whose track is divided into the coloured bands between
 * master-data checkpoints, like the stop bar of a colour picker.
 *
 * The thumb moves freely: the value is a distance, not a stake, so it must not snap.
 * Bands are decoration and are `aria-hidden`; the legend repeats every checkpoint
 * with its name, and the thumb announces the distance plus the nearest stake, so the
 * control is usable without seeing the colours.
 *
 * The band between two stakes carries the colour of the stake below it, which makes
 * each band mean "from this stake up to the next one".
 */
export function CheckpointSlider({
  checkpoints,
  value,
  onValueChange,
  min,
  max,
  step = 1,
  label,
  einheit,
  disabled = false,
  className,
}: CheckpointSliderProps) {
  const legendId = useId();
  const span = max - min;

  const sorted = [...checkpoints].sort((a, b) => a.wert - b.wert);
  const position = (wert: number) =>
    span === 0 ? 0 : Math.min(100, Math.max(0, ((wert - min) / span) * 100));

  /*
   * The track is a colour ramp through the stake colours, not a row of flat bands:
   * the gradient blends from one stake to the next, the way a colour-picker stop
   * bar does. Before the first stake and after the last one the ramp holds that
   * stake's colour, so the track never ends in an unrelated hue.
   *
   * Positions come from each stake's `wert`, so unevenly spaced Pflöcke produce
   * unevenly wide sections. With no stake the track stays plain `secondary`; the
   * control still works, because the distance is the value and the stakes are only
   * a reference.
   */
  const farbe =
    sorted.length === 0
      ? "var(--bg-surface-muted)"
      : `linear-gradient(to right, ${[
          `${sorted[0]?.farbe} 0%`,
          ...sorted.map(
            (eintrag) => `${eintrag.farbe} ${position(eintrag.wert)}%`,
          ),
          `${sorted[sorted.length - 1]?.farbe} 100%`,
        ].join(", ")})`;

  const nächster = sorted.reduce<Checkpoint | null>((bisher, eintrag) => {
    if (!bisher) {
      return eintrag;
    }
    return Math.abs(eintrag.wert - value) < Math.abs(bisher.wert - value)
      ? eintrag
      : bisher;
  }, null);

  const einheitText = einheit ? ` ${einheit}` : "";
  const wertText = `${value}${einheitText}`;
  const valuetext = nächster
    ? `${wertText}, nächster Pflock ${nächster.label}`
    : wertText;

  return (
    <div className={cn("w-full space-y-2", className)}>
      <SliderPrimitive.Root
        className="relative flex h-11 w-full touch-none items-center select-none data-[disabled]:opacity-50"
        disabled={disabled}
        value={[value]}
        onValueChange={([next]) => {
          if (typeof next === "number") {
            onValueChange(next);
          }
        }}
        min={min}
        max={max}
        step={step}
      >
        <SliderPrimitive.Track className="relative h-3 w-full grow overflow-hidden rounded-sm border border-subtle">
          <span
            aria-hidden="true"
            className="absolute inset-0"
            style={{ background: farbe }}
          />
          {/*
            Stops are drawn in the foreground colour, not in white: a stake colour
            may be pale (Weiß) or saturated (Rot), and the tick has to survive both.
          */}
          <span aria-hidden="true" className="absolute inset-0">
            {sorted.map((eintrag) => (
              <span
                key={eintrag.id}
                className="absolute top-0 h-full w-px bg-ink/40"
                style={{ left: `${position(eintrag.wert)}%` }}
              />
            ))}
          </span>
        </SliderPrimitive.Track>
        <SliderPrimitive.Thumb
          aria-label={label}
          aria-valuetext={valuetext}
          aria-describedby={legendId}
          className={cn(
            // The same round, inked bullet as `Slider`, so both sliders read alike.
            // The contour is ink rather than primary: the track underneath is a ramp
            // of caller-supplied stake colours, and ink survives every one of them.
            "relative block size-5 cursor-grab rounded-full border-2 border-ink bg-surface",
            "after:absolute after:inset-0 after:m-auto after:size-1.5 after:rounded-full",
            "after:bg-ink after:content-['']",
            "before:absolute before:-inset-[16px] before:content-['']",
            "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary",
            "active:cursor-grabbing disabled:pointer-events-none disabled:cursor-not-allowed",
          )}
        />
      </SliderPrimitive.Root>

      <ul
        id={legendId}
        className="flex flex-wrap items-center gap-x-4 gap-y-1 text-xs"
      >
        {sorted.map((eintrag) => (
          <li key={eintrag.id} className="flex items-center gap-1.5">
            <span
              aria-hidden="true"
              className="size-3 rounded-sm border border-subtle"
              style={{ background: eintrag.farbe }}
            />
            <span className="font-medium text-ink">{eintrag.label}</span>
            <span className="text-secondary tabular-nums">
              {eintrag.wert}
              {einheitText}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}
