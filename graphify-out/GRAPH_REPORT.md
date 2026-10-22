# Graph Report - logandaim.at  (2026-10-22)

## Corpus Check
- 344 files · ~364,324 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3784 nodes · 9678 edges · 190 communities (155 shown, 25 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 239 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `eddd40ba`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .add_dyn
- TestError
- devDependencies
- biome.json
- yams-persistence
- Klient
- src-tauri/src/config.rs
- EventForm.tsx
- String
- termin_from_parts
- yams-filesystemstore/src/lib.rs
- UpMigration
- compilerOptions
- Versioned
- .leistung_aus_behandlung_buchen
- BehandlungId
- api/types.ts
- E
- provider.tsx
- domain/kontakt.rs
- SQLiteRechnungRepository
- Versioned<T>
- src/sqlite_state_store.rs
- libs/database/index.ts
- tests.rs
- Clock
- makeRecordForTable
- Seminar — Domain-Spezifikation
- src-tauri/src/tracing_setup.rs
- v0004_leistungen_quelle_mwst.rs
- LeistungOffen
- EmailAdresse
- use_cases/listen.rs
- RechnungId
- KlientId
- NeueLeistung
- FakePdfRenderer
- service/pdf.rs
- ResultReport
- FixedClock
- .get_current_version
- requests/seminar.rs
- domain/adresse.rs
- schema/leistung.rs
- run
- FakeDatastore
- NotificationType
- RepoStorage
- LeistungId
- common.rs
- PdfDokument
- base_app_builder
- Seminar
- HaustierErstellung
- ProduktId
- YamsApi
- api/client.ts
- StructuredError
- RechnungRepository
- arc_up
- Json
- base_app_builder
- schema/seminar.rs
- .start
- cn
- Release Workflow
- frontend-legacy/api/schema.d.ts
- TauriYamsApi
- yams-core
- react
- <Title>
- SeminarId
- SQLiteHaustierRepository
- Wertung — Classic 3-arrow scoring
- diffx-finish-review
- index.d.ts
- requests/abrechnung.rs
- Next.js
- postcss.config.mjs
- bad_request
- Built on SurrealDB
- YAMS Banner SVG
- YAMS Logo SVG
- yams-typstreports/src/lib.rs
- HaustierErstellen
- use_cases/seminar.rs
- SQLiteUnitOfWork
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- Product Requirements Document (PRD) & Konzeptbericht
- SQLiteInstance
- mwst_19
- server/src/config.rs
- hooks/index.ts
- PdfRenderer
- useStore
- .behandlung_erstellen
- StatusCode
- Klient
- .produkt_erstellen
- logandaim.at — Product Charter
- cases/abrechnung.rs
- HaustierId
- navigation.ts
- HttpYamsApi
- KlientErstellen
- SQLiteKlientRepository
- notification.ts
- KlientErstellung
- Rechnung
- termin-detail-panel.tsx
- leistung_from_row
- yams_scheduler/config.rs
- produkt_from_row
- seminar_from_row
- behandlung_from_row
- UnitOfWorkImpl
- BehandlungCreateForm
- .contextualize_with
- YamsApiSpec
- main
- validation_error.rs
- Debug
- .haustier_erstellen
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
- LeftMenuLayout.tsx
- participation/index.tsx
- ExecutionContext
- SeminarCreateForm
- Menge
- Contributing and project management
- yams_scheduler/main.rs
- App
- Adresse
- stores/index.tsx
- SchedulerTask
- package.json
- Preis
- HaustierCreateForm
- arc_down
- 0001-baseline-architecture.md
- charter.md
- Ratio
- Haustier
- ObjectStore
- wal_connection_race.rs
- DatabaseConnection
- Seminar
- preis.rs
- SQLiteInstance
- SeminarTerminId
- openapi_service
- RowActionsMenu
- Specifications
- .teilnahmebestätigung_pdf
- StreamBody
- next
- Klient
- mise
- ProduktErstellung
- pull_request_template.md
- YAMS Frontend Migration Specification
- 3. Kernanforderungen (Funktionale Spezifikation)
- src/errors.rs
- mwst_19
- test_haustier
- test_klient

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 125 edges
2. `react` - 84 edges
3. `KlientId` - 77 edges
4. `YamsAppApi` - 72 edges
5. `Preis` - 56 edges
6. `Ratio` - 56 edges
7. `useStore()` - 52 edges
8. `useYamsApiReady()` - 44 edges
9. `cn()` - 43 edges
10. `query()` - 42 edges

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
- 3-file cycle: `frontend-legacy/libs/notification.ts -> frontend-legacy/stores/index.tsx -> frontend-legacy/stores/notificationStore.ts -> frontend-legacy/libs/notification.ts`

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (190 total, 25 thin omitted)

### Community 0 - ".add_dyn"
Cohesion: 0.18
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 1 - "TestError"
Cohesion: 0.11
Nodes (16): AppliedLog, Cell, AppliableMigration, FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Box, Display (+8 more)

### Community 2 - "devDependencies"
Cohesion: 0.18
Nodes (11): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @tauri-apps/cli, @types/node (+3 more)

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Klient"
Cohesion: 0.07
Nodes (37): Energetik Logo (legacy), Backend ER Model, Adresse, Behandlung, Behandlungsart, BehandlungsTermin, Buchungsart, Energetik Sabine Petschl (+29 more)

### Community 6 - "src-tauri/src/config.rs"
Cohesion: 0.10
Nodes (50): ConfigError, default_config_path(), default_log_dir(), DeploymentMode, deserializes_embedded_file(), deserializes_log_dir_from_file(), deserializes_log_dir_from_toml_file(), deserializes_remote_file() (+42 more)

### Community 7 - "EventForm.tsx"
Cohesion: 0.10
Nodes (44): clientRegisterAddressData(), clientRegisterAddressDataFromAddress(), emptyClientRegisterAddressFieldData, NewAddressFormData, RegisterAddressForm, EditClientForm, clientSearched(), AddClientForm (+36 more)

### Community 8 - "String"
Cohesion: 0.11
Nodes (66): YamsAppApi, From, String, alle_behandlungen(), alle_haustiere(), alle_klienten(), alle_leistungen(), alle_produkte() (+58 more)

### Community 9 - "termin_from_parts"
Cohesion: 0.16
Nodes (27): format_datetime(), parse_datetime(), DateTime, Utc, insert_params(), leistung_id_for(), load_buchungen(), load_termin() (+19 more)

### Community 10 - "yams-filesystemstore/src/lib.rs"
Cohesion: 0.14
Nodes (26): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+18 more)

