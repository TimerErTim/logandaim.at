# Graph Report - logandaim.at  (2026-10-23)

## Corpus Check
- 249 files · ~82,362 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3190 nodes · 7880 edges · 180 communities (150 shown, 20 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 207 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `14482763`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- arc_up
- devDependencies
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- SeminarBuchungId
- apply_up_migrations
- YamsAppApi
- SeminarTerminId
- yams-filesystemstore/src/lib.rs
- Migration
- compilerOptions
- schema.ts
- Clock
- BehandlungId
- Make invalid state unrepresentable
- E
- KlientId
- String
- SQLiteRechnungRepository
- Versioned<T>
- src/sqlite_state_store.rs
- <Title>
- tests.rs
- PdfRenderer
- layout.tsx
- RepositoryResult
- Wertung — Classic 3-arrow scoring
- v0004_leistungen_quelle_mwst.rs
- UseCase
- EmailAdresse
- use_cases/listen.rs
- RechnungId
- NeuerKlient
- LeistungId
- PdfDokument
- service/pdf.rs
- Uuid
- FixedClock
- .get_current_version
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- App
- FakeDatastore
- .contextualize_with
- RepoStorage
- domain/seminar_termin.rs
- common.rs
- Klientbericht
- base_app_builder
- SeminarId
- .try_from
- ProduktId
- types.ts
- logandaim.at — Product Charter
- StructuredError
- RepositoryResult
- charter.md
- StreamBinaryResponse
- base_app_builder
- schema/seminar.rs
- .start
- cn
- Release Workflow
- SeminarOrt
- Product Requirements Document (PRD) & Konzeptbericht
- yams-core
- react
- Migration
- Option
- SQLiteHaustierRepository
- UnitOfWork
- diffx-finish-review
- Migration
- requests/abrechnung.rs
- Next.js
- postcss.config.mjs
- bad_request
- Migration
- Contributing and project management
- 0001-baseline-architecture.md
- page-chrome.tsx
- yams-typstreports/src/lib.rs
- HaustierErstellen
- use_cases/seminar.rs
- SQLiteUnitOfWork
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- Specifications
- SQLiteInstance
- mwst_19
- src/config.rs
- hooks/index.ts
- instrumented.rs
- .teilnahmebestätigung_pdf
- .behandlung_erstellen
- StatusCode
- Klient
- .produkt_erstellen
- KlientRegisterForm
- cases/abrechnung.rs
- HaustierId
- navigation.ts
- HttpYamsApi
- KlientErstellen
- Versioned
- pull_request_template.md
- KlientErstellung
- Rechnung
- rechnung-download-button.tsx
- leistung_from_row
- yams_scheduler/config.rs
- YAMS Frontend Migration Specification
- seminar_from_row
- TracingObserver
- UnitOfWorkImpl
- BehandlungCreateForm
- migration_error_to_persistence_error
- YamsApiSpec
- main
- validation_error.rs
- 3. Kernanforderungen (Funktionale Spezifikation)
- .haustier_by_id
- in_memory_object_store.rs
- FakeObjectStore
- dependencies
- scripts
- InternalServerError
- TypicalJsonResponse
- seminar-overview.tsx
- Zeitraum
- .seminar_umsatz_prognose
- FakeUnitOfWork
- SQLiteStateStore
- .haustier_erstellen
- HaustierForm
- ExecutionContext
- SeminarCreateForm
- Menge
- LeistungOffen
- main
- SeminarForm
- Adresse
- package.json
- Preis
- HaustierCreateForm
- Ratio
- ProduktCreateForm
- ObjectStore
- SQLiteInstance
- preis.rs
- InstrumentedUnitOfWork
- SeminarTerminGeplant
- openapi_service
- RowActionsMenu
- SeminarBuchungForm
- .finish
- StreamBody
- KlientForm
- mise
- ProduktRepository
- LockedUnitOfWorkImpl
- mwst_19
- test_haustier
- test_klient

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 125 edges
2. `KlientId` - 77 edges
3. `Preis` - 56 edges
4. `Ratio` - 56 edges
5. `react` - 46 edges
6. `useYamsApiReady()` - 44 edges
7. `cn()` - 43 edges
8. `FakeDatastore` - 41 edges
9. `YamsAppApi` - 40 edges
10. `YamsApiSpec` - 40 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/yams-api/src/spec.rs
- `yams-scheduler` --depends_on--> `molting`  [EXTRACTED]
  backend/scheduler/Cargo.toml → crates/molting/Cargo.toml
- `ConfigError` --references--> `String`  [EXTRACTED]
  backend/scheduler/src/bin/yams_scheduler/config.rs → crates/yams-api/src/errors/internal_error.rs
- `ResolvedConfig` --references--> `String`  [EXTRACTED]
  backend/scheduler/src/bin/yams_scheduler/config.rs → crates/yams-api/src/errors/internal_error.rs
- `DaemonConfig` --references--> `String`  [EXTRACTED]
  backend/scheduler/src/bin/yams_scheduler/config.rs → crates/yams-api/src/errors/internal_error.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (180 total, 20 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

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

### Community 6 - "SeminarBuchungId"
Cohesion: 0.10
Nodes (10): Item, Iterator, Self, Seminar, Uuid, SeminarBuchung, SeminarBuchungId, SeminarTerminIn<S> (+2 more)

### Community 7 - "apply_up_migrations"
Cohesion: 0.23
Nodes (8): apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option, DoubleEndedIterator

### Community 8 - "YamsAppApi"
Cohesion: 0.11
Nodes (20): AuflistenFehler, Arc, Behandlung, Klient, KlientErstellung, Produkt, Rechnung, RechnungBezahltMarkieren (+12 more)

### Community 9 - "SeminarTerminId"
Cohesion: 0.18
Nodes (24): SeminarTerminId, insert_params(), leistung_id_for(), load_buchungen(), load_termin(), ort_from_columns(), replace_buchungen(), Arc (+16 more)

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 11 - "Migration"
Cohesion: 0.33
Nodes (6): Migration, Error, Option, Result, Transaction, table_exists()

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "schema.ts"
Cohesion: 0.08
Nodes (23): components, $defs, LeistungQuelle_LeistungQuelleBehandlungTyp, Behandlung, LeistungQuelle_LeistungQuelleManuellTyp, Manuell, LeistungQuelle_LeistungQuelleProduktTyp, Produkt (+15 more)

### Community 14 - "Clock"
Cohesion: 0.14
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 15 - "BehandlungId"
Cohesion: 0.09
Nodes (20): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+12 more)

### Community 16 - "Make invalid state unrepresentable"
Cohesion: 0.12
Nodes (15): Closed enums, Creation input and construction, Derived values, Domain model, Encapsulation, Errors, Identity newtypes, Layout (+7 more)

### Community 17 - "E"
Cohesion: 0.18
Nodes (19): AppliableMigration, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>> (+11 more)

### Community 18 - "KlientId"
Cohesion: 0.33
Nodes (11): klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), AsRef, ResultReport (+3 more)

