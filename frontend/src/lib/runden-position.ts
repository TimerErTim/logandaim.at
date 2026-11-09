import type {
  RundeZiel,
  Schuss,
  SchusspositionInput,
} from "@/api/logandaim-api";

export type PositionsModus = "staken" | "meter" | "single";

export function positionsModus(
  ziel: RundeZiel,
  pflockAnzahl: number,
): PositionsModus {
  if (pflockAnzahl === 1) return "single";
  return ziel.abstände.length >= 2 ? "meter" : "staken";
}

export function anfangsPosition(
  ziel: RundeZiel,
  schuesse: Schuss[],
): SchusspositionInput | null {
  const letzte = [...schuesse]
    .sort((a, b) => a.pfeil - b.pfeil)
    .at(-1)?.position;
  if (letzte) return letzte as SchusspositionInput;
  const naechster = [...ziel.abstände].sort((a, b) => a.meter - b.meter)[0];
  return naechster ? { Pflock: { farbe: naechster.pflock } } : null;
}

export function formatPosition(position: SchusspositionInput | null): string {
  if (!position) return "Keine Position";
  if ("Distanz" in position) return `${position.Distanz.meter} m`;
  return position.Pflock.farbe;
}

export function positionAlsMeter(
  position: SchusspositionInput | null,
  fallback: number,
): number {
  if (position && "Distanz" in position) return position.Distanz.meter;
  return fallback;
}
