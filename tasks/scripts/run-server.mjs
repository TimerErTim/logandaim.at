#!/usr/bin/env node
// Run the backend server with a throwaway database.
//
// Standalone: `verify:server` runs it as a daemon, and a deployment can run the
// same file without going through mise.
//
//   LOGANDAIM_BIND_PORT     listener port (default 3311, matching mise.toml API_PORT)
//   PORT                    overrides it; the daemon supervisor sets this
//   LOGANDAIM_SMOKE_DATA_DIR  reuse a data directory instead of a temporary one
//   usage_release           "true" builds and runs the release profile

import { spawn } from "node:child_process";
import { mkdirSync, mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const release = process.env.usage_release === "true";
const port = process.env.PORT || process.env.LOGANDAIM_BIND_PORT || "3311";

// The server rejects an empty `--log-dir`, so the variable must be absent rather than
// blank: an inherited value from a dev shell would otherwise leak into the smoke run.
const serverEnv = { ...process.env };
delete serverEnv.LOGANDAIM_LOG_DIR;
delete serverEnv.usage_release;

const dataDir = process.env.LOGANDAIM_SMOKE_DATA_DIR || mkdtempSync(join(tmpdir(), "logandaim-smoke-"));
mkdirSync(join(dataDir, "storage"), { recursive: true });

// Build here rather than as a task dependency: a usage flag is not forwarded to a
// dependency, so the dependency would build the debug profile while this task ran
// the release binary (missing, or stale from an earlier run).
const build = spawn("cargo", ["build", "-p", "logandaim-server", ...(release ? ["--release"] : [])], {
  stdio: "inherit",
});
build.on("exit", (code) => {
  if (code !== 0) process.exit(code ?? 1);

  const binary = `target/${release ? "release" : "debug"}/logandaim-server`;
  const server = spawn(binary, [], {
    stdio: "inherit",
    env: {
      ...serverEnv,
      LOGANDAIM_BIND_ADDRESS: "127.0.0.1",
      LOGANDAIM_BIND_PORT: port,
      LOGANDAIM_DATABASE_URL: join(dataDir, "smoke.db"),
      LOGANDAIM_OBJECT_STORE_DIR: join(dataDir, "storage"),
    },
  });

  // Forward the stop signals so the supervisor shuts the server down cleanly.
  for (const signal of ["SIGINT", "SIGTERM"]) {
    process.on(signal, () => server.kill(signal));
  }
  server.on("exit", (serverCode) => process.exit(serverCode ?? 0));
});
