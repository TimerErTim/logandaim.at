# Graph Report - logandaim.at  (2026-10-23)

## Corpus Check
- 122 files · ~36,333 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1060 nodes · 1736 edges · 78 communities (56 shown, 12 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 32 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `047a9c37`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UnitOfWork
- arc_up
- .finish
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- card.tsx
- button.tsx
- wal_connection_race.rs
- .partial_cmp
- compilerOptions
- schema.ts
- Clock
- Make invalid state unrepresentable
- E
- Datenschutz — Data protection, privacy, and legal obligations
- wertung.rs
- <Title>
- Betrieb — Operations and launch readiness
- Wertung — Classic 3-arrow scoring
- Konto — Accounts and login
- SQLiteUnitOfWork
- .contextualize_with
- App
- logandaim.at — Product Charter
- StructuredError
- repos.rs
- charter.md
- search-input.tsx
- Release Workflow
- Product Requirements Document (PRD) & Konzeptbericht
- yams-core
- cn
- diffx-finish-review
- Next.js
- postcss.config.mjs
- Contributing and project management
- ADR-0001: Baseline architecture
- provider.tsx
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- Specifications
- SQLiteInstance
- base_app_builder
- config.rs
- http_status.rs
- navigation.ts
- pull_request_template.md
- YAMS Frontend Migration Specification
- migration_error_to_persistence_error
- validation_error.rs
- Versioned<T>
- FakeUnitOfWorkProvider
- base_app_builder
- ADR-0002: Session cookie auth across frontend and API origins
- m1-overseer.md
- Modes
- package.json
- m1-reviewer.md
- [Unreleased]
- layout.tsx
- InstrumentedUnitOfWork
- .begin
- mise
- UnitOfWorkImpl

## God Nodes (most connected - your core abstractions)
1. `E` - 36 edges
2. `cn()` - 35 edges
3. `UnitOfWorkImpl` - 23 edges
4. `arc_up()` - 20 edges
5. `Versioned<T>` - 19 edges
6. `Datenschutz — Data protection, privacy, and legal obligations` - 19 edges
7. `UpMigration` - 16 edges
8. `apply_up_migrations()` - 16 edges
9. `compilerOptions` - 16 edges
10. `apply_down_migrations()` - 15 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/yams-api/src/bin/export_spec.rs → crates/yams-api/src/spec.rs
- `Next.js` --same_as--> `Next.js`  [INFERRED]
  frontend/README.md → .cursor/AGENTS.md
- `yams-server` --depends_on--> `yams-api`  [EXTRACTED]
  backend/server/Cargo.toml → crates/yams-api/Cargo.toml
- `yams-server` --depends_on--> `yams-core`  [EXTRACTED]
  backend/server/Cargo.toml → crates/yams-core/Cargo.toml

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (78 total, 12 thin omitted)

### Community 0 - "UnitOfWork"
Cohesion: 0.29
Nodes (5): LockedUnitOfWorkImpl, Box, RepositoryResult, UnitOfWork, Drop

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "card.tsx"
Cohesion: 0.39
Nodes (6): HomePage(), Card(), CardContent(), CardDescription(), CardHeader(), CardTitle()

### Community 7 - "button.tsx"
Cohesion: 0.29
Nodes (6): Button(), ButtonProps, ButtonSize, ButtonVariant, sizeClasses, variantClasses

### Community 8 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "schema.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 14 - "Clock"
Cohesion: 0.06
Nodes (27): DateTime, NaiveDate, Utc, SystemClock, ExecutionContext, ExecutionContext<'a>, ExecutionSource, Arc (+19 more)

### Community 16 - "Make invalid state unrepresentable"
Cohesion: 0.12
Nodes (15): Closed enums, Creation input and construction, Derived values, Domain model, Encapsulation, Errors, Identity newtypes, Layout (+7 more)

### Community 17 - "E"
Cohesion: 0.09
Nodes (39): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+31 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (17): Add, ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), Punkte, I, IntoIterator, Item (+9 more)

### Community 23 - "<Title>"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+5 more)

