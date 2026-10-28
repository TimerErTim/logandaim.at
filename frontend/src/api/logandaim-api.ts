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

  registrierungStarten(eMailAdresse: string): Promise<void>;
  registrierungAbschliessen(token: string, passwort: string): Promise<void>;
  anmelden(eMailAdresse: string, passwort: string): Promise<void>;
  sessionErneuern(): Promise<void>;
  sessionAbmelden(): Promise<void>;
  passwortVergessen(eMailAdresse: string): Promise<void>;
  passwortWiederherstellen(token: string, neuesPasswort: string): Promise<void>;
  benutzerAbrufen(): Promise<Benutzer>;
  accessTokenSetzen(token: string | null): void;
}

export type Benutzer = components["schemas"]["Benutzer"];
export type Schuss = components["schemas"]["Schuss"];
export type Wertung = components["schemas"]["Wertung"];

export type SchussErstellungInput = {
  zielId: string;
  pfeil: number;
  wertungszone: components["schemas"]["Wertungszone"];
};
