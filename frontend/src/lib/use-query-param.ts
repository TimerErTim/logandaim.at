"use client";

import { useEffect, useState } from "react";

/**
 * Read one query parameter on the client.
 *
 * The site is a static export, so a detail page cannot use a dynamic `[id]` segment:
 * it is `/verwaltung/anlage?id=…`. Reading `useSearchParams()` would force a Suspense
 * boundary on every such page, so this hook reads `window.location` after mount
 * instead — `null` means "not known yet", which a page renders as a loading state
 * rather than as "missing".
 */
export function useQueryParam(name: string): string | null {
  const [value, setValue] = useState<string | null>(null);

  useEffect(() => {
    const lesen = () =>
      setValue(new URLSearchParams(window.location.search).get(name));

    lesen();
    window.addEventListener("popstate", lesen);
    return () => window.removeEventListener("popstate", lesen);
  }, [name]);

  return value;
}
