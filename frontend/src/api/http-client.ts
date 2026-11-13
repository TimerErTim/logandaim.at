import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type {
  Anlage,
  AnlageErstellungInput,
  AnlageVerwaltungRolle,
  AnlageZugriff,
  AnlageÖffentlich,
  AuthenticatedBenutzer,
  AuthPhase,
  AuthSession,
  Benutzer,
  Domainize,
  LogandaimApi,
  ParkourInput,
  ParkourSetupInput,
  ParkourÖffentlich,
  Runde,
  RundeBasic,
  Schuss,
  SchussAenderungInput,
  SchussErstellungInput,
  ZielInput,
} from "./logandaim-api";
import type { paths } from "./schema";

// The client crosses a domain-name mapping boundary at fetch; wire shapes remain defined by OpenAPI.
type DomainizeContent<Content> = {
  [Type in keyof Content]: Type extends `application/json${string}`
    ? Domainize<Content[Type]>
    : Content[Type];
};
type DomainizeBody<Body> = Body extends { content: infer Content }
  ? Omit<Body, "content"> & { content: DomainizeContent<Content> }
  : Body;
type DomainizeResponses<Responses> = {
  [Status in keyof Responses]: DomainizeBody<Responses[Status]>;
};
type DomainizeOperation<Operation> = Operation extends {
  responses: infer Responses;
}
  ? Omit<Operation, "responses" | "requestBody"> & {
      responses: DomainizeResponses<Responses>;
    } & (Operation extends { requestBody: infer Body }
        ? { requestBody: DomainizeBody<Body> }
        : Record<never, never>)
  : Operation;
type DomainPaths = {
  [Path in keyof paths]: {
    [Method in keyof paths[Path]]: DomainizeOperation<paths[Path][Method]>;
  };
};
type JsonClient = ReturnType<typeof createClient<DomainPaths>>;

const wireToDomain: Record<string, string> = {
  strasse: "straße",
  laendercode: "ländercode",
  bestaetigt: "bestätigt",
  pfloecke: "pflöcke",
  abstaende: "abstände",
  schuesse: "schüsse",
  anzahlPfloecke: "anzahlPflöcke",
  anzahlSchuesse: "anzahlSchüsse",
  AnlageAenderungRequest: "AnlageÄnderungRequest",
  AnlageOeffentlich: "AnlageÖffentlich",
  BenutzerLoeschungRequest: "BenutzerLöschungRequest",
  ExplicitPfloecke: "ExplicitPflöcke",
  InheritFromParkourPfloecke: "InheritFromParkourPflöcke",
  Laendercode: "Ländercode",
  ParkourOeffentlich: "ParkourÖffentlich",
  SchussAenderungRequest: "SchussÄnderungRequest",
  ZielPfloecke: "ZielPflöcke",
};
const domainToWire = Object.fromEntries(
  Object.entries(wireToDomain).map(([wire, domain]) => [domain, wire]),
);

