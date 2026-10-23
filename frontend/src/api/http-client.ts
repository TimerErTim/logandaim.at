import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type {
  LogandaimApi,
  Schuss,
  SchussErstellungInput,
  Wertung,
} from "./logandaim-api";
import type { paths } from "./schema";

type JsonClient = ReturnType<typeof createClient<paths>>;

async function unwrap<T>(result: { data?: T; error?: unknown }): Promise<T> {
  if (result.error !== undefined) {
    throw ApiError.fromUnknown(result.error);
  }
  return result.data as T;
}

/**
 * HTTP implementation of [`LogandaimApi`], generated from the backend OpenAPI spec.
 * Keep this class boring: transport only, no business rules.
 */
export class HttpLogandaimApi implements LogandaimApi {
  private readonly client: JsonClient;

  constructor(baseUrl: string) {
    this.client = createClient<paths>({ baseUrl: baseUrl.replace(/\/$/, "") });
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
}
