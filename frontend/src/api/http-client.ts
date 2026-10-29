import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type {
  Anlage,
  AnlageErstellungInput,
  AnlagenZeile,
  AnlageVerwaltungRolle,
  AuthPhase,
  AuthSession,
  Benutzer,
  LogandaimApi,
  ParcoursInput,
  ParcoursSetupInput,
  ParcoursÖffentlich,
  Runde,
  RundenZeile,
  Schuss,
  SchussErstellungInput,
  ZielInput,
} from "./logandaim-api";
import type { paths } from "./schema";

type JsonClient = ReturnType<typeof createClient<paths>>;

async function unwrap<T>(result: {
  data?: T;
  error?: unknown;
  response: Response;
}): Promise<T> {
  if (result.error !== undefined) {
    throw ApiError.fromUnknown(result.error, result.response.status);
  }
  return result.data as T;
}

/** HTTP implementation of the transport-neutral API. */
export class HttpLogandaimApi implements LogandaimApi, AuthSession {
  private readonly client: JsonClient;
  private renewal: Promise<void> | null = null;
  private readonly baseUrl: string;
  private accessToken: string | null = null;
  private authRevision = 0;
  private phase: AuthPhase = Object.freeze({ status: "checking" });
  private readonly listeners = new Set<() => void>();

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.client = createClient<paths>({
      baseUrl: this.baseUrl,
      fetch: (request) =>
        fetch(new Request(request, { credentials: "include" })),
    });
  }

  private setAccessToken(token: string | null): void {
    this.authRevision += 1;
    this.accessToken = token;
    this.setPhase(
      token ? { status: "authenticated" } : { status: "anonymous" },
    );
  }

  private setPhase(phase: AuthPhase): void {
    if (this.phase.status === phase.status) return;
    this.phase = Object.freeze(phase);
    for (const listener of this.listeners) listener();
  }

  getSnapshot = (): AuthPhase => this.phase;

  subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  async restore(): Promise<void> {
    this.setPhase({ status: "checking" });
    try {
      await this.renewSession();
    } catch (error) {
      if (error instanceof ApiError && error.status === 401) {
        this.setAccessToken(null);
        return;
      }
      this.setPhase({ status: "unavailable", error });
      throw error;
    }
  }

  async logout(): Promise<void> {
    const headers = this.authorizationHeaders();
    this.setAccessToken(null);
    try {
      await unwrap(await this.client.POST("/session/abmelden", { headers }));
    } finally {
      this.setAccessToken(null);
    }
  }

  private async authenticated<T>(
    request: () => Promise<{
      data?: T;
      error?: unknown;
      response: Response;
    }>,
  ): Promise<T> {
    const token = this.accessToken;
    const result = await request();
    if (result.response.status !== 401 || !token) {
      return unwrap(result);
    }
    if (this.accessToken !== token) return unwrap(await request());

    try {
      await this.renewSession();
    } catch (error) {
      if (error instanceof ApiError && error.status === 401) {
        this.setAccessToken(null);
        return unwrap(result);
      }
      throw error;
    }
    if (this.accessToken === token) return unwrap(result);
    return unwrap(await request());
  }

  private async renewSession(): Promise<void> {
    if (!this.renewal) {
      this.renewal = this.performRenewal().finally(() => {
        this.renewal = null;
      });
    }
    return this.renewal;
  }

  private async performRenewal(): Promise<void> {
    const revision = this.authRevision;
    const renewed = await fetch(`${this.baseUrl}/session/erneuern`, {
      method: "POST",
      credentials: "include",
    });
    if (!renewed.ok) {
      let error: unknown = `Session renewal failed (${renewed.status})`;
      try {
        error = await renewed.json();
      } catch {
        // Keep a useful status-bearing message for non-JSON responses.
      }
      throw ApiError.fromUnknown(error, renewed.status);
    }
    const content = (await renewed.json()) as { accessToken: string };
    if (revision !== this.authRevision) return;
    this.setAccessToken(content.accessToken);
  }

  async health(): Promise<string> {
    return unwrap(await this.client.GET("/health"));
  }

  private authorizationHeaders() {
    return {
      Authorization: this.accessToken
        ? `Bearer ${this.accessToken}`
        : undefined,
    };
  }

  // ---------------------------------------------------------------- master data

  async anlagenAnzeigen(): Promise<AnlagenZeile[]> {
    return this.authenticated(() =>
      this.client.GET("/anlagen", { headers: this.authorizationHeaders() }),
    );
  }

  async anlageAnzeigen(id: string): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.GET("/anlagen/{anlage_id}", {
        params: { path: { anlage_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async anlageErstellen(anlage: AnlageErstellungInput): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.POST("/anlagen", {
        body: anlage,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async anlageAendern(
    id: string,
    anlage: AnlageErstellungInput,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.PUT("/anlagen/{anlage_id}", {
        params: { path: { anlage_id: id } },
        body: anlage,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async anlageDeaktivieren(id: string): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.POST("/anlagen/{anlage_id}/deaktivierung", {
        params: { path: { anlage_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async verwaltungGewaehren(
    anlageId: string,
    benutzerId: string,
    rolle: AnlageVerwaltungRolle,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.POST("/anlagen/{anlage_id}/verwaltungen", {
        params: { path: { anlage_id: anlageId } },
        body: { benutzerId, rolle },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async verwaltungAendern(
    anlageId: string,
    benutzerId: string,
    rolle: AnlageVerwaltungRolle,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.PUT("/anlagen/{anlage_id}/verwaltungen/{benutzer_id}", {
        params: { path: { anlage_id: anlageId, benutzer_id: benutzerId } },
        body: { benutzerId, rolle },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async verwaltungEntziehen(
    anlageId: string,
    benutzerId: string,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.DELETE("/anlagen/{anlage_id}/verwaltungen/{benutzer_id}", {
        params: { path: { anlage_id: anlageId, benutzer_id: benutzerId } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async zielHinzufuegen(anlageId: string, ziel: ZielInput): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.POST("/anlagen/{anlage_id}/ziele", {
        params: { path: { anlage_id: anlageId } },
        body: ziel,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async zielAendern(
    anlageId: string,
    zielId: string,
    ziel: ZielInput,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.PUT("/anlagen/{anlage_id}/ziele/{ziel_id}", {
        params: { path: { anlage_id: anlageId, ziel_id: zielId } },
        body: ziel,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async zielErsetzen(
    anlageId: string,
    zielId: string,
    ziel: ZielInput,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.POST("/anlagen/{anlage_id}/ziele/{ziel_id}/ersatz", {
        params: { path: { anlage_id: anlageId, ziel_id: zielId } },
        body: ziel,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async zielEntfernen(anlageId: string, zielId: string): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.DELETE("/anlagen/{anlage_id}/ziele/{ziel_id}", {
        params: { path: { anlage_id: anlageId, ziel_id: zielId } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parcoursHinzufuegen(
    anlageId: string,
    parcours: ParcoursInput,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.POST("/anlagen/{anlage_id}/parcours", {
        params: { path: { anlage_id: anlageId } },
        body: parcours,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parcoursSetupAendern(
    anlageId: string,
    parcoursId: string,
    setup: ParcoursSetupInput,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.PUT("/anlagen/{anlage_id}/parcours/{parcours_id}/setup", {
        params: { path: { anlage_id: anlageId, parcours_id: parcoursId } },
        body: setup,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parcoursUmbenennen(
    anlageId: string,
    parcoursId: string,
    name: string,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.PUT("/anlagen/{anlage_id}/parcours/{parcours_id}", {
        params: { path: { anlage_id: anlageId, parcours_id: parcoursId } },
        body: { name },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parcoursEntfernen(
    anlageId: string,
    parcoursId: string,
  ): Promise<Anlage> {
    return this.authenticated(() =>
      this.client.DELETE("/anlagen/{anlage_id}/parcours/{parcours_id}", {
        params: { path: { anlage_id: anlageId, parcours_id: parcoursId } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parcoursAnzeigen(): Promise<ParcoursÖffentlich[]> {
    // Public by design (AC-13): never renew a session for it, and send no header at all.
    return unwrap(await this.client.GET("/parcours"));
  }

  // ---------------------------------------------------------------- Runden

  async rundenAnzeigen(): Promise<RundenZeile[]> {
    return this.authenticated(() =>
      this.client.GET("/runden", { headers: this.authorizationHeaders() }),
    );
  }

  async rundeStarten(parcoursId: string): Promise<Runde> {
    return this.authenticated(() =>
      this.client.POST("/runden", {
        body: { parcoursId },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async rundeAnzeigen(id: string): Promise<Runde> {
    return this.authenticated(() =>
      this.client.GET("/runden/{runde_id}", {
        params: { path: { runde_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async rundeAbschliessen(id: string): Promise<Runde> {
    return this.authenticated(() =>
      this.client.POST("/runden/{runde_id}/abschluss", {
        params: { path: { runde_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async schussRegistrieren(
    rundeId: string,
    schuss: SchussErstellungInput,
  ): Promise<Schuss> {
    return this.authenticated(() =>
      this.client.POST("/runden/{runde_id}/schüsse", {
        params: { path: { runde_id: rundeId } },
        body: schuss,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async registrierungStarten(eMailAdresse: string): Promise<void> {
    await unwrap(
      await this.client.POST("/registrierung/start", {
        body: { eMailAdresse },
      }),
    );
  }

  async registrierungAbschliessen(
    token: string,
    passwort: string,
  ): Promise<void> {
    const result = await unwrap(
      await this.client.POST("/registrierung/abschluss", {
        body: { token, passwort },
      }),
    );
    this.setAccessToken(result.accessToken);
  }

  async anmelden(eMailAdresse: string, passwort: string): Promise<void> {
    const result = await unwrap(
      await this.client.POST("/anmeldung", {
        body: { eMailAdresse, passwort },
      }),
    );
    this.setAccessToken(result.accessToken);
  }

  async passwortVergessen(eMailAdresse: string): Promise<void> {
    await unwrap(
      await this.client.POST("/passwort/vergessen", { body: { eMailAdresse } }),
    );
  }

  async passwortWiederherstellen(
    token: string,
    neuesPasswort: string,
  ): Promise<void> {
    await unwrap(
      await this.client.POST("/passwort/wiederherstellen", {
        body: { token, neuesPasswort },
      }),
    );
    this.setAccessToken(null);
  }

  async benutzerAbrufen(): Promise<Benutzer> {
    return this.authenticated(() =>
      this.client.GET("/benutzer", {
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async benutzerLoeschen(passwort: string, bestätigt: boolean): Promise<void> {
    await this.authenticated(() =>
      this.client.POST("/benutzer/löschung", {
        body: { passwort, bestätigt },
        headers: this.authorizationHeaders(),
      }),
    );
    this.setAccessToken(null);
  }
}
