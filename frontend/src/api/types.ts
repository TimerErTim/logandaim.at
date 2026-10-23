/**
 * Shape of `StructuredError` from the backend error mapping.
 *
 * Declared here rather than taken from `./schema` because no endpoint currently
 * advertises the error schema in OpenAPI; when one does, switch to
 * `components["schemas"]["StructuredError"]`.
 */
export type StructuredError = {
  message: string;
  attachments: string[];
  sources: StructuredError[];
};