### Community 19 - "String"
Cohesion: 0.13
Nodes (19): From, String, email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix() (+11 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.21
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.12
Nodes (11): Clone, Deref, Option, Self, T, Target, UnitOfWork<'a>, Versioned<T> (+3 more)

### Community 22 - "src/sqlite_state_store.rs"
Cohesion: 0.23
Nodes (11): format_optional_timestamp(), libsql_error_to_store_error(), parse_optional_timestamp(), Box, DateTime, Future, Option, Output (+3 more)

### Community 23 - "<Title>"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+5 more)

### Community 24 - "tests.rs"
Cohesion: 0.20
Nodes (34): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+26 more)

### Community 25 - "PdfRenderer"
Cohesion: 0.21
Nodes (6): ExecutionContext<'a>, RepositoryResult, Self, PdfRenderer, Send, Sync

### Community 26 - "layout.tsx"
Cohesion: 0.21
Nodes (7): nextConfig, geistMono, geistSans, metadata, Providers(), createQueryClient(), next

### Community 27 - "RepositoryResult"
Cohesion: 0.13
Nodes (11): FakeHaustiereRepository, FakeRechnungenRepository, FakeSeminareRepository, FakeSeminarTermineRepository, Haustier, NaiveDate, Rechnung, RepositoryResult (+3 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.17
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.23
Nodes (21): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), converts_percentage_mwst_to_ratio(), idempotent_when_already_ratio() (+13 more)