### Community 11 - "UpMigration"
Cohesion: 0.08
Nodes (15): Migration, Option, Transaction, Sync, UpMigration, Migration, Option, Transaction (+7 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "Versioned"
Cohesion: 0.10
Nodes (14): HaustierRepository, KlientRepository, Behandlung, Haustier, Klient, Leistung, NaiveDate, Produkt (+6 more)

### Community 14 - ".leistung_aus_behandlung_buchen"
Cohesion: 0.25
Nodes (4): Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung

### Community 15 - "BehandlungId"
Cohesion: 0.17
Nodes (9): Behandlung, BehandlungFehler, BehandlungId, NeueBehandlung, preis(), Into, ResultReport, Self (+1 more)

### Community 16 - "api/types.ts"
Cohesion: 0.12
Nodes (14): ApiError, JsonClient, Adresse, Behandlung, BehandlungErstellung, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung (+6 more)

### Community 17 - "E"
Cohesion: 0.14
Nodes (23): apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>> (+15 more)

### Community 18 - "provider.tsx"
Cohesion: 0.17
Nodes (17): createYamsApi(), normalizeApiBaseUrl(), resetYamsApiCache(), YamsApiContext, YamsApiContextValue, YamsApiProvider(), bootstrap(), BaseConfig (+9 more)

### Community 19 - "domain/kontakt.rs"
Cohesion: 0.16
Nodes (11): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, AsRef (+3 more)

### Community 20 - "SQLiteRechnungRepository"
Cohesion: 0.22
Nodes (15): geladene_rechnung_from_parts(), parse_position_from_row(), parse_rechnung_header(), RechnungRowData, Arc, Mutex, NaiveDate, Option (+7 more)

### Community 21 - "Versioned<T>"
Cohesion: 0.10
Nodes (13): C, Clone, Deref, F, Formatter, Future, Output, Report (+5 more)

### Community 22 - "src/sqlite_state_store.rs"
Cohesion: 0.19
Nodes (12): format_optional_timestamp(), libsql_error_to_store_error(), parse_optional_timestamp(), Box, DateTime, Future, Option, Output (+4 more)

### Community 23 - "libs/database/index.ts"
Cohesion: 0.05
Nodes (24): config(), defaultConfig, FrontendConfig, loadPromise, TauriType, CompatibilityResult, db, live() (+16 more)

### Community 24 - "tests.rs"
Cohesion: 0.17
Nodes (38): adresse(), assert_absent(), assert_contains(), assert_valid_pdf(), collect_frame_text(), document_text(), generate(), hof_adresse() (+30 more)

### Community 25 - "Clock"
Cohesion: 0.10
Nodes (13): DateTime, NaiveDate, Utc, SystemClock, ExecutionContext<'a>, RepositoryResult, Self, Clock (+5 more)

### Community 26 - "makeRecordForTable"
Cohesion: 0.07
Nodes (21): createSeminar(), ensureSeminar(), Address, AddressResponse, Client, ClientResponse, DatabaseObject, DatabaseResponse (+13 more)

### Community 27 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 29 - "v0004_leistungen_quelle_mwst.rs"
Cohesion: 0.22
Nodes (19): adds_quelle_mwst_when_neither_column_exists(), align_column(), already_aligned_schema_is_noop(), apply(), column_exists(), columns(), converts_percentage_mwst_to_ratio(), idempotent_when_already_ratio() (+11 more)

### Community 30 - "LeistungOffen"
Cohesion: 0.15
Nodes (16): LeistungOffen, BehandlungErstellenFehler, LeistungAusBehandlungBuchen, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchen, LeistungAusProduktBuchenFehler, LeistungManuellErfassen, LeistungManuellErfassenFehler (+8 more)

### Community 31 - "EmailAdresse"
Cohesion: 0.23
Nodes (8): domain::EmailAdresse, domain::Mobilnummer, EmailAdresse, Mobilnummer, Example, From, Self, TryFrom

### Community 32 - "use_cases/listen.rs"
Cohesion: 0.16
Nodes (14): AlleBehandlungenAuflisten, AlleHaustiereAuflisten, AlleKlientenAuflisten, AlleLeistungenAuflisten, AlleProdukteAuflisten, AlleRechnungenAuflisten, AlleSeminareAuflisten, AlleSeminarTermineAuflisten (+6 more)

### Community 33 - "RechnungId"
Cohesion: 0.12
Nodes (28): aus_leistungen_maps_seminar_quelle(), aus_leistungen_rejects_klient_mismatch(), aus_leistungen_skips_already_abgerechnet(), Bezahlt, leistung_offen(), mwst_19(), Offen, position() (+20 more)

### Community 34 - "KlientId"
Cohesion: 0.11
Nodes (18): Klient, klient_rejects_empty_vorname(), klient_rejects_invalid_email_with_attach(), klient_rejects_invalid_mobilnummer_with_attach(), KlientFehler, KlientId, neu(), NeuerKlient (+10 more)

### Community 35 - "NeueLeistung"
Cohesion: 0.09
Nodes (21): Abgerechnet, behandlung_betrag_uses_snapshot_preis(), Leistung, leistung_rejects_empty_beschreibung(), LeistungAbgerechnet, LeistungFehler, LeistungIn, LeistungIn<S> (+13 more)

### Community 36 - "FakePdfRenderer"
Cohesion: 0.22
Nodes (6): FakePdfRenderer, Arc, Mutex, ResultReport, Self, Vec

### Community 37 - "service/pdf.rs"
Cohesion: 0.19
Nodes (18): RechnungOffen, mit_objekt_rollback(), nach_pdf_persistieren(), objekt_löschen_best_effort(), pdfs_rendern_und_ablegen(), rechnung_object_key(), rechnung_pdf_laden(), ObjectStoreError (+10 more)

### Community 38 - "ResultReport"
Cohesion: 0.09
Nodes (26): AuflistenFehler, Haustier, Rechnung, RechnungBezahltMarkieren, RepositoryError, ResultReport, Seminar, SeminarBuchungErstellung (+18 more)

### Community 39 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Mutex, NaiveDate, Self, Utc, Duration

### Community 40 - ".get_current_version"
Cohesion: 0.21
Nodes (9): Box, Future, Option, Output, Pin, Send, Transaction, SQLiteConnection (+1 more)

### Community 41 - "requests/seminar.rs"
Cohesion: 0.16
Nodes (21): abgehalten_use_case(), buchung_id(), parse_preis(), parse_ratio(), DateTime, Decimal, Option, Report (+13 more)

### Community 42 - "domain/adresse.rs"
Cohesion: 0.24
Nodes (4): Adresse, Ländercode, LändercodeValidierungsfehler, Self

### Community 43 - "schema/leistung.rs"
Cohesion: 0.30
Nodes (15): Leistung, LeistungQuelle, LeistungQuelleBehandlung, LeistungQuelleManuell, LeistungQuelleProdukt, LeistungQuelleSeminar, LeistungStatus, Decimal (+7 more)

### Community 44 - "run"
Cohesion: 0.12
Nodes (16): Box, Future, Output, Pin, S, SchedulerError, SchedulerReport, Send (+8 more)

### Community 45 - "FakeDatastore"
Cohesion: 0.07
Nodes (29): FakeBehandlungenRepository, FakeDatastore, FakeHaustiereRepository, FakeKlientenRepository, FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository, FakeSeminareRepository (+21 more)

### Community 46 - "NotificationType"
Cohesion: 0.20
Nodes (9): Send, Sync, ThreadSafeError, NotificationType, Error, Info, Neutral, Success (+1 more)

### Community 47 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedBehandlungRepository, InstrumentedHaustierRepository, InstrumentedKlientRepository, InstrumentedLeistungRepository (+4 more)

### Community 48 - "LeistungId"
Cohesion: 0.17
Nodes (28): LeistungId, Abgehalten, Abgesagt, absagen_archives_buchungen(), aktualisieren_rejects_max_below_confirmed(), als_abgehalten_maps_all_confirmed(), als_abgehalten_maps_confirmed_only(), als_abgehalten_rejects_extra_mapping_keys() (+20 more)

### Community 50 - "common.rs"
Cohesion: 0.24
Nodes (19): menge_to_str(), parse_decimal(), parse_haustier_id(), parse_klient_id(), parse_menge(), parse_preis(), parse_ratio(), parse_rechnung_id() (+11 more)

### Community 51 - "PdfDokument"
Cohesion: 0.12
Nodes (25): Klientbericht, PdfDokument, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht, Adresse, DateTime, EmailAdresse (+17 more)

### Community 52 - "base_app_builder"
Cohesion: 0.07
Nodes (44): klient_body(), Value, tagesabschluss_returns_rechnungen_as_json(), behandlung_erstellen_rejects_empty_name(), behandlung_erstellen_returns_mwst_ratio(), haustier_erstellen_is_listed_and_fetchable(), haustier_erstellen_unknown_klient_is_not_found(), klient_body() (+36 more)

### Community 53 - "Seminar"
Cohesion: 0.17
Nodes (12): NeuesSeminar, preis(), Into, Option, ResultReport, Self, TimeDelta, Uuid (+4 more)

### Community 54 - "HaustierErstellung"
Cohesion: 0.36
Nodes (6): HaustierErstellen, HaustierErstellung, NaiveDate, Self, TryFrom, Uuid

### Community 55 - "ProduktId"
Cohesion: 0.14
Nodes (9): NeuesProdukt, preis(), Produkt, ProduktFehler, ProduktId, Into, ResultReport, Self (+1 more)

### Community 56 - "YamsApi"
Cohesion: 0.22
Nodes (4): Rechnung, RechnungBezahltMarkieren, TagesabschlussErstellung, YamsApi

### Community 57 - "api/client.ts"
Cohesion: 0.19
Nodes (7): Address, BackendClient, getBackendClient(), HttpBackendClient, TauriBackendClient, useBackend(), openapi-fetch

### Community 58 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), Frame, From, Report, Self, Vec, StructuredError

### Community 59 - "RechnungRepository"
Cohesion: 0.06
Nodes (15): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self, BehandlungRepository, LeistungRepository, ProduktRepository (+7 more)

### Community 60 - "arc_up"
Cohesion: 0.16
Nodes (16): arc_up(), Arc, add_migration_out_of_order_results_in_sorted_apply_order(), apply_from_mid_to_latest_applies_only_pending(), apply_from_mid_to_mid_applies_only_in_range(), apply_from_none_to_latest_applies_all_migrations(), apply_from_none_to_specific_target_applies_only_up_to_target(), apply_from_none_to_target_zero_leaves_none_and_applies_nothing() (+8 more)

### Community 61 - "Json"
Cohesion: 0.27
Nodes (10): C, From, ObjectStream, Report, StatusCode, T, StreamBinaryResponse, TypicalJsonResponse<T> (+2 more)

### Community 62 - "base_app_builder"
Cohesion: 0.17
Nodes (34): base_app_builder(), AppBuilder, SetUowProvider, test_tagesabschluss_empty_day_returns_no_rechnungen(), abgehalten_maps_every_confirmed_buchung(), abgehalten_schreibt_teilnahme_pdf_für_bestätigte_buchungen(), absage_blockiert_abgehalten(), adresse() (+26 more)

### Community 63 - "schema/seminar.rs"
Cohesion: 0.13
Nodes (27): NaiveDate, SeminarUmsatzPrognose, BuchungUmsatz, domain::SeminarOrt, Adresse, DateTime, Decimal, From (+19 more)

### Community 64 - ".start"
Cohesion: 0.13
Nodes (15): Box, Default, Future, Output, Pin, S, SchedulerError, SchedulerReport (+7 more)

### Community 65 - "cn"
Cohesion: 0.05
Nodes (69): useAlleKlientenQuery(), useAlleLeistungenQuery(), useAlleRechnungenQuery(), components, $defs, LeistungQuelle_LeistungQuelleBehandlungTyp, Behandlung, LeistungQuelle_LeistungQuelleManuellTyp (+61 more)

### Community 67 - "frontend-legacy/api/schema.d.ts"
Cohesion: 0.33
Nodes (5): components, $defs, operations, paths, webhooks

### Community 68 - "TauriYamsApi"
Cohesion: 0.12
Nodes (7): invokeCommand(), TauriYamsApi, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarTerminErstellung

### Community 69 - "yams-core"
Cohesion: 0.51
Nodes (10): molting, yams-api, yams-core, yams-fakes, yams-filesystemstore, yams-scheduler, yams-server, yams-sqlite (+2 more)

### Community 70 - "react"
Cohesion: 0.09
Nodes (47): HaustierCreateFormProps, KlientRegisterFormProps, LeistungBuchenFormProps, LeistungQuelle, RechnungBezahltButtonProps, TerminBuchungFormProps, TerminPlanFormProps, Alert() (+39 more)

### Community 71 - "<Title>"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+5 more)

### Community 72 - "SeminarId"
Cohesion: 0.12
Nodes (14): SeminarId, Geplant, NeuerSeminarTermin, Adresse, From, Option, S, Self (+6 more)

### Community 73 - "SQLiteHaustierRepository"
Cohesion: 0.24
Nodes (12): haustier_from_row(), query_all_haustiere(), Arc, Haustier, IntoParams, Mutex, Option, RepositoryResult (+4 more)

### Community 74 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.17
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "index.d.ts"
Cohesion: 0.50
Nodes (3): Navigator, Window, @tauri-apps/api

### Community 77 - "requests/abrechnung.rs"
Cohesion: 0.15
Nodes (26): BehandlungErstellen, BehandlungErstellung, into_rechnung_als_bezahlt_markieren(), LeistungAusBehandlungBuchen, LeistungAusBehandlungErstellung, LeistungAusProduktBuchen, LeistungAusProduktErstellung, LeistungManuelleErstellung (+18 more)

### Community 80 - "bad_request"
Cohesion: 0.12
Nodes (14): bad_request(), Arc, C, Leistung, LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung, Report (+6 more)

### Community 88 - "yams-typstreports/src/lib.rs"
Cohesion: 0.15
Nodes (27): PdfRenderError, adresse_dict(), compile_paged(), decimal(), klient_dict(), menge(), naive_date(), praxis_dict() (+19 more)

### Community 89 - "HaustierErstellen"
Cohesion: 0.23
Nodes (10): HaustierErstellen, HaustierErstellenFehler, Context, Haustier, NaiveDate, Report, ResultReport, Vec (+2 more)

### Community 92 - "use_cases/seminar.rs"
Cohesion: 0.12
Nodes (23): buchung_umsatz(), BuchungUmsatz, NaiveDate, Report, Self, Seminar, TimeDelta, Vec (+15 more)

### Community 93 - "SQLiteUnitOfWork"
Cohesion: 0.17
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 99 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.18
Nodes (10): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich), 4.1 Für Casual User, 4.2 Für Power-User & Turnierschützen, 4.3 Für Anlagenbetreiber (B2B & Vereinsnutzen), 4. Erweiterungsvorschläge (Feature Roadmap nach Rollen), 5. Technische Architektur & Systemanforderungen (Übersicht) (+2 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, InstanceType, Arc, AsRef, Connection, Deref, Mutex, Path (+9 more)

### Community 101 - "mwst_19"
Cohesion: 0.83
Nodes (3): mwst_19(), test_behandlung_erstellen(), test_behandlung_erstellen_rejects_empty_name()

### Community 102 - "server/src/config.rs"
Cohesion: 0.15
Nodes (25): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+17 more)

