import { useQuery } from "@tanstack/react-query";
import { yamsKeys } from "../query-keys";
import { useYamsApiReady } from "./use-yams-api-ready";

export function useHealthQuery() {
  const { api, isReady } = useYamsApiReady();

  return useQuery({
    queryKey: yamsKeys.health(),
    queryFn: () => api?.health(),
    enabled: isReady,
  });
}
