"use client";

import { type ReactNode, useEffect } from "react";

import { useLogandaimApi } from "./provider";
import { useAuthPhase } from "./session-store";

export function AuthProvider({ children }: { children: ReactNode }) {
  const { authSession } = useLogandaimApi();
  const phase = useAuthPhase(authSession.subscribe, authSession.getSnapshot);

  useEffect(() => {
    if (phase.status !== "checking") return;
    void authSession.restore().catch(() => {
      // AuthSession publishes unavailable state for recoverable service failures.
    });
  }, [authSession, phase.status]);

  return children;
}

export function useAuth() {
  const { authSession } = useLogandaimApi();
  return useAuthPhase(authSession.subscribe, authSession.getSnapshot);
}