### Community 103 - "hooks/index.ts"
Cohesion: 0.06
Nodes (51): useBehandlungErstellenMutation(), useHaustierErstellenMutation(), useKlientErstellenMutation(), useLeistungAusBehandlungBuchenMutation(), useLeistungAusProduktBuchenMutation(), useLeistungManuellErfassenMutation(), useProduktErstellenMutation(), useRechnungAlsBezahltMarkierenMutation() (+43 more)

### Community 104 - "PdfRenderer"
Cohesion: 0.18
Nodes (12): InstrumentedObjectStore, InstrumentedPdfRenderer, pdf_dokument_kind(), Arc, ObjectStoreError, ObjectStream, Option, ResultReport (+4 more)

### Community 105 - "useStore"
Cohesion: 0.05
Nodes (61): useAddresses(), queryClient, AddressEditTableRowContent, AddressTableHeader, AddressTableRow, AddressUsageInfo, AddressViewTableRowContent, ClientUsages (+53 more)

### Community 106 - ".behandlung_erstellen"
Cohesion: 0.31
Nodes (7): BehandlungErstellenFehler, Behandlung, BehandlungErstellung, Behandlung, Decimal, Uuid, schema_behandlung_from_domain()

### Community 107 - "StatusCode"
Cohesion: 0.05
Nodes (36): AuflistenFehler, BehandlungErstellenFehler, HaustierErstellenFehler, HttpStatusMapping, KlientErstellenFehler, LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler, LeistungManuellErfassenFehler (+28 more)

