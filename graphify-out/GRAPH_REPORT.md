# Graph Report - logandaim.at  (2026-10-23)

## Corpus Check
- 253 files · ~84,832 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3243 nodes · 7962 edges · 185 communities (152 shown, 24 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 207 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `68930628`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- arc_up
- devDependencies
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- SeminarTerminGeplant
- apply_up_migrations
- YamsAppApi
- termin_from_parts
- yams-filesystemstore/src/lib.rs
- Migration
- compilerOptions
- schema.ts
- Clock
- Preis
- Make invalid state unrepresentable
- E
- wertung.rs
- domain/kontakt.rs
- SQLiteRechnungRepository
- .finish
- YamsApi
- <Title>
- tests.rs
- PdfRenderer
- layout.tsx
- FakeDatastore
- Wertung — Classic 3-arrow scoring
- v0004_leistungen_quelle_mwst.rs
- UseCase
- EmailAdresse
- use_cases/listen.rs
- RechnungId
- KlientId
- NeueLeistung
- FakePdfRenderer
- ObjectStore
- Uuid
- FixedClock
- AppliableMigration
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- run
- KlientRepository
- .contextualize_with
- RepoStorage
- domain/seminar_termin.rs
- common.rs
- use_cases/abrechnung.rs
- base_app_builder
- Seminar
- .try_from
- Produkt
- types.ts
- logandaim.at — Product Charter
- StructuredError
- RepositoryResult
- charter.md
- Json
- base_app_builder
- schema/seminar.rs
- .start
- cn
- Release Workflow
- blank_pdf_renderer.rs
- Product Requirements Document (PRD) & Konzeptbericht
- yams-core
- react
- Migration
- SeminarId
- SQLiteHaustierRepository
- BehandlungId
- diffx-finish-review
- Migration
- requests/abrechnung.rs
- Next.js
- postcss.config.mjs
- bad_request
- Migration
- Contributing and project management
- 0001-baseline-architecture.md
- ProduktId
- yams-typstreports/src/lib.rs
- HaustierErstellen
- SeminarTerminId
- SQLiteUnitOfWork
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- Specifications
- SQLiteInstance
- mwst_19
- String
- hooks/index.ts
- instrumented.rs
- PdfDokument
- Behandlung
- StatusCode
- Klient
- Produkt
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
- provider.tsx
- leistung_from_row
- yams_scheduler/config.rs
- YAMS Frontend Migration Specification
- seminar_from_row
- App
- UnitOfWorkImpl
- BehandlungCreateForm
- migration_error_to_persistence_error
- YamsApiSpec
- main
- validation_error.rs
- 3. Kernanforderungen (Funktionale Spezifikation)
- .haustier_erstellen
- Versioned<T>
- FakeObjectStore
- dependencies
- scripts
- providers.tsx
- TypicalJsonResponse
- seminar-overview.tsx
- Zeitraum
- .seminar_umsatz_prognose
- FakeUnitOfWork
- SQLiteStateStore
- LeistungRepository
- HaustierForm
- .seminar_umsatz_prognose
- SeminarCreateForm
- Menge
- ExecutionContext
- http-client.ts
- SeminarForm
- Adresse
- document_text
- Rechnung
- package.json
- Rechnungsposition
- HaustierCreateForm
- .leistung_aus_behandlung_buchen
- m1-overseer.md
- Modes
- Ratio
- ProduktCreateForm
- ObjectStoreError
- HaustierErstellung
- Seminar
- m1-reviewer.md
- preis.rs
- InstrumentedUnitOfWork
- LeistungId
- openapi_service
- RowActionsMenu
- SeminarBuchungForm
- ProduktErstellung
- StreamBody
- .seminar_umsatz_vorschau
- .seminar_erstellen
- mise
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

## Communities (185 total, 24 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

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

### Community 6 - "SeminarTerminGeplant"
Cohesion: 0.17
Nodes (7): Item, Iterator, Seminar, Vec, SeminarTerminGeplant, SeminarOrt, SeminarTerminPlanen

### Community 7 - "apply_up_migrations"
Cohesion: 0.23
Nodes (8): apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option, DoubleEndedIterator

### Community 8 - "YamsAppApi"
Cohesion: 0.11
Nodes (22): AuflistenFehler, BehandlungErstellenFehler, Arc, Behandlung, BehandlungErstellung, Haustier, Klient, Produkt (+14 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.19
Nodes (23): insert_params(), leistung_id_for(), load_buchungen(), load_termin(), ort_from_columns(), replace_buchungen(), Arc, FxHashMap (+15 more)

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
Cohesion: 0.09
Nodes (21): $defs, LeistungQuelle_LeistungQuelleBehandlungTyp, Behandlung, LeistungQuelle_LeistungQuelleManuellTyp, Manuell, LeistungQuelle_LeistungQuelleProduktTyp, Produkt, LeistungQuelle_LeistungQuelleSeminarTyp (+13 more)

### Community 14 - "Clock"
Cohesion: 0.14
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 15 - "Preis"
Cohesion: 0.11
Nodes (13): Behandlung, BehandlungFehler, NeueBehandlung, preis(), Into, ResultReport, Self, Uuid (+5 more)

### Community 16 - "Make invalid state unrepresentable"
Cohesion: 0.12
Nodes (15): Closed enums, Creation input and construction, Derived values, Domain model, Encapsulation, Errors, Identity newtypes, Layout (+7 more)

### Community 17 - "E"
Cohesion: 0.18
Nodes (18): ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+10 more)

### Community 18 - "wertung.rs"
Cohesion: 0.11
Nodes (17): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), Punkte, Add, I, IntoIterator, Item (+9 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (13): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+5 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.21
Nodes (16): preis_to_str(), geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate (+8 more)

### Community 21 - ".finish"
Cohesion: 0.22
Nodes (8): C, F, Formatter, Future, Output, Report, Result, T

### Community 22 - "YamsApi"
Cohesion: 0.12
Nodes (6): SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarTerminErstellung, YamsApi

### Community 23 - "<Title>"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+5 more)

### Community 24 - "tests.rs"
Cohesion: 0.24
Nodes (29): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+21 more)

