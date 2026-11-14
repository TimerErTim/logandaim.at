"use client";

import { useEffect, useState } from "react";
import type { RundeZiel, SchusspositionInput } from "@/api/logandaim-api";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/cn";
import { FreieFarbeFeld } from "./freie-farbe-feld";
import { PflockPlatzhalter } from "./pflock-platzhalter";

function FreieMeterEingabe({
  value,
  farbe,
  invalid,
  name,
  onChange,
}: {
  value: string;
  farbe: string | null;
  invalid: boolean;
  name: string;
  onChange: (value: string) => void;
}) {
  // The colour is the stored form while the field is untouched, so it stands in for the
  // number. Once the reader types, their own digits are what they must read; the swatch
  // stays visible beside them as the last valid stored form (AC-28).
  const farbeStattEingabe = farbe !== null && value === "";
  return (
    <div className="relative">
      <Input
        aria-label={name}
        inputMode="numeric"
        value={farbeStattEingabe ? "" : value}
        onChange={(event) => onChange(event.target.value)}
        placeholder={farbeStattEingabe ? "" : "Meter"}
        aria-invalid={invalid}
        className={cn(
          farbe && "pl-12",
          farbeStattEingabe && "text-transparent caret-transparent",
        )}
      />
      {farbe ? (
        <PflockPlatzhalter
          farbe={farbe}
          text={farbeStattEingabe ? "Pflock" : null}
        />
      ) : null}
    </div>
  );
}

export function FreiePositionSteuerung({
  ziel,
  farben,
  value,
  invalid,
  onValidityChange,
  onChange,
}: {
  ziel: RundeZiel;
  farben: string[];
  value: SchusspositionInput | null;
  invalid: boolean;
  onValidityChange: (invalid: boolean) => void;
  onChange: (position: SchusspositionInput | null) => void;
}) {
  const storedMeter =
    value && "Distanz" in value ? value.Distanz.meter : undefined;
  const storedFarbe =
    value && "Pflock" in value ? value.Pflock.farbe : undefined;
  const [meter, setMeter] = useState(
    storedMeter === undefined ? "" : String(storedMeter),
  );
  const [farbe, setFarbe] = useState<string | null>(storedFarbe ?? null);
  const [meterInvalid, setMeterInvalid] = useState(false);
  const validMeter = /^[1-9]\d*$/.test(meter) && Number(meter) <= 4_294_967_295;

  useEffect(() => {
    setMeter(storedMeter === undefined ? "" : String(storedMeter));
    setFarbe(storedFarbe ?? null);
    setMeterInvalid(false);
  }, [storedMeter, storedFarbe]);

  return (
    <div>
      <div className="flex min-w-0 items-center gap-3">
        <div className="min-w-0 flex-1">
          <FreieFarbeFeld
            farben={farben}
            value={farbe}
            onChange={(next) => {
              setFarbe(next);
              setMeter("");
              setMeterInvalid(false);
              onValidityChange(false);
              onChange({ Pflock: { farbe: next } });
            }}
          />
        </div>
        <div className="relative ml-auto w-20 shrink-0 sm:w-24">
          <FreieMeterEingabe
            value={meter}
            farbe={farbe}
            invalid={invalid || meterInvalid || (meter !== "" && !validMeter)}
            name={`Exakte Entfernung für ${ziel.tiername} in Metern`}
            onChange={(next) => {
              setMeter(next);
              if (next === "") {
                // A cleared field is no form at all, so it and the swatch go together:
                // the display never claims a stored colour that is gone.
                setFarbe(null);
                setMeterInvalid(false);
                onValidityChange(false);
                onChange(null);
              } else if (
                /^[1-9]\d*$/.test(next) &&
                Number(next) <= 4_294_967_295
              ) {
                setMeterInvalid(false);
                onValidityChange(false);
                // A valid meter is the sole stored form, so the colour is dropped here
                // and only here: an invalid edit must leave the last valid form (AC-28).
                setFarbe(null);
                onChange({ Distanz: { meter: Number(next) } });
              } else {
                setMeterInvalid(true);
                onValidityChange(true);
              }
            }}
          />
        </div>
      </div>
      {invalid || meterInvalid || (meter !== "" && !validMeter) ? (
        <p role="alert" className="text-xs text-impact">
          Bitte geben Sie eine positive ganze Meterzahl ein.
        </p>
      ) : null}
    </div>
  );
}