### Community 108 - "Klient"
Cohesion: 0.20
Nodes (12): Klient, KlientErstellung, Klient, Adresse, EmailAdresse, Haustier, Mobilnummer, NaiveDate (+4 more)

### Community 110 - ".produkt_erstellen"
Cohesion: 0.28
Nodes (7): Produkt, ProduktErstellung, Produkt, Decimal, Uuid, schema_produkt_from_domain(), ProduktErstellenFehler

### Community 111 - "logandaim.at — Product Charter"
Cohesion: 0.18
Nodes (11): Decision log, Domain terms, logandaim.at — Product Charter, Milestones, MVP scope, Non-goals for MVP, Open questions (not blocking M0), Problem (+3 more)

### Community 112 - "cases/abrechnung.rs"
Cohesion: 0.26
Nodes (15): AbrechnungSetup, app_with_pdf_fakes(), menge(), mwst_19(), rechnung_kann_als_bezahlt_markiert_werden(), Arc, Klient, NaiveDate (+7 more)

### Community 113 - "HaustierId"
Cohesion: 0.15
Nodes (13): Haustier, haustier_from_parts_rejects_empty_name(), haustier_neu_keeps_klient_id(), haustier_rejects_empty_name(), HaustierFehler, HaustierId, neues(), NeuesHaustier (+5 more)

