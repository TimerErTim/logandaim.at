"use client";

import { useMemo, useState } from "react";

import type {
  RundePflock,
  RundeZiel,
  SchusspositionInput,
} from "@/api/logandaim-api";
import {
  type Checkpoint,
  CheckpointSlider,
} from "@/components/ui/checkpoint-slider";
import { Field } from "@/components/ui/field";
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
  const abstaende = ziel.abstände;
  const abstandByFarbe = useMemo(
    () => new Map(abstaende.map((abstand) => [abstand.pflock, abstand.meter])),
    [abstaende],
  );
  const checkpoints = useMemo<Checkpoint[]>(
    () =>
      pflöcke.map((pflock, index) => ({
        id: `${pflock.position}-${pflock.farbe}`,
        wert:
          modus === "meter"
            ? (abstandByFarbe.get(pflock.farbe) ?? index + 1)
            : index + 1,
        farbe: pflock.farbe,
        label: `Pflock ${index + 1}`,
      })),
    [pflöcke, modus, abstandByFarbe],
  );
  const [eingabe, setEingabe] = useState("");
  const meter =
    value && "Distanz" in value
      ? value.Distanz.meter
      : (checkpoints[0]?.wert ?? 1);

  if (modus === "single") {
    return (
      <p className="text-sm text-secondary">
        Position: {formatPosition(value)}
      </p>
    );
  }

  return (
    <div className="space-y-4">
      <CheckpointSlider
        checkpoints={checkpoints}
        value={meter}
        onValueChange={(next) => {
          if (modus === "meter")
            onChange({ Distanz: { meter: Math.max(1, Math.round(next)) } });
          else {
            const pflock =
              pflöcke[
                Math.max(0, Math.min(pflöcke.length - 1, Math.round(next) - 1))
              ];
            if (pflock) onChange({ Pflock: { farbe: pflock.farbe } });
          }
        }}
        min={
          modus === "meter"
            ? Math.min(...checkpoints.map((item) => item.wert))
            : 1
        }
        max={
          modus === "meter"
            ? Math.max(...checkpoints.map((item) => item.wert))
            : Math.max(1, pflöcke.length)
        }
        step={1}
        label="Schussposition"
        einheit={modus === "meter" ? "m" : "Pflock"}
      />
      <p className="text-sm tabular-nums text-ink">{formatPosition(value)}</p>
      <Field
        label="Exakte Entfernung"
        error={
          eingabe !== "" && (!/^\\d+$/.test(eingabe) || Number(eingabe) < 1)
            ? "Bitte geben Sie eine positive ganze Meterzahl ein."
            : undefined
        }
      >
        <Input
          inputMode="numeric"
          value={eingabe}
          onChange={(event) => {
            const next = event.target.value;
            setEingabe(next);
            if (/^[1-9]\\d*$/.test(next))
              onChange({ Distanz: { meter: Number(next) } });
          }}
          placeholder="Meter"
        />
      </Field>
    </div>
  );
}
