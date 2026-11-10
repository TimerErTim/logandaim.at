"use client";

import { useEffect, useState } from "react";
import type { RundeZiel, SchusspositionInput } from "@/api/logandaim-api";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { FreieFarbeFeld } from "./freie-farbe-feld";

export function FreiePositionSteuerung({
  ziel,
  farben,
  value,
  onChange,
}: {
  ziel: RundeZiel;
  farben: string[];
  value: SchusspositionInput | null;
  onChange: (position: SchusspositionInput | null) => void;
}) {
  const [meter, setMeter] = useState(
    value && "Distanz" in value ? String(value.Distanz.meter) : "",
  );
  const [farbe, setFarbe] = useState<string | null>(
    value && "Pflock" in value ? value.Pflock.farbe : null,
  );
  useEffect(() => {
    setMeter(value && "Distanz" in value ? String(value.Distanz.meter) : "");
    setFarbe(value && "Pflock" in value ? value.Pflock.farbe : null);
  }, [value]);

  const validMeter = /^[1-9]\d*$/.test(meter);
  return (
    <div className="space-y-4">
      <FreieFarbeFeld
        farben={farben}
        value={farbe}
        onChange={(next) => {
          setFarbe(next);
          setMeter("");
          onChange({ Pflock: { farbe: next } });
        }}
      />
      <Field
        label="Exakte Entfernung"
        optional
        hint="Ganze Meter; eine Entfernung ersetzt die gewählte Farbe."
      >
        <Input
          aria-label={`Exakte Entfernung für ${ziel.tiername} in Metern`}
          inputMode="numeric"
          value={meter}
          onChange={(event) => {
            const next = event.target.value;
            setMeter(next);
            if (next === "") {
              if (farbe) onChange({ Pflock: { farbe } });
              else onChange(null);
            } else if (/^[1-9]\d*$/.test(next)) {
              setFarbe(null);
              onChange({ Distanz: { meter: Number(next) } });
            }
          }}
          placeholder="Meter"
          aria-invalid={meter !== "" && !validMeter}
        />
      </Field>
      {meter !== "" && !validMeter ? (
        <p role="alert" className="text-xs text-impact">
          Bitte geben Sie eine positive ganze Meterzahl ein.
        </p>
      ) : null}
    </div>
  );
}
