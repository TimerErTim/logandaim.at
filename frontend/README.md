# Frontend

The user interface is a TypeScript React app built with [Next.js](https://nextjs.org/). It runs as a web application and talks to `logandaim-server` over HTTP.

## Development

Start the backend and frontend in separate terminals:

```bash
mise run dev:server
mise run dev:frontend
```

The server task starts Mailpit. Open its web UI at `http://127.0.0.1:8025` to read registration and password-recovery mail. For local authentication, keep the browser app and API on the same hostname (`localhost`): the refresh token is an API-host-only cookie, while the access token stays in frontend memory. The defaults use `http://localhost:3001` for the app and `http://localhost:3000/api` for the API.

## Building

```bash
npm run build
```

Dev-only routes are gated by the Next phase in `next.config.ts`: `.dev.tsx` / `.dev.ts` page
extensions are added only for `next dev`, so a route written as `page.dev.tsx` cannot reach the
static export.

## Design system

**Read [`.cursor/rules/design-system.mdc`](../.cursor/rules/design-system.mdc) before changing UI.**
It is the operative rulebook — tokens, components, accessibility, screen composition. Rationale and
history are in [`specs/product/visual-identity.md`](../specs/product/visual-identity.md); do not
restate either document here.

Three things this README adds:

- **Values**: every colour, radius, shadow, and font lives in `src/app/globals.css`. Raw palette
  classes and hex values in a component are rejected by `mise run check:design`.
- **Primitives**: `src/components/ui/`, variants declared via `class-variance-authority`. Add
  variants there, not at call sites.
- **Gallery**: `mise run dev:frontend`, then open `/dev`. Light and dark side by side, per-token
  contrast checks, a touch-target overlay, and one composed page. It needs no backend.

## Metadata and icons

The favicon set and the web manifest are the single copy in `public/`
(`favicon.ico`, `favicon-16x16.png`, `favicon-32x32.png`, `apple-touch-icon.png`,
`android-chrome-*.png`, `site.webmanifest`). They live there rather than in `src/app/` so the
brand assets are one set, not a second file-convention copy.

`src/app/layout.tsx` links them explicitly and prefixes every URL with
`NEXT_PUBLIC_LOGANDAIM_BASE_PATH`. Next applies the base path to file-convention metadata it
generates itself, but not to hand-written `metadata.icons` / `metadata.manifest` URLs, so the
prefix has to be added by hand. Manifest icon and `start_url` entries are relative on purpose:
they resolve against the manifest's own URL and follow the deployment prefix without help.
The manifest's `background_color` / `theme_color` mirror `--bg-canvas` and `--color-primary`;
JSON cannot hold a `var()`, so those two are the only literal palette values outside
`globals.css`. Change them only together with the tokens.

## Configuration

There is no runtime configuration file: every value is a `NEXT_PUBLIC_*` variable read at build time.

| Variable | Purpose |
| --- | --- |
| `NEXT_PUBLIC_LOGANDAIM_API_URL` | Base URL the browser sends requests to. Required; the Pages deploy fails without it, `mise run dev:frontend` sets `http://127.0.0.1:3000/api`. |
| `NEXT_PUBLIC_LOGANDAIM_DEV` | `true` enables TanStack Query devtools outside development. |
| `NEXT_PUBLIC_LOGANDAIM_BASE_PATH` | Base path of the deployment (`/logandaim.at` for the project site, empty at a custom domain). |

Because these are baked into the export, changing one needs a rebuild; `mise run --force build:frontend` skips the task cache.

Both values are the complete base URL the client sends requests to, including any path prefix such as `/api` (the server mounts its routes there). Only a trailing slash is normalized away; the frontend never invents a prefix.
