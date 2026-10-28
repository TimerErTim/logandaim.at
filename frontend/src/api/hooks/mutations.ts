import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAuthSession } from "../auth-provider";
import { useLogandaimApi } from "../provider";
import { logandaimKeys } from "../query-keys";

export function useSchussRegistrierenMutation() {
  const { api } = useLogandaimApi();
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: api.schussRegistrieren.bind(api),
    onSuccess: () => {
      void queryClient.invalidateQueries({
        queryKey: logandaimKeys.schüsse.all(),
      });
    },
  });
}

export function useWertungMutation() {
  const { api } = useLogandaimApi();
  return useMutation({ mutationFn: api.wertungBerechnen.bind(api) });
}

export function useAuthMutations() {
  const { api } = useLogandaimApi();
  const { clearSession, markAuthenticated } = useAuthSession();
  const queryClient = useQueryClient();
  const loggedIn = () => {
    markAuthenticated();
    return queryClient.invalidateQueries({
      queryKey: logandaimKeys.benutzer(),
    });
  };
  const clear = () => {
    clearSession();
  };

  return {
    login: useMutation({
      mutationFn: ({
        eMailAdresse,
        passwort,
      }: {
        eMailAdresse: string;
        passwort: string;
      }) => api.anmelden(eMailAdresse, passwort),
      onSuccess: loggedIn,
    }),
    registerStart: useMutation({
      mutationFn: api.registrierungStarten.bind(api),
    }),
    registerFinish: useMutation({
      mutationFn: ({ token, passwort }: { token: string; passwort: string }) =>
        api.registrierungAbschliessen(token, passwort),
      onSuccess: loggedIn,
    }),
    forgotPassword: useMutation({
      mutationFn: api.passwortVergessen.bind(api),
    }),
    resetPassword: useMutation({
      mutationFn: ({
        token,
        neuesPasswort,
      }: {
        token: string;
        neuesPasswort: string;
      }) => api.passwortWiederherstellen(token, neuesPasswort),
      onSuccess: clear,
    }),
    logout: useMutation({
      mutationFn: api.sessionAbmelden.bind(api),
      onSettled: clear,
    }),
  };
}
