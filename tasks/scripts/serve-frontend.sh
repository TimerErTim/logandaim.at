#!/usr/bin/env bash
# Serve the built static frontend export.
#
# Standalone: `verify:frontend` runs it as a daemon, and a deployment can run the
# same file (for example on the VPS) without going through mise.
#
#   LOGANDAIM_FRONTEND_PORT  listener port (default 4173)
#   PORT                     overrides it; the daemon supervisor sets this
#
# Bash rather than node because `exec` is load-bearing: the supervisor must be able
# to stop the process by signalling the pid it started.
set -eu

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PORT="${PORT:-${LOGANDAIM_FRONTEND_PORT:-4173}}"

# `npm:serve` is the task tool; `serve` must be on PATH.
exec serve "$ROOT/frontend/out" \
  --listen "tcp://127.0.0.1:$PORT" \
  --no-clipboard \
  --no-port-switching
