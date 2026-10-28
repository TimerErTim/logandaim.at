import { useSyncExternalStore } from "react";

import type { AuthPhase } from "./logandaim-api";

const checking: AuthPhase = Object.freeze({ status: "checking" });

export function useAuthPhase(
  subscribe: (listener: () => void) => () => void,
  getSnapshot: () => AuthPhase,
): AuthPhase {
  return useSyncExternalStore(subscribe, getSnapshot, () => checking);
}
