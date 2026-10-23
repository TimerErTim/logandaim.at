/** Hierarchical keys for TanStack Query — invalidate at the right granularity. */
export const yamsKeys = {
  root: ["yams"] as const,
  health: () => [...yamsKeys.root, "health"] as const,
};