### Community 25 - "PdfRenderer"
Cohesion: 0.21
Nodes (6): ExecutionContext<'a>, RepositoryResult, Self, PdfRenderer, Send, Sync

### Community 26 - "layout.tsx"
Cohesion: 0.22
Nodes (6): nextConfig, geistMono, geistSans, metadata, AppChrome(), next

### Community 27 - "FakeDatastore"
Cohesion: 0.09
Nodes (19): FakeBehandlungenRepository, FakeDatastore, FakeLeistungenRepository, FakeProdukteRepository, FakeSeminareRepository, Arc, Behandlung, Clone (+11 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.17
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.23
Nodes (21): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), converts_percentage_mwst_to_ratio(), idempotent_when_already_ratio() (+13 more)

### Community 30 - "UseCase"
Cohesion: 0.22
Nodes (12): LeistungOffen, Context, Output, ResultReport, UseCase, LeistungAusBehandlungBuchen, LeistungAusProduktBuchen, LeistungManuellErfassen (+4 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.21
Nodes (10): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Result (+2 more)

### Community 32 - "use_cases/listen.rs"
Cohesion: 0.14
Nodes (16): AlleBehandlungenAuflisten, AlleHaustiereAuflisten, AlleKlientenAuflisten, AlleLeistungenAuflisten, AlleProdukteAuflisten, AlleRechnungenAuflisten, AlleSeminareAuflisten, AlleSeminarTermineAuflisten (+8 more)

### Community 33 - "RechnungId"
Cohesion: 0.12
Nodes (30): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+22 more)