### Community 114 - "navigation.ts"
Cohesion: 0.08
Nodes (30): geistMono, geistSans, metadata, Providers(), AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue (+22 more)

### Community 116 - "KlientErstellen"
Cohesion: 0.28
Nodes (7): KlientErstellen, KlientErstellenFehler, Adresse, EmailAdresse, Klient, Mobilnummer, NaiveDate

### Community 117 - "SQLiteKlientRepository"
Cohesion: 0.24
Nodes (10): klient_from_row(), Arc, Klient, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 118 - "notification.ts"
Cohesion: 0.13
Nodes (14): ActionButton, buttonColor(), Notification, Notifications, notificationTypeValues(), NotificationActions, NotificationBehaviour, NotificationContent (+6 more)

### Community 119 - "KlientErstellung"
Cohesion: 0.24
Nodes (8): KlientErstellen, KlientErstellung, Adresse, EmailAdresse, Mobilnummer, NaiveDate, Self, TryFrom

### Community 120 - "Rechnung"
Cohesion: 0.29
Nodes (14): Rechnung, Rechnungsposition, RechnungStatus, Decimal, NaiveDate, Option, S, Uuid (+6 more)

### Community 121 - "termin-detail-panel.tsx"
Cohesion: 0.09
Nodes (20): useSeminarBuchungStornierenMutation(), useSeminarTerminAbgehaltenMutation(), useSeminarTerminAbsagenMutation(), SeminarBuchungStatus, Best_tigt, Storniert, SeminarTerminStatus, Abgehalten (+12 more)

