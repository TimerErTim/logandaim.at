import createClient from "openapi-fetch";

import { ApiError } from "./errors";
import type { paths } from "./schema";
import type { YamsApi } from "./yams-api";

type JsonClient = ReturnType<typeof createClient<paths>>;

async function unwrap<T>(result: { data?: T; error?: unknown }): Promise<T> {
  if (result.error !== undefined) {
    throw ApiError.fromUnknown(result.error);
  }
  return result.data as T;
}

export class HttpYamsApi implements YamsApi {
  private readonly client: JsonClient;
  private readonly baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, "");
    this.client = createClient<paths>({ baseUrl: this.baseUrl });
  }

  async health(): Promise<string> {
    return unwrap(await this.client.GET("/health"));
  }
}
