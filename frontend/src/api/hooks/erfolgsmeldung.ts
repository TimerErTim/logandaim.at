import { useCallback, useEffect, useRef, useState } from "react";

/** How long a success note stays on the page before it clears itself. */
const DAUER_MS = 5000;

/**
 * One success message per section. A write that succeeds has to say so: every
 * mutation in the logbook is silent otherwise, because only failures render an
 * `Alert`.
 *
 * The note belongs to a *section*, not to a mutation: several actions in one card
 * share one slot, so the newest sentence replaces the older one. `verwerfen` clears
 * the slot before the next mutation starts, so a note never lingers while a different
 * write is in flight and never describes a finished action while the next one is
 * pending. The timer clears it anyway, so a note is never mistaken for a permanent
 * state of the page.
 */
export function useErfolgsmeldung(dauer = DAUER_MS) {
  const [meldung, setMeldung] = useState<string | null>(null);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const stoppe = useCallback(() => {
    if (timer.current !== null) {
      clearTimeout(timer.current);
      timer.current = null;
    }
  }, []);

  const melden = useCallback(
    (text: string) => {
      stoppe();
      setMeldung(text);
      timer.current = setTimeout(() => {
        setMeldung(null);
        timer.current = null;
      }, dauer);
    },
    [dauer, stoppe],
  );

  /** Clear before the next mutation, so the note always describes the last result. */
  const verwerfen = useCallback(() => {
    stoppe();
    setMeldung(null);
  }, [stoppe]);

  useEffect(() => stoppe, [stoppe]);

  return { meldung, melden, verwerfen };
}
