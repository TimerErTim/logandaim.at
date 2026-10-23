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

The browser reads the API URL from [`src/standalone-config.json`](src/standalone-config.json). Set `NEXT_PUBLIC_LOGANDAIM_API_URL` to override it and `NEXT_PUBLIC_LOGANDAIM_DEV=true` to enable TanStack Query devtools outside development.

Both values are the complete base URL the client sends requests to, including any path prefix such as `/api` (the server mounts its routes there). Only a trailing slash is normalized away; the frontend never invents a prefix.
