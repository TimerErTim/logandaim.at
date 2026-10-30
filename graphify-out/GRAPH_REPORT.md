# Graph Report - logandaim.at  (2026-10-30)

## Corpus Check
- 288 files · ~185,465 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4142 nodes · 10925 edges · 182 communities (161 shown, 13 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 249 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3023c11c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PdfRenderer
- arc_up
- Specifications
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- useAuth
- ExecutionContext
- http_status.rs
- anlage/page.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- package.json
- SQLiteRundeRepository
- SecretTokenHash
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- Konto
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
- EmailBestätigung
- jwt.rs
- neuer_benutzer
- cases/konto.rs
- anlagen/detail/page.tsx
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- PasswortWiederherstellung
- SQLiteKontoRepository
- FakeObjectStore
- SessionAusgestellt
- .get_current_version
- Vertical slices
- Ländercode
- in_memory_object_store.rs
- components.json
- domain/runde.rs
- ObjectStore
- SQLiteSchussRepository
- domain/kontakt.rs
- InMemorySigning
- ParcoursInhalt
- logandaim.at — Product Charter
- dependencies
- DateTime
- charter.md
- Session
- Visual identity and design system
- Mail
- SQLiteBenutzerRepository
- AnlageId
- String
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- konto/session.rs
- preis.rs
- check-version.mjs
- ClientKey
- UnitOfWorkImpl
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- FakeDatastore
- Next.js
- postcss.config.mjs
- AnlageKontakt
- StammdatenFehler
- Contributing and project management
- AnlageName
- unwrap
- schema/stammdaten.rs
- mailpit.rs
- SchussRegistrieren
- FakeUnitOfWork
- ziel.rs
- ParcoursSnapshot
- LogandaimApiSpec
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- parcours.rs
- SQLiteInstance
- UpMigration
- config.rs
- RepoStorage
- .add_dyn
- Versioned<T>
- PflockId
- AnlageIn<S>
- cn
- FixedClock
- SQLiteUnitOfWork
- AnlageAktiv
- EmailAdresse
- app-shell.tsx
- Versioned
- BenutzerId
- parse_uuid
- pull_request_template.md
- LogandaimApi
- ZielId
- react
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- Benutzer
- Runde
- logandaim-api.ts
- Anlage-Anfrage — Requesting a new Anlage
- revoke_all_sessions
- format_datetime
- StructuredError
- StreamBinaryResponse
- App
- parcours_mit_einem_ziel
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- HttpLogandaimApi
- ParcoursId
- spend_open_recoveries
- dates.ts
- .begin
- KontoFehler
- benutzer_laden
- Anlage
- openapi_service
- in_memory_throttle.rs
- apply_up_migrations
- instrumented.rs
- StreamBody
- Deferred capabilities
- ADR-0002: Session cookie auth across frontend and API origins
- .from
- m1-overseer.md
- Modes
- AccessTokenClaims
- Wertungszone
- .benutzer_id
- m1-reviewer.md
- use_cases/runde.rs
- Parcours
- InstrumentedUnitOfWork
- Zuordnung
- ParcoursÖffentlich
- anlage/mod.rs
- user.rs
- client_key
- Migration
- provider.tsx
- .auth_context
- RundeStartenRequest
- .finish
- Release pull request
- [Unreleased]
- ParcoursName
- mise
- spec.rs
- .anlagen_verzeichnis
- SessionIn<S>
- CountryCode

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 165 edges
2. `BenutzerId` - 112 edges
3. `cn()` - 111 edges
4. `AnlageId` - 92 edges
5. `ZielId` - 81 edges
6. `ParcoursId` - 62 edges
7. `react` - 58 edges
8. `AuthContext` - 55 edges
9. `FakeDatastore` - 54 edges
10. `StammdatenFehler` - 52 edges

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

## Communities (182 total, 13 thin omitted)

### Community 0 - "PdfRenderer"
Cohesion: 0.10
Nodes (18): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, Vec, PdfDokument, PdfRenderer, PdfRenderError (+10 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "useAuth"
Cohesion: 0.11
Nodes (40): useAuth(), apiErrorMessage(), useErfolgsmeldung(), useAnlageMutation(), useAuthMutations(), useStammdatenMutations(), useAnlagenQuery(), useAnlagenVerzeichnisQuery() (+32 more)

### Community 7 - "ExecutionContext"
Cohesion: 0.14
Nodes (37): ExecutionContext, UseCase, aktive_anlage(), aktive_anlage_ändern(), AnlageAnsicht, AnlageBeschreibendeDatenÄndern, AnlageDeaktivieren, AnlageErstellen (+29 more)

### Community 8 - "http_status.rs"
Cohesion: 0.06
Nodes (45): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+37 more)

### Community 9 - "anlage/page.tsx"
Cohesion: 0.08
Nodes (41): Pflock, ZONEN, laender, leer, leeresZiel, Erfolgsmeldung(), PFLOCK_FARBE_STANDARD, PFLOCK_FARBEN (+33 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "package.json"
Cohesion: 0.05
Nodes (37): name, private, version, CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow() (+29 more)

### Community 14 - "SQLiteRundeRepository"
Cohesion: 0.15
Nodes (21): ersetze_referenzen(), PflockDto, Arc, Mutex, Option, RepositoryError, RepositoryResult, Result (+13 more)

### Community 15 - "SecretTokenHash"
Cohesion: 0.10
Nodes (26): AccessToken, basis64url(), basis64url_decode(), constant_time_eq(), debug_gibt_das_token_nicht_preis(), hash_ist_stabil_und_vergleicht_konstantzeitig(), konstantzeit_vergleich_erkennt_jedes_gekippte_byte(), parsing_ist_ein_roundtrip_über_die_bytes() (+18 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.23
Nodes (14): E, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+6 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "Konto"
Cohesion: 0.07
Nodes (42): email(), from_parts_roundtrip(), hash(), jetzt(), Konto, neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime (+34 more)

### Community 20 - "Clock"
Cohesion: 0.13
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 21 - "cases/stammdaten.rs"
Cohesion: 0.09
Nodes (67): anlage_ansicht(), build(), fremder_schluessel(), Harness, jetzt(), AppBuilder, Arc, DateTime (+59 more)

### Community 22 - "auth-pages.tsx"
Cohesion: 0.11
Nodes (9): AuthFailure, AuthFailureMessages, AuthPageProps, LoginPage(), PasswordPage(), passwordWithinPolicy(), RegistrationPage(), useMailToken() (+1 more)

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
Cohesion: 0.13
Nodes (9): ContextPorts, ExecutionContext<'a>, ExecutionSource, Arc, RepositoryResult, die_konfiguration_hält_nur_die_claim_identität(), IdentityProviderConfiguration, Into (+1 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 30 - "Konto — Benutzer, registration, and login"
Cohesion: 0.12
Nodes (16): Acceptance criteria, Approval, Decision log, Domain terms and business rules, Implementation sketches, Konto — Benutzer, registration, and login, Outcome, Problem (+8 more)

### Community 31 - "EmailBestätigung"
Cohesion: 0.11
Nodes (24): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+16 more)

### Community 32 - "jwt.rs"
Cohesion: 0.17
Nodes (23): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), claims(), decode(), DecodedToken, encode(), encode_session() (+15 more)

### Community 33 - "neuer_benutzer"
Cohesion: 0.05
Nodes (82): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+74 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.18
Nodes (38): SessionPrüfen, pruefen(), pruefen_mit(), ac10_wiederherstellung_antwortet_für_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac17_fremder_schlüssel_passt_nicht_zur_session_zeile() (+30 more)

### Community 35 - "anlagen/detail/page.tsx"
Cohesion: 0.12
Nodes (28): RundenZeile, laender, PaperFurnitureSection(), PageChromeContext, PageChromeContextValue, SetBreadcrumbLabel(), usePageChrome(), PageHeader() (+20 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.10
Nodes (24): KontoId, from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime (+16 more)

### Community 40 - "SQLiteKontoRepository"
Cohesion: 0.21
Nodes (10): konto_from_row(), Arc, EmailAdresse, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 42 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 44 - "SessionAusgestellt"
Cohesion: 0.13
Nodes (19): Error, Result, claims_for_session(), new_session(), Benutzer, DateTime, NeueSession, Utc (+11 more)

### Community 46 - ".get_current_version"
Cohesion: 0.21
Nodes (11): Box, Error, Future, Option, Output, Pin, Result, Send (+3 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "Ländercode"
Cohesion: 0.10
Nodes (11): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self, AnlageAdresse, AnlageAdresseFehler, AsRef (+3 more)

### Community 49 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "domain/runde.rs"
Cohesion: 0.10
Nodes (18): ac8_die_faltung_ist_unabhaengig_von_der_menge_reihenfolge(), ac8_die_summe_ist_die_summe_der_zielergebnisse(), ac8_ein_ziel_ausserhalb_des_snapshots_zaehlt_nicht(), ac8_ziele_ohne_schuss_werden_uebersprungen(), eine_leere_runde_hat_null(), menge(), IntoIterator, Item (+10 more)

### Community 52 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 53 - "SQLiteSchussRepository"
Cohesion: 0.17
Nodes (17): Arc, IntoParams, Mutex, Option, RepositoryError, RepositoryResult, Result, Row (+9 more)

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.13
Nodes (16): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe(), normalisiert_ist_idempotent() (+8 more)

### Community 55 - "InMemorySigning"
Cohesion: 0.08
Nodes (24): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex(), ResultReport, Send, Sync (+16 more)

### Community 56 - "ParcoursInhalt"
Cohesion: 0.22
Nodes (7): ParcoursListe(), parcoursErstellen(), pflockHinzufuegen(), ParcoursInhalt(), pflockHinzufuegen(), freiePflockFarbe(), neuerPflock()

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.04
Nodes (47): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+39 more)

### Community 59 - "DateTime"
Cohesion: 0.15
Nodes (11): NeueSession, RefreshedSession, RenewableSession, RevokedSession, DateTime, Error, Into, Result (+3 more)

### Community 61 - "Session"
Cohesion: 0.10
Nodes (18): Uuid, Session, SessionId, FakeSessionRepository, NeueSession, option_datetime(), Arc, DateTime (+10 more)

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
Nodes (16): LogandaimAppApi, Anlage, AnlagenZeile, AnlageÖffentlich, Arc, BenutzerSuchtreffer, ParcoursÖffentlich, ResultReport (+8 more)

### Community 66 - "String"
Cohesion: 0.15
Nodes (22): From, String, DateTime, From, Option, Self, Utc, Uuid (+14 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "konto/session.rs"
Cohesion: 0.22
Nodes (17): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), hash(), jetzt(), neue_session_ist_gültig_und_läuft_in_einem_monat_ab() (+9 more)

### Community 71 - "preis.rs"
Cohesion: 0.07
Nodes (25): Menge, MengeFehler, Decimal, Result, Self, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis() (+17 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "ClientKey"
Cohesion: 0.15
Nodes (16): InstrumentedThrottle, DateTime, Utc, ClientKey, DateTime, Duration, Into, ResultReport (+8 more)

### Community 74 - "UnitOfWorkImpl"
Cohesion: 0.10
Nodes (6): LockedUnitOfWorkImpl, Box, RepositoryResult, UnitOfWork, UnitOfWorkImpl, Drop

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "FakeDatastore"
Cohesion: 0.09
Nodes (24): FakeAnlageRepository, FakeBenutzerRepository, FakeDatastore, FakeEmailBestätigungRepository, FakeKontoRepository, FakeRundeRepository, FakeSchussRepository, Anlage (+16 more)

### Community 80 - "AnlageKontakt"
Cohesion: 0.16
Nodes (12): alle_felder_duerfen_fehlen(), AnlageKontakt, jedes_feld_ist_einzeln_optional(), AsRef, EmailAdresse, Option, Result, S (+4 more)

### Community 81 - "StammdatenFehler"
Cohesion: 0.14
Nodes (33): AnlageAdresseSchema, AnlageKontaktSchema, adresse(), AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest, kontakt(), name() (+25 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "AnlageName"
Cohesion: 0.11
Nodes (13): adresse(), AnlageIn, AnlageName, AnlageNameLeer, AnlageVerwaltung, NeueAnlage, AnlageAdresse, AnlageKontakt (+5 more)

### Community 84 - "unwrap"
Cohesion: 0.16
Nodes (4): ApiError, unwrap(), AuthPhase, StructuredError

### Community 86 - "schema/stammdaten.rs"
Cohesion: 0.10
Nodes (33): AnlagenZeileAnsicht, Anlage, AnlageAdresse, AnlageKontakt, AnlagenZeile, AnlageVerwaltung, AnlageVerwaltungRolle, AnlageZustand (+25 more)

### Community 87 - "mailpit.rs"
Cohesion: 0.28
Nodes (5): basis_url(), letzter_text_an(), nachrichten(), Value, Vec

### Community 88 - "SchussRegistrieren"
Cohesion: 0.14
Nodes (14): Report, Result, Uuid, Wertungszone, SchussErstellungRequest, Error, From, Result (+6 more)

### Community 89 - "FakeUnitOfWork"
Cohesion: 0.22
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 90 - "ziel.rs"
Cohesion: 0.10
Nodes (17): aendern_haelt_die_identitaet(), archiviertes_ziel_verweigert_jede_aenderung(), from_parts_roundtrip(), NeuesZiel, AsRef, Err, FromStr, Result (+9 more)

### Community 91 - "ParcoursSnapshot"
Cohesion: 0.11
Nodes (22): from_parts_roundtrip(), leerer_name_wird_abgelehnt(), ParcoursSnapshot, pflock(), quelle(), AsRef, Into, Option (+14 more)

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.23
Nodes (14): LogandaimApiSpec, Anlage, AnlagenZeile, BenutzerSuchtreffer, From, Json, Path, Runde (+6 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "parcours.rs"
Cohesion: 0.16
Nodes (13): ac11_archivierter_parcours_verweigert_jede_aenderung(), ac4_dasselbe_ziel_darf_nur_einmal_vorkommen(), ac6_abstaende_nehmen_von_fern_nach_nah_streng_ab(), ac6_fehlende_abstaende_sind_erlaubt(), ac6_pflock_farben_sind_eindeutig(), ac6_reihenfolge_ist_unabhaengig_von_der_farbe(), drei_pflöcke(), ein_parcours_braucht_mindestens_einen_pflock() (+5 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (13): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+5 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedAnlageRepository, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository (+4 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 106 - "Versioned<T>"
Cohesion: 0.10
Nodes (14): Clone, Debug, Deref, Option, Self, T, Target, UnitOfWork<'a> (+6 more)

### Community 107 - "PflockId"
Cohesion: 0.23
Nodes (10): Abstand, NeuerParcours, Pflock, PflockId, Option, Self, Uuid, Vec (+2 more)

### Community 108 - "AnlageIn<S>"
Cohesion: 0.13
Nodes (9): AnlageIn<S>, AnlagenSicht, AnlageVerwaltungRolle, ist_systemadministrator_oder_verwalter(), Benutzer, FromStr, Option, Parcours (+1 more)

### Community 109 - "cn"
Cohesion: 0.04
Nodes (87): PLATES, ComponentSection(), Example(), ThemePlate(), touchTargetOutline, ComposedPageSection(), AA_NORMAL_TEXT, contrastRatio() (+79 more)

### Community 110 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Duration, Mutex, NaiveDate, Self, Utc

### Community 111 - "SQLiteUnitOfWork"
Cohesion: 0.18
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteUnitOfWork

### Community 112 - "AnlageAktiv"
Cohesion: 0.23
Nodes (6): AnlageAktiv, AnlageFehler, Err, Pflock, Result, Zielplatzierung

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "app-shell.tsx"
Cohesion: 0.14
Nodes (21): Sidebar(), SidebarLink(), SidebarProps, DropdownMenu, DropdownMenuContent(), DropdownMenuGroup, DropdownMenuItem(), DropdownMenuLabel() (+13 more)

### Community 115 - "Versioned"
Cohesion: 0.07
Nodes (21): AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, RepositoryError, Anlage, Benutzer (+13 more)

### Community 116 - "BenutzerId"
Cohesion: 0.11
Nodes (17): id(), ResultReport, befoerderung_ist_eine_domain_mutation(), Benutzer, BenutzerId, from_parts_roundtrip(), jetzt(), NeuerBenutzer (+9 more)

### Community 117 - "parse_uuid"
Cohesion: 0.15
Nodes (29): AnlageVerwaltung, anlage_aus_zeile(), ersetze_kinder(), kinder_loeschen_sql(), lade_parcours(), lade_pflöcke(), lade_platzierungen(), lade_verwaltungen() (+21 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "LogandaimApi"
Cohesion: 0.06
Nodes (4): LogandaimApi, AnlageVerwaltungRolle, Administrator, Mitarbeiter

### Community 120 - "ZielId"
Cohesion: 0.07
Nodes (32): ZielId, ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), anonymisieren_entfernt_den_benutzer(), die_faltung_ist_unabhängig_von_der_eingabereihenfolge(), ergebnis(), from_parts_roundtrip(), menge() (+24 more)

### Community 121 - "react"
Cohesion: 0.10
Nodes (16): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, AuthProvider(), checking, useAuthPhase(), geistMono, inter, lora (+8 more)

### Community 122 - "Appendix: implementation sketch (non-normative)"
Cohesion: 0.17
Nodes (12): ADR-0004: JWT access tokens with rotating refresh tokens, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Crypto is domain code (sketch) (+4 more)

### Community 123 - "Stammdaten — Master data and `Anlage` administration"
Cohesion: 0.15
Nodes (13): Acceptance criteria, Decision log, Domain terms and business rules, Draft — Anlagen-Verzeichnis, Outcome, Problem, Requirements, Revision history (+5 more)

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 125 - "Benutzer"
Cohesion: 0.33
Nodes (9): Benutzer, BenutzerRolle, BenutzerSuchtreffer, NeueSession, EmailAdresse, From, Self, Uuid (+1 more)

### Community 126 - "Runde"
Cohesion: 0.12
Nodes (16): abschliessen_ist_einmalig(), anonymisieren_laesst_den_snapshot_unberuehrt(), jetzt(), NeueRunde, DateTime, Err, FromStr, Option (+8 more)

### Community 127 - "logandaim-api.ts"
Cohesion: 0.06
Nodes (42): RFC-3339, JsonClient, AnlageAdresse, AnlageErstellungInput, AnlageKontakt, AnlagenZeile, AnlageVerwaltung, AnlageÖffentlich (+34 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "revoke_all_sessions"
Cohesion: 0.16
Nodes (12): Abmeldung, AbmeldungFehler, ErneuerungFehler, Erneuerungsausgang, revoke_all_sessions(), DateTime, Error, F (+4 more)

### Community 131 - "format_datetime"
Cohesion: 0.16
Nodes (21): delete_versioned(), format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_version() (+13 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "StreamBinaryResponse"
Cohesion: 0.21
Nodes (17): InternalServerError, Self, C, From, Json, ObjectStream, PlainText, Report (+9 more)

### Community 134 - "App"
Cohesion: 0.11
Nodes (16): main(), Box, RepositoryResult, Unimplemented, App, Arc, Box, Context (+8 more)

### Community 135 - "parcours_mit_einem_ziel"
Cohesion: 0.26
Nodes (21): base_app_builder(), AppBuilder, SetUowProvider, abschliessen_ist_einmalig_und_setzt_den_zustand(), adresse(), benutzer_mit_ip(), client_key(), ein_schuss_auf_ein_ziel_ausserhalb_des_snapshots_wird_abgelehnt() (+13 more)

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.40
Nodes (5): ADR-0001: Baseline architecture, Appendix: spelling German identifiers, Consequences, Context, Decision

### Community 138 - "HttpLogandaimApi"
Cohesion: 0.19
Nodes (4): HttpLogandaimApi, Anlage, Runde, ZielInput

### Community 139 - "ParcoursId"
Cohesion: 0.56
Nodes (8): ac11_archivieren_entfernt_das_ziel_aus_den_parcours(), ac12_entfernen_loescht_das_ziel_und_seine_platzierungen(), ac4_ein_parcours_verwendet_nur_ziele_dieser_anlage(), ac4_ein_ziel_kann_in_mehreren_parcours_liegen(), ac7_der_snapshot_ist_eine_wertkopie(), parcours_neu(), ziel_id(), ParcoursId

### Community 140 - "spend_open_recoveries"
Cohesion: 0.09
Nodes (24): AnmeldungRequest, BenutzerLöschungRequest, PasswortVergessenRequest, PasswortWiederherstellenRequest, RegistrierungAbschlussRequest, RegistrierungStartRequest, EmailAdresse, Report (+16 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - ".begin"
Cohesion: 0.28
Nodes (6): Box, RepositoryError, RepositoryResult, ResultReport, Self, SQLiteInstance

### Community 144 - "KontoFehler"
Cohesion: 0.67
Nodes (3): KontoFehler, From, SessionNotRenewable

### Community 145 - "benutzer_laden"
Cohesion: 0.20
Nodes (15): anlage_laden(), anmeldung(), benutzer_laden(), darf_anlage_lesen(), darf_anlage_verwalten(), darf_masterdaten_pflegen(), ist_nicht_gefunden(), Anlage (+7 more)

### Community 146 - "Anlage"
Cohesion: 0.21
Nodes (7): Anlage, AnlageArchiviert, Archiviert, DateTime, From, Utc, Zielverwendung

### Community 147 - "openapi_service"
Cohesion: 0.39
Nodes (8): ApiTag, openapi_service(), openapi_service_from_api(), Into, IntoIterator, Item, OpenApiService, ServerObject

### Community 148 - "in_memory_throttle.rs"
Cohesion: 0.25
Nodes (17): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), fehlversuche_erschöpfen_das_budget_und_blockieren_weitere_anfragen(), gültige_anfragen_verbrauchen_das_budget_nicht(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now() (+9 more)

### Community 149 - "apply_up_migrations"
Cohesion: 0.20
Nodes (10): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationUp, MigrationError, MigrationTarget, Item, Option (+2 more)

### Community 150 - "instrumented.rs"
Cohesion: 0.24
Nodes (8): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option, ResultReport

### Community 151 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 152 - "Deferred capabilities"
Cohesion: 0.17
Nodes (12): Account security, Administration, Auth audit, Deferred capabilities, Email change, Email delivery, Email verification, External identity providers (+4 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.20
Nodes (10): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+2 more)

### Community 154 - ".from"
Cohesion: 0.17
Nodes (10): Cookie, Benutzer, Option, RealIp, Self, Ok, Default, Example (+2 more)

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "AccessTokenClaims"
Cohesion: 0.19
Nodes (18): abgelaufenes_token_wird_abgelehnt(), AccessTokenClaims, claims(), fenster_mit_exp_vor_iat_wird_abgelehnt(), gültiges_fenster_wird_akzeptiert(), herkunft_muss_exakt_stimmen(), iat_in_der_zukunft_wird_abgelehnt(), iat_innerhalb_der_toleranz_wird_akzeptiert() (+10 more)

### Community 158 - "Wertungszone"
Cohesion: 0.40
Nodes (5): Wertungszone, Body, Kill, Miss, Spot

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "use_cases/runde.rs"
Cohesion: 0.19
Nodes (14): Error, Result, Runde, Schuss, Self, Vec, RundeAbschliessen, RundeAbschliessenFehler (+6 more)

### Community 162 - "Parcours"
Cohesion: 0.41
Nodes (3): Parcours, ParcoursFehler, Result

### Community 163 - "InstrumentedUnitOfWork"
Cohesion: 0.17
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 164 - "Zuordnung"
Cohesion: 0.26
Nodes (9): domain::Wertungszone, From, Option, Self, Uuid, Schuss, Wertungszone, Zielergebnis (+1 more)

### Community 167 - "anlage/mod.rs"
Cohesion: 0.23
Nodes (9): ac2_die_letzte_administrator_verwaltung_kann_nicht_entzogen_werden(), ac2_eine_verwaltung_existiert_nur_einmal_pro_benutzer(), Aktiv, AnlageVerwaltungRolleUnbekannt, benutzer(), deaktivieren_ist_eine_einbahnstrasse(), eine_mitarbeiter_verwaltung_kann_nicht_zum_null_administrator_fuehren(), pflock() (+1 more)

### Community 168 - "user.rs"
Cohesion: 0.25
Nodes (8): Beförderung, BenutzerLöschen, BenutzerLöschenFehler, EmailAdresse, Error, Result, SystemadministratorBefördern, SystemadministratorBefördernFehler

### Community 169 - "client_key"
Cohesion: 0.30
Nodes (11): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), bearer_from_request(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), register_bearer(), Option (+3 more)

### Community 170 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 172 - "provider.tsx"
Cohesion: 0.21
Nodes (7): createLogandaimApi(), AuthSession, LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config, @tanstack/react-query-devtools

### Community 173 - ".auth_context"
Cohesion: 0.33
Nodes (4): bearer_token(), LogandaimApiSpec, Option, ResultReport

### Community 175 - "RundeStartenRequest"
Cohesion: 0.38
Nodes (5): From, Self, Uuid, RundeStarten, RundeStartenRequest

### Community 176 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 177 - "Release pull request"
Cohesion: 0.33
Nodes (5): Checklist, Release description, Release pull request, Test evidence, Version

### Community 178 - "[Unreleased]"
Cohesion: 0.50
Nodes (4): Added, Changelog, Removed, [Unreleased]

### Community 180 - "ParcoursName"
Cohesion: 0.19
Nodes (4): ParcoursName, PflockFarbe, AsRef, S

### Community 186 - "spec.rs"
Cohesion: 0.27
Nodes (11): bearer_schema_ist_veröffentlicht(), jede_operation_hat_genau_einen_veröffentlichten_tag(), jede_operation_und_jedes_schema_ist_beschrieben(), keine_internen_verweise_in_der_öffentlichen_spec(), operationen(), Box, RepositoryResult, Value (+3 more)

### Community 189 - ".anlagen_verzeichnis"
Cohesion: 0.40
Nodes (4): BearerAuthorization, AnlageÖffentlich, ParcoursÖffentlich, Vec

### Community 191 - "CountryCode"
Cohesion: 0.50
Nodes (4): CountryCode, AT, CH, DE

## Knowledge Gaps
- **507 isolated node(s):** `ValidationError`, `LogandaimApiSpec`, `ApiTag`, `AnlageVerwaltungRolleUnbekannt`, `ZustandUnbekannt` (+502 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1132 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **13 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `format_datetime`, `StructuredError`, `StreamBinaryResponse`, `ExecutionContext`, `http_status.rs`, `spend_open_recoveries`, `SQLiteRundeRepository`, `SecretTokenHash`, `Konto`, `cases/stammdaten.rs`, `.from`, `ContextPorts`, `AccessTokenClaims`, `jwt.rs`, `neuer_benutzer`, `Parcours`, `cases/konto.rs`, `Zuordnung`, `client_key`, `FakeObjectStore`, `Ländercode`, `in_memory_object_store.rs`, `ParcoursName`, `SQLiteSchussRepository`, `domain/kontakt.rs`, `InMemorySigning`, `DateTime`, `Session`, `Mail`, `AnlageId`, `konto/session.rs`, `preis.rs`, `ClientKey`, `AnlageKontakt`, `StammdatenFehler`, `AnlageName`, `schema/stammdaten.rs`, `mailpit.rs`, `FakeUnitOfWork`, `ziel.rs`, `ParcoursSnapshot`, `LogandaimApiSpec`, `config.rs`, `EmailAdresse`, `parse_uuid`, `Benutzer`?**
  _High betweenness centrality (0.255) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `revoke_all_sessions`, `ExecutionContext`, `SQLiteRundeRepository`, `benutzer_laden`, `Konto`, `cases/stammdaten.rs`, `AccessTokenClaims`, `.benutzer_id`, `jwt.rs`, `cases/konto.rs`, `anlage/mod.rs`, `SQLiteKontoRepository`, `SessionAusgestellt`, `domain/runde.rs`, `SQLiteSchussRepository`, `DateTime`, `Session`, `SessionIn<S>`, `SQLiteBenutzerRepository`, `AnlageId`, `konto/session.rs`, `FakeDatastore`, `StammdatenFehler`, `AnlageName`, `LogandaimApiSpec`, `AnlageIn<S>`, `AnlageAktiv`, `Versioned`, `parse_uuid`, `ZielId`, `Runde`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **Why does `ZielId` connect `ZielId` to `ExecutionContext`, `parcours_mit_einem_ziel`, `ParcoursId`, `SQLiteRundeRepository`, `Anlage`, `cases/stammdaten.rs`, `Parcours`, `domain/runde.rs`, `SQLiteSchussRepository`, `AnlageId`, `String`, `FakeDatastore`, `StammdatenFehler`, `SchussRegistrieren`, `ziel.rs`, `ParcoursSnapshot`, `LogandaimApiSpec`, `parcours.rs`, `PflockId`, `AnlageIn<S>`, `AnlageAktiv`, `Versioned`, `parse_uuid`, `Runde`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 15 inferred relationships involving `AnlageId` (e.g. with `.anlage_anzeigen()` and `.anlage_deaktivieren()`) actually correct?**
  _`AnlageId` has 15 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `ApiTag` to the rest of the system?**
  _507 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09885057471264368 - nodes in this community are weakly interconnected._