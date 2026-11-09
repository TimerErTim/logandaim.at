import { useCallback, useId } from "react";
import { useToast } from "@/components/feedback/toast-provider";

/** One result slot per section; a new write replaces its previous toast. */
export function useErfolgsmeldung() {
  const id = useId();
  const toast = useToast();

  const melden = useCallback(
    (text: string) => toast.success(id, text),
    [id, toast],
  );
  const verwerfen = useCallback(() => toast.dismiss(id), [id, toast]);

  return { melden, verwerfen };
}
