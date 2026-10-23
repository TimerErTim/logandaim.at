"use client";

import { useQuery } from "@tanstack/react-query";

import { useYamsApiReady } from "../provider";
import { yamsKeys } from "../query-keys";

export function useHealthQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.health(),
    queryFn: () => {
      if (!api) {
        throw new Error("YamsApi not ready");
      }
      return api.health();
    },
    enabled: isReady,
  });
}
