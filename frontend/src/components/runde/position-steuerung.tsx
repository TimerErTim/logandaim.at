"use client";

import type { CSSProperties } from "react";
import { useEffect, useMemo, useRef, useState } from "react";

import type {
  RundePflock,
  RundeZiel,
  SchusspositionInput,
} from "@/api/logandaim-api";
import { PflockSwatch } from "@/components/stammdaten/pflock-swatch";
import {
  type Checkpoint,
  CheckpointSlider,
} from "@/components/ui/checkpoint-slider";
import { Input } from "@/components/ui/input";
import { formatPosition, positionsModus } from "@/lib/runden-position";

export function PositionSteuerung({
  ziel,
  pflöcke,
  value,
  onChange,
}: {
  ziel: RundeZiel;
  pflöcke: RundePflock[];
  value: SchusspositionInput | null;
  onChange: (position: SchusspositionInput | null) => void;
}) {
  const modus = positionsModus(ziel, pflöcke.length);
  const abstandByFarbe = useMemo(
    () =>
      new Map(ziel.abstände.map((abstand) => [abstand.pflock, abstand.meter])),
    [ziel.abstände],
  );
  const checkpoints = useMemo<Checkpoint[]>(
    () =>
      [...pflöcke]
        .sort((a, b) => a.position - b.position)
        .map((pflock, index) => ({
          id: `${pflock.position}-${pflock.farbe}`,
          wert:
            modus === "meter"
              ? (abstandByFarbe.get(pflock.farbe) ?? index + 1)
              : pflock.position + 1,
          farbe: pflock.farbe,
          label: pflock.farbe,
          anzeige:
            modus === "meter" && abstandByFarbe.has(pflock.farbe)
              ? `${abstandByFarbe.get(pflock.farbe)} m`
              : undefined,
        }))
        .sort((a, b) => a.wert - b.wert),
    [pflöcke, modus, abstandByFarbe],
  );
  const min =
    modus === "meter"
      ? Math.min(...checkpoints.map((checkpoint) => checkpoint.wert))
      : 1;
  const max =
    modus === "meter"
      ? Math.max(...checkpoints.map((checkpoint) => checkpoint.wert))
      : Math.max(1, pflöcke.length);
  const positionPflock =
    value && "Pflock" in value
      ? pflöcke.find((pflock) => pflock.farbe === value.Pflock.farbe)
      : undefined;
  const meterFürPflock = positionPflock
    ? abstandByFarbe.get(positionPflock.farbe)
    : undefined;
  const aktuellerMeter =
    value && "Distanz" in value
      ? value.Distanz.meter
      : modus === "meter" && meterFürPflock !== undefined
        ? meterFürPflock
        : undefined;
  const sliderWert =
    modus === "meter"
      ? Math.min(max, Math.max(min, aktuellerMeter ?? min))
      : positionPflock
        ? (checkpoints.find((checkpoint) =>
            checkpoint.id.endsWith(`-${positionPflock.farbe}`),
          )?.wert ?? 1)
        : 1;
  const sliderVertretbar =
    modus === "meter"
      ? (aktuellerMeter === undefined ||
          (aktuellerMeter >= min && aktuellerMeter <= max)) &&
        checkpoints.length >= 2 &&
        max > min
      : modus === "staken"
        ? !value || "Pflock" in value
        : false;

  const initialEingabe =
    aktuellerMeter !== undefined &&
    (modus !== "meter" || (aktuellerMeter >= min && aktuellerMeter <= max))
      ? String(aktuellerMeter)
      : "";
  const [eingabe, setEingabe] = useState(initialEingabe);
  const vorigeEingabe = useRef(initialEingabe);
  const [letzteEingabe, setLetzteEingabe] = useState(initialEingabe);
  useEffect(() => {
    if (initialEingabe !== vorigeEingabe.current) {
      vorigeEingabe.current = initialEingabe;
      if (letzteEingabe !== initialEingabe) {
        setEingabe(letzteEingabe);
      }
    }
  }, [initialEingabe, letzteEingabe]);

  const pflockFarbe =
    modus === "staken" && eingabe === "" && value && "Pflock" in value
      ? positionPflock?.farbe
      : undefined;
  const eingabeUngueltig = eingabe !== "" && !/^[1-9]\d*$/.test(eingabe);

  return (
    <fieldset className="min-w-0 space-y-3">
      <legend className="min-h-[18px] text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
        Entfernung
      </legend>
      {modus !== "single" ? (
        <CheckpointSlider
          checkpoints={checkpoints}
          value={sliderWert}
          onValueChange={(next) => {
            if (modus === "meter") {
              const distanz = Math.max(1, Math.round(next));
              onChange({ Distanz: { meter: distanz } });
              const nächsteEingabe = String(distanz);
              setEingabe(nächsteEingabe);
              setLetzteEingabe(nächsteEingabe);
            } else {
              const pflock =
                pflöcke[
                  Math.max(
                    0,
                    Math.min(pflöcke.length - 1, Math.round(next) - 1),
                  )
                ];
              if (pflock) {
                onChange({ Pflock: { farbe: pflock.farbe } });
                setEingabe("");
                setLetzteEingabe("");
              }
            }
          }}
          min={min}
          max={max}
          step={1}
          label="Entfernung"
          einheit={modus === "meter" ? "m" : undefined}
          inactive={!sliderVertretbar}
        />
      ) : (
        <p className="text-sm text-secondary">
          Position: {formatPosition(value)}
        </p>
      )}
      <div
        className="relative"
        style={
          pflockFarbe
            ? ({ "--pflock-farbe": pflockFarbe } as CSSProperties)
            : undefined
        }
      >
        <Input
          aria-label="Entfernung in Metern"
          aria-invalid={eingabeUngueltig}
          inputMode="numeric"
          value={eingabe}
          onChange={(event) => {
            const next = event.target.value;
            setEingabe(next);
            setLetzteEingabe(next);
            if (/^[1-9]\d*$/.test(next)) {
              onChange({ Distanz: { meter: Number(next) } });
            }
          }}
          placeholder={pflockFarbe ? "" : "Meter"}
          onFocus={() => {
            if (pflockFarbe) {
              setEingabe("");
              setLetzteEingabe("");
            }
          }}
          className={
            pflockFarbe
              ? "border-transparent text-transparent caret-transparent"
              : undefined
          }
        />
        {pflockFarbe ? (
          <PflockSwatch
            farbe={pflockFarbe}
            className="pointer-events-none absolute inset-0 size-full rounded-sm border-0 bg-[var(--pflock-farbe)]"
          />
        ) : null}
      </div>
      {eingabeUngueltig ? (
        <p role="alert" className="text-xs text-impact">
          Bitte geben Sie eine positive ganze Meterzahl ein.
        </p>
      ) : null}
    </fieldset>
  );
}
