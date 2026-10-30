"use client";

import { Check } from "lucide-react";

import { alertVariants } from "@/components/ui/alert";
import { cn } from "@/lib/cn";

/**
 * The positive counterpart of the failure `Alert`: one short German sentence that a
 * write succeeded — „Aufstellung gespeichert.“, „Pflock hinzugefügt.“
 *
 * It is the `success` note the design system already defines, not a second visual
 * language: the same tinted wash and left rule, the text in ink so it clears AA, and a
 * check icon as a second channel besides colour. The sentence itself carries the
 * message, which is why this takes text rather than children — and why it never ends in
 * an exclamation mark.
 *
 * `<output>` is the native polite live region for a result the user did not ask to
 * focus, so a success is spoken without interrupting the reader. It stays mounted while
 * empty (`sr-only`), because a live region inserted together with its text is not
 * reliably announced; the element keeps its identity and only its content changes.
 */
export function Erfolgsmeldung({
  meldung,
  className,
}: {
  meldung: string | null;
  className?: string;
}) {
  return (
    <output
      aria-atomic="true"
      className={cn(
        meldung
          ? cn(alertVariants({ variant: "success" }), className)
          : "sr-only",
      )}
    >
      {meldung ? (
        <>
          <span aria-hidden="true" className="mt-0.5 shrink-0 text-success">
            <Check className="size-4" />
          </span>
          <span className="min-w-0 flex-1">{meldung}</span>
        </>
      ) : null}
    </output>
  );
}
