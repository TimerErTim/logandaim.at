"use client";

import { useQueryClient } from "@tanstack/react-query";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useRef,
} from "react";

import { ApiError } from "./errors";
import { useLogandaimApi } from "./provider";
import { logandaimKeys } from "./query-keys";
import { setAuthBootstrapped, useAuthSnapshot } from "./session-store";

export function AuthProvider({ children }: { children: ReactNode }) {
  const { api } = useLogandaimApi();
  const queryClient = useQueryClient();
  const renewal = useRef<Promise<void> | null>(null);

  const renew = useCallback(() => {
    if (renewal.current) return renewal.current;
    const request = api
      .sessionErneuern()
      .catch((error: unknown) => {
        api.accessTokenSetzen(null);
        if (error instanceof ApiError && error.status === 401) return;
        throw error;
      })
      .finally(() => {
        renewal.current = null;
      });
    renewal.current = request;
    return request;
  }, [api]);

  const clearSession = useCallback(() => {
    api.accessTokenSetzen(null);
    queryClient.setQueryData(logandaimKeys.benutzer(), null);
  }, [api, queryClient]);

  useEffect(() => {
    let active = true;
    void renew()
      .catch(() => undefined)
      .finally(() => {
        if (active) setAuthBootstrapped(true);
      });
    return () => {
      active = false;
    };
  }, [renew]);

  return (
    <AuthContext.Provider value={{ renew, clearSession }}>
      {children}
    </AuthContext.Provider>
  );
}

type AuthContextValue = {
  renew(): Promise<void>;
  clearSession(): void;
};

const AuthContext = createContext<AuthContextValue | null>(null);

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within AuthProvider");
  return { ...context, ...useAuthSnapshot() };
}

export function useAuthSession() {
  return useAuth();
}