### Community 34 - "KlientId"
Cohesion: 0.11
Nodes (18): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), NeuerKlient (+10 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.08
Nodes (23): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungIn<S> (+15 more)

### Community 36 - "FakePdfRenderer"
Cohesion: 0.22
Nodes (6): FakePdfRenderer, Arc, Mutex, ResultReport, Self, Vec

### Community 37 - "ObjectStore"
Cohesion: 0.14
Nodes (27): ObjectStore, Send, Sync, klient_bericht(), mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen() (+19 more)

### Community 38 - "Uuid"
Cohesion: 0.12
Nodes (16): ObjectStoreError, ObjectStream, Self, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarTerminErstellung (+8 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - "AppliableMigration"
Cohesion: 0.19
Nodes (12): AppliableMigration, Box, Error, Future, Option, Output, Pin, Result (+4 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.16
Nodes (23): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Error, Option (+15 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.22
Nodes (5): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "run"
Cohesion: 0.12
Nodes (17): Box, Future, Output, Pin, Result, S, SchedulerError, SchedulerReport (+9 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 47 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedBehandlungRepository, InstrumentedHaustierRepository, InstrumentedKlientRepository, InstrumentedLeistungRepository (+4 more)

### Community 48 - "domain/seminar_termin.rs"
Cohesion: 0.21
Nodes (20): absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient() (+12 more)

### Community 50 - "common.rs"
Cohesion: 0.20
Nodes (23): format_datetime(), menge_to_str(), parse_datetime(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis() (+15 more)

### Community 51 - "use_cases/abrechnung.rs"
Cohesion: 0.22
Nodes (8): BehandlungErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler, ProduktErstellenFehler, RechnungAlsBezahltMarkierenFehler, TagesabschlussDurchführenFehler, DerefMut

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (44): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+36 more)

### Community 53 - "Seminar"
Cohesion: 0.14
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - ".try_from"
Cohesion: 0.27
Nodes (8): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Result, Self, TryFrom, Uuid

### Community 55 - "Produkt"
Cohesion: 0.12
Nodes (8): NeuesProdukt, preis(), Produkt, ProduktFehler, Into, ResultReport, Self, Uuid

### Community 56 - "types.ts"
Cohesion: 0.11
Nodes (12): components, Adresse, Behandlung, BehandlungErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung (+4 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.18
Nodes (11): Decision log, Domain terms, logandaim.at — Product Charter, Milestones, MVP scope, Non-goals for MVP, Open questions (not blocking M0), Problem (+3 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "RepositoryResult"
Cohesion: 0.08
Nodes (17): BehandlungRepository, HaustierRepository, RechnungRepository, RepositoryError, Behandlung, Haustier, NaiveDate, Option (+9 more)

### Community 60 - "charter.md"
Cohesion: 0.27
Nodes (3): Changed, Changelog, [Unreleased]

### Community 61 - "Json"
Cohesion: 0.19
Nodes (13): InternalServerError, C, From, ObjectStream, Report, Result, StatusCode, T (+5 more)

### Community 62 - "base_app_builder"
Cohesion: 0.18
Nodes (33): base_app_builder(), AppBuilder, SetUowProvider, abgehalten_maps_every_confirmed_buchung(), abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen(), absage_blockiert_abgehalten(), adresse(), aktualisieren_blockiert_nach_abgehalten() (+25 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.14
Nodes (26): BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, Error, From, NaiveDate (+18 more)

### Community 64 - ".start"
Cohesion: 0.12
Nodes (16): Box, Default, Future, Output, Pin, Result, S, SchedulerError (+8 more)

### Community 65 - "cn"
Cohesion: 0.07
Nodes (52): useAlleLeistungenQuery(), RechnungStatus, Bezahlt, Offen, AbrechnungPanel(), KatalogTab, TabButton(), KlientDetail() (+44 more)

### Community 67 - "blank_pdf_renderer.rs"
Cohesion: 0.36
Nodes (5): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.20
Nodes (10): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich), 4.1 Für Casual User, 4.2 Für Power-User & Turnierschützen, 4.3 Für Anlagenbetreiber (B2B & Vereinsnutzen), 4. Erweiterungsvorschläge (Feature Roadmap nach Rollen), 5. Technische Architektur & Systemanforderungen (Übersicht) (+2 more)

### Community 69 - "yams-core"
Cohesion: 0.47
Nodes (9): molting, yams-api, yams-core, yams-fakes, yams-filesystemstore, yams-scheduler, yams-server, yams-sqlite (+1 more)

### Community 70 - "react"
Cohesion: 0.10
Nodes (44): Klient, HaustierCreateFormProps, KlientRegisterFormProps, LeistungBuchenFormProps, LeistungQuelle, TerminBuchungFormProps, Alert(), AlertProps (+36 more)

### Community 71 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 72 - "SeminarId"
Cohesion: 0.15
Nodes (9): SeminarId, Geplant, NeuerSeminarTermin, Adresse, Option, S, SeminarOrt, SeminarTerminIn (+1 more)

### Community 73 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 74 - "BehandlungId"
Cohesion: 0.23
Nodes (11): BehandlungId, behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row (+3 more)

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

### Community 84 - "ProduktId"
Cohesion: 0.25
Nodes (11): ProduktId, produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row (+3 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (28): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+20 more)

### Community 89 - "HaustierErstellen"
Cohesion: 0.19
Nodes (12): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, Result (+4 more)

### Community 92 - "SeminarTerminId"
Cohesion: 0.11
Nodes (33): SeminarTerminId, buchung_umsatz(), BuchungUmsatz, Error, NaiveDate, Option, Report, Result (+25 more)

### Community 93 - "SQLiteUnitOfWork"
Cohesion: 0.21
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

### Community 102 - "String"
Cohesion: 0.12
Nodes (30): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+22 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.07
Nodes (51): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useSeminarBuchungAnlegenMutation(), useSeminarBuchungStornierenMutation() (+43 more)

### Community 104 - "instrumented.rs"
Cohesion: 0.20
Nodes (10): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, Pin (+2 more)

### Community 105 - "PdfDokument"
Cohesion: 0.14
Nodes (21): Klientbericht, PdfDokument, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse (+13 more)

### Community 106 - "Behandlung"
Cohesion: 0.70
Nodes (4): Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "StatusCode"
Cohesion: 0.05
Nodes (36): AuflistenFehler, BehandlungErstellenFehler, HaustierErstellenFehler, HttpStatusMapping, KlientErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler (+28 more)

### Community 108 - "Klient"
Cohesion: 0.22
Nodes (11): KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate, Uuid (+3 more)

### Community 110 - "Produkt"
Cohesion: 0.60
Nodes (4): Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "KlientRegisterForm"
Cohesion: 0.17
Nodes (6): useKlientErstellenMutation(), getYamsApi(), defaultKlient(), KlientRegisterForm(), defaultKlient(), KlientForm()

### Community 112 - "cases/abrechnung.rs"
Cohesion: 0.23
Nodes (17): AbrechnungSetup, app_with_pdf_fakes(), menge(), mwst_19(), rechnung_kann_als_bezahlt_markiert_werden(), Arc, Klient, NaiveDate (+9 more)

### Community 113 - "HaustierId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "navigation.ts"
Cohesion: 0.11
Nodes (22): KlientDetailContent(), AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar() (+14 more)

### Community 116 - "KlientErstellen"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate (+1 more)

### Community 117 - "Versioned"
Cohesion: 0.10
Nodes (22): Versioned, FakeHaustiereRepository, FakeKlientenRepository, FakeRechnungenRepository, FakeSeminarTermineRepository, Haustier, Klient, NaiveDate (+14 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "KlientErstellung"
Cohesion: 0.20
Nodes (10): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Result (+2 more)

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "provider.tsx"
Cohesion: 0.26
Nodes (11): createYamsApi(), normalizeApiBaseUrl(), resetYamsApiCache(), YamsApiContext, YamsApiContextValue, YamsApiProvider(), bootstrap(), FrontendConfig (+3 more)

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

### Community 126 - "App"
Cohesion: 0.26
Nodes (8): App, Arc, Box, Context, F, ResultReport, O, U

### Community 127 - "UnitOfWorkImpl"
Cohesion: 0.08
Nodes (21): main(), Box, RepositoryResult, Unimplemented, Arc<T>, LockedUnitOfWorkImpl, Box, Option (+13 more)

### Community 128 - "BehandlungCreateForm"
Cohesion: 1.00
Nodes (3): BehandlungCreateForm(), handleSubmit(), emptyBehandlung()

### Community 129 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 130 - "YamsApiSpec"
Cohesion: 0.22
Nodes (9): Path, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarTerminErstellung, SeminarUmsatzVorschau, Uuid (+1 more)

### Community 131 - "main"
Cohesion: 0.16
Nodes (14): BackendServerError, catch_panic(), main(), Report, Result, SchedulerError, SchedulerReport, SchedulerTask (+6 more)

### Community 133 - "3. Kernanforderungen (Funktionale Spezifikation)"
Cohesion: 0.40
Nodes (5): 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation)

### Community 134 - ".haustier_erstellen"
Cohesion: 0.36
Nodes (6): HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "Versioned<T>"
Cohesion: 0.10
Nodes (22): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+14 more)

### Community 136 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 137 - "dependencies"
Cohesion: 0.25
Nodes (8): dependencies, lucide-react, next, openapi-fetch, react, react-dom, @tanstack/react-query, @tanstack/react-query-devtools

### Community 138 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 139 - "providers.tsx"
Cohesion: 0.70
Nodes (3): Providers(), createQueryClient(), @tanstack/react-query

### Community 140 - "TypicalJsonResponse"
Cohesion: 0.11
Nodes (13): Behandlung, BehandlungErstellung, Haustier, HaustierErstellung, Klient, KlientErstellung, Produkt, ProduktErstellung (+5 more)

### Community 141 - "seminar-overview.tsx"
Cohesion: 0.09
Nodes (30): useRechnungAlsBezahltMarkierenMutation(), useSeminarTerminPlanenMutation(), RechnungBezahltButton(), RechnungBezahltButtonProps, SeminarOverview(), statusBadgeVariant(), TerminFilter, buildTerminEventsByDay() (+22 more)

### Community 142 - "Zeitraum"
Cohesion: 0.19
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "FakeUnitOfWork"
Cohesion: 0.27
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 145 - "SQLiteStateStore"
Cohesion: 0.06
Nodes (39): Cli, main(), Option, PathBuf, Report, Result, SchedulerMainError, init_tracing() (+31 more)

### Community 148 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, SeminarUmsatzPrognoseBisDatumFehler

### Community 149 - "SeminarCreateForm"
Cohesion: 1.00
Nodes (3): emptySeminar(), SeminarCreateForm(), handleSubmit()

### Community 150 - "Menge"
Cohesion: 0.24
Nodes (5): Menge, MengeFehler, Decimal, Result, Self

### Community 151 - "ExecutionContext"
Cohesion: 0.22
Nodes (11): ExecutionContext, ExecutionSource, Arc, BehandlungErstellen, ProduktErstellen, RechnungAlsBezahltMarkieren, Behandlung, Error (+3 more)

### Community 152 - "http-client.ts"
Cohesion: 0.27
Nodes (5): ApiError, JsonClient, paths, KlientErstellung, StructuredError

### Community 154 - "Adresse"
Cohesion: 0.23
Nodes (9): Adresse, domain::Adresse, Ländercode, Error, Example, From, Result, Self (+1 more)

### Community 155 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

### Community 156 - "Rechnung"
Cohesion: 0.22
Nodes (3): Rechnung, RechnungBezahltMarkieren, TagesabschlussErstellung

### Community 157 - "package.json"
Cohesion: 0.13
Nodes (14): name, private, version, babel-plugin-react-compiler, @biomejs/biome, openapi-fetch, openapi-typescript, react-dom (+6 more)

### Community 159 - "HaustierCreateForm"
Cohesion: 0.67
Nodes (3): emptyHaustier(), HaustierCreateForm(), handleSubmit()

### Community 160 - ".leistung_aus_behandlung_buchen"
Cohesion: 0.29
Nodes (4): Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung

### Community 161 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 162 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 163 - "Ratio"
Cohesion: 0.17
Nodes (5): Ratio, RatioFehler, Decimal, Result, Self

### Community 164 - "ProduktCreateForm"
Cohesion: 1.00
Nodes (3): emptyProdukt(), ProduktCreateForm(), handleSubmit()

### Community 165 - "ObjectStoreError"
Cohesion: 0.27
Nodes (8): collect_object(), ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport, Vec

### Community 168 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 169 - "preis.rs"
Cohesion: 0.26
Nodes (9): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal (+1 more)

### Community 170 - "InstrumentedUnitOfWork"
Cohesion: 0.12
Nodes (6): InstrumentedUnitOfWork, Box, RepositoryResult, Self, ProduktRepository, Produkt

### Community 171 - "LeistungId"
Cohesion: 0.12
Nodes (18): LeistungId, Abgehalten, Abgesagt, DateTime, From, FxHashMap, Into, Self (+10 more)

### Community 172 - "openapi_service"
Cohesion: 0.25
Nodes (7): openapi_service(), Into, IntoIterator, Item, Self, OpenApiService, ServerObject

### Community 174 - "SeminarBuchungForm"
Cohesion: 1.00
Nodes (3): defaultBuchung(), SeminarBuchungForm(), handleSubmit()

### Community 177 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 189 - "mwst_19"
Cohesion: 0.83
Nodes (3): mwst_19(), test_produkt_erstellen(), test_produkt_erstellen_rejects_empty_name()

## Knowledge Gaps
- **326 isolated node(s):** `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+321 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 862 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **24 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `.haustier_erstellen`, `Versioned<T>`, `FakeObjectStore`, `termin_from_parts`, `Preis`, `FakeUnitOfWork`, `SQLiteStateStore`, `domain/kontakt.rs`, `SQLiteRechnungRepository`, `ExecutionContext`, `tests.rs`, `Adresse`, `document_text`, `v0004_leistungen_quelle_mwst.rs`, `Rechnungsposition`, `EmailAdresse`, `UseCase`, `KlientId`, `NeueLeistung`, `ObjectStore`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `LeistungId`, `common.rs`, `base_app_builder`, `Seminar`, `.try_from`, `Produkt`, `StructuredError`, `Json`, `schema/seminar.rs`, `SeminarId`, `requests/abrechnung.rs`, `HaustierErstellen`, `SeminarTerminId`, `PdfDokument`, `Behandlung`, `Klient`, `Produkt`, `HaustierId`, `KlientErstellen`, `KlientErstellung`, `Rechnung`, `leistung_from_row`, `yams_scheduler/config.rs`?**
  _High betweenness centrality (0.239) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `.start`, `YamsApiSpec`, `ObjectStore`, `Uuid`, `YamsAppApi`, `openapi_service`, `run`, `Clock`, `cases/abrechnung.rs`, `base_app_builder`, `PdfRenderer`, `base_app_builder`, `UnitOfWorkImpl`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._
- **Why does `ObjectStore` connect `ObjectStore` to `ObjectStoreError`, `Versioned<T>`, `instrumented.rs`, `FakeObjectStore`, `yams-filesystemstore/src/lib.rs`, `ExecutionContext`, `PdfRenderer`, `App`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler` to the rest of the system?**
  _326 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._