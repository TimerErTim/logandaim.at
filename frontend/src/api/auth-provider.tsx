"use client";

import { useQueryClient } from "@tanstack/react-query";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";

import { ApiError } from "./errors";
import { useLogandaimApi } from "./provider";
import { logandaimKeys } from "./query-keys";
import { setAuthBootstrapped, useAuthSnapshot } from "./session-store";

type AuthContextValue = {
  renew(): Promise<void>;
  markAuthenticated(): void;
  clearSession(): void;
  bootstrapError: unknown;
};

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const { api } = useLogandaimApi();
  const queryClient = useQueryClient();
  const renewal = useRef<Promise<void> | null>(null);
  const [bootstrapError, setBootstrapError] = useState<unknown>(null);

  const renew = useCallback(() => {
    if (renewal.current) return renewal.current;
    const request = api
      .sessionErneuern()
      .then(() => {
        setBootstrapError(null);
      })
      .catch((error: unknown) => {
        if (error instanceof ApiError && error.status === 401) {
          api.accessTokenSetzen(null);
          setBootstrapError(null);
          return;
        }
        setBootstrapError(error);
        throw error;
      })
      .finally(() => {
        renewal.current = null;
      });
    renewal.current = request;
    return request;
  }, [api]);

  const markAuthenticated = useCallback(() => {
    setBootstrapError(null);
    setAuthBootstrapped(true);
  }, []);

  const clearSession = useCallback(() => {
    queryClient.removeQueries({ queryKey: logandaimKeys.benutzer() });
    api.accessTokenSetzen(null);
  }, [api, queryClient]);

  useEffect(() => {
    let active = true;
    void renew()
      .catch((error: unknown) => {
        if (active && !(error instanceof ApiError && error.status === 401)) {
          setBootstrapError(error);
        }
      })
      .finally(() => {
        if (active) setAuthBootstrapped(true);
      });
    return () => {
      active = false;
    };
  }, [renew]);

  return (
    <AuthContext.Provider
      value={{ renew, markAuthenticated, clearSession, bootstrapError }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) throw new Error("useAuth must be used within AuthProvider");
  return { ...context, ...useAuthSnapshot() };
}

export function useAuthSession() {
  return useAuth();
}
