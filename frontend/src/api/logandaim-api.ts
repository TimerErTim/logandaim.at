import {
  AnlageVerwaltungRolle,
  AnlageZustand,
  type components,
  // openapi-typescript transliterates the umlaut in `Ländercode` to `LNdercode`; the
  // seam keeps the domain spelling so call sites never see the codegen artefact.
  LNdercode as Ländercode,
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
  anlagenAnzeigen(): Promise<AnlageZugriff[]>;
  anlageAnzeigen(id: string): Promise<Anlage>;
  anlageErstellen(anlage: AnlageErstellungInput): Promise<Anlage>;
  anlageAendern(id: string, anlage: AnlageErstellungInput): Promise<Anlage>;
  anlageDeaktivieren(id: string): Promise<Anlage>;

  benutzerNachEmailSuchen(
    anlageId: string,
    eMailAdresse: string,
  ): Promise<Benutzer[]>;
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

  parkourHinzufuegen(anlageId: string, parkour: ParkourInput): Promise<Anlage>;
  parkourSetupAendern(
    anlageId: string,
    parkourId: string,
    setup: ParkourSetupInput,
  ): Promise<Anlage>;
  parkourUmbenennen(
    anlageId: string,
    parkourId: string,
    name: string,
  ): Promise<Anlage>;
  parkourEntfernen(anlageId: string, parkourId: string): Promise<Anlage>;

  /** AC-13: no credentials needed; a stale token is answered as anonymous. */
  parkourAnzeigen(): Promise<ParkourÖffentlich[]>;

  /** The public Anlagen directory: no credentials, no contact data, no session. */
  anlagenVerzeichnis(): Promise<AnlageÖffentlich[]>;

  // ---------------------------------------------------------------- Runden
  rundenAnzeigen(): Promise<RundeBasic[]>;
  rundeStarten(parkourId: string): Promise<Runde>;
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
  benutzerAbrufen(): Promise<AuthenticatedBenutzer>;
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
/** The authenticated caller: the only shape that carries a `rolle`. */
export type AuthenticatedBenutzer = Schemas["AuthenticatedBenutzer"];
export type Anlage = Schemas["Anlage"];
export type AnlageZugriff = Schemas["AnlageZugriff"];
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
export type Parkour = Schemas["Parkour"];
export type ParkourÖffentlich = Schemas["ParkourÖffentlich"];
export type AnlageÖffentlich = Schemas["AnlageÖffentlich"];
export type ParkourKurz = Schemas["ParkourKurz"];
export type Runde = Schemas["Runde"];
export type RundeBasic = Schemas["RundeBasic"];
export type RundePflock = Schemas["RundePflock"];
export type RundeZiel = Schemas["RundeZiel"];
export type Schuss = Schemas["Schuss"];
export type Zielwertung = Schemas["Zielwertung"];
export type Erfasser = Schemas["Erfasser"];

export type AnlageErstellungInput = Schemas["AnlageErstellungRequest"];
export type ZielInput = Schemas["ZielRequest"];
export type ParkourInput = Schemas["ParkourErstellungRequest"];
export type ParkourSetupInput = Schemas["ParkourSetupRequest"];

export type SchussErstellungInput = {
  zielId: string;
  pfeil: number;
  wertungszone: Wertungszone;
};
