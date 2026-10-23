"use client";

import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { getConfig } from "@/lib/config";
import { createLogandaimApi, resetLogandaimApiCache } from "./index";
import type { LogandaimApi } from "./logandaim-api";

type LogandaimApiContextValue = {
  api: LogandaimApi | null;
  remoteApiBaseUrl: string | null;
  dev: boolean;
  loading: boolean;
  error: string | null;
  reload: () => void;
};

const LogandaimApiContext = createContext<LogandaimApiContextValue | null>(
  null,
);

export function LogandaimApiProvider({ children }: { children: ReactNode }) {
  const [api, setApi] = useState<LogandaimApi | null>(null);
  const [remoteApiBaseUrl, setRemoteApiBaseUrl] = useState<string | null>(null);
  const [dev, setDev] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);

  const reload = useCallback(() => {
    resetLogandaimApiCache();
    setReloadToken((value) => value + 1);
  }, []);

  // reloadToken retriggers bootstrap after resetLogandaimApiCache()
  useEffect(() => {
    void reloadToken;
    let cancelled = false;

    async function bootstrap() {
      setLoading(true);
      setError(null);
      setApi(null);

      try {
        const config = await getConfig();
        const resolvedApi = await createLogandaimApi();

        if (!cancelled) {
          setRemoteApiBaseUrl(config.remoteApiUrl);
          setDev(config.dev);
          setApi(resolvedApi);
        }
      } catch (bootstrapError) {
        if (!cancelled) {
          setError(String(bootstrapError));
          setApi(null);
          setRemoteApiBaseUrl(null);
          setDev(false);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    bootstrap();

    return () => {
      cancelled = true;
    };
  }, [reloadToken]);

  const value = useMemo(
    () => ({
      api,
      remoteApiBaseUrl,
      dev,
      loading,
      error,
      reload,
    }),
    [api, remoteApiBaseUrl, dev, loading, error, reload],
  );

  return (
    <LogandaimApiContext.Provider value={value}>
      {children}
      {dev ? <ReactQueryDevtools initialIsOpen={false} /> : null}
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
