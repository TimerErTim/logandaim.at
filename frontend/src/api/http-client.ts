import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type {
  AuthPhase,
  AuthSession,
  Benutzer,
  LogandaimApi,
  Schuss,
  SchussErstellungInput,
  Wertung,
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

  async schussRegistrieren(erstellung: SchussErstellungInput): Promise<Schuss> {
    return this.authenticated(() =>
      this.client.POST("/schuss", {
        body: erstellung,
        headers: this.authorizationHeaders(),
      }),
    );
  }

  async wertungBerechnen(schussIds: string[]): Promise<Wertung> {
    return this.authenticated(() =>
      this.client.POST("/wertung", {
        body: { schussIds },
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
}
