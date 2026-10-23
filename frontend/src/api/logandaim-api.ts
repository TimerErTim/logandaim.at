/** Framework-agnostic logandaim.at API — implemented via OpenAPI HTTP. */
export interface LogandaimApi {
  health(): Promise<string>;
}
