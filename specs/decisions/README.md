# Architecture decision records

One file per decision: `NNNN-<kebab-title>.md`, numbered in order. Status is `Proposed`,
`Accepted`, or `Superseded by NNNN`. Agents may write `Proposed` ADRs; a human accepts them.

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-baseline-architecture.md) | Keep Rust backend and static-export Next.js; remove Tauri for now | Accepted |
| [0002](0002-session-cookie-auth.md) | Session cookies for auth; explicit-origin CORS | Proposed |