### Community 122 - "leistung_from_row"
Cohesion: 0.17
Nodes (16): format_naive_date(), parse_naive_date(), NaiveDate, leistung_from_row(), leistung_offen_from_row(), Arc, Leistung, Mutex (+8 more)

### Community 123 - "yams_scheduler/config.rs"
Cohesion: 0.24
Nodes (17): CliOverlay, ConfigError, DaemonConfig, load_file(), overlay(), overlays_database_url_from_cli(), parse_config_file(), parses_timezone_from_file() (+9 more)

### Community 124 - "produkt_from_row"
Cohesion: 0.25
Nodes (10): produkt_from_row(), Arc, Mutex, Option, Produkt, RepositoryResult, Row, Transaction (+2 more)

### Community 125 - "seminar_from_row"
Cohesion: 0.25
Nodes (10): Arc, Mutex, Option, RepositoryResult, Row, Seminar, Transaction, Vec (+2 more)

### Community 126 - "behandlung_from_row"
Cohesion: 0.25
Nodes (10): behandlung_from_row(), Arc, Behandlung, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 127 - "UnitOfWorkImpl"
Cohesion: 0.09
Nodes (21): main(), Box, RepositoryResult, Unimplemented, Arc<T>, LockedUnitOfWorkImpl, Box, Option (+13 more)

### Community 128 - "BehandlungCreateForm"
Cohesion: 1.00
Nodes (3): BehandlungCreateForm(), handleSubmit(), emptyBehandlung()

### Community 129 - ".contextualize_with"
Cohesion: 0.44
Nodes (6): ErrorReportExt, Result<T, E>, C, Report, T, Fn

### Community 130 - "YamsApiSpec"
Cohesion: 0.19
Nodes (10): Path, RechnungBezahltMarkieren, SeminarBuchungErstellung, SeminarTermin, SeminarTerminAbsage, SeminarTerminAktualisierung, SeminarTerminErstellung, SeminarUmsatzVorschau (+2 more)

### Community 131 - "main"
Cohesion: 0.17
Nodes (13): BackendServerError, catch_panic(), main(), JoinHandle, Report, SchedulerError, SchedulerReport, SchedulerTask (+5 more)

