import { useQuery } from "@tanstack/react-query";
import { logandaimKeys } from "../query-keys";
import { useLogandaimApiReady } from "./use-logandaim-api-ready";

export function useHealthQuery() {
  const { api, isReady } = useLogandaimApiReady();

  return useQuery({
    queryKey: logandaimKeys.health(),
    queryFn: () => api?.health(),
    enabled: isReady,
  });
}
