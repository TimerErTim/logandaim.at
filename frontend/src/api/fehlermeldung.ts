/**
 * The general failure slot, one per app.
 *
 * The `QueryClient` is created outside React, so it cannot reach the toast that draws
 * the note. This module is the seam: a mutation that fails and that no screen reports
 * itself publishes one German sentence here, and the existing `ToastProvider` — the
 * app's only feedback surface — shows it. There is no second notification system.
 *
 * A listener *replaces* the previous sentence rather than queueing another, so a burst
 * of failures reads as one note.
 */
type Listener = (text: string) => void;

const listeners = new Set<Listener>();

/** What a failure says when nothing more specific is known. */
export const GENERAL_ERROR_MESSAGE =
  "Die Aktion konnte nicht ausgeführt werden. Bitte versuchen Sie es erneut.";

export function reportGeneralError(text: string): void {
  for (const listener of listeners) listener(text);
}

export function subscribeToGeneralErrors(listener: Listener): () => void {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}
