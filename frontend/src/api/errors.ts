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
