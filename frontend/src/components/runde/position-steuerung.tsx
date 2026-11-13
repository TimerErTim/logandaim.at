"use client";

import type { CSSProperties } from "react";
import { useEffect, useMemo, useRef, useState } from "react";

import type { RundeZiel, SchusspositionInput } from "@/api/logandaim-api";
import { PflockSwatch } from "@/components/stammdaten/pflock-swatch";
import {
  type Checkpoint,
  CheckpointSlider,
} from "@/components/ui/checkpoint-slider";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/cn";
import { formatPosition, positionsModus } from "@/lib/runden-position";
import { PflockPlatzhalter } from "./pflock-platzhalter";

export function PositionSteuerung({
  ziel,
  value,
  onChange,
}: {
  ziel: RundeZiel;
  value: SchusspositionInput | null;
  onChange: (position: SchusspositionInput | null) => void;
}) {
  const effektivePflöcke = ziel.pflöcke;
  const modus = positionsModus(ziel, effektivePflöcke.length);
  const abstandByFarbe = useMemo(
    () =>
      new Map(
        ziel.pflöcke.flatMap((pflock) =>
          pflock.abstand === undefined
            ? []
            : [[pflock.farbe, pflock.abstand] as const],
        ),
      ),
    [ziel.pflöcke],
  );
  const checkpoints = useMemo<Checkpoint[]>(
    () =>
      effektivePflöcke
        .map((pflock, index) => ({
          id: `${index}-${pflock.farbe}`,
          wert:
            modus === "meter"
              ? (abstandByFarbe.get(pflock.farbe) ?? index + 1)
              : index + 1,
          farbe: pflock.farbe,
          label: pflock.farbe,
          anzeige:
            modus === "meter" && abstandByFarbe.has(pflock.farbe)
              ? `${abstandByFarbe.get(pflock.farbe)} m`
              : undefined,
        }))
        .sort((a, b) => a.wert - b.wert),
    [effektivePflöcke, modus, abstandByFarbe],
  );
  const min =
    modus === "meter"
      ? Math.min(...checkpoints.map((checkpoint) => checkpoint.wert))
      : 1;
  const max =
    modus === "meter"
      ? Math.max(...checkpoints.map((checkpoint) => checkpoint.wert))
      : Math.max(1, effektivePflöcke.length);
  const positionPflock =
    value && "Pflock" in value
      ? effektivePflöcke.find((pflock) => pflock.farbe === value.Pflock.farbe)
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
            checkpoint.id.startsWith(
              `${effektivePflöcke.indexOf(positionPflock)}-`,
            ),
          )?.wert ?? 1)
        : 1;
  const sliderAuflösbar =
    modus === "meter"
      ? checkpoints.length >= 2 &&
        max > min &&
        (aktuellerMeter === undefined ||
          (aktuellerMeter >= min && aktuellerMeter <= max))
      : modus === "staken"
        ? !value || "Pflock" in value
        : false;

  const initialEingabe =
    aktuellerMeter !== undefined ? String(aktuellerMeter) : "";
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
    value && "Pflock" in value ? positionPflock?.farbe : undefined;
  const pflockHatDistanz =
    positionPflock !== undefined && abstandByFarbe.has(positionPflock.farbe);
  const pflockPlaceholder =
    pflockFarbe !== undefined && !pflockHatDistanz && eingabe === "";
  const eingabeUngueltig = eingabe !== "" && !/^[1-9]\d*$/.test(eingabe);

  return (
    <fieldset className="min-w-0 space-y-1.5">
      <legend className="min-h-[18px] text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
        Entfernung
      </legend>
      <div className="flex min-w-0 items-center gap-3">
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
                  effektivePflöcke[
                    Math.max(
                      0,
                      Math.min(
                        effektivePflöcke.length - 1,
                        Math.round(next) - 1,
                      ),
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
            thumbHidden={!sliderAuflösbar}
          />
        ) : (
          <p className="text-sm text-secondary">
            Position: {formatPosition(value)}
          </p>
        )}
        <div
          className="relative ml-auto w-20 shrink-0 sm:w-24"
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
            placeholder={pflockPlaceholder ? "" : "Meter"}
            onFocus={() => {
              if (pflockFarbe && !pflockHatDistanz) {
                setEingabe("");
                setLetzteEingabe("");
              }
            }}
            className={cn(
              "w-full px-2 tabular-nums",
              pflockPlaceholder && "text-transparent caret-ink",
              pflockFarbe &&
                pflockHatDistanz &&
                "border-transparent bg-transparent text-transparent caret-transparent",
            )}
          />
          {pflockPlaceholder ? <PflockPlatzhalter farbe={pflockFarbe} /> : null}
          {pflockFarbe && pflockHatDistanz ? (
            <PflockSwatch
              farbe={pflockFarbe}
              className="pointer-events-none absolute inset-0 size-full rounded-sm border-0 bg-[var(--pflock-farbe)]"
            />
          ) : null}
        </div>
      </div>
      {eingabeUngueltig ? (
        <p role="alert" className="text-xs text-impact">
          Bitte geben Sie eine positive ganze Meterzahl ein.
        </p>
      ) : null}
    </fieldset>
  );
}
