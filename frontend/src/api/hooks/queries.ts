import { useQuery } from "@tanstack/react-query";
import { useAuthSession } from "../auth-provider";
import { useLogandaimApi } from "../provider";
import { logandaimKeys } from "../query-keys";

export function useHealthQuery() {
  const { api } = useLogandaimApi();
  return useQuery({
    queryKey: logandaimKeys.health(),
    queryFn: () => api.health(),
  });
}

export function useBenutzerQuery() {
  const { api } = useLogandaimApi();
  const { accessToken, bootstrapped, bootstrapError } = useAuthSession();
  return useQuery({
    queryKey: logandaimKeys.benutzer(),
    queryFn: () => api.benutzerAbrufen(),
    enabled: bootstrapped && Boolean(accessToken) && !bootstrapError,
    retry: false,
  });
}
