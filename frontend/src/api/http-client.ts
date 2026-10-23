import { ApiError } from "./errors";
import type { StructuredError } from "./types";
import type { YamsApi } from "./yams-api";

export class HttpYamsApi implements YamsApi {
  constructor(private readonly baseUrl: string) {}

  async health(): Promise<string> {
    return this.request<string>("/health");
  }

  private async request<T>(path: string, init?: RequestInit): Promise<T> {
    const response = await fetch(`${this.baseUrl}${path}`, {
      headers: { Accept: "application/json" },
      ...init,
    });

    const text = await response.text();

    if (!response.ok) {
      let structured: StructuredError | undefined;
      try {
        structured = JSON.parse(text) as StructuredError;
      } catch {
        structured = undefined;
      }
      throw new ApiError(
        structured?.message ?? `Request failed with ${response.status}`,
        structured,
      );
    }

    if (!text) {
      return undefined as T;
    }

    try {
      return JSON.parse(text) as T;
    } catch {
      return text as T;
    }
  }
}
