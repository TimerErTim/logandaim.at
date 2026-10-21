---
id: ADR-0001
title: Keep Rust backend and static-export Next.js; remove Tauri for now
status: Accepted
date: 2026-10-21
deciders: TimerErTim
---

# ADR-0001: Baseline architecture

## Context

logandaim.at starts from the YAMS proof of concept: a Rust workspace (hexagonal core, API layer,
SQLite adapter, Poem HTTP server, scheduler) and a Next.js App Router frontend built as static
export, embedded in a Tauri desktop shell.

The PRD sketches a Node, Go, or Python backend with REST or tRPC and a mobile PWA. Those are
suggestions, not requirements. The product needs are:

- Mobile web app for archers in the field. Offline capture is the milestone after MVP.
- Accounts and a central database (cross-user benchmarks later).
- Strict, testable domain rules (scoring, distances).

## Decision

1. **Keep the Rust backend.** Hexagonal core, API layer, SQLite (libsql) adapter, Poem OpenAPI server.
   Domain rules live in the core and are covered by domain unit tests.
2. **Keep Next.js as static export.** The frontend is served as static files and talks to the HTTP
   API. Static export keeps a later PWA (service worker, IndexedDB) and a later native shell simple.
3. **Remove Tauri now.** Delete `frontend/src-tauri`, the Tauri adapter in the frontend API layer,
   Tauri tasks, and Tauri CI dependencies.
4. **Stay shell-agnostic.** A desktop or mobile shell (Tauri or other) may return. Therefore:
   - The core and API crates must not depend on HTTP-server or shell specifics.
   - The frontend keeps one transport interface (`*Api`) with adapters behind it, even with only
     the HTTP adapter left.
   - The frontend keeps static export: no server components that need a Node runtime, no
     dynamic route segments.
5. **Keep REST + OpenAPI.** Generated TypeScript types (`mise run sync:openapi`) give end-to-end typing.
   No tRPC.
6. **Hosting.** The static Next.js export is hosted on GitHub Pages. The Rust backend (HTTP server
   and database) runs on a private VPS. The frontend reaches the backend over HTTPS on a separate origin.

## Consequences

- Rebrand milestone M0 includes Tauri removal and the YAMS domain removal.
- Offline (M2) needs an ADR for client-side storage and sync; static export and the transport
  interface keep that change local to the frontend.
- Re-adding a native shell later means adding a transport adapter and a shell crate, not
  changing core or API.
- GitHub Pages serves static files only: the frontend must stay a pure static export, and the API
  base URL is set at build time.
- Frontend and backend are on different origins, so the backend needs CORS for the Pages origin and
  TLS on the VPS. Authentication must work cross-origin (decided in the accounts spec).
- Deployment needs a Pages workflow for the frontend and a separate deployment path for the VPS.
