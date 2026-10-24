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
      stderr: error.stderr ?? "",
    };
  }
};

/**
 * `gh` exits 4 when no account or token is available. That is the only failure a
 * developer machine may skip; anything else (network, permissions, a broken
 * install) is a real error.
 */
export const isUnauthenticated = (result) => result.status === 4;