### Community 133 - "Debug"
Cohesion: 0.31
Nodes (6): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, dummy_rechnung(), ResultReport, Vec, Debug

### Community 134 - ".haustier_erstellen"
Cohesion: 0.36
Nodes (6): HaustierErstellung, Haustier, NaiveDate, Uuid, schema_haustier_from_domain(), HaustierErstellenFehler

### Community 135 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 136 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 137 - "dependencies"
Cohesion: 0.22
Nodes (9): dependencies, lucide-react, next, openapi-fetch, react, react-dom, @tanstack/react-query, @tanstack/react-query-devtools (+1 more)

### Community 138 - "scripts"
Cohesion: 0.20
Nodes (10): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+2 more)

### Community 139 - "InternalServerError"
Cohesion: 0.33
Nodes (3): InternalServerError, Self, StructuredError

### Community 140 - "TypicalJsonResponse"
Cohesion: 0.11
Nodes (14): Behandlung, BehandlungErstellung, Haustier, HaustierErstellung, Klient, KlientErstellung, Produkt, ProduktErstellung (+6 more)

### Community 141 - "seminar-overview.tsx"
Cohesion: 0.10
Nodes (26): useAlleSeminarTermineQuery(), RechnungBezahltButton(), SeminarOverview(), statusBadgeVariant(), TerminFilter, buildTerminEventsByDay(), formatSelectedDate(), TerminCalendar() (+18 more)

### Community 142 - "Zeitraum"
Cohesion: 0.13
Nodes (15): SeminarTerminGeplant, DateTime, Display, Formatter, Self, Utc, utc(), Zeitraum (+7 more)

### Community 143 - ".seminar_umsatz_prognose"
Cohesion: 0.50
Nodes (3): NaiveDate, SeminarUmsatzPrognose, Query

### Community 144 - "FakeUnitOfWork"
Cohesion: 0.25
Nodes (9): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Mutex, RepositoryResult, Self, Vec (+1 more)

### Community 145 - "SQLiteStateStore"
Cohesion: 0.20
Nodes (7): Arc, Self, Send, Transaction, SQLiteStateStore, StateStore, StoreErrorKind

### Community 147 - "participation/index.tsx"
Cohesion: 0.29
Nodes (11): ClientParticipation, askAddParticipantEvent(), askEditEventParticipation(), submitAddParticipantEvent(), submitDeleteEventParticipation(), submitUpdateEventParticipation(), deleteEventParticipation(), relateClientParticipateEvent() (+3 more)

### Community 148 - "ExecutionContext"
Cohesion: 0.14
Nodes (16): ExecutionContext, ExecutionSource, Arc, Context, Output, ResultReport, UseCase, BehandlungErstellen (+8 more)

### Community 149 - "SeminarCreateForm"
Cohesion: 1.00
Nodes (3): emptySeminar(), SeminarCreateForm(), handleSubmit()

### Community 150 - "Menge"
Cohesion: 0.27
Nodes (4): Menge, MengeFehler, Decimal, Self

### Community 151 - "Contributing and project management"
Cohesion: 0.22
Nodes (9): Agents, Commits, Contributing and project management, Delivery gate, GitHub Project, Labels and priority, Ready gate, Records (+1 more)

### Community 152 - "yams_scheduler/main.rs"
Cohesion: 0.24
Nodes (9): Cli, main(), Option, PathBuf, Report, SchedulerMainError, init_tracing(), Option (+1 more)

### Community 153 - "App"
Cohesion: 0.26
Nodes (8): App, Arc, Box, Context, F, ResultReport, O, U

### Community 154 - "Adresse"
Cohesion: 0.29
Nodes (7): Adresse, domain::Adresse, Ländercode, Example, From, Self, TryFrom

### Community 155 - "stores/index.tsx"
Cohesion: 0.06
Nodes (22): AnimalAddItem, AnimalComboBox, AnimalRow, deleteAnimal(), patchAnimal(), patchRace(), DialogComponent, DialogInfo (+14 more)

### Community 156 - "SchedulerTask"
Cohesion: 0.43
Nodes (6): main(), JoinHandle, SchedulerError, SchedulerReport, SchedulerState, SchedulerTask

### Community 157 - "package.json"
Cohesion: 0.13
Nodes (14): name, private, version, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, react-dom, tailwindcss (+6 more)

### Community 158 - "Preis"
Cohesion: 0.09
Nodes (8): Add, Preis, Output, Self, position_from_leistung(), RechnungIn<S>, Rechnungsposition, Mul

### Community 159 - "HaustierCreateForm"
Cohesion: 0.67
Nodes (3): emptyHaustier(), HaustierCreateForm(), handleSubmit()

