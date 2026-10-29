/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const logandaimKeys = {
  root: ["logandaim.at"] as const,
  health: () => [...logandaimKeys.root, "health"] as const,
  benutzer: () => [...logandaimKeys.root, "benutzer"] as const,
  anlagen: {
    all: () => [...logandaimKeys.root, "anlagen"] as const,
    detail: (id: string) => [...logandaimKeys.anlagen.all(), id] as const,
    benutzerSuche: (anlageId: string, email: string) =>
      [
        ...logandaimKeys.anlagen.detail(anlageId),
        "benutzer-suche",
        email,
      ] as const,
  },
  parcours: {
    /** The public list (AC-13). */
    all: () => [...logandaimKeys.root, "parcours"] as const,
  },
  anlagenVerzeichnis: {
    /** The public Anlagen directory: readable without a session. */
    all: () => [...logandaimKeys.root, "anlagen-verzeichnis"] as const,
  },
  runden: {
    all: () => [...logandaimKeys.root, "runden"] as const,
    detail: (id: string) => [...logandaimKeys.runden.all(), id] as const,
  },
};
