import type { components } from "./schema";

/** Re-exported so components import their vocabulary from one place. */
export { Wertungszone } from "./schema";

/**
 * Transport-neutral API surface. Replacing this implementation is the seam for a
 * wasm/embedded core later — components never see HTTP.
 */
export interface LogandaimApi {
  health(): Promise<string>;

  schussRegistrieren(erstellung: SchussErstellungInput): Promise<Schuss>;

  wertungBerechnen(schussIds: string[]): Promise<Wertung>;
}

export type Schuss = components["schemas"]["Schuss"];
export type Wertung = components["schemas"]["Wertung"];

export type SchussErstellungInput = {
  zielId: string;
  pfeil: number;
  wertungszone: components["schemas"]["Wertungszone"];
};
