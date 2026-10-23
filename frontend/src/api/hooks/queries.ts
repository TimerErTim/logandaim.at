import { useQuery } from "@tanstack/react-query";

import { useLogandaimApi } from "../provider";
import { logandaimKeys } from "../query-keys";

export function useHealthQuery() {
  const { api } = useLogandaimApi();

  return useQuery({
    queryKey: logandaimKeys.health(),
    queryFn: () => api.health(),
  });
}
