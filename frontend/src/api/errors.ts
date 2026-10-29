import type { StructuredError } from "./types";

export class ApiError extends Error {
  readonly structured?: StructuredError;
  readonly status?: number;

  constructor(message: string, structured?: StructuredError, status?: number) {
    super(message);
    this.name = "ApiError";
    this.structured = structured;
    this.status = status;
  }

  static fromUnknown(error: unknown, status?: number): ApiError {
    if (error instanceof ApiError) {
      return error;
    }

    if (typeof error === "object" && error !== null && "message" in error) {
      const structured = error as StructuredError;
      return new ApiError(String(structured.message), structured, status);
    }

    return new ApiError(String(error), undefined, status);
  }
}

export function apiErrorMessage(
  error: unknown,
  messages: {
    unauthorized?: string;
    badRequest?: string;
    tooManyRequests?: string;
    fallback: string;
  },
): string | null {
  if (!error) return null;

  if (error instanceof ApiError) {
    const message = error.structured?.message?.trim();
    const genericMessage =
      message?.toLocaleLowerCase("de-DE") === "die anfrage ist ungültig" ||
      message?.toLocaleLowerCase("de-DE") === "validation error";
    if (
      message &&
      !genericMessage &&
      error.status !== undefined &&
      error.status < 500
    ) {
      return message.charAt(0).toLocaleUpperCase("de-DE") + message.slice(1);
    }
    if (error.status === 401 && messages.unauthorized)
      return messages.unauthorized;
    if (error.status === 400 && messages.badRequest) return messages.badRequest;
    if (error.status === 429 && messages.tooManyRequests) {
      return messages.tooManyRequests;
    }
  }

  return messages.fallback;
}
