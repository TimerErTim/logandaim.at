import type { Pflock } from "@/api/logandaim-api";

import { PFLOCK_FARBE_STANDARD, PFLOCK_FARBEN } from "./pflock-farbe-feld";

/**
 * The colour a new Pflock row may start with: the standard one while it is free,
 * otherwise the first unused colour of the club's vocabulary, otherwise `null`.
 *
 * The domain refuses two Pflöcke of one colour in a Parcours (AC-6), so `null` is a
 * real answer: every colour a new row could take is already in use. A caller that
 * added the row anyway would build a setup the API rejects, and the rejection names no
 * field — so the caller disables "Pflock hinzufügen" and says why instead.
 */
export function freiePflockFarbe(vorhandene: readonly Pflock[]): string | null {
  const belegt = new Set(
    vorhandene.map((pflock) => pflock.farbe.toLowerCase()),
  );
  if (!belegt.has(PFLOCK_FARBE_STANDARD)) return PFLOCK_FARBE_STANDARD;
  return (
    PFLOCK_FARBEN.find((eintrag) => !belegt.has(eintrag.farbe))?.farbe ?? null
  );
}

/**
 * A fresh row for a Parcours' Pflock setup: a new identity and a colour that is not
 * yet in use. The row is appended at the near end — its position in the array is the
 * far-to-near order the user then drags into place.
 *
 * Call only where `freiePflockFarbe` answered; the fallback keeps the return type
 * simple, and a setup with a repeated colour is refused by the API anyway (AC-6).
 */
export function neuerPflock(vorhandene: readonly Pflock[]): Pflock {
  return {
    id: crypto.randomUUID(),
    farbe: freiePflockFarbe(vorhandene) ?? PFLOCK_FARBE_STANDARD,
  };
}
