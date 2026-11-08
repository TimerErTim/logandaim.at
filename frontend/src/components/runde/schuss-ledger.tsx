"use client";

import { Trash2 } from "lucide-react";

import type {
  RundeZiel,
  Schuss,
  SchusspositionInput,
} from "@/api/logandaim-api";
import { Wertungszone } from "@/api/logandaim-api";
import { PflockSwatch } from "@/components/stammdaten/pflock-swatch";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { formatPosition } from "@/lib/runden-position";

const ZONEN = [
  Wertungszone.Spot,
  Wertungszone.Kill,
  Wertungszone.Body,
  Wertungszone.Miss,
];

export function SchussLedger({
  ziel,
  reihenfolge,
  editable,
  onEdit,
  onDelete,
}: {
  ziel: RundeZiel;
  reihenfolge: number;
  editable: boolean;
  onEdit: (
    schussId: string,
    wertungszone: Wertungszone,
    position: SchusspositionInput | null,
  ) => void;
  onDelete: (schussId: string) => void;
}) {
  return (
    <div>
      <div
        aria-hidden="true"
        className="flex items-center gap-3 border-b border-subtle bg-surface-muted py-3 text-[11px] font-semibold tracking-[0.07em] text-secondary uppercase"
      >
        <span className="w-10 shrink-0">Pfeil</span>
        <span className="w-20 shrink-0">Wertung</span>
        <span className="min-w-0 flex-1">Position</span>
        <span className="w-12 shrink-0">Punkte</span>
        <span className="w-10 shrink-0" />
      </div>
      <ol
        aria-label={`Schüsse bei Ziel ${reihenfolge}`}
        className="divide-y divide-subtle"
      >
        {ziel.schüsse.map((schuss: Schuss) => (
          <li key={schuss.id} className="flex min-h-11 items-center gap-3 py-2">
            <span className="w-10 shrink-0 font-mono text-sm tabular-nums text-secondary">
              {schuss.pfeil}.
            </span>
            {editable ? (
              <Select
                aria-label={`Wertungszone für Schuss ${schuss.pfeil}`}
                value={schuss.wertungszone}
                className="w-20 shrink-0 pr-7 pl-2"
                onChange={(event) => {
                  const nextZone = event.target.value as Wertungszone;
                  if (nextZone !== schuss.wertungszone) {
                    onEdit(schuss.id, nextZone, schuss.position ?? null);
                  }
                }}
              >
                {ZONEN.map((zone) => (
                  <option key={zone} value={zone}>
                    {zone}
                  </option>
                ))}
              </Select>
            ) : (
              <span className="w-20 shrink-0 text-sm font-medium text-ink">
                {schuss.wertungszone}
              </span>
            )}
            <span className="min-w-0 flex-1 text-sm text-secondary">
              {schuss.position && "Pflock" in schuss.position ? (
                <span className="inline-flex items-center gap-2">
                  <PflockSwatch farbe={schuss.position.Pflock.farbe} />
                  <span>Pflock</span>
                </span>
              ) : (
                formatPosition(schuss.position as SchusspositionInput | null)
              )}
            </span>
            <span className="w-12 shrink-0 text-left text-sm tabular-nums text-ink">
              {punkte(schuss)}
            </span>
            {editable ? (
              <Button
                size="icon"
                variant="destructive"
                aria-label={`Schuss ${schuss.pfeil} löschen`}
                onClick={() => onDelete(schuss.id)}
              >
                <Trash2 aria-hidden="true" />
              </Button>
            ) : null}
          </li>
        ))}
        {ziel.schüsse.length === 0 ? (
          <li className="py-2 text-sm text-secondary">
            Noch kein Schuss erfasst.
          </li>
        ) : null}
      </ol>
    </div>
  );
}

function punkte(schuss: Schuss): number {
  if (schuss.wertungszone === Wertungszone.Miss) return 0;
  const erster =
    schuss.pfeil === 1
      ? 20
      : schuss.pfeil === 2
        ? 14
        : schuss.pfeil === 3
          ? 8
          : 0;
  const versatz =
    schuss.wertungszone === Wertungszone.Kill
      ? -2
      : schuss.wertungszone === Wertungszone.Body
        ? -4
        : 0;
  return Math.max(0, erster + versatz);
}
