// `gh` invocation shared by the release tasks.
//
// `gh` is a task tool, so mise puts it on PATH; a missing binary is a broken
// setup, not a condition to tolerate. `LOGANDAIM_GH` only exists so the tasks can
// be exercised against a stub.

import { execFileSync } from "node:child_process";

export const GH = process.env.LOGANDAIM_GH || "gh";

/**
 * Run `gh` and return stdout (trailing whitespace trimmed). Throws the underlying
 * error, which carries `status` (the exit code) and `stderr`.
 */
export const gh = (...args) =>
  execFileSync(GH, args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trimEnd();

/** Run `gh` and report the outcome instead of throwing. */
export const tryGh = (...args) => {
  try {
    return { ok: true, stdout: gh(...args), status: 0, stderr: "" };
  } catch (error) {
    return {
      ok: false,
      stdout: (error.stdout ?? "").trimEnd(),
      status: error.status ?? null,
      stderr: (error.stderr ?? "").trimEnd(),
    };
  }
};

/**
 * True when GitHub is unreachable because there is no usable login: either no
 * credentials at all, or a token GitHub rejects.
 *
 * Probes with an authenticated API call rather than `gh auth status`: that command
 * exits 1 both for a missing login and for an invalid token, which is also the exit
 * code of a network failure, so it cannot separate "not logged in" from "broken".
 * `gh api` exits 4 when no credentials exist and reports 401 for a bad token.
 */
export const isUnauthenticated = () => {
  const result = tryGh("api", "user");
  if (result.ok) return false;
  if (result.status === 4) return true;
  return /\b401\b|Bad credentials|Requires authentication/i.test(
    `${result.stdout}\n${result.stderr}`,
  );
};
