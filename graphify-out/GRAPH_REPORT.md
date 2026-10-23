# Graph Report - logandaim.at  (2026-10-23)

## Corpus Check
- 257 files · ~92,990 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3318 nodes · 8047 edges · 182 communities (154 shown, 19 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 207 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `99a8e5f5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- arc_up
- devDependencies
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- SeminarTerminId
- apply_up_migrations
- Vec
- termin_from_parts
- yams-filesystemstore/src/lib.rs
- Migration
- compilerOptions
- schema.ts
- Clock
- BehandlungId
- Make invalid state unrepresentable
- E
- Datenschutz — Data protection, privacy, and legal obligations
- String
- SQLiteRechnungRepository
- wertung.rs
- behandlung_from_row
- <Title>
- tests.rs
- Betrieb — Operations and launch readiness
- layout.tsx
- ExecutionContext
- Wertung — Classic 3-arrow scoring
- v0004_leistungen_quelle_mwst.rs
- Konto — Accounts and login
- EmailAdresse
- use_cases/listen.rs
- RechnungId
- NeuerKlient
- LeistungId
- FakePdfRenderer
- ObjectStore
- YamsAppApi
- FixedClock
- AppliableMigration
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- run
- FakeDatastore
- .contextualize_with
- RepoStorage
- domain/seminar_termin.rs
- common.rs
- PdfDokument
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
- cases/seminar.rs
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
- produkt_from_row
- diffx-finish-review
- Migration
- requests/abrechnung.rs
- Next.js
- postcss.config.mjs
- bad_request
- Migration
- Contributing and project management
- ADR-0001: Baseline architecture
- provider.tsx
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
- base_app_builder
- src/config.rs
- hooks/index.ts
- instrumented.rs
- PdfRenderer
- Behandlung
- StatusCode
- Klient
- Produkt
- teilnahme_dokument
- cases/abrechnung.rs
- KlientId
- navigation.ts
- HttpYamsApi
- KlientErstellen
- Versioned
- pull_request_template.md
- KlientErstellung
- Rechnung
- SeminarTermin
- leistung_from_row
- yams_scheduler/config.rs
- YAMS Frontend Migration Specification
- seminar_from_row
- App
- ProduktRepository
- behandlung-create-form.tsx
- migration_error_to_persistence_error
- YamsApiSpec
- main
- validation_error.rs
- SeminarTerminIn<S>
- .haustier_erstellen
- Versioned<T>
- FakeObjectStore
- dependencies
- scripts
- LeistungRepository
- Vec
- termin-calendar.tsx
- Zeitraum
- .seminar_umsatz_prognose
- FakeUnitOfWork
- SQLiteStateStore
- Json
- klient-detail.tsx
- Deferred capabilities
- seminar-create-form.tsx
- Rechnungsposition
- UseCase
- blank_pdf_renderer.rs
- ADR-0002: Session cookie auth across frontend and API origins
- Adresse
- m1-overseer.md
- Modes
- package.json
- Preis
- LeistungIn<S>
- m1-reviewer.md
- [Unreleased]
- document_text
- Ratio
- page-chrome.tsx
- ObjectStoreError
- .rechnung_als_bezahlt_markieren
- RechnungIn<S>
- SeminarOverview
- preis.rs
- InstrumentedUnitOfWork
- .als_abgehalten
- openapi_service
- RowActionsMenu
- SeminarBuchungForm
- .perform
- .produkt_erstellen
- KlientRegisterForm
- mise
- UnitOfWorkImpl

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

## Communities (182 total, 19 thin omitted)

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

### Community 6 - "SeminarTerminId"
Cohesion: 0.16
Nodes (13): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), S, Self, Seminar, Uuid, Vec, SeminarBuchung (+5 more)

### Community 7 - "apply_up_migrations"
Cohesion: 0.23
Nodes (8): apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option, DoubleEndedIterator

### Community 8 - "Vec"
Cohesion: 0.16
Nodes (9): AuflistenFehler, Haustier, Produkt, ProduktErstellung, Seminar, SeminarErstellung, Vec, ProduktErstellenFehler (+1 more)

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
Cohesion: 0.08
Nodes (23): components, $defs, LeistungQuelle_LeistungQuelleBehandlungTyp, Behandlung, LeistungQuelle_LeistungQuelleManuellTyp, Manuell, LeistungQuelle_LeistungQuelleProduktTyp, Produkt (+15 more)

### Community 14 - "Clock"
Cohesion: 0.14
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 15 - "BehandlungId"
Cohesion: 0.13
Nodes (10): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+2 more)

### Community 16 - "Make invalid state unrepresentable"
Cohesion: 0.12
Nodes (15): Closed enums, Creation input and construction, Derived values, Domain model, Encapsulation, Errors, Identity newtypes, Layout (+7 more)

### Community 17 - "E"
Cohesion: 0.18
Nodes (18): ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+10 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "String"
Cohesion: 0.14
Nodes (16): From, Self, String, email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits() (+8 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.22
Nodes (15): geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (17): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), Punkte, Add, I, IntoIterator, Item (+9 more)

### Community 22 - "behandlung_from_row"
Cohesion: 0.25
Nodes (10): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 23 - "<Title>"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+5 more)

### Community 24 - "tests.rs"
Cohesion: 0.21
Nodes (33): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), generate(), hof_adresse(), klient(), klient_named() (+25 more)

### Community 25 - "Betrieb — Operations and launch readiness"
Cohesion: 0.14
Nodes (14): Abuse, Backups, Betrieb — Operations and launch readiness, Decision log, Domain terms and business rules, Outcome, Problem, Revision history (+6 more)

### Community 26 - "layout.tsx"
Cohesion: 0.21
Nodes (8): nextConfig, geistMono, geistSans, metadata, Providers(), createQueryClient(), next, @tanstack/react-query

### Community 27 - "ExecutionContext"
Cohesion: 0.23
Nodes (11): ExecutionContext, ExecutionSource, Arc, Error, Report, Result, SeminarTermin, SeminarBuchungAnlegen (+3 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.23
Nodes (21): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), converts_percentage_mwst_to_ratio(), idempotent_when_already_ratio() (+13 more)

### Community 30 - "Konto — Accounts and login"
Cohesion: 0.14
Nodes (14): Acceptance criteria, Decision log, Domain terms and business rules, Implementation sketches, Konto — Accounts and login, Outcome, Problem, Requirements (+6 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.21
Nodes (10): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Error, Example, From, Result (+2 more)

### Community 32 - "use_cases/listen.rs"
Cohesion: 0.14
Nodes (16): AlleBehandlungenAuflisten, AlleHaustiereAuflisten, AlleKlientenAuflisten, AlleLeistungenAuflisten, AlleProdukteAuflisten, AlleRechnungenAuflisten, AlleSeminareAuflisten, AlleSeminarTermineAuflisten (+8 more)

### Community 33 - "RechnungId"
Cohesion: 0.12
Nodes (29): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+21 more)

### Community 34 - "NeuerKlient"
Cohesion: 0.11
Nodes (17): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, neu(), NeuerKlient, Adresse (+9 more)

### Community 35 - "LeistungId"
Cohesion: 0.13
Nodes (20): Abgerechnet, Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungId, LeistungIn, LeistungOffen (+12 more)

### Community 36 - "FakePdfRenderer"
Cohesion: 0.22
Nodes (6): FakePdfRenderer, Arc, Mutex, ResultReport, Self, Vec

### Community 37 - "ObjectStore"
Cohesion: 0.19
Nodes (20): ObjectStore, Send, Sync, mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen(), rechnung_key_uses_uuid() (+12 more)

### Community 38 - "YamsAppApi"
Cohesion: 0.12
Nodes (21): ObjectStoreError, ObjectStream, RepositoryError, ResultReport, Self, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage (+13 more)

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

### Community 45 - "FakeDatastore"
Cohesion: 0.07
Nodes (30): FakeBehandlungenRepository, FakeDatastore, FakeHaustiereRepository, FakeKlientenRepository, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, FakeSeminareRepository (+22 more)

### Community 46 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 47 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedBehandlungRepository, InstrumentedHaustierRepository, InstrumentedKlientRepository, InstrumentedLeistungRepository (+4 more)

### Community 48 - "domain/seminar_termin.rs"
Cohesion: 0.38
Nodes (14): absagen_archives_buchungen(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys(), als_abgehalten_rejects_incomplete_mapping(), buchung_anlegen_enforces_capacity(), buchung_anlegen_rejects_duplicate_klient(), klient() (+6 more)

### Community 50 - "common.rs"
Cohesion: 0.21
Nodes (24): format_datetime(), menge_to_str(), parse_datetime(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis() (+16 more)

### Community 51 - "PdfDokument"
Cohesion: 0.21
Nodes (13): Klientbericht, PdfDokument, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse, NaiveDate (+5 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (44): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+36 more)

### Community 53 - "SeminarId"
Cohesion: 0.14
Nodes (14): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+6 more)

### Community 54 - ".try_from"
Cohesion: 0.27
Nodes (8): HaustierErstellen, HaustierErstellung, Error, NaiveDate, Result, Self, TryFrom, Uuid

### Community 55 - "ProduktId"
Cohesion: 0.13
Nodes (10): produkt_betrag_multiplies_menge(), NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport (+2 more)

### Community 56 - "types.ts"
Cohesion: 0.07
Nodes (32): ApiError, JsonClient, yamsKeys, Adresse, Behandlung, BehandlungErstellung, Haustier, HaustierErstellung (+24 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "RepositoryResult"
Cohesion: 0.08
Nodes (17): BehandlungRepository, HaustierRepository, RechnungRepository, RepositoryError, Behandlung, Haustier, NaiveDate, Option (+9 more)

### Community 61 - "StreamBinaryResponse"
Cohesion: 0.15
Nodes (16): InternalServerError, C, From, ObjectStream, Report, Result, StatusCode, T (+8 more)

### Community 62 - "cases/seminar.rs"
Cohesion: 0.21
Nodes (28): abgehalten_maps_every_confirmed_buchung(), abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen(), absage_blockiert_abgehalten(), adresse(), aktualisieren_blockiert_nach_abgehalten(), aktualisieren_nur_geplant(), assert_offen_mwst(), kapazität_und_doppelbuchung() (+20 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.18
Nodes (21): NaiveDate, SeminarUmsatzPrognose, BuchungUmsatz, DateTime, Decimal, NaiveDate, Option, Utc (+13 more)

### Community 64 - ".start"
Cohesion: 0.12
Nodes (16): Box, Default, Future, Output, Pin, Result, S, SchedulerError (+8 more)

### Community 65 - "cn"
Cohesion: 0.09
Nodes (46): useAlleRechnungenQuery(), RechnungStatus, Bezahlt, Offen, KatalogTab, TabButton(), formatEuroSum(), KlientLeistungenSection() (+38 more)

### Community 67 - "SeminarOrt"
Cohesion: 0.24
Nodes (8): domain::SeminarOrt, Adresse, Error, From, Result, Self, TryFrom, SeminarOrt

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "yams-core"
Cohesion: 0.47
Nodes (9): molting, yams-api, yams-core, yams-fakes, yams-filesystemstore, yams-scheduler, yams-server, yams-sqlite (+1 more)

### Community 70 - "react"
Cohesion: 0.09
Nodes (50): useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), KlientRegisterFormProps, LeistungBuchenForm(), LeistungBuchenFormProps, LeistungQuelle, TerminFilter, TerminBuchungFormProps (+42 more)

### Community 71 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 72 - "Option"
Cohesion: 0.21
Nodes (8): aktualisieren_rejects_max_below_confirmed(), Geplant, NeuerSeminarTermin, Adresse, Option, SeminarOrt, SeminarTerminGeplant, zeitraum()

### Community 73 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 74 - "produkt_from_row"
Cohesion: 0.25
Nodes (10): produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row, Transaction (+2 more)

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
Cohesion: 0.10
Nodes (15): BehandlungErstellenFehler, bad_request(), Arc, Behandlung, BehandlungErstellung, C, Leistung, LeistungAusBehandlungErstellung (+7 more)

### Community 81 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 82 - "Contributing and project management"
Cohesion: 0.22
Nodes (9): Agents, Commits, Contributing and project management, Delivery gate, GitHub Project, Labels and priority, Ready gate, Records (+1 more)

### Community 83 - "ADR-0001: Baseline architecture"
Cohesion: 0.50
Nodes (4): ADR-0001: Baseline architecture, Consequences, Context, Decision

### Community 84 - "provider.tsx"
Cohesion: 0.26
Nodes (11): createYamsApi(), normalizeApiBaseUrl(), resetYamsApiCache(), YamsApiContext, YamsApiContextValue, YamsApiProvider(), bootstrap(), FrontendConfig (+3 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (28): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+20 more)

### Community 89 - "HaustierErstellen"
Cohesion: 0.19
Nodes (12): HaustierErstellen, HaustierErstellenFehler, Context, Error, Haustier, NaiveDate, Report, Result (+4 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.11
Nodes (26): buchung_umsatz(), BuchungUmsatz, NaiveDate, Option, Self, Seminar, SeminarOrt, TimeDelta (+18 more)

### Community 93 - "SQLiteUnitOfWork"
Cohesion: 0.19
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 99 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, InstanceType, Arc, AsRef, Connection, Deref, Mutex, Path (+9 more)

### Community 101 - "base_app_builder"
Cohesion: 0.16
Nodes (12): base_app_builder(), AppBuilder, SetUowProvider, test_tagesabschluss_empty_day_returns_no_rechnungen(), test_haustier(), mwst_19(), test_behandlung_erstellen(), test_behandlung_erstellen_rejects_empty_name() (+4 more)

### Community 102 - "src/config.rs"
Cohesion: 0.14
Nodes (27): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+19 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.08
Nodes (42): useLeistungManuellErfassenMutation(), useSeminarBuchungAnlegenMutation(), useSeminarBuchungStornierenMutation(), useSeminarTerminAbgehaltenMutation(), useSeminarTerminAbsagenMutation(), useSeminarTerminAktualisierenMutation(), useTagesabschlussDurchführenMutation(), requireApi() (+34 more)

### Community 104 - "instrumented.rs"
Cohesion: 0.22
Nodes (9): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, ResultReport (+1 more)

### Community 105 - "PdfRenderer"
Cohesion: 0.21
Nodes (6): ExecutionContext<'a>, RepositoryResult, Self, PdfRenderer, Send, Sync

### Community 106 - "Behandlung"
Cohesion: 0.70
Nodes (4): Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "StatusCode"
Cohesion: 0.05
Nodes (36): AuflistenFehler, BehandlungErstellenFehler, HaustierErstellenFehler, HttpStatusMapping, KlientErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler (+28 more)

### Community 108 - "Klient"
Cohesion: 0.19
Nodes (12): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+4 more)

### Community 110 - "Produkt"
Cohesion: 0.60
Nodes (4): Produkt, Decimal, Uuid, schema_produkt_from_domain()

### Community 111 - "teilnahme_dokument"
Cohesion: 0.24
Nodes (12): PraxisAngaben, klient_bericht(), rechnungsdokument(), Klient, S, Seminar, SeminarBuchung, teilnahme_dokument() (+4 more)

### Community 112 - "cases/abrechnung.rs"
Cohesion: 0.26
Nodes (15): AbrechnungSetup, app_with_pdf_fakes(), menge(), mwst_19(), rechnung_kann_als_bezahlt_markiert_werden(), Arc, Klient, NaiveDate (+7 more)

### Community 113 - "KlientId"
Cohesion: 0.16
Nodes (14): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+6 more)

### Community 114 - "navigation.ts"
Cohesion: 0.16
Nodes (16): AppShell(), Sidebar(), SidebarLink(), SidebarProps, BreadcrumbSegment, breadcrumbsForPathname(), homeNavItem, isExactPath() (+8 more)

### Community 116 - "KlientErstellen"
Cohesion: 0.22
Nodes (9): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Error, Klient, Mobilnummer, NaiveDate (+1 more)

### Community 117 - "Versioned"
Cohesion: 0.17
Nodes (13): KlientRepository, Klient, Versioned, klient_from_row(), Arc, Klient, Mutex, Option (+5 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "KlientErstellung"
Cohesion: 0.20
Nodes (10): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Error, Mobilnummer, NaiveDate, Result (+2 more)

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "SeminarTermin"
Cohesion: 0.21
Nodes (4): From, SeminarTermin, SeminarTerminAbgehalten, SeminarTerminAbgesagt

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
Cohesion: 0.23
Nodes (9): App, Box, Context, F, ResultReport, app_with_pdf_fakes(), Arc, O (+1 more)

### Community 128 - "behandlung-create-form.tsx"
Cohesion: 0.23
Nodes (9): useBehandlungErstellenMutation(), useProduktErstellenMutation(), BehandlungCreateForm(), handleSubmit(), emptyBehandlung(), emptyProdukt(), ProduktCreateForm(), handleSubmit() (+1 more)

### Community 129 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 130 - "YamsApiSpec"
Cohesion: 0.17
Nodes (13): Path, RechnungBezahltMarkieren, Seminar, SeminarBuchungErstellung, SeminarErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung (+5 more)

### Community 131 - "main"
Cohesion: 0.16
Nodes (14): BackendServerError, catch_panic(), main(), Report, Result, SchedulerError, SchedulerReport, SchedulerTask (+6 more)

### Community 133 - "SeminarTerminIn<S>"
Cohesion: 0.18
Nodes (3): Item, Iterator, SeminarTerminIn<S>

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

### Community 140 - "Vec"
Cohesion: 0.14
Nodes (8): Behandlung, BehandlungErstellung, Haustier, Klient, KlientErstellung, Rechnung, TagesabschlussErstellung, Vec

### Community 141 - "termin-calendar.tsx"
Cohesion: 0.12
Nodes (25): useRechnungAlsBezahltMarkierenMutation(), useSeminarTerminPlanenMutation(), RechnungBezahltButton(), RechnungBezahltButtonProps, buildTerminEventsByDay(), formatSelectedDate(), TerminCalendar(), TerminCalendarProps (+17 more)

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

### Community 146 - "Json"
Cohesion: 0.20
Nodes (6): HaustierErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Json

### Community 147 - "klient-detail.tsx"
Cohesion: 0.12
Nodes (12): useHaustierErstellenMutation(), KlientDetailContent(), emptyHaustier(), HaustierCreateForm(), handleSubmit(), HaustierCreateFormProps, KlientDetail(), KlientDetailProps (+4 more)

### Community 148 - "Deferred capabilities"
Cohesion: 0.18
Nodes (11): Account security, Administration, Auth audit, Deferred capabilities, Email change, Email delivery, Email verification, External identity providers (+3 more)

### Community 149 - "seminar-create-form.tsx"
Cohesion: 0.36
Nodes (5): useSeminarErstellenMutation(), emptySeminar(), SeminarCreateForm(), handleSubmit(), SeminarForm()

### Community 150 - "Rechnungsposition"
Cohesion: 0.16
Nodes (7): Menge, MengeFehler, Decimal, Result, Self, position_from_leistung(), Rechnungsposition

### Community 151 - "UseCase"
Cohesion: 0.12
Nodes (24): UseCase, BehandlungErstellen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen (+16 more)

### Community 152 - "blank_pdf_renderer.rs"
Cohesion: 0.36
Nodes (5): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.25
Nodes (8): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Consequences, Context, Data model (sketch), Decision

### Community 154 - "Adresse"
Cohesion: 0.23
Nodes (9): Adresse, domain::Adresse, Ländercode, Error, Example, From, Result, Self (+1 more)

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "package.json"
Cohesion: 0.13
Nodes (14): name, private, version, babel-plugin-react-compiler, @biomejs/biome, openapi-fetch, openapi-typescript, react-dom (+6 more)

### Community 158 - "Preis"
Cohesion: 0.17
Nodes (5): Preis, Add, Output, Self, Mul

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "[Unreleased]"
Cohesion: 0.50
Nodes (4): Added, Changed, Changelog, [Unreleased]

### Community 162 - "document_text"
Cohesion: 0.50
Nodes (4): collect_frame_text(), document_text(), Frame, PagedDocument

### Community 163 - "Ratio"
Cohesion: 0.18
Nodes (5): Ratio, RatioFehler, Decimal, Result, Self

### Community 164 - "page-chrome.tsx"
Cohesion: 0.32
Nodes (6): AppChrome(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome()

### Community 165 - "ObjectStoreError"
Cohesion: 0.27
Nodes (8): collect_object(), ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport, Vec

### Community 166 - ".rechnung_als_bezahlt_markieren"
Cohesion: 0.29
Nodes (5): Rechnung, RechnungBezahltMarkieren, TagesabschlussErstellung, RechnungAlsBezahltMarkierenFehler, TagesabschlussDurchführenFehler

### Community 169 - "preis.rs"
Cohesion: 0.26
Nodes (9): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal (+1 more)

### Community 170 - "InstrumentedUnitOfWork"
Cohesion: 0.18
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 171 - ".als_abgehalten"
Cohesion: 0.24
Nodes (10): Abgehalten, Abgesagt, DateTime, FxHashMap, Into, ResultReport, Utc, SeminarBuchungStatus (+2 more)

### Community 172 - "openapi_service"
Cohesion: 0.12
Nodes (12): main(), Box, RepositoryResult, Unimplemented, openapi_service(), Into, IntoIterator, Item (+4 more)

### Community 174 - "SeminarBuchungForm"
Cohesion: 1.00
Nodes (3): defaultBuchung(), SeminarBuchungForm(), handleSubmit()

### Community 175 - ".perform"
Cohesion: 0.40
Nodes (3): Context, Output, ResultReport

### Community 179 - "KlientRegisterForm"
Cohesion: 0.17
Nodes (6): useKlientErstellenMutation(), getYamsApi(), defaultKlient(), KlientRegisterForm(), defaultKlient(), KlientForm()

### Community 184 - "UnitOfWorkImpl"
Cohesion: 0.06
Nodes (26): Arc, Arc<T>, LockedUnitOfWorkImpl, Box, C, F, Formatter, Future (+18 more)

## Knowledge Gaps
- **390 isolated node(s):** `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+385 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 926 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **19 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `.haustier_erstellen`, `Versioned<T>`, `FakeObjectStore`, `termin_from_parts`, `BehandlungId`, `FakeUnitOfWork`, `SQLiteStateStore`, `Json`, `SQLiteRechnungRepository`, `Rechnungsposition`, `UseCase`, `tests.rs`, `Adresse`, `ExecutionContext`, `v0004_leistungen_quelle_mwst.rs`, `EmailAdresse`, `NeuerKlient`, `LeistungId`, `document_text`, `ObjectStore`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `.als_abgehalten`, `common.rs`, `PdfDokument`, `base_app_builder`, `SeminarId`, `.try_from`, `ProduktId`, `StructuredError`, `StreamBinaryResponse`, `schema/seminar.rs`, `SeminarOrt`, `Option`, `requests/abrechnung.rs`, `HaustierErstellen`, `use_cases/seminar.rs`, `src/config.rs`, `Behandlung`, `Klient`, `Produkt`, `teilnahme_dokument`, `KlientId`, `KlientErstellen`, `KlientErstellung`, `Rechnung`, `leistung_from_row`, `yams_scheduler/config.rs`?**
  _High betweenness centrality (0.223) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `.start`, `YamsApiSpec`, `ObjectStore`, `YamsAppApi`, `PdfRenderer`, `openapi_service`, `run`, `Clock`, `bad_request`, `cases/abrechnung.rs`, `base_app_builder`, `UnitOfWorkImpl`, `cases/seminar.rs`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **Why does `ObjectStore` connect `ObjectStore` to `ObjectStoreError`, `Versioned<T>`, `instrumented.rs`, `PdfRenderer`, `FakeObjectStore`, `yams-filesystemstore/src/lib.rs`, `ExecutionContext`, `App`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler` to the rest of the system?**
  _390 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._
