"use client";

import * as PopoverPrimitive from "@radix-ui/react-popover";
import { useEffect, useMemo, useRef, useState } from "react";

import type { RundeZiel, SchusspositionInput } from "@/api/logandaim-api";
import {
  type Checkpoint,
  CheckpointSlider,
} from "@/components/ui/checkpoint-slider";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/cn";
import { positionsModus } from "@/lib/runden-position";
import { FreieFarbeFeld } from "./freie-farbe-feld";
import { PflockPlatzhalter } from "./pflock-platzhalter";

export function SchussDistanzEditor({
  pfeil,
  ziel,
  position,
  onCommit,
  positionArt = "parkour",
  farben = [],
}: {
  pfeil: number;
  ziel: RundeZiel;
  position: SchusspositionInput | null;
  onCommit: (position: SchusspositionInput) => void;
  positionArt?: "parkour" | "frei";
  farben?: string[];
}) {
  const effektivePflöcke = ziel.pflöcke;
  const modus = positionsModus(ziel, effektivePflöcke.length);
  const abstände = useMemo(
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
  const sortedPflöcke = useMemo(() => effektivePflöcke, [effektivePflöcke]);
  const checkpoints = useMemo<Checkpoint[]>(
    () =>
      sortedPflöcke
        .flatMap((pflock) => {
          const meter = abstände.get(pflock.farbe);
          return meter === undefined
            ? []
            : [
                {
                  id: `${pflock.farbe}`,
                  wert: meter,
                  farbe: pflock.farbe,
                  label: pflock.farbe,
                  anzeige: `${meter} m`,
                },
              ];
        })
        .sort((a, b) => a.wert - b.wert),
    [sortedPflöcke, abstände],
  );
  const stakeCheckpoints = useMemo<Checkpoint[]>(
    () =>
      sortedPflöcke.map((pflock, index) => ({
        id: `${pflock.farbe}`,
        wert: index,
        farbe: pflock.farbe,
        label: pflock.farbe,
      })),
    [sortedPflöcke],
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
  const committedMeter = useRef(storedMeter);
  const committedStake = useRef(storedStake);
  const [draftStake, setDraftStake] = useState(storedStake);
  const [draft, setDraft] = useState(
    storedMeter === undefined ? "" : String(storedMeter),
  );
  const previousPositionKey = useRef(
    `${storedMeter ?? ""}:${storedStake ?? ""}`,
  );
  const cancelDraft = useRef(false);

  useEffect(() => {
    const nextPositionKey = `${storedMeter ?? ""}:${storedStake ?? ""}`;
    if (previousPositionKey.current === nextPositionKey) return;
    previousPositionKey.current = nextPositionKey;
    committedMeter.current = storedMeter;
    committedStake.current = storedStake;
    setDraftStake(storedStake);
    setDraft(storedMeter === undefined ? "" : String(storedMeter));
  }, [storedMeter, storedStake]);

  const canUseMeterSlider =
    modus === "meter" && checkpoints.length >= 2 && max > min;
  const canUseStakeSlider = modus === "staken" && stakeCheckpoints.length >= 2;
  const stakeIndex = draftStake
    ? sortedPflöcke.findIndex((pflock) => pflock.farbe === draftStake)
    : -1;
  const sliderValue = Math.min(
    max,
    Math.max(min, Number(draft) || storedMeter || min),
  );
  const valid = /^[1-9]\d*$/.test(draft) && Number(draft) <= 4_294_967_295;
  const saveMeter = (meter: number) => {
    committedMeter.current = meter;
    committedStake.current = undefined;
    setDraftStake(undefined);
    onCommit({ Distanz: { meter } });
  };
  const saveStake = (farbe: string) => {
    committedStake.current = farbe;
    committedMeter.current = undefined;
    setDraftStake(farbe);
    setDraft("");
    onCommit({ Pflock: { farbe } });
  };
  const commitDraft = () => {
    if (valid) {
      const meter = Number(draft);
      if (
        meter !== committedMeter.current ||
        committedStake.current !== undefined
      ) {
        saveMeter(meter);
      }
      return;
    }
    if (draft === "" && draftStake !== committedStake.current && draftStake) {
      saveStake(draftStake);
      return;
    }
    setDraft(
      committedMeter.current === undefined
        ? ""
        : String(committedMeter.current),
    );
    setDraftStake(committedStake.current);
  };

  const pflockPlaceholder = draft === "" && draftStake !== undefined;
  const freiesFarbFeld = positionArt === "frei" && pflockPlaceholder;

  return (
    <div className="min-w-0">
      <div className="w-20 shrink-0 sm:w-24">
        <PopoverPrimitive.Root open={open} onOpenChange={setOpen}>
          <PopoverPrimitive.Anchor asChild>
            <div className="relative w-full">
              <Input
                aria-label={`${positionArt === "frei" ? "Position" : "Exakte Entfernung"} für Schuss ${pfeil}${positionArt === "frei" ? "" : " in Metern"}`}
                aria-invalid={draft !== "" && !valid}
                inputMode="numeric"
                value={freiesFarbFeld ? "" : draft}
                placeholder={pflockPlaceholder ? "" : "Meter"}
                onChange={(event) => {
                  const next = event.target.value;
                  setDraft(next);
                  if (next !== "") setDraftStake(undefined);
                }}
                onFocus={() => setOpen(true)}
                onBlur={() => {
                  if (cancelDraft.current) {
                    cancelDraft.current = false;
                    return;
                  }
                  commitDraft();
                }}
                onKeyDown={(event) => {
                  if (event.key === "Enter") {
                    event.preventDefault();
                    event.currentTarget.blur();
                  }
                  if (event.key === "Escape") {
                    cancelDraft.current = true;
                    setDraft(
                      committedMeter.current === undefined
                        ? ""
                        : String(committedMeter.current),
                    );
                    setDraftStake(committedStake.current);
                    event.currentTarget.blur();
                  }
                }}
                className={cn(
                  "h-11 w-full px-2 tabular-nums",
                  pflockPlaceholder && "text-transparent caret-ink",
                  freiesFarbFeld && "pl-12",
                )}
              />
              {pflockPlaceholder ? (
                <PflockPlatzhalter farbe={draftStake} />
              ) : null}
            </div>
          </PopoverPrimitive.Anchor>
          <PopoverPrimitive.Portal>
            <PopoverPrimitive.Content
              sideOffset={6}
              align="start"
              onOpenAutoFocus={(event) => event.preventDefault()}
              onCloseAutoFocus={(event) => event.preventDefault()}
              onPointerDown={(event) => {
                if (
                  positionArt === "frei" &&
                  event.target instanceof Element &&
                  event.target.closest("button")
                ) {
                  event.preventDefault();
                }
              }}
              className="paper-overlay z-50 w-[min(20rem,calc(100vw-2rem))] rounded-sm border border-subtle bg-elevated p-4 text-ink"
            >
              {positionArt === "frei" ? (
                <FreieFarbeFeld
                  farben={farben}
                  label={`Farbe für Schuss ${pfeil}`}
                  value={draftStake ?? null}
                  onChange={(farbe) => saveStake(farbe)}
                />
              ) : canUseMeterSlider ? (
                <CheckpointSlider
                  checkpoints={checkpoints}
                  value={sliderValue}
                  onValueChange={(value) => {
                    setDraft(String(Math.round(value)));
                    setDraftStake(undefined);
                  }}
                  min={min}
                  max={max}
                  step={1}
                  label={`Entfernung für Schuss ${pfeil}`}
                  einheit="m"
                />
              ) : null}
              {canUseStakeSlider ? (
                <CheckpointSlider
                  checkpoints={stakeCheckpoints}
                  value={Math.max(0, stakeIndex)}
                  onValueChange={(value) => {
                    const pflock = sortedPflöcke[Math.round(value)];
                    if (pflock) {
                      setDraftStake(pflock.farbe);
                      setDraft("");
                      saveStake(pflock.farbe);
                    }
                  }}
                  min={0}
                  max={stakeCheckpoints.length - 1}
                  step={1}
                  label={`Pflock für Schuss ${pfeil}`}
                />
              ) : null}
            </PopoverPrimitive.Content>
          </PopoverPrimitive.Portal>
        </PopoverPrimitive.Root>
      </div>
    </div>
  );
}
