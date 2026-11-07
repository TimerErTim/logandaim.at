"use client";

import { useState } from "react";

import type {
  RundeZiel,
  Schuss,
  SchusspositionInput,
} from "@/api/logandaim-api";
import { Wertungszone } from "@/api/logandaim-api";
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
  const [editing, setEditing] = useState<string | null>(null);
  return (
    <ol
      aria-label={`Schüsse bei Ziel ${reihenfolge}`}
      className="divide-y divide-subtle"
    >
      {ziel.schüsse.map((schuss: Schuss) => (
        <li
          key={schuss.id}
          className="flex min-h-11 flex-wrap items-center gap-3 py-2"
        >
          <span className="w-10 font-mono text-sm tabular-nums text-secondary">
            {schuss.pfeil}.
          </span>
          {editing === schuss.id ? (
            <Select
              aria-label="Wertungszone"
              value={schuss.wertungszone}
              onChange={(event) => {
                onEdit(
                  schuss.id,
                  event.target.value as Wertungszone,
                  schuss.position ?? null,
                );
                setEditing(null);
              }}
            >
              {ZONEN.map((zone) => (
                <option key={zone} value={zone}>
                  {zone}
                </option>
              ))}
            </Select>
          ) : (
            <span className="min-w-20 text-sm font-medium text-ink">
              {schuss.wertungszone}
            </span>
          )}
          <span className="text-sm text-secondary">
            {formatPosition(schuss.position as SchusspositionInput | null)}
          </span>
          <span className="ml-auto text-sm tabular-nums text-ink">
            {punkte(schuss)}
          </span>
          {editable ? (
            <Button
              size="sm"
              variant="secondary"
              onClick={() =>
                setEditing(editing === schuss.id ? null : schuss.id)
              }
            >
              {editing === schuss.id ? "Fertig" : "Ändern"}
            </Button>
          ) : null}
          {editable ? (
            <Button
              size="sm"
              variant="destructive"
              onClick={() => onDelete(schuss.id)}
            >
              Löschen
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