### Community 30 - "UseCase"
Cohesion: 0.16
Nodes (16): Context, Output, ResultReport, UseCase, Option, Seminar, SeminarOrt, SeminarTermin (+8 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.21
Nodes (10): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Result (+2 more)

### Community 32 - "use_cases/listen.rs"
Cohesion: 0.14
Nodes (16): AlleBehandlungenAuflisten, AlleHaustiereAuflisten, AlleKlientenAuflisten, AlleLeistungenAuflisten, AlleProdukteAuflisten, AlleRechnungenAuflisten, AlleSeminareAuflisten, AlleSeminarTermineAuflisten (+8 more)

### Community 33 - "RechnungId"
Cohesion: 0.11
Nodes (32): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+24 more)

### Community 34 - "NeuerKlient"
Cohesion: 0.13
Nodes (7): Klient, NeuerKlient, Adresse, EmailAdresse, Into, Mobilnummer, NaiveDate

### Community 35 - "LeistungId"
Cohesion: 0.09
Nodes (22): Abgerechnet, Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungId, LeistungIn, LeistungIn<S> (+14 more)

### Community 36 - "PdfDokument"
Cohesion: 0.13
Nodes (17): PdfDokument, PdfRenderError, Rechnungsbericht, Rechnungspositionsbericht, NaiveDate, ResultReport, Vec, FakePdfRenderer (+9 more)

### Community 37 - "service/pdf.rs"
Cohesion: 0.22
Nodes (17): mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen(), rechnung_object_key(), rechnung_pdf_laden(), ObjectStoreError, ObjectStream (+9 more)

