# Frontend

The user interface is a TypeScript React app built with [Next.js](https://nextjs.org/). It runs as a web application and talks to `yams-server` over HTTP.

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

The browser reads the API URL from [`src/standalone-config.json`](src/standalone-config.json). Set `NEXT_PUBLIC_YAMS_API_URL` to override it and `NEXT_PUBLIC_YAMS_DEV=true` to enable TanStack Query devtools outside development.
