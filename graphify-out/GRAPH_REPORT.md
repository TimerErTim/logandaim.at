# Graph Report - logandaim.at  (2026-10-23)

## Corpus Check
- 156 files · ~46,172 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1553 nodes · 2752 edges · 96 communities (75 shown, 14 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 47 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a5845c12`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FakePdfRenderer
- arc_up
- devDependencies
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- button.tsx
- SchussErstellung
- http_status.rs
- search-input.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- in_memory_object_store.rs
- FakeDatastore
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- FakeObjectStore
- Clock
- wertung.rs
- ObjectStore
- <Title>
- Make invalid state unrepresentable
- Betrieb — Operations and launch readiness
- provider.tsx
- ExecutionContext
- Wertung — Classic 3-arrow scoring
- UseCase
- Konto — Accounts and login
- .finish
- .begin
- RepoStorage
- base_app_builder
- UnitOfWorkImpl
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- FixedClock
- SQLiteUnitOfWork
- .contextualize_with
- Vertical slices
- domain/schuss.rs
- common.rs
- instrumented.rs
- LogandaimApiTestClient
- InstrumentedUnitOfWork
- .rendern
- ApiError
- schema.ts
- logandaim.at — Product Charter
- StructuredError
- charter.md
- App
- .partial_cmp
- Release Workflow
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- cn
- diffx-finish-review
- Next.js
- postcss.config.mjs
- Contributing and project management
- ADR-0001: Baseline architecture
- http-client.ts
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- Specifications
- SQLiteInstance
- String
- card.tsx
- navigation.ts
- UnitOfWork
- pull_request_template.md
- YAMS Frontend Migration Specification
- dependencies
- scripts
- dates.ts
- preis.rs
- Versioned<T>
- ADR-0002: Session cookie auth across frontend and API origins
- m1-overseer.md
- Modes
- package.json
- m1-reviewer.md
- [Unreleased]
- mise

## God Nodes (most connected - your core abstractions)
1. `cn()` - 37 edges
2. `E` - 36 edges
3. `UnitOfWorkImpl` - 27 edges
4. `arc_up()` - 20 edges
5. `App` - 19 edges
6. `Versioned<T>` - 19 edges
7. `Datenschutz — Data protection, privacy, and legal obligations` - 19 edges
8. `UpMigration` - 18 edges
9. `ObjectStore` - 17 edges
10. `SchussId` - 17 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/logandaim-api/src/spec.rs
- `main()` --calls--> `openapi_service()`  [INFERRED]
  crates/logandaim-api/src/bin/export_spec.rs → crates/logandaim-api/src/spec.rs
- `Next.js` --same_as--> `Next.js`  [INFERRED]
  frontend/README.md → .cursor/AGENTS.md
- `logandaim-server` --depends_on--> `logandaim-api`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-api/Cargo.toml
- `logandaim-server` --depends_on--> `logandaim-core`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-core/Cargo.toml

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (96 total, 14 thin omitted)

### Community 0 - "FakePdfRenderer"
Cohesion: 0.15
Nodes (11): Vec, PdfDokument, PdfRenderError, ResultReport, Vec, FakePdfRenderer, Arc, Mutex (+3 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "button.tsx"
Cohesion: 0.33
Nodes (5): ButtonProps, ButtonSize, ButtonVariant, sizeClasses, variantClasses

### Community 7 - "SchussErstellung"
Cohesion: 0.10
Nodes (23): ResultReport, Schuss, Wertung, From, Self, Uuid, Vec, Wertungszone (+15 more)

### Community 8 - "http_status.rs"
Cohesion: 0.14
Nodes (16): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+8 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 14 - "in_memory_object_store.rs"
Cohesion: 0.15
Nodes (18): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+10 more)

### Community 15 - "FakeDatastore"
Cohesion: 0.11
Nodes (22): FakeDatastore, FakeSchussRepository, Arc, Clone, Default, FxHashMap, Mutex, RepositoryResult (+14 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.07
Nodes (48): E, libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError, Migration, Error, Option (+40 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 20 - "Clock"
Cohesion: 0.14
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (21): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), pfeil(), Punkte, Add (+13 more)

### Community 22 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 23 - "<Title>"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+5 more)

### Community 24 - "Make invalid state unrepresentable"
Cohesion: 0.12
Nodes (15): Closed enums, Creation input and construction, Derived values, Domain model, Encapsulation, Errors, Identity newtypes, Layout (+7 more)

### Community 25 - "Betrieb — Operations and launch readiness"
Cohesion: 0.14
Nodes (14): Abuse, Backups, Betrieb — Operations and launch readiness, Decision log, Domain terms and business rules, Outcome, Problem, Revision history (+6 more)

### Community 26 - "provider.tsx"
Cohesion: 0.12
Nodes (16): createLogandaimApi(), LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), geistMono, geistSans, metadata, Providers() (+8 more)

### Community 27 - "ExecutionContext"
Cohesion: 0.18
Nodes (9): ExecutionContext, ExecutionContext<'a>, ExecutionSource, Arc, RepositoryResult, Self, PdfRenderer, Send (+1 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.17
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - "UseCase"
Cohesion: 0.40
Nodes (4): Context, Output, ResultReport, UseCase

### Community 30 - "Konto — Accounts and login"
Cohesion: 0.08
Nodes (25): Acceptance criteria, Account security, Administration, Auth audit, Decision log, Deferred capabilities, Domain terms and business rules, Email change (+17 more)

### Community 31 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 32 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 33 - "RepoStorage"
Cohesion: 0.38
Nodes (5): RepoStorage, Box, Pin, Self, InstrumentedSchussRepository

### Community 34 - "base_app_builder"
Cohesion: 0.31
Nodes (8): base_app_builder(), AppBuilder, SetUowProvider, schuss_registrieren_persists_and_can_be_loaded(), wertung_leere_liste_ist_fehler(), wertung_summiert_nach_pfeil_innerhalb_eines_ziels(), wertung_unbekannte_id_ist_fehler(), ziel_id()

### Community 35 - "UnitOfWorkImpl"
Cohesion: 0.18
Nodes (9): Arc<T>, LockedUnitOfWorkImpl, Send, Sync, UnitOfWorkImpl, UnitOfWorkProvider, RepositoryError, ResultReport (+1 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - "SQLiteUnitOfWork"
Cohesion: 0.12
Nodes (19): Box, Error, Future, Option, Output, Pin, Result, Send (+11 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.05
Nodes (52): RepositoryResult, Schuss, Send, Sync, Vec, SchussRepository, from_parts_roundtrip(), id() (+44 more)

### Community 49 - "common.rs"
Cohesion: 0.11
Nodes (15): format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), DateTime (+7 more)

### Community 51 - "instrumented.rs"
Cohesion: 0.28
Nodes (7): InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option, ResultReport

### Community 52 - "LogandaimApiTestClient"
Cohesion: 0.16
Nodes (17): assert_status_ok(), base_app_builder(), health_returns_ok(), json_decimal(), json_response(), LogandaimApiTestClient, Decimal, Self (+9 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.43
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - ".rendern"
Cohesion: 0.38
Nodes (4): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec

### Community 56 - "schema.ts"
Cohesion: 0.18
Nodes (10): components, $defs, operations, paths, webhooks, Wertungszone, Body, Kill (+2 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 61 - "App"
Cohesion: 0.05
Nodes (50): BackendServerError, catch_panic(), main(), Report, Result, init_tracing(), Option, Path (+42 more)

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.43
Nodes (7): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-sqlite, molting

### Community 70 - "cn"
Cohesion: 0.12
Nodes (25): zonen, Alert(), AlertProps, AlertVariant, variantClasses, Button(), Checkbox(), CheckboxProps (+17 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 82 - "Contributing and project management"
Cohesion: 0.22
Nodes (9): Agents, Commits, Contributing and project management, Delivery gate, GitHub Project, Labels and priority, Ready gate, Records (+1 more)

### Community 83 - "ADR-0001: Baseline architecture"
Cohesion: 0.50
Nodes (4): ADR-0001: Baseline architecture, Consequences, Context, Decision

### Community 84 - "http-client.ts"
Cohesion: 0.19
Nodes (8): HttpLogandaimApi, JsonClient, unwrap(), LogandaimApi, Schuss, SchussErstellungInput, Wertung, config

### Community 99 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 102 - "String"
Cohesion: 0.07
Nodes (42): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+34 more)

### Community 103 - "card.tsx"
Cohesion: 0.21
Nodes (13): useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), useLogandaimApi(), logandaimKeys, HomePage(), SchussPage(), Card() (+5 more)

### Community 114 - "navigation.ts"
Cohesion: 0.12
Nodes (23): AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar(), SidebarLink() (+15 more)

### Community 117 - "UnitOfWork"
Cohesion: 0.32
Nodes (4): Box, RepositoryResult, UnitOfWork, Drop

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 137 - "dependencies"
Cohesion: 0.25
Nodes (8): dependencies, lucide-react, next, openapi-fetch, react, react-dom, @tanstack/react-query, @tanstack/react-query-devtools

### Community 138 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 150 - "preis.rs"
Cohesion: 0.05
Nodes (30): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self, Menge, MengeFehler, Decimal (+22 more)

### Community 151 - "Versioned<T>"
Cohesion: 0.13
Nodes (11): Clone, Deref, Self, T, Target, UnitOfWork<'a>, Versioned, Versioned<T> (+3 more)

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
Cohesion: 0.11
Nodes (17): nextConfig, name, private, version, babel-plugin-react-compiler, @biomejs/biome, next, openapi-fetch (+9 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "[Unreleased]"
Cohesion: 0.50
Nodes (4): Added, Changed, Changelog, [Unreleased]

## Knowledge Gaps
- **326 isolated node(s):** `ValidationError`, `SchussRegistrierenFehler`, `WertungFehler`, `logandaim-fakes`, `molting` (+321 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 576 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **14 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `in_memory_object_store.rs`, `FakeDatastore`, `common.rs`, `FakeObjectStore`, `LogandaimApiTestClient`, `preis.rs`, `StructuredError`, `App`?**
  _High betweenness centrality (0.166) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `UnitOfWorkImpl`, `LogandaimApiTestClient`, `Clock`, `ObjectStore`, `ExecutionContext`?**
  _High betweenness centrality (0.083) - this node is a cross-community bridge._
- **Why does `ObjectStore` connect `ObjectStore` to `logandaim-filesystemstore/src/lib.rs`, `in_memory_object_store.rs`, `FakeObjectStore`, `instrumented.rs`, `ExecutionContext`, `App`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._
- **What connects `ValidationError`, `SchussRegistrierenFehler`, `WertungFehler` to the rest of the system?**
  _326 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._
- **Should `yams-persistence` be split into smaller, more focused modules?**
  _Cohesion score 0.08307692307692308 - nodes in this community are weakly interconnected._