### Community 38 - "Uuid"
Cohesion: 0.14
Nodes (14): RepositoryError, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarTerminErstellung, Uuid, schema_seminar_termin_from_domain() (+6 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - ".get_current_version"
Cohesion: 0.21
Nodes (11): Box, Error, Future, Option, Output, Pin, Result, Send (+3 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.16
Nodes (23): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Error, Option (+15 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.22
Nodes (5): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "App"
Cohesion: 0.11
Nodes (21): Box, Future, Output, Pin, Result, S, SchedulerError, SchedulerReport (+13 more)

### Community 45 - "FakeDatastore"
Cohesion: 0.10
Nodes (17): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, Arc, Behandlung, Clone, Default (+9 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 47 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedBehandlungRepository, InstrumentedHaustierRepository, InstrumentedKlientRepository, InstrumentedLeistungRepository (+4 more)

### Community 48 - "domain/seminar_termin.rs"
Cohesion: 0.31
Nodes (17): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+9 more)

### Community 50 - "common.rs"
Cohesion: 0.20
Nodes (23): format_datetime(), menge_to_str(), parse_datetime(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis() (+15 more)

### Community 51 - "Klientbericht"
Cohesion: 0.16
Nodes (18): Klientbericht, PraxisAngaben, Adresse, DateTime, EmailAdresse, Option, Utc, Teilnahmebestätigung (+10 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (44): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+36 more)

### Community 53 - "SeminarId"
Cohesion: 0.13
Nodes (13): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+5 more)

### Community 54 - ".try_from"
Cohesion: 0.27
Nodes (8): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Result, Self, TryFrom, Uuid

### Community 55 - "ProduktId"
Cohesion: 0.09
Nodes (20): produkt_betrag_multiplies_menge(), NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport (+12 more)

### Community 56 - "types.ts"
Cohesion: 0.07
Nodes (31): ApiError, JsonClient, Adresse, Behandlung, BehandlungErstellung, Haustier, HaustierErstellung, Klient (+23 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.18
Nodes (11): Decision log, Domain terms, logandaim.at — Product Charter, Milestones, MVP scope, Non-goals for MVP, Open questions (not blocking M0), Problem (+3 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "RepositoryResult"
Cohesion: 0.07
Nodes (19): BehandlungRepository, HaustierRepository, LeistungRepository, RechnungRepository, RepositoryError, Behandlung, Haustier, Leistung (+11 more)

### Community 60 - "charter.md"
Cohesion: 0.27
Nodes (3): Changed, Changelog, [Unreleased]

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.31
Nodes (10): C, From, ObjectStream, Report, Result, StatusCode, T, StreamBinaryResponse (+2 more)

### Community 62 - "base_app_builder"
Cohesion: 0.18
Nodes (33): base_app_builder(), AppBuilder, SetUowProvider, abgehalten_maps_every_confirmed_buchung(), abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen(), absage_blockiert_abgehalten(), adresse(), aktualisieren_blockiert_nach_abgehalten() (+25 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.18
Nodes (21): NaiveDate, SeminarUmsatzPrognose, BuchungUmsatz, DateTime, Decimal, NaiveDate, Option, Utc (+13 more)

### Community 64 - ".start"
Cohesion: 0.11
Nodes (17): Box, Default, Future, Output, Pin, Result, S, SchedulerError (+9 more)

### Community 65 - "cn"
Cohesion: 0.07
Nodes (51): useRechnungenFürKlientQuery(), RechnungStatus, Bezahlt, Offen, KlientDetailContent(), KatalogTab, TabButton(), KlientDetail() (+43 more)

### Community 67 - "SeminarOrt"
Cohesion: 0.24
Nodes (8): domain::SeminarOrt, Adresse, Error, From, Result, Self, TryFrom, SeminarOrt

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.20
Nodes (10): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich), 4.1 Für Casual User, 4.2 Für Power-User & Turnierschützen, 4.3 Für Anlagenbetreiber (B2B & Vereinsnutzen), 4. Erweiterungsvorschläge (Feature Roadmap nach Rollen), 5. Technische Architektur & Systemanforderungen (Übersicht) (+2 more)

### Community 69 - "yams-core"
Cohesion: 0.47
Nodes (9): molting, yams-api, yams-core, yams-fakes, yams-filesystemstore, yams-scheduler, yams-server, yams-sqlite (+1 more)

### Community 70 - "react"
Cohesion: 0.10
Nodes (46): HaustierCreateFormProps, KlientRegisterFormProps, LeistungBuchenFormProps, LeistungQuelle, RechnungBezahltButtonProps, TerminBuchungFormProps, TerminDetailPanelProps, TerminPlanFormProps (+38 more)

### Community 71 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 72 - "Option"
Cohesion: 0.23
Nodes (7): NeuerSeminarTermin, Adresse, Option, S, Vec, SeminarOrt, SeminarTerminIn

### Community 73 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 74 - "UnitOfWork"
Cohesion: 0.44
Nodes (4): Box, RepositoryResult, UnitOfWork, Drop

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 77 - "requests/abrechnung.rs"
Cohesion: 0.16
Nodes (28): BehandlungErstellen, BehandlungErstellung, into_rechnung_als_bezahlt_markieren(), LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung (+20 more)

### Community 80 - "bad_request"
Cohesion: 0.15
Nodes (11): bad_request(), C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report, ValidationError (+3 more)

### Community 81 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 82 - "Contributing and project management"
Cohesion: 0.22
Nodes (9): Agents, Commits, Contributing and project management, Delivery gate, GitHub Project, Labels and priority, Ready gate, Records (+1 more)

### Community 83 - "0001-baseline-architecture.md"
Cohesion: 0.22
Nodes (6): ADR-0001: Baseline architecture, Consequences, Context, Decision, Architecture decision records, Legacy specs (YAMS proof of concept)

### Community 84 - "page-chrome.tsx"
Cohesion: 0.32
Nodes (6): AppChrome(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome()

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.19
Nodes (22): adresse_dict(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict(), preis(), ratio() (+14 more)

### Community 89 - "HaustierErstellen"
Cohesion: 0.19
Nodes (12): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, Result (+4 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.15
Nodes (19): BuchungUmsatz, NaiveDate, Report, Self, Vec, SeminarBuchungAnlegenFehler, SeminarBuchungStornierenFehler, SeminarErstellenFehler (+11 more)

### Community 93 - "SQLiteUnitOfWork"
Cohesion: 0.23
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 99 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, InstanceType, Arc, AsRef, Connection, Deref, Mutex, Path (+9 more)

### Community 101 - "mwst_19"
Cohesion: 0.83
Nodes (3): mwst_19(), test_behandlung_erstellen(), test_behandlung_erstellen_rejects_empty_name()

### Community 102 - "src/config.rs"
Cohesion: 0.14
Nodes (27): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+19 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.08
Nodes (51): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useRechnungAlsBezahltMarkierenMutation() (+43 more)

### Community 104 - "instrumented.rs"
Cohesion: 0.20
Nodes (10): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, Pin (+2 more)

### Community 105 - ".teilnahmebestätigung_pdf"
Cohesion: 0.47
Nodes (3): ObjectStoreError, ObjectStream, Self

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.36
Nodes (6): BehandlungErstellenFehler, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "StatusCode"
Cohesion: 0.05
Nodes (36): AuflistenFehler, BehandlungErstellenFehler, HaustierErstellenFehler, HttpStatusMapping, KlientErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler (+28 more)

### Community 108 - "Klient"
Cohesion: 0.31
Nodes (9): Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid, Vec (+1 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.32
Nodes (6): ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain(), ProduktErstellenFehler

### Community 112 - "cases/abrechnung.rs"
Cohesion: 0.23
Nodes (17): AbrechnungSetup, app_with_pdf_fakes(), menge(), mwst_19(), rechnung_kann_als_bezahlt_markiert_werden(), Arc, Klient, NaiveDate (+9 more)

### Community 113 - "HaustierId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "navigation.ts"
Cohesion: 0.18
Nodes (17): AppShell(), Sidebar(), SidebarLink(), SidebarProps, BreadcrumbSegment, breadcrumbsForPathname(), homeNavItem, isExactPath() (+9 more)

### Community 115 - "HttpYamsApi"
Cohesion: 0.07
Nodes (13): HttpYamsApi, unwrap(), createYamsApi(), normalizeApiBaseUrl(), resetYamsApiCache(), YamsApiContext, YamsApiContextValue, YamsApiProvider() (+5 more)

### Community 116 - "KlientErstellen"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate (+1 more)

### Community 117 - "Versioned"
Cohesion: 0.14
Nodes (15): KlientRepository, Klient, Versioned, FakeKlientenRepository, Klient, klient_from_row(), Arc, Klient (+7 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "KlientErstellung"
Cohesion: 0.20
Nodes (10): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Result (+2 more)

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "rechnung-download-button.tsx"
Cohesion: 0.33
Nodes (6): RechnungDownloadButton(), handleDownload(), RechnungDownloadButtonProps, downloadTeilnahmePdf(), downloadTeilnahmePdf(), downloadBlob()

### Community 122 - "leistung_from_row"
Cohesion: 0.18
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 123 - "yams_scheduler/config.rs"
Cohesion: 0.22
Nodes (19): CliOverlay, ConfigError, DaemonConfig, load_file(), overlay(), overlays_database_url_from_cli(), parse_config_file(), parses_timezone_from_file() (+11 more)

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 125 - "seminar_from_row"
Cohesion: 0.25
Nodes (10): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, Vec (+2 more)

### Community 126 - "TracingObserver"
Cohesion: 0.40
Nodes (3): TracingObserver, SchedulerEvent, SchedulerObserver

### Community 127 - "UnitOfWorkImpl"
Cohesion: 0.15
Nodes (12): main(), Box, RepositoryResult, Unimplemented, Arc<T>, Send, Sync, UnitOfWorkImpl (+4 more)

### Community 128 - "BehandlungCreateForm"
Cohesion: 1.00
Nodes (3): BehandlungCreateForm(), handleSubmit(), emptyBehandlung()

### Community 129 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 130 - "YamsApiSpec"
Cohesion: 0.21
Nodes (9): Path, RechnungBezahltMarkieren, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarUmsatzVorschau, Uuid (+1 more)

### Community 131 - "main"
Cohesion: 0.16
Nodes (14): BackendServerError, catch_panic(), main(), Report, Result, SchedulerError, SchedulerReport, SchedulerTask (+6 more)

### Community 133 - "3. Kernanforderungen (Funktionale Spezifikation)"
Cohesion: 0.40
Nodes (5): 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation)

### Community 134 - ".haustier_by_id"
Cohesion: 0.29
Nodes (7): Haustier, HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "in_memory_object_store.rs"
Cohesion: 0.13
Nodes (21): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted() (+13 more)

### Community 136 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 137 - "dependencies"
Cohesion: 0.25
Nodes (8): dependencies, lucide-react, next, openapi-fetch, react, react-dom, @tanstack/react-query, @tanstack/react-query-devtools

### Community 138 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 140 - "TypicalJsonResponse"
Cohesion: 0.10
Nodes (18): Behandlung, BehandlungErstellung, Klient, KlientErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung (+10 more)

### Community 141 - "seminar-overview.tsx"
Cohesion: 0.10
Nodes (25): RechnungBezahltButton(), SeminarOverview(), statusBadgeVariant(), TerminFilter, buildTerminEventsByDay(), formatSelectedDate(), TerminCalendar(), TerminCalendarProps (+17 more)

### Community 142 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "FakeUnitOfWork"
Cohesion: 0.23
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 145 - "SQLiteStateStore"
Cohesion: 0.21
Nodes (10): Arc, Error, Result, Self, Send, Transaction, SQLiteStateStore, JobState (+2 more)

### Community 148 - "ExecutionContext"
Cohesion: 0.32
Nodes (5): ExecutionContext, ExecutionSource, Arc, Error, Result

### Community 149 - "SeminarCreateForm"
Cohesion: 1.00
Nodes (3): emptySeminar(), SeminarCreateForm(), handleSubmit()

### Community 150 - "Menge"
Cohesion: 0.22
Nodes (5): Menge, MengeFehler, Decimal, Result, Self

### Community 151 - "LeistungOffen"
Cohesion: 0.13
Nodes (22): LeistungOffen, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen (+14 more)

### Community 152 - "main"
Cohesion: 0.21
Nodes (10): Cli, main(), Option, PathBuf, Report, Result, SchedulerMainError, init_tracing() (+2 more)

### Community 154 - "Adresse"
Cohesion: 0.23
Nodes (9): Adresse, domain::Adresse, Ländercode, Error, Example, From, Result, Self (+1 more)

### Community 157 - "package.json"
Cohesion: 0.13
Nodes (14): name, private, version, babel-plugin-react-compiler, @biomejs/biome, openapi-fetch, openapi-typescript, react-dom (+6 more)

### Community 158 - "Preis"
Cohesion: 0.13
Nodes (8): Add, Preis, Output, Self, position_from_leistung(), RechnungIn<S>, Rechnungsposition, Mul

### Community 159 - "HaustierCreateForm"
Cohesion: 0.67
Nodes (3): emptyHaustier(), HaustierCreateForm(), handleSubmit()

### Community 163 - "Ratio"
Cohesion: 0.20
Nodes (5): Ratio, RatioFehler, Decimal, Result, Self

### Community 164 - "ProduktCreateForm"
Cohesion: 1.00
Nodes (3): emptyProdukt(), ProduktCreateForm(), handleSubmit()

### Community 165 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 166 - "SQLiteInstance"
Cohesion: 0.21
Nodes (8): base_app_builder(), AppBuilder, SetUowProvider, app(), neuer_klient(), parallel_execute_fn_survives_wal_connection_init(), Arc, SQLiteInstance

### Community 169 - "preis.rs"
Cohesion: 0.26
Nodes (9): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal (+1 more)

### Community 170 - "InstrumentedUnitOfWork"
Cohesion: 0.22
Nodes (4): InstrumentedUnitOfWork, Box, RepositoryResult, Self

### Community 171 - "SeminarTerminGeplant"
Cohesion: 0.14
Nodes (16): Abgehalten, Abgesagt, Geplant, DateTime, From, FxHashMap, Into, ResultReport (+8 more)

### Community 172 - "openapi_service"
Cohesion: 0.25
Nodes (7): openapi_service(), Into, Item, Self, IntoIterator, OpenApiService, ServerObject

### Community 174 - "SeminarBuchungForm"
Cohesion: 1.00
Nodes (3): defaultBuchung(), SeminarBuchungForm(), handleSubmit()

### Community 175 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 177 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 189 - "mwst_19"
Cohesion: 0.83
Nodes (3): mwst_19(), test_produkt_erstellen(), test_produkt_erstellen_rejects_empty_name()

## Knowledge Gaps
- **312 isolated node(s):** `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+307 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 838 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **20 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `.haustier_by_id`, `in_memory_object_store.rs`, `FakeObjectStore`, `SeminarTerminId`, `InternalServerError`, `TypicalJsonResponse`, `BehandlungId`, `FakeUnitOfWork`, `KlientId`, `SQLiteRechnungRepository`, `src/sqlite_state_store.rs`, `LeistungOffen`, `main`, `tests.rs`, `Adresse`, `v0004_leistungen_quelle_mwst.rs`, `Preis`, `EmailAdresse`, `UseCase`, `NeuerKlient`, `LeistungId`, `PdfDokument`, `service/pdf.rs`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `SeminarTerminGeplant`, `common.rs`, `Klientbericht`, `base_app_builder`, `SeminarId`, `.try_from`, `ProduktId`, `StructuredError`, `schema/seminar.rs`, `SeminarOrt`, `Option`, `requests/abrechnung.rs`, `HaustierErstellen`, `src/config.rs`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `HaustierId`, `KlientErstellen`, `KlientErstellung`, `Rechnung`, `leistung_from_row`, `yams_scheduler/config.rs`?**
  _High betweenness centrality (0.251) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `.start`, `YamsApiSpec`, `ObjectStore`, `YamsAppApi`, `.teilnahmebestätigung_pdf`, `openapi_service`, `Clock`, `cases/abrechnung.rs`, `base_app_builder`, `PdfRenderer`, `base_app_builder`, `UnitOfWorkImpl`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._
- **Why does `KlientId` connect `KlientId` to `SeminarBuchungId`, `in_memory_object_store.rs`, `YamsAppApi`, `SeminarTerminId`, `SQLiteRechnungRepository`, `LeistungOffen`, `tests.rs`, `RepositoryResult`, `UseCase`, `Preis`, `RechnungId`, `NeuerKlient`, `LeistungId`, `requests/seminar.rs`, `domain/seminar_termin.rs`, `common.rs`, `Klientbericht`, `.try_from`, `RepositoryResult`, `SQLiteHaustierRepository`, `requests/abrechnung.rs`, `HaustierErstellen`, `use_cases/seminar.rs`, `HaustierId`, `Versioned`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler` to the rest of the system?**
  _312 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._