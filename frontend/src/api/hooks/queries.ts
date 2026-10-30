import { useQuery } from "@tanstack/react-query";

import { useAuth } from "../auth-provider";
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
  const phase = useAuth();
  return useQuery({
    queryKey: logandaimKeys.benutzer(),
    queryFn: () => api.benutzerAbrufen(),
    enabled: phase.status === "authenticated",
    retry: false,
  });
}

/**
 * The Anlagen the caller may administer. A 403 is a normal answer for an account with no
 * Verwaltung, so it is not retried.
 */
export function useAnlagenQuery() {
  const { api } = useLogandaimApi();
  const phase = useAuth();
  return useQuery({
    queryKey: logandaimKeys.anlagen.all(),
    queryFn: () => api.anlagenAnzeigen(),
    enabled: phase.status === "authenticated",
    retry: false,
  });
}

export function useBenutzerNachEmailQuery(
  anlageId: string,
  email: string,
  enabled: boolean,
) {
  const { api } = useLogandaimApi();
  const phase = useAuth();
  return useQuery({
    queryKey: logandaimKeys.anlagen.benutzerSuche(anlageId, email),
    queryFn: () => api.benutzerNachEmailSuchen(anlageId, email),
    enabled: phase.status === "authenticated" && enabled && Boolean(email),
    retry: false,
  });
}

export function useAnlageQuery(id: string | null) {
  const { api } = useLogandaimApi();
  const phase = useAuth();
  return useQuery({
    queryKey: logandaimKeys.anlagen.detail(id ?? ""),
    queryFn: () => api.anlageAnzeigen(id ?? ""),
    enabled: phase.status === "authenticated" && Boolean(id),
    retry: false,
  });
}

/** AC-13: no sign-in needed, so this one runs for every visitor. */
export function useParkourQuery() {
  const { api } = useLogandaimApi();
  return useQuery({
    queryKey: logandaimKeys.parkour.all(),
    queryFn: () => api.parkourAnzeigen(),
  });
}

/**
 * The public Anlagen directory. Public like the Parkour list, so it deliberately has
 * no `enabled` gate on `useAuth`: a guest reads it, and a signed-in visitor does not
 * refetch it when the session changes.
 */
export function useAnlagenVerzeichnisQuery() {
  const { api } = useLogandaimApi();
  return useQuery({
    queryKey: logandaimKeys.anlagenVerzeichnis.all(),
    queryFn: () => api.anlagenVerzeichnis(),
  });
}

export function useRundenQuery() {
  const { api } = useLogandaimApi();
  const phase = useAuth();
  return useQuery({
    queryKey: logandaimKeys.runden.all(),
    queryFn: () => api.rundenAnzeigen(),
    enabled: phase.status === "authenticated",
    retry: false,
  });
}

export function useRundeQuery(id: string | null) {
  const { api } = useLogandaimApi();
  const phase = useAuth();
  return useQuery({
    queryKey: logandaimKeys.runden.detail(id ?? ""),
    queryFn: () => api.rundeAnzeigen(id ?? ""),
    enabled: phase.status === "authenticated" && Boolean(id),
    retry: false,
  });
}
