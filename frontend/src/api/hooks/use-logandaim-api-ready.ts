import { useLogandaimApi } from "../provider";

/** LogandaimApi context with a boolean gate for TanStack Query `enabled`. */
export function useLogandaimApiReady() {
  const context = useLogandaimApi();
  const isReady = !context.loading && context.api !== null;

  return {
    ...context,
    isReady,
    api: context.api,
  };
}
