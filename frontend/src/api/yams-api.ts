/** Framework-agnostic YAMS API — implemented via OpenAPI HTTP. */
export interface YamsApi {
  health(): Promise<string>;
}