### Community 160 - "arc_down"
Cohesion: 0.31
Nodes (9): arc_down(), apply_down_from_current_to_lower_target_applies_down_migrations(), apply_down_from_current_to_mid_target_applies_only_relevant_down(), apply_down_from_one_to_none(), apply_down_one_step_from_2_to_1(), down_registry_apply_down_failure_propagates_as_migration_failed(), down_registry_apply_up_to_latest_applies_all_migrations(), down_registry_apply_when_current_equals_target_is_no_op() (+1 more)

### Community 161 - "0001-baseline-architecture.md"
Cohesion: 0.22
Nodes (6): ADR-0001: Baseline architecture, Consequences, Context, Decision, Architecture decision records, Legacy specs (YAMS proof of concept)

### Community 163 - "Ratio"
Cohesion: 0.16
Nodes (4): Ratio, RatioFehler, Decimal, Self

### Community 165 - "ObjectStore"
Cohesion: 0.26
Nodes (10): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, ResultReport, Send (+2 more)

### Community 166 - "wal_connection_race.rs"
Cohesion: 0.70
Nodes (4): app(), neuer_klient(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 169 - "preis.rs"
Cohesion: 0.29
Nodes (8): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal

### Community 170 - "SQLiteInstance"
Cohesion: 0.29
Nodes (4): base_app_builder(), AppBuilder, SetUowProvider, SQLiteInstance

### Community 171 - "SeminarTerminId"
Cohesion: 0.11
Nodes (12): seminar_betrag_full_rabatt_is_zero(), seminar_betrag_uses_nach_rabatt(), Item, Iterator, Seminar, Uuid, SeminarBuchung, SeminarBuchungId (+4 more)

### Community 172 - "openapi_service"
Cohesion: 0.25
Nodes (7): openapi_service(), Into, Item, Self, IntoIterator, OpenApiService, ServerObject

### Community 174 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 175 - ".teilnahmebestätigung_pdf"
Cohesion: 0.47
Nodes (3): ObjectStoreError, ObjectStream, Self

### Community 177 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 179 - "next"
Cohesion: 0.33
Nodes (3): Data, nextConfig, next

### Community 183 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 184 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 185 - "3. Kernanforderungen (Funktionale Spezifikation)"
Cohesion: 0.40
Nodes (5): 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation)

### Community 186 - "src/errors.rs"
Cohesion: 1.00
Nodes (3): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), RepositoryError

### Community 189 - "mwst_19"
Cohesion: 0.83
Nodes (3): mwst_19(), test_produkt_erstellen(), test_produkt_erstellen_rejects_empty_name()

## Knowledge Gaps
- **375 isolated node(s):** `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler`, `BehandlungErstellenFehler`, `LeistungAusProduktBuchenFehler` (+370 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 986 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **25 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `TestError`, `.haustier_erstellen`, `in_memory_object_store.rs`, `FakeObjectStore`, `termin_from_parts`, `src-tauri/src/config.rs`, `InternalServerError`, `BehandlungId`, `FakeUnitOfWork`, `domain/kontakt.rs`, `ExecutionContext`, `SQLiteRechnungRepository`, `src/sqlite_state_store.rs`, `yams_scheduler/main.rs`, `tests.rs`, `Adresse`, `v0004_leistungen_quelle_mwst.rs`, `Preis`, `EmailAdresse`, `LeistungOffen`, `KlientId`, `NeueLeistung`, `service/pdf.rs`, `requests/seminar.rs`, `domain/adresse.rs`, `schema/leistung.rs`, `LeistungId`, `common.rs`, `PdfDokument`, `base_app_builder`, `Seminar`, `HaustierErstellung`, `ProduktId`, `StructuredError`, `Json`, `schema/seminar.rs`, `SeminarId`, `requests/abrechnung.rs`, `HaustierErstellen`, `use_cases/seminar.rs`, `server/src/config.rs`, `.behandlung_erstellen`, `Klient`, `.produkt_erstellen`, `HaustierId`, `KlientErstellen`, `KlientErstellung`, `Rechnung`, `leistung_from_row`, `yams_scheduler/config.rs`?**
  _High betweenness centrality (0.520) - this node is a cross-community bridge._
- **Why does `Error` connect `NotificationType` to `TestError`?**
  _High betweenness centrality (0.373) - this node is a cross-community bridge._
- **Why does `NotificationType` connect `NotificationType` to `notification.ts`?**
  _High betweenness centrality (0.372) - this node is a cross-community bridge._
- **Are the 9 inferred relationships involving `KlientId` (e.g. with `.rechnungen_für_klient()` and `.try_from()`) actually correct?**
  _`KlientId` has 9 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `RechnungFehler`, `ProduktErstellenFehler` to the rest of the system?**
  _375 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TestError` be split into smaller, more focused modules?**
  _Cohesion score 0.11363636363636363 - nodes in this community are weakly interconnected._
- **Should `biome.json` be split into smaller, more focused modules?**
  _Cohesion score 0.07692307692307693 - nodes in this community are weakly interconnected._