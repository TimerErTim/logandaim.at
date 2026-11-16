import {
  AnlageVerwaltungRolle,
  AnlageZustand,
  type components,
  Laendercode,
  RundeZustand,
  Wertungszone,
  ZielPfloecke_ExplicitArt,
  ZielPfloecke_InheritFromParkourArt,
} from "./schema";

export {
  AnlageVerwaltungRolle,
  AnlageZustand,
  Laendercode,
  RundeZustand,
  Wertungszone,
  ZielPfloecke_ExplicitArt,
  ZielPfloecke_InheritFromParkourArt,
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
  freieRundeStarten(name: string): Promise<Runde>;
  freiesZielHinzufuegen(rundeId: string, name?: string): Promise<Runde>;
  freiesZielUmbenennen(
    rundeId: string,
    zielId: string,
    name: string,
  ): Promise<Runde>;
  zielNotizSpeichern(
    rundeId: string,
    zielId: string,
    notiz: string,
  ): Promise<Runde>;
  freiesZielEntfernen(rundeId: string, zielId: string): Promise<Runde>;
  rundeLoeschen(id: string): Promise<void>;
  rundeAnzeigen(id: string): Promise<Runde>;
  rundeAbschliessen(id: string): Promise<Runde>;
  schussRegistrieren(
    rundeId: string,
    schuss: SchussErstellungInput,
  ): Promise<Schuss>;
  schussAendern(
    rundeId: string,
    schussId: string,
    schuss: SchussAenderungInput,
  ): Promise<Schuss>;
  schussLoeschen(rundeId: string, schussId: string): Promise<void>;

  // ---------------------------------------------------------------- konto
  registrierungStarten(eMailAdresse: string): Promise<void>;
  registrierungAbschliessen(token: string, passwort: string): Promise<void>;
  anmelden(eMailAdresse: string, passwort: string): Promise<void>;
  passwortVergessen(eMailAdresse: string): Promise<void>;
  passwortWiederherstellen(token: string, neuesPasswort: string): Promise<void>;
  benutzerAbrufen(): Promise<AuthenticatedBenutzer>;
  benutzerLoeschen(passwort: string, bestaetigt: boolean): Promise<void>;
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

export type Benutzer = components["schemas"]["Benutzer"];
/** The authenticated caller: the only shape that carries a `rolle`. */
export type AuthenticatedBenutzer =
  components["schemas"]["AuthenticatedBenutzer"];
export type Anlage = components["schemas"]["Anlage"];
export type AnlageZugriff = components["schemas"]["AnlageZugriff"];
export type AnlageAdresse = components["schemas"]["AnlageAdresse"];
export type AnlageKontakt = components["schemas"]["AnlageKontakt"];
export type AnlageVerwaltung = components["schemas"]["AnlageVerwaltung"];
export type AnlageZiel = components["schemas"]["AnlageZiel"];
export type ZielPfloecke = components["schemas"]["ZielPfloecke"];
export type ZielPfloeckeExplicit =
  components["schemas"]["ZielPfloecke_Explicit"];
export type ZielPfloeckeInheritFromParkour =
  components["schemas"]["ZielPfloecke_InheritFromParkour"];
export type Zielverwendung = components["schemas"]["Zielverwendung"];
export type ParkourPflock = components["schemas"]["ParkourPflock"];
export type PflockAbstand = components["schemas"]["PflockAbstand"];
export type Ziel = components["schemas"]["Ziel"];
type RundenPflock = Omit<components["schemas"]["Pflock"], "abstand"> & {
  abstand?: number | null;
};
export type RundeZiel = Omit<Ziel, "pfloecke"> & { pfloecke: RundenPflock[] };
export type Zielplatzierung = components["schemas"]["Zielplatzierung"];
export type Parkour = components["schemas"]["Parkour"];
export type ParkourÖffentlich = components["schemas"]["ParkourOeffentlich"];
export type AnlageOeffentlich = components["schemas"]["AnlageOeffentlich"];
export type AnlageÖffentlich = AnlageOeffentlich;
export type ParkourKurz = components["schemas"]["ParkourKurz"];
export type RundeHerkunft = components["schemas"]["RundeHerkunft"];
export type RundeBasic = components["schemas"]["RundeBasic"];
export type Runde = components["schemas"]["Runde"];
export type Schuss = components["schemas"]["Schuss"];
export type Zielwertung = components["schemas"]["Zielwertung"];
export type Erfasser = components["schemas"]["Erfasser"];

export type AnlageErstellungInput =
  components["schemas"]["AnlageErstellungRequest"];
export type ZielInput = components["schemas"]["ZielRequest"];
export type ParkourInput = components["schemas"]["ParkourErstellungRequest"];
export type ParkourSetupInput = components["schemas"]["ParkourSetupRequest"];

export type SchusspositionInput =
  | components["schemas"]["SchusspositionRequest"]
  | components["schemas"]["Schussposition"];

export type SchussErstellungInput = {
  zielId: string;
  wertungszone: Wertungszone;
  position: SchusspositionInput | null;
};

export type SchussAenderungInput = {
  wertungszone: Wertungszone;
  position: SchusspositionInput | null;
};
