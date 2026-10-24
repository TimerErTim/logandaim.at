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

Dev-only routes are gated by the Next phase in `next.config.ts`: `.dev.tsx` / `.dev.ts` page
extensions are added only for `next dev`, so a route written as `page.dev.tsx` cannot reach the
static export. `build:frontend` verifies that afterwards and fails if `out/dev` exists.

## Design system

The visual identity is described in [`.cursor/rules/design-system.mdc`](../.cursor/rules/design-system.mdc)
and [`specs/product/visual-identity.md`](../specs/product/visual-identity.md). Read the rule before
changing UI.

- **Tokens**: every colour, radius, and font value lives in `src/app/globals.css`. Components use
  the semantic utilities it maps (`bg-surface`, `text-muted-foreground`, `border-border`,
  `bg-primary`); raw palette classes and hex values are rejected by `mise run check:design`.
- **Primitives**: `src/components/ui/` holds shadcn-style components with variants declared via
  `class-variance-authority`. Add variants there, not in call sites.
- **Gallery**: run `mise run dev:frontend` and open `/dev`. Every component, variant, and state
  renders in a light and a dark frame, with per-token contrast checks and a touch-target overlay.
  The page needs no backend.
- **Changing the look**: edit a token in `globals.css`, look at `/dev`, and on acceptance update the
  design rule and the spec decision log in the same commit.

## Configuration

There is no runtime configuration file: every value is a `NEXT_PUBLIC_*` variable read at build time.

| Variable | Purpose |
| --- | --- |
| `NEXT_PUBLIC_LOGANDAIM_API_URL` | Base URL the browser sends requests to. Required; the Pages deploy fails without it, `mise run dev:frontend` sets `http://127.0.0.1:3000/api`. |
| `NEXT_PUBLIC_LOGANDAIM_DEV` | `true` enables TanStack Query devtools outside development. |
| `NEXT_PUBLIC_LOGANDAIM_BASE_PATH` | Base path of the deployment (`/logandaim.at` for the project site, empty at a custom domain). |

Because these are baked into the export, changing one needs a rebuild; `mise run --force build:frontend` skips the task cache.

Both values are the complete base URL the client sends requests to, including any path prefix such as `/api` (the server mounts its routes there). Only a trailing slash is normalized away; the frontend never invents a prefix.