### Community 25 - "Betrieb — Operations and launch readiness"
Cohesion: 0.14
Nodes (14): Abuse, Backups, Betrieb — Operations and launch readiness, Decision log, Domain terms and business rules, Outcome, Problem, Revision history (+6 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 30 - "Konto — Accounts and login"
Cohesion: 0.08
Nodes (25): Acceptance criteria, Account security, Administration, Auth audit, Decision log, Deferred capabilities, Domain terms and business rules, Email change (+17 more)

### Community 40 - "SQLiteUnitOfWork"
Cohesion: 0.12
Nodes (19): Box, Error, Future, Option, Output, Pin, Result, Send (+11 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 52 - "App"
Cohesion: 0.05
Nodes (47): BackendServerError, catch_panic(), main(), Report, Result, init_tracing(), Option, Path (+39 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 59 - "repos.rs"
Cohesion: 0.40
Nodes (3): RepositoryError, Option, Debug

### Community 65 - "search-input.tsx"
Cohesion: 0.50
Nodes (3): SearchInput(), SearchInputProps, lucide-react

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "yams-core"
Cohesion: 0.47
Nodes (6): molting, yams-api, yams-core, yams-fakes, yams-server, yams-sqlite

### Community 70 - "cn"
Cohesion: 0.11
Nodes (25): Alert(), AlertProps, AlertVariant, variantClasses, Badge(), BadgeProps, BadgeVariant, variantClasses (+17 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 82 - "Contributing and project management"
Cohesion: 0.22
Nodes (9): Agents, Commits, Contributing and project management, Delivery gate, GitHub Project, Labels and priority, Ready gate, Records (+1 more)

### Community 83 - "ADR-0001: Baseline architecture"
Cohesion: 0.50
Nodes (4): ADR-0001: Baseline architecture, Consequences, Context, Decision

### Community 84 - "provider.tsx"
Cohesion: 0.10
Nodes (22): ApiError, useHealthQuery(), HttpYamsApi, createYamsApi(), getYamsApi(), normalizeApiBaseUrl(), resetYamsApiCache(), useYamsApi() (+14 more)

### Community 99 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AsRef, AtomicBool, Connection, InstanceType, Arc, Deref, Mutex, Path (+9 more)

### Community 101 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 102 - "config.rs"
Cohesion: 0.11
Nodes (31): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+23 more)

### Community 107 - "http_status.rs"
Cohesion: 0.19
Nodes (14): HttpStatusMapping, mapped(), nested_http_status(), RepositoryError, C, Option, Report, StatusCode (+6 more)

### Community 114 - "navigation.ts"
Cohesion: 0.18
Nodes (17): AppShell(), Sidebar(), SidebarLink(), SidebarProps, BreadcrumbSegment, breadcrumbsForPathname(), homeNavItem, isExactPath() (+9 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 129 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 135 - "Versioned<T>"
Cohesion: 0.14
Nodes (10): Clone, Deref, Self, T, Target, Versioned, Versioned<T>, DerefMut (+2 more)

### Community 144 - "FakeUnitOfWorkProvider"
Cohesion: 0.25
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 145 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.25
Nodes (8): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Consequences, Context, Data model (sketch), Decision

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "package.json"
Cohesion: 0.05
Nodes (42): nextConfig, dependencies, lucide-react, next, react, react-dom, @tanstack/react-query, @tanstack/react-query-devtools (+34 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "[Unreleased]"
Cohesion: 0.50
Nodes (4): Added, Changed, Changelog, [Unreleased]

### Community 164 - "layout.tsx"
Cohesion: 0.19
Nodes (9): geistMono, geistSans, metadata, AppChrome(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel() (+1 more)

### Community 170 - "InstrumentedUnitOfWork"
Cohesion: 0.46
Nodes (4): InstrumentedUnitOfWork, Box, RepositoryResult, Self

### Community 172 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 184 - "UnitOfWorkImpl"
Cohesion: 0.19
Nodes (9): Arc<T>, Send, Sync, UnitOfWork<'a>, UnitOfWorkImpl, UnitOfWorkProvider, RepositoryError, ResultReport (+1 more)

## Knowledge Gaps
- **312 isolated node(s):** `molting`, `ValidationError`, `yams-fakes`, `$schema`, `enabled` (+307 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 458 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `E` connect `E` to `StructuredError`, `App`, `.contextualize_with`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `UnitOfWorkImpl`, `Clock`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Why does `String` connect `config.rs` to `FakeUnitOfWorkProvider`, `arc_up`, `StructuredError`, `App`?**
  _High betweenness centrality (0.073) - this node is a cross-community bridge._
- **Are the 14 inferred relationships involving `arc_up()` (e.g. with `add_migration_out_of_order_results_in_sorted_apply_order()` and `apply_from_mid_to_latest_applies_only_pending()`) actually correct?**
  _`arc_up()` has 14 INFERRED edges - model-reasoned connections that need verification._
- **What connects `molting`, `ValidationError`, `yams-fakes` to the rest of the system?**
  _312 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._
