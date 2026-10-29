/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const logandaimKeys = {
  root: ["logandaim.at"] as const,
  health: () => [...logandaimKeys.root, "health"] as const,
  benutzer: () => [...logandaimKeys.root, "benutzer"] as const,
  anlagen: {
    all: () => [...logandaimKeys.root, "anlagen"] as const,
    detail: (id: string) => [...logandaimKeys.anlagen.all(), id] as const,
  },
  parcours: {
    /** The public list (AC-13). */
    all: () => [...logandaimKeys.root, "parcours"] as const,
  },
  runden: {
    all: () => [...logandaimKeys.root, "runden"] as const,
    detail: (id: string) => [...logandaimKeys.runden.all(), id] as const,
  },
};
