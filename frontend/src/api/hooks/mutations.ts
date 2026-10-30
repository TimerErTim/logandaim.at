import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useLogandaimApi } from "../provider";
import { logandaimKeys } from "../query-keys";

/**
 * Master-data writes all answer the whole `Anlage`, so every one of them invalidates the
 * same two keys: the list (an archive changes it) and that Anlage's detail read.
 */
function useAnlageMutation<TVariables>(
  mutationFn: (variables: TVariables) => Promise<{ id: string }>,
) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn,
    onSuccess: (anlage) => {
      void queryClient.invalidateQueries({
        queryKey: logandaimKeys.anlagen.all(),
      });
      void queryClient.invalidateQueries({
        queryKey: logandaimKeys.anlagen.detail(anlage.id),
      });
      // Archiving a Parkour or Ziel changes the public list too (AC-13).
      void queryClient.invalidateQueries({
        queryKey: logandaimKeys.parkour.all(),
      });
      // …and the public Anlagen directory, which prints the Parkour counts.
      void queryClient.invalidateQueries({
        queryKey: logandaimKeys.anlagenVerzeichnis.all(),
      });
    },
  });
}

export function useStammdatenMutations() {
  const { api } = useLogandaimApi();
  const queryClient = useQueryClient();

  return {
    anlageErstellen: useAnlageMutation(
      (vars: Parameters<typeof api.anlageErstellen>[0]) =>
        api.anlageErstellen(vars),
    ),
    anlageAendern: useAnlageMutation(
      (vars: Parameters<typeof api.anlageAendern>) =>
        api.anlageAendern(vars[0], vars[1]),
    ),
    anlageDeaktivieren: useAnlageMutation((id: string) =>
      api.anlageDeaktivieren(id),
    ),
    verwaltungGewaehren: useAnlageMutation(
      (vars: Parameters<typeof api.verwaltungGewaehren>) =>
        api.verwaltungGewaehren(vars[0], vars[1], vars[2]),
    ),
    verwaltungAendern: useAnlageMutation(
      (vars: Parameters<typeof api.verwaltungAendern>) =>
        api.verwaltungAendern(vars[0], vars[1], vars[2]),
    ),
    verwaltungEntziehen: useAnlageMutation(
      (vars: Parameters<typeof api.verwaltungEntziehen>) =>
        api.verwaltungEntziehen(vars[0], vars[1]),
    ),
    zielHinzufuegen: useAnlageMutation(
      (vars: Parameters<typeof api.zielHinzufuegen>) =>
        api.zielHinzufuegen(vars[0], vars[1]),
    ),
    zielAendern: useAnlageMutation((vars: Parameters<typeof api.zielAendern>) =>
      api.zielAendern(vars[0], vars[1], vars[2]),
    ),
    zielErsetzen: useAnlageMutation(
      (vars: Parameters<typeof api.zielErsetzen>) =>
        api.zielErsetzen(vars[0], vars[1], vars[2]),
    ),
    zielEntfernen: useAnlageMutation(
      (vars: Parameters<typeof api.zielEntfernen>) =>
        api.zielEntfernen(vars[0], vars[1]),
    ),
    parkourHinzufuegen: useAnlageMutation(
      (vars: Parameters<typeof api.parkourHinzufuegen>) =>
        api.parkourHinzufuegen(vars[0], vars[1]),
    ),
    parkourSetupAendern: useAnlageMutation(
      (vars: Parameters<typeof api.parkourSetupAendern>) =>
        api.parkourSetupAendern(vars[0], vars[1], vars[2]),
    ),
    parkourUmbenennen: useAnlageMutation(
      (vars: Parameters<typeof api.parkourUmbenennen>) =>
        api.parkourUmbenennen(vars[0], vars[1], vars[2]),
    ),
    parkourEntfernen: useAnlageMutation(
      (vars: Parameters<typeof api.parkourEntfernen>) =>
        api.parkourEntfernen(vars[0], vars[1]),
    ),
    /** Starting a Runde changes the caller's own list and consumes the public one. */
    rundeStarten: useMutation({
      mutationFn: (parkourId: string) => api.rundeStarten(parkourId),
      onSuccess: () => {
        void queryClient.invalidateQueries({
          queryKey: logandaimKeys.runden.all(),
        });
      },
    }),
    rundeAbschliessen: useMutation({
      mutationFn: (rundeId: string) => api.rundeAbschliessen(rundeId),
      onSuccess: (runde) => {
        void queryClient.invalidateQueries({
          queryKey: logandaimKeys.runden.all(),
        });
        void queryClient.invalidateQueries({
          queryKey: logandaimKeys.runden.detail(runde.id),
        });
      },
    }),
    /** One arrow can change the per-Ziel and the whole-Runde Wertung. */
    schussRegistrieren: useMutation({
      mutationFn: (vars: {
        rundeId: string;
        schuss: Parameters<typeof api.schussRegistrieren>[1];
      }) => api.schussRegistrieren(vars.rundeId, vars.schuss),
      onSuccess: (_schuss, vars) => {
        void queryClient.invalidateQueries({
          queryKey: logandaimKeys.runden.detail(vars.rundeId),
        });
        void queryClient.invalidateQueries({
          queryKey: logandaimKeys.runden.all(),
        });
      },
    }),
  };
}

export function useAuthMutations() {
  const { api, authSession } = useLogandaimApi();
  const queryClient = useQueryClient();
  const loggedIn = () =>
    queryClient.invalidateQueries({ queryKey: logandaimKeys.benutzer() });
  const clear = () => {
    queryClient.removeQueries({ queryKey: logandaimKeys.benutzer() });
    queryClient.removeQueries({ queryKey: logandaimKeys.anlagen.all() });
    queryClient.removeQueries({ queryKey: logandaimKeys.runden.all() });
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
      mutationFn: () => authSession.logout(),
      onSettled: clear,
    }),
    deleteAccount: useMutation({
      mutationFn: ({
        passwort,
        bestätigt,
      }: {
        passwort: string;
        bestätigt: boolean;
      }) => api.benutzerLoeschen(passwort, bestätigt),
      onSettled: clear,
    }),
  };
}
