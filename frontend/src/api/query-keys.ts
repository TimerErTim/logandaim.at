/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const logandaimKeys = {
  root: ["logandaim.at"] as const,
  health: () => [...logandaimKeys.root, "health"] as const,
};
