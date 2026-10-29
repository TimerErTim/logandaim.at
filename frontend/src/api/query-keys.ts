/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const logandaimKeys = {
  root: ["logandaim.at"] as const,
  health: () => [...logandaimKeys.root, "health"] as const,
  benutzer: () => [...logandaimKeys.root, "benutzer"] as const,
  schüsse: {
    all: () => [...logandaimKeys.root, "schüsse"] as const,
  },
  wertung: {
    all: () => [...logandaimKeys.root, "wertung"] as const,
  },
};
