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

## Appendix: spelling German identifiers

Domain identifiers are German, and they are spelled with their real letters:
`Empfänger`, `Bestätigung`, `öffentlicher_schlüssel`, `MIN_PASSWORT_LÄNGE`. Rust accepts
non-ASCII identifiers, and so do JSON keys, SQLite column names, and TypeScript names, so
transliterating them into ASCII — shrinking the receiver to four letters plus an escape — is
simply wrong here.

Three things stay ASCII, because something outside the compiler consumes them:

1. **Module names.** `mod x;` resolves to a file path, and Rust refuses a non-ASCII one. So
   `mod bestaetigung;` in `domain/konto/mod.rs` is correct, and the file keeps that name.
2. **Environment variables.** An operator types them, and a shell cannot be relied on to carry
   an umlaut through every hop. `LOGANDAIM_SMTP_VERSCHLUESSELUNG` stays as it is.
3. **Words borrowed from English** (`Request`, `Issuer`, `Value`) are not transliterations, and
   the check ignores them.

The rule is enforced by review, not by a task: no reliable scanner distinguishes a German word
from a transliteration, so an automated check would pass the mistakes it exists to catch. The
three ASCII exceptions above are named in `.cursor/AGENTS.md`, where every contributor and agent
reads them.