function mapApiKeys(value: unknown, names: Record<string, string>): unknown {
  if (Array.isArray(value)) return value.map((item) => mapApiKeys(item, names));
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.entries(value).map(([key, item]) => [
      names[key] ?? key,
      mapApiKeys(item, names),
    ]),
  );
}

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
    this.client = createClient<DomainPaths>({
      baseUrl: this.baseUrl,
      fetch: async (request) => {
        let outgoing = request;
        if (request.headers.get("content-type")?.includes("application/json")) {
          const body = await request.clone().json();
          outgoing = new Request(request, {
            body: JSON.stringify(mapApiKeys(body, domainToWire)),
          });
        }
        const response = await fetch(
          new Request(outgoing, { credentials: "include" }),
        );
        if (
          !response.headers.get("content-type")?.includes("application/json")
        ) {
          return response;
        }
        const body = mapApiKeys(await response.clone().json(), wireToDomain);
        const headers = new Headers(response.headers);
        headers.delete("content-length");
        headers.delete("content-encoding");
        const mappedResponse = new Response(JSON.stringify(body), {
          status: response.status,
          statusText: response.statusText,
          headers,
        });
        return mappedResponse;
      },
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
      return unwrap<T>(result);
    }
    if (this.accessToken !== token) return unwrap<T>(await request());

    try {
      await this.renewSession();
    } catch (error) {
      if (error instanceof ApiError && error.status === 401) {
        this.setAccessToken(null);
        return unwrap<T>(result);
      }
      throw error;
    }
    if (this.accessToken === token) return unwrap<T>(result);
    return unwrap<T>(await request());
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

  async anlagenAnzeigen(): Promise<AnlageZugriff[]> {
    return this.authenticated(async () => {
      const result = await this.client.GET("/anlagen", {
        headers: this.authorizationHeaders(),
      });
      if (result.data) {
        result.data = result.data.map((entry) => ({
          ...entry,
          anlage: entry.anlage,
        }));
      }
      return result;
    });
  }

  async anlageAnzeigen(id: string): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.GET("/anlagen/{anlage_id}", {
        params: { path: { anlage_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async anlageErstellen(anlage: AnlageErstellungInput): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
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
    return this.authenticated<Anlage>(() =>
      this.client.PUT("/anlagen/{anlage_id}", {
        params: { path: { anlage_id: id } },
        body: anlage,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async anlageDeaktivieren(id: string): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.POST("/anlagen/{anlage_id}/deaktivierung", {
        params: { path: { anlage_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async benutzerNachEmailSuchen(
    anlageId: string,
    eMailAdresse: string,
  ): Promise<Benutzer[]> {
    return this.authenticated(() =>
      this.client.GET("/anlagen/{anlage_id}/benutzer-suche", {
        params: {
          path: { anlage_id: anlageId },
          query: { e_mail_adresse: eMailAdresse },
        },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async verwaltungGewaehren(
    anlageId: string,
    benutzerId: string,
    rolle: AnlageVerwaltungRolle,
  ): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
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
    return this.authenticated<Anlage>(() =>
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
    return this.authenticated<Anlage>(() =>
      this.client.DELETE("/anlagen/{anlage_id}/verwaltungen/{benutzer_id}", {
        params: { path: { anlage_id: anlageId, benutzer_id: benutzerId } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async zielHinzufuegen(anlageId: string, ziel: ZielInput): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
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
    return this.authenticated<Anlage>(() =>
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
    return this.authenticated<Anlage>(() =>
      this.client.POST("/anlagen/{anlage_id}/ziele/{ziel_id}/ersatz", {
        params: { path: { anlage_id: anlageId, ziel_id: zielId } },
        body: ziel,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async zielEntfernen(anlageId: string, zielId: string): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.DELETE("/anlagen/{anlage_id}/ziele/{ziel_id}", {
        params: { path: { anlage_id: anlageId, ziel_id: zielId } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parkourHinzufuegen(
    anlageId: string,
    parkour: ParkourInput,
  ): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.POST("/anlagen/{anlage_id}/parkour", {
        params: { path: { anlage_id: anlageId } },
        body: parkour as never,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parkourSetupAendern(
    anlageId: string,
    parkourId: string,
    setup: ParkourSetupInput,
  ): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.PUT("/anlagen/{anlage_id}/parkour/{parkour_id}/setup", {
        params: { path: { anlage_id: anlageId, parkour_id: parkourId } },
        body: setup as never,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parkourUmbenennen(
    anlageId: string,
    parkourId: string,
    name: string,
  ): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.PUT("/anlagen/{anlage_id}/parkour/{parkour_id}", {
        params: { path: { anlage_id: anlageId, parkour_id: parkourId } },
        body: { name },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parkourEntfernen(anlageId: string, parkourId: string): Promise<Anlage> {
    return this.authenticated<Anlage>(() =>
      this.client.DELETE("/anlagen/{anlage_id}/parkour/{parkour_id}", {
        params: { path: { anlage_id: anlageId, parkour_id: parkourId } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async parkourAnzeigen(): Promise<ParkourÖffentlich[]> {
    // Public by design (AC-13): never renew a session for it, and send no header at all.
    return unwrap(await this.client.GET("/parkour"));
  }

  async anlagenVerzeichnis(): Promise<AnlageÖffentlich[]> {
    // Public by design (Anlagen-Verzeichnis): a stale token is answered as anonymous, so
    // this read neither sends a header nor renews a session.
    return unwrap(await this.client.GET("/anlagen/oeffentlich"));
  }

  // ---------------------------------------------------------------- Runden

  async rundenAnzeigen(): Promise<RundeBasic[]> {
    const runden = await this.authenticated(() =>
      this.client.GET("/runden", { headers: this.authorizationHeaders() }),
    );
    return runden.map((runde) => ({
      ...runde,
      herkunft: runde.herkunft ?? null,
    }));
  }

  async rundeStarten(parkourId: string): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.POST("/runden/mit-parkour", {
        body: { parkourId },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async freieRundeStarten(name: string): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.POST("/runden/frei", {
        body: { name },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async freiesZielHinzufuegen(rundeId: string, name?: string): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.POST("/runden/{runde_id}/ziele", {
        params: { path: { runde_id: rundeId } },
        body: name === undefined ? {} : { name },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async freiesZielUmbenennen(
    rundeId: string,
    zielId: string,
    name: string,
  ): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.PUT("/runden/{runde_id}/ziele/{ziel_id}", {
        params: { path: { runde_id: rundeId, ziel_id: zielId } },
        body: { name },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async freiesZielEntfernen(rundeId: string, zielId: string): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.DELETE("/runden/{runde_id}/ziele/{ziel_id}", {
        params: { path: { runde_id: rundeId, ziel_id: zielId } },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async rundeLoeschen(id: string): Promise<void> {
    await this.authenticated(() =>
      this.client.DELETE("/runden/{runde_id}", {
        params: { path: { runde_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async rundeAnzeigen(id: string): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.GET("/runden/{runde_id}", {
        params: { path: { runde_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async rundeAbschliessen(id: string): Promise<Runde> {
    const runde = await this.authenticated(() =>
      this.client.POST("/runden/{runde_id}/abschluss", {
        params: { path: { runde_id: id } },
        headers: this.authorizationHeaders(),
      }),
    );
    return { ...runde, herkunft: runde.herkunft ?? null };
  }

  async schussRegistrieren(
    rundeId: string,
    schuss: SchussErstellungInput,
  ): Promise<Schuss> {
    return this.authenticated(() =>
      this.client.POST("/runden/{runde_id}/schuesse", {
        params: { path: { runde_id: rundeId } },
        body: { ...schuss, position: schuss.position ?? undefined },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async schussAendern(
    rundeId: string,
    schussId: string,
    schuss: SchussAenderungInput,
  ): Promise<Schuss> {
    return this.authenticated(() =>
      this.client.PUT("/runden/{runde_id}/schuesse/{schuss_id}", {
        params: { path: { runde_id: rundeId, schuss_id: schussId } },
        body: { ...schuss, position: schuss.position ?? undefined },
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async schussLoeschen(rundeId: string, schussId: string): Promise<void> {
    await this.authenticated(() =>
      this.client.DELETE("/runden/{runde_id}/schuesse/{schuss_id}", {
        params: { path: { runde_id: rundeId, schuss_id: schussId } },
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

  async benutzerAbrufen(): Promise<AuthenticatedBenutzer> {
    return this.authenticated(() =>
      this.client.GET("/benutzer", {
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async benutzerLoeschen(passwort: string, bestätigt: boolean): Promise<void> {
    await this.authenticated(() =>
      this.client.POST("/benutzer/loeschung", {
        body: { passwort, bestätigt: bestätigt },
        headers: this.authorizationHeaders(),
      }),
    );
    this.setAccessToken(null);
  }
}
