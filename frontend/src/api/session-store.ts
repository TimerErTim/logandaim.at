import { useSyncExternalStore } from "react";

export type AuthSnapshot = Readonly<{
  accessToken: string | null;
  bootstrapped: boolean;
}>;

const initialSnapshot: AuthSnapshot = Object.freeze({
  accessToken: null,
  bootstrapped: false,
});

let snapshot = initialSnapshot;
const listeners = new Set<() => void>();

export function getAuthSnapshot(): AuthSnapshot {
  return snapshot;
}

export function subscribeToAuth(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function setAccessToken(accessToken: string | null): void {
  if (snapshot.accessToken === accessToken) return;
  snapshot = Object.freeze({ ...snapshot, accessToken });
  for (const listener of listeners) listener();
}

export function setAuthBootstrapped(bootstrapped: boolean): void {
  if (snapshot.bootstrapped === bootstrapped) return;
  snapshot = Object.freeze({ ...snapshot, bootstrapped });
  for (const listener of listeners) listener();
}

export function useAuthSnapshot(): AuthSnapshot {
  return useSyncExternalStore(
    subscribeToAuth,
    getAuthSnapshot,
    () => initialSnapshot,
  );
}
