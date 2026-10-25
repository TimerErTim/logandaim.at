# Graph Report - logandaim.at  (2026-10-25)

## Corpus Check
- 189 files · ~66,810 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1797 nodes · 3267 edges · 113 communities (95 shown, 11 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 47 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5c3e9309`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FakePdfRenderer
- arc_up
- CONTRIBUTING.md
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- cn
- SchussErstellung
- http_status.rs
- sections.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- button.tsx
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
- check-release.mjs
- ExecutionContext
- Wertung — Classic 3-arrow scoring
- .contextualize_with
- Konto — Accounts and login
- .finish
- App
- RepoStorage
- base_app_builder
- hooks/index.ts
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- FixedClock
- SQLiteUnitOfWork
- application/uow.rs
- Vertical slices
- domain/schuss.rs
- common.rs
- components.json
- instrumented.rs
- LogandaimApiTestClient
- InstrumentedUnitOfWork
- .rendern
- schema.ts
- token-swatches.tsx
- logandaim.at — Product Charter
- dependencies
- layout.tsx
- charter.md
- StreamBinaryResponse
- Visual identity and design system
- main
- base_app_builder
- devDependencies
- InternalServerError
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- react
- toggle-group.tsx
- check-version.mjs
- provider.tsx
- sortable-table-section.tsx
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- UnitOfWorkProvider
- Next.js
- postcss.config.mjs
- UseCase
- card.tsx
- Contributing and project management
- ADR-0001: Baseline architecture
- http-client.ts
- StreamBody
- openapi_service
- next.config.ts
- ApiError
- radio-group.tsx
- search-input.tsx
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- Specifications
- SQLiteInstance
- String
- navigation.ts
- UnitOfWorkImpl
- pull_request_template.md
- YAMS Frontend Migration Specification
- dates.ts
- preis.rs
- Versioned<T>
- ADR-0002: Session cookie auth across frontend and API origins
- m1-overseer.md
- Modes
- package.json
- m1-reviewer.md
- mise

## God Nodes (most connected - your core abstractions)
1. `cn()` - 88 edges
2. `E` - 36 edges
3. `react` - 35 edges
4. `UnitOfWorkImpl` - 27 edges
5. `arc_up()` - 20 edges
6. `App` - 19 edges
7. `Versioned<T>` - 19 edges
8. `Datenschutz — Data protection, privacy, and legal obligations` - 19 edges
9. `UpMigration` - 18 edges
10. `ObjectStore` - 17 edges

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

## Communities (113 total, 11 thin omitted)

### Community 0 - "FakePdfRenderer"
Cohesion: 0.15
Nodes (11): Vec, PdfDokument, PdfRenderError, ResultReport, Vec, FakePdfRenderer, Arc, Mutex (+3 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "CONTRIBUTING.md"
Cohesion: 0.18
Nodes (7): Changelog, [Unreleased], Checklist, Release description, Release pull request, Test evidence, Version

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "cn"
Cohesion: 0.10
Nodes (27): GridPaper(), Legend(), LegendEntry, RingMark, RingPlot(), ScaleBar(), DataFiguresSection(), SeriesPlot() (+19 more)

### Community 7 - "SchussErstellung"
Cohesion: 0.10
Nodes (23): ResultReport, Schuss, Wertung, From, Self, Uuid, Vec, Wertungszone (+15 more)

### Community 8 - "http_status.rs"
Cohesion: 0.14
Nodes (16): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+8 more)

### Community 9 - "sections.tsx"
Cohesion: 0.11
Nodes (25): PLATES, AlertSection(), BadgeSection(), ButtonSection(), CardSection(), DataSection(), FormSection(), MiscSection() (+17 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "button.tsx"
Cohesion: 0.22
Nodes (9): Alert(), AlertProps, alertVariants, iconColour, Button(), ButtonProps, buttonVariants, class-variance-authority (+1 more)

### Community 14 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

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

### Community 26 - "check-release.mjs"
Cohesion: 0.12
Nodes (21): contentLines(), readChangelog(), section(), sectionContent(), stripComments(), changelog, dated, escaped (+13 more)

### Community 27 - "ExecutionContext"
Cohesion: 0.18
Nodes (9): ExecutionContext, ExecutionContext<'a>, ExecutionSource, Arc, RepositoryResult, Self, PdfRenderer, Send (+1 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 30 - "Konto — Accounts and login"
Cohesion: 0.08
Nodes (25): Acceptance criteria, Account security, Administration, Auth audit, Decision log, Deferred capabilities, Domain terms and business rules, Email change (+17 more)

### Community 31 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 32 - "App"
Cohesion: 0.16
Nodes (12): LogandaimAppApi, Arc, Self, Self, App, Arc, Box, Context (+4 more)

### Community 33 - "RepoStorage"
Cohesion: 0.38
Nodes (5): RepoStorage, Box, Pin, Self, InstrumentedSchussRepository

### Community 34 - "base_app_builder"
Cohesion: 0.31
Nodes (8): base_app_builder(), AppBuilder, SetUowProvider, schuss_registrieren_persists_and_can_be_loaded(), wertung_leere_liste_ist_fehler(), wertung_summiert_nach_pfeil_innerhalb_eines_ziels(), wertung_unbekannte_id_ist_fehler(), ziel_id()

### Community 35 - "hooks/index.ts"
Cohesion: 0.27
Nodes (8): useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), useLogandaimApi(), logandaimKeys, HomePage(), SchussPage(), DeploymentStatus()

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

### Community 46 - "application/uow.rs"
Cohesion: 0.20
Nodes (6): RepositoryError, Option, Option, Versioned, Debug, Ordering

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.06
Nodes (51): RepositoryResult, Schuss, Send, Sync, Vec, SchussRepository, from_parts_roundtrip(), id() (+43 more)

### Community 49 - "common.rs"
Cohesion: 0.11
Nodes (15): format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), DateTime (+7 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "instrumented.rs"
Cohesion: 0.28
Nodes (7): InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option, ResultReport

### Community 52 - "LogandaimApiTestClient"
Cohesion: 0.23
Nodes (12): assert_status_ok(), health_returns_ok(), json_decimal(), json_response(), LogandaimApiTestClient, Decimal, StatusCode, Vec (+4 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.43
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - ".rendern"
Cohesion: 0.38
Nodes (4): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec

### Community 55 - "schema.ts"
Cohesion: 0.18
Nodes (10): components, $defs, operations, paths, webhooks, Wertungszone, Body, Kill (+2 more)

### Community 56 - "token-swatches.tsx"
Cohesion: 0.12
Nodes (20): ComponentSection(), Example(), ThemePlate(), touchTargetOutline, AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour() (+12 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.05
Nodes (37): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+29 more)

### Community 59 - "layout.tsx"
Cohesion: 0.25
Nodes (7): geistMono, inter, lora, metadata, Providers(), createQueryClient(), @tanstack/react-query

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.21
Nodes (15): LogandaimApiSpec, C, From, ObjectStream, Report, Result, Schuss, StatusCode (+7 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Requirements, Revision history (+4 more)

### Community 63 - "main"
Cohesion: 0.21
Nodes (10): BackendServerError, catch_panic(), main(), Report, Result, init_tracing(), Option, Path (+2 more)

### Community 64 - "base_app_builder"
Cohesion: 0.60
Nodes (4): base_app_builder(), leere_schussliste_ist_bad_request(), pfeil_null_ist_bad_request(), schuss_erfassen_und_wertung_berechnen()

### Community 65 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 66 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.43
Nodes (7): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-sqlite, molting

### Community 70 - "react"
Cohesion: 0.10
Nodes (17): Checkbox(), CheckboxProps, Field(), FieldProps, Label(), Progress(), Select(), SelectProps (+9 more)

### Community 71 - "toggle-group.tsx"
Cohesion: 0.50
Nodes (4): ToggleGroup(), ToggleGroupItem(), toggleGroupItemVariants, @radix-ui/react-toggle-group

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "provider.tsx"
Cohesion: 0.36
Nodes (6): createLogandaimApi(), LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config, @tanstack/react-query-devtools

### Community 74 - "sortable-table-section.tsx"
Cohesion: 0.10
Nodes (19): zonen, CHECKPOINTS, INITIAL_ROWS, Row, SortableTableSection(), State, STATES, Checkpoint (+11 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "UnitOfWorkProvider"
Cohesion: 0.14
Nodes (11): main(), Box, RepositoryResult, Unimplemented, Arc<T>, Send, Sync, UnitOfWorkProvider (+3 more)

### Community 80 - "UseCase"
Cohesion: 0.40
Nodes (4): Context, Output, ResultReport, UseCase

### Community 81 - "card.tsx"
Cohesion: 0.47
Nodes (6): Card(), CardContent(), CardDescription(), CardFooter(), CardHeader(), CardTitle()

### Community 82 - "Contributing and project management"
Cohesion: 0.13
Nodes (15): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Commits, Contributing and project management, Delivery gate (+7 more)

### Community 83 - "ADR-0001: Baseline architecture"
Cohesion: 0.50
Nodes (4): ADR-0001: Baseline architecture, Consequences, Context, Decision

### Community 84 - "http-client.ts"
Cohesion: 0.18
Nodes (8): HttpLogandaimApi, JsonClient, unwrap(), LogandaimApi, Schuss, SchussErstellungInput, Wertung, openapi-fetch

### Community 86 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 87 - "openapi_service"
Cohesion: 0.25
Nodes (7): openapi_service(), Into, IntoIterator, Item, Self, OpenApiService, ServerObject

### Community 88 - "next.config.ts"
Cohesion: 0.40
Nodes (3): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, next

### Community 90 - "radio-group.tsx"
Cohesion: 0.40
Nodes (4): RadioGroup(), RadioGroupItem(), RadioGroupOption(), @radix-ui/react-radio-group

### Community 91 - "search-input.tsx"
Cohesion: 0.50
Nodes (3): SearchInput(), SearchInputProps, lucide-react

### Community 99 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 102 - "String"
Cohesion: 0.06
Nodes (48): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+40 more)

### Community 114 - "navigation.ts"
Cohesion: 0.12
Nodes (23): AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar() (+15 more)

### Community 117 - "UnitOfWorkImpl"
Cohesion: 0.19
Nodes (7): LockedUnitOfWorkImpl, Box, RepositoryResult, UnitOfWork, UnitOfWork<'a>, UnitOfWorkImpl, Drop

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 150 - "preis.rs"
Cohesion: 0.05
Nodes (30): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self, Menge, MengeFehler, Decimal (+22 more)

### Community 151 - "Versioned<T>"
Cohesion: 0.15
Nodes (9): Clone, Deref, Self, T, Target, Versioned<T>, DerefMut, PartialEq (+1 more)

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
Cohesion: 0.08
Nodes (23): name, private, version, Slider(), babel-plugin-react-compiler, @biomejs/biome, @dnd-kit/core, @dnd-kit/sortable (+15 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

## Knowledge Gaps
- **413 isolated node(s):** `ValidationError`, `SchussRegistrierenFehler`, `WertungFehler`, `logandaim-fakes`, `molting` (+408 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 681 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `InternalServerError`, `in_memory_object_store.rs`, `FakeDatastore`, `common.rs`, `FakeObjectStore`, `LogandaimApiTestClient`, `preis.rs`, `StreamBinaryResponse`?**
  _High betweenness centrality (0.125) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `base_app_builder`, `UnitOfWorkProvider`, `Clock`, `LogandaimApiTestClient`, `ObjectStore`, `openapi_service`, `ExecutionContext`, `StreamBinaryResponse`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `App`, `.contextualize_with`, `String`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **What connects `ValidationError`, `SchussRegistrierenFehler`, `WertungFehler` to the rest of the system?**
  _413 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._
- **Should `yams-persistence` be split into smaller, more focused modules?**
  _Cohesion score 0.08307692307692308 - nodes in this community are weakly interconnected._