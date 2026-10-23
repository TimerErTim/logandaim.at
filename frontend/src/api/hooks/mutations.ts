import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useLogandaimApi } from "../provider";
import { logandaimKeys } from "../query-keys";

/** Register one Schuss at a Ziel. */
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

/** Summarize Erster Treffer and Mehrschuss for a selection of Schüsse. */
export function useWertungMutation() {
  const { api } = useLogandaimApi();

  return useMutation({
    mutationFn: api.wertungBerechnen.bind(api),
  });
}
