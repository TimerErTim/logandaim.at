import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type {
  Benutzer,
  LogandaimApi,
  Schuss,
  SchussErstellungInput,
  Wertung,
} from "./logandaim-api";
import type { paths } from "./schema";
import { getAuthSnapshot, setAccessToken } from "./session-store";

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
export class HttpLogandaimApi implements LogandaimApi {
  private readonly client: JsonClient;
  private renewal: Promise<void> | null = null;
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.client = createClient<paths>({
      baseUrl: this.baseUrl,
      fetch: (request) => this.authorizedFetch(request),
    });
  }

  private async authorizedFetch(request: Request): Promise<Response> {
    const token = getAuthSnapshot().accessToken;
    const template = request.clone();
    const makeRequest = () => {
      const accessToken = getAuthSnapshot().accessToken;
      const headers = new Headers(template.headers);
      if (accessToken) headers.set("Authorization", `Bearer ${accessToken}`);
      else headers.delete("Authorization");
      return fetch(
        new Request(template.clone(), { headers, credentials: "include" }),
      );
    };
    const response = await makeRequest();
    const path = new URL(template.url).pathname;
    if (
      response.status !== 401 ||
      !token ||
      path.endsWith("/session/erneuern") ||
      path.endsWith("/session/abmelden") ||
      path.endsWith("/anmeldung") ||
      path.endsWith("/registrierung/start") ||
      path.endsWith("/registrierung/abschluss") ||
      path.endsWith("/passwort/vergessen") ||
      path.endsWith("/passwort/wiederherstellen")
    ) {
      return response;
    }
    if (getAuthSnapshot().accessToken !== token) return makeRequest();
    try {
      if (!this.renewal) {
        this.renewal = fetch(`${this.baseUrl}/session/erneuern`, {
          method: "POST",
          credentials: "include",
        })
          .then(async (renewed) => {
            if (!renewed.ok) throw new Error("Session renewal failed");
            const content = (await renewed.json()) as { accessToken: string };
            setAccessToken(content.accessToken);
          })
          .finally(() => {
            this.renewal = null;
          });
      }
      await this.renewal;
      return makeRequest();
    } catch {
      setAccessToken(null);
      return response;
    }
  }

  accessTokenSetzen(token: string | null): void {
    setAccessToken(token);
  }

  async health(): Promise<string> {
    return unwrap(await this.client.GET("/health"));
  }

  async schussRegistrieren(erstellung: SchussErstellungInput): Promise<Schuss> {
    return unwrap(await this.client.POST("/schuss", { body: erstellung }));
  }

  async wertungBerechnen(schussIds: string[]): Promise<Wertung> {
    return unwrap(await this.client.POST("/wertung", { body: { schussIds } }));
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
    this.accessTokenSetzen(result.accessToken);
  }

  async anmelden(eMailAdresse: string, passwort: string): Promise<void> {
    const result = await unwrap(
      await this.client.POST("/anmeldung", {
        body: { eMailAdresse, passwort },
      }),
    );
    this.accessTokenSetzen(result.accessToken);
  }

  async sessionErneuern(): Promise<void> {
    const result = await unwrap(await this.client.POST("/session/erneuern"));
    this.accessTokenSetzen(result.accessToken);
  }

  async sessionAbmelden(): Promise<void> {
    await unwrap(await this.client.POST("/session/abmelden"));
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
    this.accessTokenSetzen(null);
  }

  async benutzerAbrufen(): Promise<Benutzer> {
    return unwrap(await this.client.GET("/benutzer"));
  }
}
