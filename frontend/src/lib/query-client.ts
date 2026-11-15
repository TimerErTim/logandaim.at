import { MutationCache, QueryClient } from "@tanstack/react-query";

import { apiErrorMessage } from "@/api/errors";
import { GENERAL_ERROR_MESSAGE, reportGeneralError } from "@/api/fehlermeldung";

/**
 * Whether a mutation carries its own failure note. `meta.handlesError` is set by the
 * call site (or the hook) that renders a specific message, so the general note below
 * stays silent for it: a failure is reported once, by whichever layer can say
 * something useful about it.
 */
declare module "@tanstack/react-query" {
  interface Register {
    mutationMeta: {
      handlesError?: boolean;
    };
  }
}

export function createQueryClient(): QueryClient {
  return new QueryClient({
    mutationCache: new MutationCache({
      onError: (error, _variables, _result, mutation) => {
        if (mutation.meta?.handlesError) return;
        reportGeneralError(
          apiErrorMessage(error, { fallback: GENERAL_ERROR_MESSAGE }) ??
            GENERAL_ERROR_MESSAGE,
        );
      },
    }),
    defaultOptions: {
      queries: {
        staleTime: 30_000,
        gcTime: 5 * 60_000,
        retry: 1,
        refetchOnWindowFocus: true,
      },
      mutations: {
        retry: 0,
      },
    },
  });
}
