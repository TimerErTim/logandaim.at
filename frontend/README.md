# Frontend

The user interface is a TypeScript React app built with [Next.js](https://nextjs.org/). It runs as a web application and talks to `logandaim-server` over HTTP.

## Development

Run the frontend and backend together:

```bash
mise run dev:frontend+server
```

## Building

```bash
npm run build
```

## Configuration

There is no runtime configuration file: every value is a `NEXT_PUBLIC_*` variable read at build time.

| Variable | Purpose |
| --- | --- |
| `NEXT_PUBLIC_LOGANDAIM_API_URL` | Base URL the browser sends requests to. Required; the Pages deploy fails without it, `mise run dev:frontend` sets `http://127.0.0.1:3000/api`. |
| `NEXT_PUBLIC_LOGANDAIM_DEV` | `true` enables TanStack Query devtools outside development. |
| `NEXT_PUBLIC_LOGANDAIM_BASE_PATH` | Base path of the deployment (`/logandaim.at` for the project site, empty at a custom domain). |

Because these are baked into the export, changing one needs a rebuild; `mise run --force build:frontend` skips the task cache.

Both values are the complete base URL the client sends requests to, including any path prefix such as `/api` (the server mounts its routes there). Only a trailing slash is normalized away; the frontend never invents a prefix.
