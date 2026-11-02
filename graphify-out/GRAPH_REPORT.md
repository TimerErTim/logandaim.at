# Graph Report - logandaim.at  (2026-11-02)

## Corpus Check
- 290 files · ~187,179 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4181 nodes · 11003 edges · 177 communities (158 shown, 11 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 261 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `65c20428`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PdfRenderer
- arc_up
- charter.md
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- neu/page.tsx
- ExecutionContext
- http_status.rs
- Session
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- package.json
- SQLiteRundeRepository
- cases/konto.rs
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- ParkourId
- Clock
- cases/stammdaten.rs
- auth-pages.tsx
- <Title>
- Make invalid state unrepresentable
- Betrieb — Operations and launch readiness
- check-release.mjs
- ContextPorts
- Wertung — Classic 3-arrow scoring
- .contextualize_with
- Konto — Benutzer, registration, and login
- SecretTokenHash
- jwt.rs
- neuer_benutzer
- konto/session.rs
- anlage/page.tsx
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- PasswortWiederherstellung
- KlartextPasswort
- parkour.rs
- spend_open_confirmations
- .get_current_version
- Vertical slices
- AnlageAdresse
- in_memory_object_store.rs
- components.json
- FakeObjectStore
- domain/common.rs
- SQLiteSchussRepository
- domain/kontakt.rs
- InMemorySigning
- ParkourInhalt
- logandaim.at — Product Charter
- dependencies
- DateTime
- .try_from
- NeuesKonto
- Visual identity and design system
- Mail
- SQLiteBenutzerRepository
- AnlageId
- Runde
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- SchussRegistrieren
- preis.rs
- check-version.mjs
- ClientKey
- ObjectStore
- diffx-finish-review
- NeueAnlage
- Versioned
- Next.js
- postcss.config.mjs
- use_cases/runde.rs
- StammdatenFehler
- Contributing and project management
- Anlage
- HttpLogandaimApi
- schema/stammdaten.rs
- spend_open_recoveries
- devDependencies
- FakeUnitOfWork
- NonEmptyString
- anlage
- LogandaimApiSpec
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- e2e/konto.rs
- SQLiteInstance
- UpMigration
- config.rs
- RepoStorage
- .add_dyn
- UnitOfWorkImpl
- CookieConfig
- .poll_next
- scripts
- NewSessionResponse
- SQLiteUnitOfWork
- anlage/mod.rs
- EmailAdresse
- app-shell.tsx
- RepositoryResult
- Benutzer
- anlage_aus_zeile
- pull_request_template.md
- LogandaimApi
- ZielId
- layout.tsx
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- AuthenticatedBenutzer
- ParkourSnapshot
- logandaim-api.ts
- Anlage-Anfrage — Requesting a new Anlage
- Erfasser
- format_datetime
- StructuredError
- StreamBinaryResponse
- .perform
- parkour_mit_einem_ziel
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- .authenticated
- String
- requests/konto.rs
- dates.ts
- cn
- Konto
- Vec
- http/README.md
- openapi_service
- in_memory_throttle.rs
- apply_up_migrations
- instrumented.rs
- domain/runde.rs
- ADR-0002: Session cookie auth across frontend and API origins
- .from
- m1-overseer.md
- Modes
- AccessTokenClaims
- schema.ts
- RundeIn<S>
- m1-reviewer.md
- NeuerSchuss
- Deferred capabilities
- InstrumentedUnitOfWork
- schema/schuss.rs
- tests/throttle.rs
- user.rs
- client_key
- Migration
- StreamBody
- provider.tsx
- .auth_context
- Specifications
- RundeStartenRequest
- mise
- spec.rs
- .anlagen_anzeigen
- BenutzerId

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 164 edges
2. `cn()` - 111 edges
3. `BenutzerId` - 106 edges
4. `AnlageId` - 91 edges
5. `ZielId` - 85 edges
6. `ParkourId` - 65 edges
7. `NonEmptyString` - 64 edges
8. `react` - 58 edges
9. `AuthContext` - 55 edges
10. `FakeDatastore` - 54 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/logandaim-api/src/http/mod.rs
- `logandaim-server` --depends_on--> `logandaim-api`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-api/Cargo.toml
- `logandaim-server` --depends_on--> `logandaim-filesystemstore`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-filesystemstore/Cargo.toml
- `logandaim-server` --depends_on--> `logandaim-smtp`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-smtp/Cargo.toml
- `ConfigError` --references--> `String`  [EXTRACTED]
  backend/server/src/config.rs → crates/logandaim-api/src/errors/internal_error.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (177 total, 11 thin omitted)

### Community 0 - "PdfRenderer"
Cohesion: 0.10
Nodes (18): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, Vec, PdfDokument, PdfRenderer, PdfRenderError (+10 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "charter.md"
Cohesion: 0.14
Nodes (9): Added, Changelog, [Unreleased], Checklist, Release description, Release pull request, Test evidence, Version (+1 more)

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "neu/page.tsx"
Cohesion: 0.09
Nodes (43): AuthProvider(), useAuth(), useErfolgsmeldung(), useAnlageMutation(), useAuthMutations(), useStammdatenMutations(), useAnlagenQuery(), useAnlagenVerzeichnisQuery() (+35 more)

### Community 7 - "ExecutionContext"
Cohesion: 0.12
Nodes (34): ExecutionContext, darf_anlage_lesen(), darf_anlage_verwalten(), darf_masterdaten_pflegen(), Anlage, Benutzer, Context, Output (+26 more)

### Community 8 - "http_status.rs"
Cohesion: 0.12
Nodes (14): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+6 more)

### Community 9 - "Session"
Cohesion: 0.13
Nodes (15): Session, SessionId, option_datetime(), Arc, DateTime, Mutex, NeueSession, Option (+7 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.20
Nodes (19): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing(), put_get_roundtrip() (+11 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "package.json"
Cohesion: 0.04
Nodes (47): name, private, version, ComposedPageSection(), DemoPage(), CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS (+39 more)

### Community 14 - "SQLiteRundeRepository"
Cohesion: 0.17
Nodes (19): ersetze_referenzen(), PflockDto, Arc, Erfasser, Mutex, Option, RepositoryResult, Row (+11 more)

### Community 15 - "cases/konto.rs"
Cohesion: 0.06
Nodes (76): AccessToken, basis64url(), basis64url_decode(), constant_time_eq(), debug_gibt_das_token_nicht_preis(), hash_ist_stabil_und_vergleicht_konstantzeitig(), konstantzeit_vergleich_erkennt_jedes_gekippte_byte(), parsing_ist_ein_roundtrip_über_die_bytes() (+68 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.27
Nodes (10): E, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+2 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "ParkourId"
Cohesion: 0.17
Nodes (8): AnlageAktiv, AnlageFehler, Err, NonZeroU32, Pflock, Result, Zielplatzierung, ParkourId

### Community 20 - "Clock"
Cohesion: 0.09
Nodes (17): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+9 more)

### Community 21 - "cases/stammdaten.rs"
Cohesion: 0.08
Nodes (71): anlage_laden(), build(), Harness, jetzt(), Anlage, AppBuilder, Arc, DateTime (+63 more)

### Community 22 - "auth-pages.tsx"
Cohesion: 0.07
Nodes (19): AuthFailure, AuthFailureMessages, AuthPageProps, LoginPage(), PasswordPage(), passwordWithinPolicy(), RegistrationPage(), useMailToken() (+11 more)

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

### Community 27 - "ContextPorts"
Cohesion: 0.14
Nodes (8): ContextPorts, ExecutionContext<'a>, Arc, RepositoryResult, die_konfiguration_hält_nur_die_claim_identität(), IdentityProviderConfiguration, Into, Self

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Fn, Report, Result, Send (+3 more)

### Community 30 - "Konto — Benutzer, registration, and login"
Cohesion: 0.12
Nodes (16): Acceptance criteria, Approval, Decision log, Domain terms and business rules, Implementation sketches, Konto — Benutzer, registration, and login, Outcome, Problem (+8 more)

### Community 31 - "SecretTokenHash"
Cohesion: 0.07
Nodes (30): EmailBestätigungRepository, bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung (+22 more)

### Community 32 - "jwt.rs"
Cohesion: 0.17
Nodes (23): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), claims(), decode(), DecodedToken, encode(), encode_session() (+15 more)

### Community 33 - "neuer_benutzer"
Cohesion: 0.06
Nodes (72): assert_status_ok(), base_app_builder(), befördere_zum_systemadministrator(), cookie_pair(), cookie_response(), E2eHarness, json_response(), LogandaimApiTestClient (+64 more)

### Community 34 - "konto/session.rs"
Cohesion: 0.24
Nodes (13): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), jetzt(), KontoFehler, neue_session_ist_gültig_und_läuft_in_einem_monat_ab(), From (+5 more)

### Community 35 - "anlage/page.tsx"
Cohesion: 0.09
Nodes (39): apiErrorMessage(), Pflock, RundeBasic, AnlageDetailPage(), laender, ParkourLedger(), AnlagenPage(), ZONEN (+31 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.11
Nodes (22): from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime, Option (+14 more)

### Community 40 - "KlartextPasswort"
Cohesion: 0.14
Nodes (20): KontoFehler, argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gültige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password() (+12 more)

### Community 42 - "parkour.rs"
Cohesion: 0.11
Nodes (30): ac11_archivierter_parkour_verweigert_jede_aenderung(), ac4_dasselbe_ziel_darf_nur_einmal_vorkommen(), ac6_abstaende_nehmen_vom_ersten_zum_letzten_pflock_streng_zu(), ac6_fehlende_abstaende_sind_erlaubt(), ac6_pflock_farben_sind_eindeutig(), ac6_reihenfolge_ist_unabhaengig_von_der_farbe(), drei_pflöcke(), ein_parkour_braucht_mindestens_einen_pflock() (+22 more)

### Community 44 - "spend_open_confirmations"
Cohesion: 0.22
Nodes (10): RegistrierungAbschließen, RegistrierungStarten, RegistrierungStartenFehler, DateTime, EmailAdresse, Error, Report, Result (+2 more)

### Community 46 - ".get_current_version"
Cohesion: 0.21
Nodes (11): Box, Error, Future, Option, Output, Pin, Result, Send (+3 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "AnlageAdresse"
Cohesion: 0.20
Nodes (7): AnlageAdresse, AnlageAdresseFehler, AsRef, Result, Self, vollstaendige_adresse_wird_angenommen(), Ländercode

### Community 49 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 52 - "domain/common.rs"
Cohesion: 0.08
Nodes (22): Abstand, AbstandFehler, Farbe, farbe_keeps_channels_after_roundtrip(), farbe_meldet_die_abgelehnte_eingabe(), farbe_roundtrips_hex(), FarbeFehler, NonEmptyStringFehler (+14 more)

### Community 53 - "SQLiteSchussRepository"
Cohesion: 0.20
Nodes (15): Arc, Erfasser, IntoParams, Mutex, Option, RepositoryResult, Row, Schuss (+7 more)

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.06
Nodes (34): alle_felder_duerfen_fehlen(), AnlageKontakt, fehlendes_protokoll_wird_zu_https(), jedes_feld_ist_einzeln_optional(), protokoll(), AsRef, Display, EmailAdresse (+26 more)

### Community 55 - "InMemorySigning"
Cohesion: 0.08
Nodes (25): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex(), ResultReport, Send, Sync (+17 more)

### Community 56 - "ParkourInhalt"
Cohesion: 0.13
Nodes (13): ParkourListe(), parkourErstellen(), pflockHinzufuegen(), ParkourInhalt(), pflockHinzufuegen(), freiePflockFarbe(), neuerPflock(), PFLOCK_FARBE_STANDARD (+5 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.07
Nodes (28): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+20 more)

### Community 59 - "DateTime"
Cohesion: 0.13
Nodes (18): from_parts_roundtrip_über_alle_drei_zustände(), NeueSession, Refreshed, RefreshedSession, Renewable, RenewableSession, Revoked, RevokedSession (+10 more)

### Community 60 - ".try_from"
Cohesion: 0.38
Nodes (5): domain::AnlageVerwaltungRolle, domain::Farbe, Error, Result, TryFrom

### Community 61 - "NeuesKonto"
Cohesion: 0.16
Nodes (13): email(), from_parts_roundtrip(), hash(), jetzt(), neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime, EmailAdresse (+5 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "Mail"
Cohesion: 0.06
Nodes (34): AsyncSmtpTransport, NullMailSender, ResultReport, adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), Mail, MailArt (+26 more)

### Community 64 - "SQLiteBenutzerRepository"
Cohesion: 0.24
Nodes (9): benutzer_from_row(), Arc, Benutzer, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 65 - "AnlageId"
Cohesion: 0.11
Nodes (16): LogandaimAppApi, Anlage, AnlageZugriff, AnlageÖffentlich, Arc, Benutzer, ParkourÖffentlich, ResultReport (+8 more)

### Community 66 - "Runde"
Cohesion: 0.19
Nodes (18): Abstand, DateTime, Erfasser, Farbe, From, Option, Schuss, Self (+10 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "SchussRegistrieren"
Cohesion: 0.14
Nodes (14): Report, Result, Uuid, Wertungszone, SchussErstellungRequest, Error, From, Result (+6 more)

### Community 71 - "preis.rs"
Cohesion: 0.07
Nodes (25): Add, Menge, MengeFehler, Decimal, Result, Self, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent() (+17 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "ClientKey"
Cohesion: 0.15
Nodes (17): InstrumentedThrottle, DateTime, ResultReport, Utc, ClientKey, DateTime, Duration, Into (+9 more)

### Community 74 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "NeueAnlage"
Cohesion: 0.22
Nodes (6): adresse(), AnlageIn, NeueAnlage, AnlageAdresse, AnlageKontakt, S

### Community 77 - "Versioned"
Cohesion: 0.09
Nodes (26): Versioned, FakeAnlageRepository, FakeBenutzerRepository, FakeDatastore, FakeKontoRepository, FakePasswortWiederherstellungRepository, FakeRundeRepository, FakeSchussRepository (+18 more)

### Community 80 - "use_cases/runde.rs"
Cohesion: 0.24
Nodes (9): Error, Result, Runde, RundeAbschliessen, RundeAbschliessenFehler, RundeAnzeigenFehler, RundenAnzeigenFehler, RundeStarten (+1 more)

### Community 81 - "StammdatenFehler"
Cohesion: 0.16
Nodes (32): adresse(), anlage_kontext(), AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest, farbe(), kontakt(), name() (+24 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "Anlage"
Cohesion: 0.07
Nodes (17): Anlage, AnlageArchiviert, AnlageIn<S>, AnlagenSicht, AnlageVerwaltung, AnlageVerwaltungRolle, Archiviert, ist_systemadministrator_oder_verwalter() (+9 more)

### Community 86 - "schema/stammdaten.rs"
Cohesion: 0.13
Nodes (27): Anlage, AnlageAdresse, AnlageKontakt, AnlageVerwaltung, AnlageVerwaltungRolle, AnlageZugriff, AnlageZustand, AnlageÖffentlich (+19 more)

### Community 87 - "spend_open_recoveries"
Cohesion: 0.25
Nodes (8): PasswortVergessenFehler, PasswortWiederherstellen, DateTime, Error, Report, Result, Utc, spend_open_recoveries()

### Community 88 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 89 - "FakeUnitOfWork"
Cohesion: 0.24
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 90 - "NonEmptyString"
Cohesion: 0.09
Nodes (21): AnlageVerwaltung, aendern_haelt_identitaet_und_nummer(), archiviertes_ziel_verweigert_jede_aenderung(), die_zielnummer_ist_aenderbar(), die_zielnummer_unterscheidet_gleichnamige_ziele(), from_parts_roundtrip(), name(), NeuesZiel (+13 more)

### Community 91 - "anlage"
Cohesion: 0.11
Nodes (25): anlage(), archivistand_bleibt_im_snapshot_unberuehrt(), from_parts_roundtrip(), name(), pflock(), PflockSnapshot, Abstand, Anlage (+17 more)

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.22
Nodes (13): LogandaimApiSpec, Anlage, Benutzer, From, Json, Path, Runde, RundeBasic (+5 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "e2e/konto.rs"
Cohesion: 0.14
Nodes (19): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+11 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedAnlageRepository, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository (+4 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, Into (+4 more)

### Community 106 - "UnitOfWorkImpl"
Cohesion: 0.04
Nodes (38): Box, RepositoryResult, Unimplemented, ExecutionSource, Arc<T>, LockedUnitOfWorkImpl, Box, C (+30 more)

### Community 107 - "CookieConfig"
Cohesion: 0.19
Nodes (16): attribute(), clear_cookie_header(), CookieConfig, das_löschen_setzt_max_age_auch_ohne_lebensdauer(), das_löschen_trifft_dasselbe_cookie(), die_attribute_sind_vollständig(), die_lebensdauer_und_die_session_variante_setzen_max_age(), die_vorgaben_schreiben_nur_das_nötigste() (+8 more)

### Community 108 - ".poll_next"
Cohesion: 0.22
Nodes (8): FileStream, Context, Item, Pin, Self, File, Poll, Stream

### Community 109 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 110 - "NewSessionResponse"
Cohesion: 0.24
Nodes (15): EndSessionResponse, EndSessionResponse<T>, NewSessionResponse, refresh_cookie_config(), From, Json, NeueSession, Option (+7 more)

### Community 111 - "SQLiteUnitOfWork"
Cohesion: 0.19
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 112 - "anlage/mod.rs"
Cohesion: 0.19
Nodes (22): ac11_archivieren_entfernt_das_ziel_aus_den_parkour(), ac12_entfernen_loescht_das_ziel_und_seine_platzierungen(), ac1_neue_anlage_ist_aktiv_und_ohne_administrator(), ac2_die_letzte_administrator_verwaltung_kann_nicht_entzogen_werden(), ac2_eine_verwaltung_existiert_nur_einmal_pro_benutzer(), ac4_ein_parkour_verwendet_nur_ziele_dieser_anlage(), ac4_ein_ziel_kann_in_mehreren_parkour_liegen(), ac7_der_snapshot_ist_eine_wertkopie() (+14 more)

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "app-shell.tsx"
Cohesion: 0.12
Nodes (21): AppShell(), usePageChrome(), Sidebar(), SidebarLink(), SidebarProps, DropdownMenu, DropdownMenuContent(), DropdownMenuGroup (+13 more)

### Community 115 - "RepositoryResult"
Cohesion: 0.06
Nodes (18): AnlageRepository, BenutzerRepository, PasswortWiederherstellungRepository, RepositoryError, Anlage, Benutzer, EmailAdresse, NeueSession (+10 more)

### Community 116 - "Benutzer"
Cohesion: 0.17
Nodes (13): befoerderung_ist_eine_domain_mutation(), Benutzer, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle, DateTime (+5 more)

### Community 117 - "anlage_aus_zeile"
Cohesion: 0.17
Nodes (26): anlage_aus_zeile(), ersetze_kinder(), kinder_loeschen_sql(), lade_parkour(), lade_pflöcke(), lade_platzierungen(), lade_verwaltungen(), lade_ziele() (+18 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "LogandaimApi"
Cohesion: 0.06
Nodes (4): LogandaimApi, AnlageVerwaltungRolle, Administrator, Mitarbeiter

### Community 120 - "ZielId"
Cohesion: 0.11
Nodes (26): ZielId, Wertungszone, schuss(), ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), anonymisieren_entfernt_den_benutzer(), die_faltung_ist_unabhängig_von_der_eingabereihenfolge(), ein_schuss_eines_anderen_ziels_wird_abgelehnt() (+18 more)

### Community 121 - "layout.tsx"
Cohesion: 0.13
Nodes (12): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, Providers(), AppChrome() (+4 more)

### Community 122 - "Appendix: implementation sketch (non-normative)"
Cohesion: 0.17
Nodes (12): ADR-0004: JWT access tokens with rotating refresh tokens, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Crypto is domain code (sketch) (+4 more)

### Community 123 - "Stammdaten — Master data and `Anlage` administration"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Draft — Anlagen-Verzeichnis, Outcome, Problem, Requirements, Revision history (+5 more)

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 125 - "AuthenticatedBenutzer"
Cohesion: 0.33
Nodes (9): AuthenticatedBenutzer, Benutzer, BenutzerRolle, NeueSession, EmailAdresse, From, Self, Uuid (+1 more)

### Community 126 - "ParkourSnapshot"
Cohesion: 0.10
Nodes (21): ParkourSnapshot, Abgeschlossen, abschliessen_ist_eine_einbahnstrasse(), anonymisieren_laesst_den_snapshot_unberuehrt(), ein_abschlusszeitpunkt_macht_die_geladene_runde_abgeschlossen(), from_parts_roundtrip_beider_zustaende(), InBearbeitung, jetzt() (+13 more)

### Community 127 - "logandaim-api.ts"
Cohesion: 0.09
Nodes (25): JsonClient, AnlageAdresse, AnlageKontakt, AnlageVerwaltung, AnlageZugriff, AnlageÖffentlich, AuthenticatedBenutzer, Benutzer (+17 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "Erfasser"
Cohesion: 0.39
Nodes (4): Erfasser, DateTime, Option, Utc

### Community 131 - "format_datetime"
Cohesion: 0.14
Nodes (20): delete_versioned(), format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_version() (+12 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "StreamBinaryResponse"
Cohesion: 0.21
Nodes (17): InternalServerError, Self, C, From, Json, ObjectStream, PlainText, Report (+9 more)

### Community 134 - ".perform"
Cohesion: 0.25
Nodes (5): Anmeldung, AnmeldungFehler, EmailAdresse, Error, Result

### Community 135 - "parkour_mit_einem_ziel"
Cohesion: 0.26
Nodes (21): base_app_builder(), AppBuilder, SetUowProvider, abschliessen_ist_einmalig_und_setzt_den_zustand(), adresse(), benutzer_mit_ip(), client_key(), ein_schuss_auf_ein_ziel_ausserhalb_des_snapshots_wird_abgelehnt() (+13 more)

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.40
Nodes (5): ADR-0001: Baseline architecture, Appendix: spelling German identifiers, Consequences, Context, Decision

### Community 138 - ".authenticated"
Cohesion: 0.19
Nodes (4): Anlage, AnlageErstellungInput, Runde, ZielInput

### Community 139 - "String"
Cohesion: 0.11
Nodes (13): From, String, basis_url(), letzter_text_an(), nachrichten(), Value, Vec, Adresse (+5 more)

### Community 140 - "requests/konto.rs"
Cohesion: 0.18
Nodes (12): AnmeldungRequest, PasswortVergessenRequest, PasswortWiederherstellenRequest, RegistrierungAbschlussRequest, RegistrierungStartRequest, EmailAdresse, Report, Result (+4 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - "cn"
Cohesion: 0.05
Nodes (93): PLATES, ComponentSection(), Example(), ThemePlate(), touchTargetOutline, AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS (+85 more)

### Community 144 - "Konto"
Cohesion: 0.14
Nodes (13): KontoRepository, Konto, KontoId, konto_from_row(), Arc, EmailAdresse, Mutex, Option (+5 more)

### Community 145 - "Vec"
Cohesion: 0.40
Nodes (4): Abstand, Vec, Zielverwendung, Zielverwendung

### Community 146 - "http/README.md"
Cohesion: 0.50
Nodes (3): Authentication, Errors, Sections

### Community 147 - "openapi_service"
Cohesion: 0.19
Nodes (12): main(), Box, RepositoryResult, Unimplemented, ApiTag, openapi_service(), openapi_service_from_api(), Into (+4 more)

### Community 148 - "in_memory_throttle.rs"
Cohesion: 0.24
Nodes (17): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), fehlversuche_erschöpfen_das_budget_und_blockieren_weitere_anfragen(), gültige_anfragen_verbrauchen_das_budget_nicht(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now() (+9 more)

### Community 149 - "apply_up_migrations"
Cohesion: 0.17
Nodes (15): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationUp, MigrationError, MigrationTarget, Box, Future (+7 more)

### Community 150 - "instrumented.rs"
Cohesion: 0.24
Nodes (7): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option

### Community 151 - "domain/runde.rs"
Cohesion: 0.21
Nodes (11): ac8_die_summe_ist_die_summe_der_zielwertungen(), ac8_fremde_schüsse_zaehlen_nicht(), ac8_ziele_ohne_schuss_werden_uebersprungen(), eine_leere_runde_hat_null(), IntoIterator, Item, Option, Schuss (+3 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.11
Nodes (15): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+7 more)

### Community 154 - ".from"
Cohesion: 0.17
Nodes (10): AuthenticatedBenutzer, Cookie, Option, RealIp, Self, Ok, Default, Example (+2 more)

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "AccessTokenClaims"
Cohesion: 0.23
Nodes (17): abgelaufenes_token_wird_abgelehnt(), AccessTokenClaims, claims(), fenster_mit_exp_vor_iat_wird_abgelehnt(), gültiges_fenster_wird_akzeptiert(), herkunft_muss_exakt_stimmen(), iat_in_der_zukunft_wird_abgelehnt(), iat_innerhalb_der_toleranz_wird_akzeptiert() (+9 more)

### Community 158 - "schema.ts"
Cohesion: 0.08
Nodes (25): ApiError, AnlageZustand, Aktiv, Archiviert, BenutzerRolle, Benutzer, Systemadministrator, components (+17 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "NeuerSchuss"
Cohesion: 0.13
Nodes (6): NeuerSchuss, pfeil(), FromStr, Wertungszone, Zielwertung, NonZeroUsize

### Community 162 - "Deferred capabilities"
Cohesion: 0.17
Nodes (12): Account security, Administration, Auth audit, Deferred capabilities, Email change, Email delivery, Email verification, External identity providers (+4 more)

### Community 163 - "InstrumentedUnitOfWork"
Cohesion: 0.17
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 164 - "schema/schuss.rs"
Cohesion: 0.21
Nodes (12): AnonymisierterErfasser, BenutzerErfasser, domain::Wertungszone, Erfasser, DateTime, From, Self, Utc (+4 more)

### Community 167 - "tests/throttle.rs"
Cohesion: 0.36
Nodes (9): ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac16_registrierung_ist_auf_fünf_pro_stunde_gedrosselt(), adresse(), app(), client_key(), passwort(), EmailAdresse, Report (+1 more)

### Community 168 - "user.rs"
Cohesion: 0.22
Nodes (9): BenutzerLöschungRequest, Beförderung, BenutzerLöschen, BenutzerLöschenFehler, EmailAdresse, Error, Result, SystemadministratorBefördern (+1 more)

### Community 169 - "client_key"
Cohesion: 0.30
Nodes (11): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), bearer_from_request(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), register_bearer(), Option (+3 more)

### Community 170 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 171 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 172 - "provider.tsx"
Cohesion: 0.21
Nodes (7): createLogandaimApi(), AuthSession, LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config, @tanstack/react-query-devtools

### Community 173 - ".auth_context"
Cohesion: 0.33
Nodes (4): bearer_token(), LogandaimApiSpec, Option, ResultReport

### Community 174 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 175 - "RundeStartenRequest"
Cohesion: 0.38
Nodes (5): From, Self, Uuid, RundeStarten, RundeStartenRequest

### Community 186 - "spec.rs"
Cohesion: 0.47
Nodes (8): bearer_schema_ist_veröffentlicht(), jede_operation_hat_genau_einen_veröffentlichten_tag(), jede_operation_und_jedes_schema_ist_beschrieben(), keine_internen_verweise_in_der_öffentlichen_spec(), operationen(), Value, Vec, spec()

### Community 189 - ".anlagen_anzeigen"
Cohesion: 0.29
Nodes (5): BearerAuthorization, AnlageZugriff, AnlageÖffentlich, ParkourÖffentlich, Vec

### Community 190 - "BenutzerId"
Cohesion: 0.10
Nodes (10): id(), ResultReport, BenutzerId, Uuid, SessionIn<S>, claims_for_session(), new_session(), DateTime (+2 more)

## Knowledge Gaps
- **505 isolated node(s):** `ValidationError`, `LogandaimApiSpec`, `ApiTag`, `AnlageVerwaltungRolleUnbekannt`, `ZustandUnbekannt` (+500 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1116 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `format_datetime`, `StructuredError`, `StreamBinaryResponse`, `Session`, `requests/konto.rs`, `SQLiteRundeRepository`, `cases/konto.rs`, `cases/stammdaten.rs`, `.from`, `ContextPorts`, `AccessTokenClaims`, `SecretTokenHash`, `jwt.rs`, `neuer_benutzer`, `tests/throttle.rs`, `user.rs`, `client_key`, `KlartextPasswort`, `AnlageAdresse`, `in_memory_object_store.rs`, `FakeObjectStore`, `domain/common.rs`, `SQLiteSchussRepository`, `domain/kontakt.rs`, `InMemorySigning`, `DateTime`, `NeuesKonto`, `Mail`, `AnlageId`, `Runde`, `preis.rs`, `ClientKey`, `StammdatenFehler`, `schema/stammdaten.rs`, `FakeUnitOfWork`, `NonEmptyString`, `LogandaimApiSpec`, `e2e/konto.rs`, `config.rs`, `CookieConfig`, `NewSessionResponse`, `EmailAdresse`, `anlage_aus_zeile`, `AuthenticatedBenutzer`?**
  _High betweenness centrality (0.267) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `Erfasser`, `ExecutionContext`, `Session`, `SQLiteRundeRepository`, `cases/konto.rs`, `Konto`, `ParkourId`, `cases/stammdaten.rs`, `domain/runde.rs`, `AccessTokenClaims`, `RundeIn<S>`, `jwt.rs`, `SQLiteSchussRepository`, `DateTime`, `NeuesKonto`, `SQLiteBenutzerRepository`, `AnlageId`, `Versioned`, `StammdatenFehler`, `Anlage`, `LogandaimApiSpec`, `anlage/mod.rs`, `RepositoryResult`, `Benutzer`, `anlage_aus_zeile`, `ZielId`, `ParkourSnapshot`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._
- **Why does `NonEmptyString` connect `NonEmptyString` to `ExecutionContext`, `parkour.rs`, `String`, `NeueAnlage`, `anlage/mod.rs`, `StammdatenFehler`, `ParkourId`, `Anlage`, `domain/common.rs`, `cases/stammdaten.rs`, `anlage`, `ParkourSnapshot`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 16 inferred relationships involving `AnlageId` (e.g. with `.anlage_anzeigen()` and `.anlage_deaktivieren()`) actually correct?**
  _`AnlageId` has 16 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `ApiTag` to the rest of the system?**
  _505 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09885057471264368 - nodes in this community are weakly interconnected._