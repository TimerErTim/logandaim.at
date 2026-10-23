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
import { createYamsApi, resetYamsApiCache } from "./index";
import type { YamsApi } from "./yams-api";

type YamsApiContextValue = {
  api: YamsApi | null;
  remoteApiBaseUrl: string | null;
  dev: boolean;
  loading: boolean;
  error: string | null;
  reload: () => void;
};

const YamsApiContext = createContext<YamsApiContextValue | null>(null);

export function YamsApiProvider({ children }: { children: ReactNode }) {
  const [api, setApi] = useState<YamsApi | null>(null);
  const [remoteApiBaseUrl, setRemoteApiBaseUrl] = useState<string | null>(null);
  const [dev, setDev] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reloadToken, setReloadToken] = useState(0);

  const reload = useCallback(() => {
    resetYamsApiCache();
    setReloadToken((value) => value + 1);
  }, []);

  // reloadToken retriggers bootstrap after resetYamsApiCache()
  useEffect(() => {
    void reloadToken;
    let cancelled = false;

    async function bootstrap() {
      setLoading(true);
      setError(null);
      setApi(null);

      try {
        const config = await getConfig();
        const resolvedApi = await createYamsApi();

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
    <YamsApiContext.Provider value={value}>
      {children}
      {dev ? <ReactQueryDevtools initialIsOpen={false} /> : null}
    </YamsApiContext.Provider>
  );
}

export function useYamsApi(): YamsApiContextValue {
  const context = useContext(YamsApiContext);
  if (!context) {
    throw new Error("useYamsApi must be used within YamsApiProvider");
  }
  return context;
}
