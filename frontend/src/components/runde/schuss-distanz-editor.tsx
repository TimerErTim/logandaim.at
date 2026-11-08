"use client";

import * as PopoverPrimitive from "@radix-ui/react-popover";
import { useEffect, useMemo, useState } from "react";

import type {
  RundePflock,
  RundeZiel,
  SchusspositionInput,
} from "@/api/logandaim-api";
import { PflockSwatch } from "@/components/stammdaten/pflock-swatch";
import { Button } from "@/components/ui/button";
import {
  type Checkpoint,
  CheckpointSlider,
} from "@/components/ui/checkpoint-slider";
import { Input } from "@/components/ui/input";
import { formatPosition, positionsModus } from "@/lib/runden-position";

export function SchussDistanzEditor({
  pfeil,
  ziel,
  pflöcke,
  position,
  onCommit,
}: {
  pfeil: number;
  ziel: RundeZiel;
  pflöcke: RundePflock[];
  position: SchusspositionInput | null;
  onCommit: (position: SchusspositionInput) => void;
}) {
  const modus = positionsModus(ziel, pflöcke.length);
  const abstände = useMemo(
    () =>
      new Map(ziel.abstände.map((abstand) => [abstand.pflock, abstand.meter])),
    [ziel.abstände],
  );
  const checkpoints = useMemo<Checkpoint[]>(
    () =>
      [...pflöcke]
        .sort((a, b) => a.position - b.position)
        .flatMap((pflock) => {
          const meter = abstände.get(pflock.farbe);
          return meter === undefined
            ? []
            : [
                {
                  id: `${pflock.position}-${pflock.farbe}`,
                  wert: meter,
                  farbe: pflock.farbe,
                  label: pflock.farbe,
                  anzeige: `${meter} m`,
                },
              ];
        })
        .sort((a, b) => a.wert - b.wert),
    [pflöcke, abstände],
  );
  const min = checkpoints.length
    ? Math.min(...checkpoints.map((checkpoint) => checkpoint.wert))
    : 1;
  const max = checkpoints.length
    ? Math.max(...checkpoints.map((checkpoint) => checkpoint.wert))
    : 1;
  const storedMeter =
    position && "Distanz" in position ? position.Distanz.meter : undefined;
  const storedStake =
    position && "Pflock" in position ? position.Pflock.farbe : undefined;
  const [open, setOpen] = useState(false);
  const [draft, setDraft] = useState(
    storedMeter === undefined ? "" : String(storedMeter),
  );

  useEffect(() => {
    setDraft(storedMeter === undefined ? "" : String(storedMeter));
  }, [storedMeter]);

  const canUseSlider =
    modus === "meter" && checkpoints.length >= 2 && max > min;
  const sliderValue = Math.min(
    max,
    Math.max(min, Number(draft) || storedMeter || min),
  );
  const valid = /^[1-9]\d*$/.test(draft);
  const save = (meter: number) => {
    setDraft(String(meter));
    onCommit({ Distanz: { meter } });
    setOpen(false);
  };

  return (
    <div className="min-w-0">
      <PopoverPrimitive.Root open={open} onOpenChange={setOpen}>
        <PopoverPrimitive.Anchor asChild>
          <div className="min-w-0">
            <Input
              aria-label={`Exakte Entfernung für Schuss ${pfeil} in Metern`}
              inputMode="numeric"
              value={draft}
              placeholder={storedStake ? "Pflock" : "Meter"}
              onChange={(event) => setDraft(event.target.value)}
              onFocus={() => setOpen(true)}
              onKeyDown={(event) => {
                if (event.key === "Enter" && valid) save(Number(draft));
                if (event.key === "Escape") setOpen(false);
              }}
              className="h-11 min-w-0 w-full px-2 tabular-nums"
            />
          </div>
        </PopoverPrimitive.Anchor>
        <PopoverPrimitive.Portal>
          <PopoverPrimitive.Content
            sideOffset={6}
            align="start"
            onOpenAutoFocus={(event) => event.preventDefault()}
            className="paper-overlay z-50 w-[min(20rem,calc(100vw-2rem))] space-y-3 rounded-sm border border-subtle bg-elevated p-4 text-ink"
          >
            <p className="text-xs font-semibold tracking-[0.07em] text-secondary uppercase">
              Entfernung · Schuss {pfeil}
            </p>
            {canUseSlider ? (
              <CheckpointSlider
                checkpoints={checkpoints}
                value={sliderValue}
                onValueChange={(value) => setDraft(String(Math.round(value)))}
                onValueCommit={(value) => save(Math.round(value))}
                min={min}
                max={max}
                step={1}
                label={`Entfernung für Schuss ${pfeil}`}
                einheit="m"
              />
            ) : null}
            <div className="flex items-center gap-2">
              {storedStake ? <PflockSwatch farbe={storedStake} /> : null}
              <span className="text-sm text-secondary">
                {storedStake ? "Pflock gespeichert" : formatPosition(position)}
              </span>
              <Button
                size="field"
                disabled={!valid}
                onClick={() => save(Number(draft))}
                className="ml-auto"
              >
                Übernehmen
              </Button>
            </div>
          </PopoverPrimitive.Content>
        </PopoverPrimitive.Portal>
      </PopoverPrimitive.Root>
    </div>
  );
}
