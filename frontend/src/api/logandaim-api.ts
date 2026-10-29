import {
  AnlageVerwaltungRolle,
  AnlageZustand,
  type components,
  CountryCode as Ländercode,
  RundeZustand,
  Wertungszone,
} from "./schema";

/** Re-exported so components import their vocabulary from one place. */
export {
  AnlageVerwaltungRolle,
  AnlageZustand,
  Ländercode,
  RundeZustand,
  Wertungszone,
};

/**
 * Transport-neutral API surface. Replacing this implementation is the seam for a
 * wasm/embedded core later — components never see HTTP.
 */
export interface LogandaimApi {
  health(): Promise<string>;

  // ---------------------------------------------------------------- master data
  anlagenAnzeigen(): Promise<AnlagenZeile[]>;
  anlageAnzeigen(id: string): Promise<Anlage>;
  anlageErstellen(anlage: AnlageErstellungInput): Promise<Anlage>;
  anlageAendern(id: string, anlage: AnlageErstellungInput): Promise<Anlage>;
  anlageDeaktivieren(id: string): Promise<Anlage>;

  benutzerNachEmailSuchen(
    anlageId: string,
    eMailAdresse: string,
  ): Promise<BenutzerSuchtreffer[]>;
  verwaltungGewaehren(
    anlageId: string,
    benutzerId: string,
    rolle: AnlageVerwaltungRolle,
  ): Promise<Anlage>;
  verwaltungAendern(
    anlageId: string,
    benutzerId: string,
    rolle: AnlageVerwaltungRolle,
  ): Promise<Anlage>;
  verwaltungEntziehen(anlageId: string, benutzerId: string): Promise<Anlage>;

  zielHinzufuegen(anlageId: string, ziel: ZielInput): Promise<Anlage>;
  zielAendern(
    anlageId: string,
    zielId: string,
    ziel: ZielInput,
  ): Promise<Anlage>;
  zielErsetzen(
    anlageId: string,
    zielId: string,
    ziel: ZielInput,
  ): Promise<Anlage>;
  zielEntfernen(anlageId: string, zielId: string): Promise<Anlage>;

  parcoursHinzufuegen(
    anlageId: string,
    parcours: ParcoursInput,
  ): Promise<Anlage>;
  parcoursSetupAendern(
    anlageId: string,
    parcoursId: string,
    setup: ParcoursSetupInput,
  ): Promise<Anlage>;
  parcoursUmbenennen(
    anlageId: string,
    parcoursId: string,
    name: string,
  ): Promise<Anlage>;
  parcoursEntfernen(anlageId: string, parcoursId: string): Promise<Anlage>;

  /** AC-13: no credentials needed; a stale token is answered as anonymous. */
  parcoursAnzeigen(): Promise<ParcoursÖffentlich[]>;

  // ---------------------------------------------------------------- Runden
  rundenAnzeigen(): Promise<RundenZeile[]>;
  rundeStarten(parcoursId: string): Promise<Runde>;
  rundeAnzeigen(id: string): Promise<Runde>;
  rundeAbschliessen(id: string): Promise<Runde>;
  schussRegistrieren(
    rundeId: string,
    schuss: SchussErstellungInput,
  ): Promise<Schuss>;

  // ---------------------------------------------------------------- konto
  registrierungStarten(eMailAdresse: string): Promise<void>;
  registrierungAbschliessen(token: string, passwort: string): Promise<void>;
  anmelden(eMailAdresse: string, passwort: string): Promise<void>;
  passwortVergessen(eMailAdresse: string): Promise<void>;
  passwortWiederherstellen(token: string, neuesPasswort: string): Promise<void>;
  benutzerAbrufen(): Promise<Benutzer>;
  benutzerLoeschen(passwort: string, bestätigt: boolean): Promise<void>;
}

/** Authentication lifecycle exposed without transport credentials or protocol details. */
export type AuthPhase =
  | { status: "checking" }
  | { status: "anonymous" }
  | { status: "authenticated" }
  | { status: "unavailable"; error: unknown };

/** Session lifecycle boundary; implementations own credentials and renewal mechanics. */
export interface AuthSession {
  restore(): Promise<void>;
  logout(): Promise<void>;
  getSnapshot(): AuthPhase;
  subscribe(listener: () => void): () => void;
}

type Schemas = components["schemas"];

export type Benutzer = Schemas["Benutzer"];
export type BenutzerSuchtreffer = Schemas["BenutzerSuchtreffer"];
export type Anlage = Schemas["Anlage"];
export type AnlagenZeile = Schemas["AnlagenZeile"];
export type AnlageAdresse = Schemas["AnlageAdresse"];
export type AnlageKontakt = Schemas["AnlageKontakt"];
export type AnlageVerwaltung = Schemas["AnlageVerwaltung"];
// `AnlageVerwaltungRolle`, `AnlageZustand`, `RundeZustand`, and `Wertungszone` are
// re-exported above as enums, which are their own type — a second alias would collide.
export type Ziel = Schemas["Ziel"];
export type Zielverwendung = Schemas["Zielverwendung"];
export type Pflock = Schemas["Pflock"];
export type PflockAbstand = Schemas["PflockAbstand"];
export type Zielplatzierung = Schemas["Zielplatzierung"];
export type ZielplatzierungAbstand = Schemas["ZielplatzierungAbstand"];
export type Parcours = Schemas["Parcours"];
export type ParcoursÖffentlich = Schemas["ParcoursÖffentlich"];
export type Runde = Schemas["Runde"];
export type RundenZeile = Schemas["RundenZeile"];
export type SnapshotPflock = Schemas["SnapshotPflock"];
export type SnapshotZiel = Schemas["SnapshotZiel"];
export type Schuss = Schemas["Schuss"];
export type Zielergebnis = Schemas["Zielergebnis"];
export type Zuordnung = Schemas["Zuordnung"];

export type AnlageErstellungInput = Schemas["AnlageErstellungRequest"];
export type ZielInput = Schemas["ZielRequest"];
export type ParcoursInput = Schemas["ParcoursErstellungRequest"];
export type ParcoursSetupInput = Schemas["ParcoursSetupRequest"];

export type SchussErstellungInput = {
  zielId: string;
  pfeil: number;
  wertungszone: Wertungszone;
};
