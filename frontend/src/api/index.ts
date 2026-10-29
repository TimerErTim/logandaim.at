import { config } from "@/lib/config";
import { HttpLogandaimApi } from "./http-client";
import type { AuthSession, LogandaimApi } from "./logandaim-api";

/**
 * The one place that picks a transport.
 *
 * `LogandaimApi` stays an interface on purpose: swapping this single line for an
 * embedded/wasm implementation of the core domain is the intended future move.
 */
export function createLogandaimApi(): {
  api: LogandaimApi;
  authSession: AuthSession;
} {
  const transport = new HttpLogandaimApi(config.apiBaseUrl);
  return { api: transport, authSession: transport };
}
