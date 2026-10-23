"use client";

import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { createContext, type ReactNode, useContext, useMemo } from "react";

import { config } from "@/lib/config";
import { createLogandaimApi } from "./index";
import type { LogandaimApi } from "./logandaim-api";

type LogandaimApiContextValue = {
  api: LogandaimApi;
  apiBaseUrl: string;
  dev: boolean;
};

const LogandaimApiContext = createContext<LogandaimApiContextValue | null>(
  null,
);

/**
 * Creates the API instance once per client and exposes it. Config is static, so there
 * is no async bootstrap, loading state, or reload path.
 */
export function LogandaimApiProvider({ children }: { children: ReactNode }) {
  // One transport instance per client; `config` never changes at runtime.
  const value = useMemo<LogandaimApiContextValue>(
    () => ({
      api: createLogandaimApi(),
      apiBaseUrl: config.apiBaseUrl,
      dev: config.dev,
    }),
    [],
  );

  return (
    <LogandaimApiContext.Provider value={value}>
      {children}
      {value.dev ? <ReactQueryDevtools initialIsOpen={false} /> : null}
    </LogandaimApiContext.Provider>
  );
}

export function useLogandaimApi(): LogandaimApiContextValue {
  const context = useContext(LogandaimApiContext);
  if (!context) {
    throw new Error("useLogandaimApi must be used within LogandaimApiProvider");
  }
  return context;
}
