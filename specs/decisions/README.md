# Architecture decision records

One file per decision: `NNNN-<kebab-title>.md`, numbered in order. Status is `Proposed`,
`Accepted`, or `Superseded by NNNN`. Agents may write `Proposed` ADRs; a human accepts them.

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-baseline-architecture.md) | Keep Rust backend and static-export Next.js; remove Tauri for now | Accepted |
| [0002](0002-session-cookie-auth.md) | Session cookies for auth; explicit-origin CORS | Superseded by [0004](0004-jwt-refresh-auth.md) |
| [0003](0003-design-system-and-dev-gallery.md) | Semantic design tokens, shadcn-style primitives, and a dev-only component gallery | Proposed |
| [0004](0004-jwt-refresh-auth.md) | JWT access tokens with single-use rotating refresh tokens; server-checked revocation | Proposed |
