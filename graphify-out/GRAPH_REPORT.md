# Graph Report - logandaim.at  (2026-10-30)

## Corpus Check
- 289 files · ~186,518 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4216 nodes · 11145 edges · 183 communities (160 shown, 14 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 263 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a8f2cbbe`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PdfRenderer
- arc_up
- charter.md
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- useAuth
- ExecutionContext
- StatusCode
- anlage/page.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- package.json
- SQLiteRundeRepository
- String
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
- EmailBestätigung
- jwt.rs
- neuer_benutzer
- cases/konto.rs
- verwaltung/parkour/page.tsx
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- PasswortWiederherstellung
- KlartextPasswort
- parkour.rs
- SessionAusgestellt
- .get_current_version
- Vertical slices
- AnlageAdresse
- ObjectStore
- components.json
- domain/runde.rs
- domain/common.rs
- SQLiteSchussRepository
- domain/kontakt.rs
- Signature
- ParkourInhalt
- logandaim.at — Product Charter
- dependencies
- Session
- Self
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
- LockedUnitOfWorkImpl
- diffx-finish-review
- Harness
- FakeDatastore
- Next.js
- postcss.config.mjs
- SchussId
- StammdatenFehler
- Contributing and project management
- Anlage
- HttpLogandaimApi
- schema/stammdaten.rs
- mailpit.rs
- SQLiteEmailBestätigungRepository
- FakeUnitOfWork
- ZielId
- ParkourSnapshot
- .new
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
- Versioned<T>
- CookieConfig
- revoke_all_sessions
- cn
- NewSessionResponse
- SQLiteUnitOfWork
- anlage/mod.rs
- EmailAdresse
- app-shell.tsx
- Versioned
- BenutzerId
- parse_uuid
- pull_request_template.md
- LogandaimApi
- RundeId
- layout.tsx
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- AuthenticatedBenutzer
- NeueRunde
- logandaim-api.ts
- Anlage-Anfrage — Requesting a new Anlage
- ParkourÖffentlich
- format_datetime
- StructuredError
- StreamBinaryResponse
- UnitOfWorkImpl
- parkour_mit_einem_ziel
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- .authenticated
- domain/adresse.rs
- spend_open_recoveries
- dates.ts
- sections.tsx
- SQLiteKontoRepository
- UnitOfWork
- Box
- openapi_service
- in_memory_throttle.rs
- apply_up_migrations
- instrumented.rs
- .wertung
- InMemorySigning
- ADR-0002: Session cookie auth across frontend and API origins
- LogandaimApiSpec
- m1-overseer.md
- Modes
- AccessTokenClaims
- schema.ts
- RundeIn<S>
- m1-reviewer.md
- Erfasser
- Deferred capabilities
- InstrumentedUnitOfWork
- schema/schuss.rs
- Signing
- tests/throttle.rs
- user.rs
- client_key
- Migration
- StreamBody
- api/index.ts
- .auth_context
- Specifications
- RundeStartenRequest
- .finish
- .begin
- Wertungszone
- mise
- spec.rs
- .anlagen_anzeigen
- application/auth.rs

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 165 edges
2. `cn()` - 111 edges
3. `BenutzerId` - 110 edges
4. `AnlageId` - 98 edges
5. `ZielId` - 87 edges
6. `ParkourId` - 62 edges
7. `react` - 58 edges
8. `AuthContext` - 55 edges
9. `FakeDatastore` - 54 edges
10. `NonEmptyString` - 53 edges

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

## Communities (183 total, 14 thin omitted)

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

### Community 6 - "useAuth"
Cohesion: 0.07
Nodes (52): AuthProvider(), useAuth(), apiErrorMessage(), useErfolgsmeldung(), useAnlageMutation(), useStammdatenMutations(), useAnlagenQuery(), useAnlagenVerzeichnisQuery() (+44 more)

### Community 7 - "ExecutionContext"
Cohesion: 0.13
Nodes (38): ExecutionContext, UseCase, aktive_anlage(), aktive_anlage_ändern(), AnlageAnsicht, AnlageBeschreibendeDatenÄndern, AnlageDeaktivieren, AnlageErstellen (+30 more)

### Community 8 - "StatusCode"
Cohesion: 0.21
Nodes (14): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+6 more)

### Community 9 - "anlage/page.tsx"
Cohesion: 0.11
Nodes (26): Pflock, leeresZiel, PFLOCK_FARBE_STANDARD, PFLOCK_FARBEN, PflockFarbeFeld(), PflockFarbeFeldProps, PflockMarker(), PflockSwatch() (+18 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "package.json"
Cohesion: 0.05
Nodes (38): name, private, version, ComposedPageSection(), DemoPage(), CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS (+30 more)

### Community 14 - "SQLiteRundeRepository"
Cohesion: 0.15
Nodes (21): ersetze_referenzen(), PflockDto, Arc, Erfasser, Mutex, Option, RepositoryError, RepositoryResult (+13 more)

### Community 15 - "String"
Cohesion: 0.12
Nodes (26): From, String, AccessToken, basis64url(), basis64url_decode(), constant_time_eq(), debug_gibt_das_token_nicht_preis(), hash_ist_stabil_und_vergleicht_konstantzeitig() (+18 more)

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
Cohesion: 0.11
Nodes (10): AnlageIn<S>, AnlagenSicht, ist_systemadministrator_oder_verwalter(), Abstand, Benutzer, Option, Parkour, Zielverwendung (+2 more)

### Community 20 - "Clock"
Cohesion: 0.09
Nodes (17): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+9 more)

### Community 21 - "cases/stammdaten.rs"
Cohesion: 0.12
Nodes (54): anlage_ansicht(), runde_ansicht(), ac10_deaktivieren_verhindert_neue_runden_und_haelt_alte_lesbar(), ac10_ein_mitarbeiter_deaktiviert_nicht(), ac11_ein_archiviertes_ziel_verweigert_jede_aenderung(), ac11_ein_referenzierter_parkour_wird_archiviert(), ac11_ein_referenziertes_ziel_wird_archiviert_nicht_geloescht(), ac11_eine_deaktivierte_anlage_verweigert_schreibzugriffe() (+46 more)

### Community 22 - "auth-pages.tsx"
Cohesion: 0.13
Nodes (8): useAuthMutations(), AuthFailureMessages, AuthPageProps, LoginPage(), PasswordPage(), passwordWithinPolicy(), RegistrationPage(), useMailToken()

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

### Community 31 - "EmailBestätigung"
Cohesion: 0.17
Nodes (14): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+6 more)

### Community 32 - "jwt.rs"
Cohesion: 0.17
Nodes (23): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), claims(), decode(), DecodedToken, encode(), encode_session() (+15 more)

### Community 33 - "neuer_benutzer"
Cohesion: 0.06
Nodes (70): assert_status_ok(), base_app_builder(), befördere_zum_systemadministrator(), cookie_response(), E2eHarness, json_response(), LogandaimApiTestClient, neuer_benutzer() (+62 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.18
Nodes (38): SessionPrüfen, pruefen(), pruefen_mit(), ac10_wiederherstellung_antwortet_für_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac17_fremder_schlüssel_passt_nicht_zur_session_zeile() (+30 more)

### Community 35 - "verwaltung/parkour/page.tsx"
Cohesion: 0.11
Nodes (33): laender, ZONEN, Erfolgsmeldung(), PageChromeContext, PageChromeContextValue, SetBreadcrumbLabel(), usePageChrome(), PageHeader() (+25 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.10
Nodes (24): KontoId, from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime (+16 more)

### Community 40 - "KlartextPasswort"
Cohesion: 0.13
Nodes (21): argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gültige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password(), hash_und_verify_stimmen_überein() (+13 more)

### Community 42 - "parkour.rs"
Cohesion: 0.13
Nodes (27): ac11_archivierter_parkour_verweigert_jede_aenderung(), ac4_dasselbe_ziel_darf_nur_einmal_vorkommen(), ac6_abstaende_nehmen_vom_ersten_zum_letzten_pflock_streng_zu(), ac6_fehlende_abstaende_sind_erlaubt(), ac6_pflock_farben_sind_eindeutig(), ac6_reihenfolge_ist_unabhaengig_von_der_farbe(), drei_pflöcke(), ein_parkour_braucht_mindestens_einen_pflock() (+19 more)

### Community 44 - "SessionAusgestellt"
Cohesion: 0.11
Nodes (22): Anmeldung, AnmeldungFehler, EmailAdresse, Error, Result, claims_for_session(), new_session(), Benutzer (+14 more)

### Community 46 - ".get_current_version"
Cohesion: 0.21
Nodes (11): Box, Error, Future, Option, Output, Pin, Result, Send (+3 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "AnlageAdresse"
Cohesion: 0.20
Nodes (7): AnlageAdresse, AnlageAdresseFehler, AsRef, Ländercode, Result, Self, vollstaendige_adresse_wird_angenommen()

### Community 49 - "ObjectStore"
Cohesion: 0.07
Nodes (36): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+28 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "domain/runde.rs"
Cohesion: 0.12
Nodes (20): abgeschlossen_ohne_zeitpunkt_ist_korrupt(), abschliessen_ist_eine_einbahnstrasse(), ac8_fremde_schüsse_zaehlen_nicht(), ac8_ziele_ohne_schuss_werden_uebersprungen(), anonymisieren_laesst_den_snapshot_unberuehrt(), from_parts_roundtrip_beider_zustaende(), InBearbeitung, jetzt() (+12 more)

### Community 52 - "domain/common.rs"
Cohesion: 0.08
Nodes (21): Abstand, AbstandFehler, Farbe, farbe_keeps_channels_after_roundtrip(), farbe_roundtrips_hex(), FarbeFehler, NonEmptyStringFehler, Display (+13 more)

### Community 53 - "SQLiteSchussRepository"
Cohesion: 0.17
Nodes (17): Arc, Erfasser, IntoParams, Mutex, Option, RepositoryError, RepositoryResult, Result (+9 more)

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.06
Nodes (34): alle_felder_duerfen_fehlen(), AnlageKontakt, fehlendes_protokoll_wird_zu_https(), jedes_feld_ist_einzeln_optional(), protokoll(), AsRef, Display, EmailAdresse (+26 more)

### Community 55 - "Signature"
Cohesion: 0.14
Nodes (14): falscher_schlüssel_wird_abgelehnt(), gekipptes_byte_wird_abgelehnt(), jedes_byte_ist_signierbar(), kaputtes_pem_wird_abgelehnt(), keypair(), Debug, Formatter, KontoFehler (+6 more)

### Community 56 - "ParkourInhalt"
Cohesion: 0.22
Nodes (7): ParkourListe(), parkourErstellen(), pflockHinzufuegen(), ParkourInhalt(), pflockHinzufuegen(), freiePflockFarbe(), neuerPflock()

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.04
Nodes (47): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+39 more)

### Community 59 - "Session"
Cohesion: 0.05
Nodes (50): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), hash(), jetzt(), KontoFehler (+42 more)

### Community 60 - "Self"
Cohesion: 0.12
Nodes (8): domain::AnlageVerwaltungRolle, domain::Farbe, Farbe, Error, Result, Self, TryFrom, zustand()

### Community 61 - "NeuesKonto"
Cohesion: 0.18
Nodes (11): email(), from_parts_roundtrip(), hash(), jetzt(), neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime, EmailAdresse (+3 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "Mail"
Cohesion: 0.06
Nodes (34): AsyncSmtpTransport, NullMailSender, ResultReport, adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), Mail, MailArt (+26 more)

### Community 64 - "SQLiteBenutzerRepository"
Cohesion: 0.26
Nodes (9): benutzer_from_row(), Arc, Benutzer, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 65 - "AnlageId"
Cohesion: 0.11
Nodes (16): LogandaimAppApi, Anlage, AnlageZugriff, AnlageÖffentlich, Arc, Benutzer, ParkourÖffentlich, ResultReport (+8 more)

### Community 66 - "Runde"
Cohesion: 0.08
Nodes (36): Abstand, DateTime, Erfasser, Farbe, From, Option, Self, Utc (+28 more)

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

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "Harness"
Cohesion: 0.17
Nodes (12): build(), fremder_schluessel(), Harness, jetzt(), AppBuilder, Arc, DateTime, Duration (+4 more)

### Community 77 - "FakeDatastore"
Cohesion: 0.08
Nodes (24): Konto, FakeAnlageRepository, FakeBenutzerRepository, FakeDatastore, FakeEmailBestätigungRepository, FakeKontoRepository, FakeRundeRepository, FakeSessionRepository (+16 more)

### Community 80 - "SchussId"
Cohesion: 0.42
Nodes (4): SchussId, FakeSchussRepository, Schuss, Vec

### Community 81 - "StammdatenFehler"
Cohesion: 0.14
Nodes (35): AnlageAdresseSchema, AnlageKontaktSchema, adresse(), anlage_kontext(), AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest, farbe() (+27 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "Anlage"
Cohesion: 0.07
Nodes (23): adresse(), Aktiv, Anlage, AnlageArchiviert, AnlageIn, AnlageName, AnlageNameLeer, AnlageVerwaltung (+15 more)

### Community 84 - "HttpLogandaimApi"
Cohesion: 0.19
Nodes (4): HttpLogandaimApi, unwrap(), AnlageÖffentlich, AuthPhase

### Community 86 - "schema/stammdaten.rs"
Cohesion: 0.22
Nodes (24): Anlage, AnlageAdresse, AnlageKontakt, AnlageVerwaltung, AnlageVerwaltungRolle, AnlageZugriff, AnlageZustand, AnlageÖffentlich (+16 more)

### Community 87 - "mailpit.rs"
Cohesion: 0.28
Nodes (5): basis_url(), letzter_text_an(), nachrichten(), Value, Vec

### Community 88 - "SQLiteEmailBestätigungRepository"
Cohesion: 0.21
Nodes (10): bestätigung_from_row(), Arc, EmailAdresse, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 89 - "FakeUnitOfWork"
Cohesion: 0.17
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 90 - "ZielId"
Cohesion: 0.10
Nodes (21): aendern_haelt_identitaet_und_nummer(), archiviertes_ziel_verweigert_jede_aenderung(), die_zielnummer_unterscheidet_gleichnamige_ziele(), from_parts_roundtrip(), name(), NeuesZiel, nummer(), Err (+13 more)

### Community 91 - "ParkourSnapshot"
Cohesion: 0.11
Nodes (29): ac1_neue_anlage_ist_aktiv_und_ohne_administrator(), die_summe_delegiert_die_gemeinsamen_zugriffe(), anlage(), archivistand_bleibt_im_snapshot_unberuehrt(), from_parts_roundtrip(), name(), ParkourSnapshot, pflock() (+21 more)

### Community 92 - ".new"
Cohesion: 0.28
Nodes (10): Anlage, Benutzer, Json, Path, Runde, Schuss, Uuid, TypicalJsonResponse (+2 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "e2e/konto.rs"
Cohesion: 0.13
Nodes (20): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+12 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
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

### Community 106 - "Versioned<T>"
Cohesion: 0.10
Nodes (14): Clone, Debug, Deref, Option, PartialEq, Self, T, Target (+6 more)

### Community 107 - "CookieConfig"
Cohesion: 0.19
Nodes (16): attribute(), clear_cookie_header(), CookieConfig, das_löschen_setzt_max_age_auch_ohne_lebensdauer(), das_löschen_trifft_dasselbe_cookie(), die_attribute_sind_vollständig(), die_lebensdauer_und_die_session_variante_setzen_max_age(), die_vorgaben_schreiben_nur_das_nötigste() (+8 more)

### Community 108 - "revoke_all_sessions"
Cohesion: 0.16
Nodes (12): Abmeldung, AbmeldungFehler, ErneuerungFehler, Erneuerungsausgang, revoke_all_sessions(), DateTime, Error, F (+4 more)

### Community 109 - "cn"
Cohesion: 0.06
Nodes (58): laender, leer, GridPaper(), Legend(), LegendEntry, RingMark, RingPlot(), ScaleBar() (+50 more)

### Community 110 - "NewSessionResponse"
Cohesion: 0.24
Nodes (15): EndSessionResponse, EndSessionResponse<T>, NewSessionResponse, refresh_cookie_config(), From, Json, NeueSession, Option (+7 more)

### Community 111 - "SQLiteUnitOfWork"
Cohesion: 0.14
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 112 - "anlage/mod.rs"
Cohesion: 0.12
Nodes (27): ac11_archivieren_entfernt_das_ziel_aus_den_parkour(), ac12_entfernen_loescht_das_ziel_und_seine_platzierungen(), ac2_die_letzte_administrator_verwaltung_kann_nicht_entzogen_werden(), ac2_eine_verwaltung_existiert_nur_einmal_pro_benutzer(), ac4_ein_parkour_verwendet_nur_ziele_dieser_anlage(), ac4_ein_ziel_kann_in_mehreren_parkour_liegen(), ac7_der_snapshot_ist_eine_wertkopie(), AnlageAktiv (+19 more)

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "app-shell.tsx"
Cohesion: 0.12
Nodes (21): Sidebar(), SidebarLink(), SidebarProps, DropdownMenu, DropdownMenuContent(), DropdownMenuGroup, DropdownMenuItem(), DropdownMenuLabel() (+13 more)

### Community 115 - "Versioned"
Cohesion: 0.07
Nodes (21): AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, RepositoryError, Anlage, Benutzer (+13 more)

### Community 116 - "BenutzerId"
Cohesion: 0.15
Nodes (15): befoerderung_ist_eine_domain_mutation(), Benutzer, BenutzerId, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle (+7 more)

### Community 117 - "parse_uuid"
Cohesion: 0.14
Nodes (29): AnlageVerwaltung, anlage_aus_zeile(), ersetze_kinder(), kinder_loeschen_sql(), lade_parkour(), lade_pflöcke(), lade_platzierungen(), lade_verwaltungen() (+21 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "LogandaimApi"
Cohesion: 0.06
Nodes (4): LogandaimApi, AnlageVerwaltungRolle, Administrator, Mitarbeiter

### Community 120 - "RundeId"
Cohesion: 0.07
Nodes (30): Wertungszone, schuss(), ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), anonymisieren_entfernt_den_benutzer(), die_faltung_ist_unabhängig_von_der_eingabereihenfolge(), ein_schuss_eines_anderen_ziels_wird_abgelehnt(), from_parts_roundtrip() (+22 more)

### Community 121 - "layout.tsx"
Cohesion: 0.15
Nodes (9): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, AppChrome(), PageChromeProvider() (+1 more)

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

### Community 126 - "NeueRunde"
Cohesion: 0.20
Nodes (8): Abgeschlossen, NeueRunde, DateTime, Erfasser, S, Utc, RundeAbgeschlossen, RundeIn

### Community 127 - "logandaim-api.ts"
Cohesion: 0.08
Nodes (24): JsonClient, AnlageAdresse, AnlageKontakt, AnlageVerwaltung, AnlageZugriff, AuthenticatedBenutzer, Benutzer, Erfasser (+16 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 131 - "format_datetime"
Cohesion: 0.12
Nodes (21): delete_versioned(), format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_version() (+13 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "StreamBinaryResponse"
Cohesion: 0.21
Nodes (17): InternalServerError, Self, C, From, Json, ObjectStream, PlainText, Report (+9 more)

### Community 134 - "UnitOfWorkImpl"
Cohesion: 0.19
Nodes (11): Box, RepositoryResult, Unimplemented, ExecutionSource, Send, Sync, UnitOfWorkImpl, UnitOfWorkProvider (+3 more)

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
Cohesion: 0.20
Nodes (4): Anlage, AnlageErstellungInput, Runde, ZielInput

### Community 139 - "domain/adresse.rs"
Cohesion: 0.22
Nodes (5): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self

### Community 140 - "spend_open_recoveries"
Cohesion: 0.11
Nodes (21): AnmeldungRequest, BenutzerLöschungRequest, PasswortVergessenRequest, PasswortWiederherstellenRequest, RegistrierungAbschlussRequest, RegistrierungStartRequest, EmailAdresse, Report (+13 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - "sections.tsx"
Cohesion: 0.07
Nodes (39): PLATES, ComponentSection(), Example(), ThemePlate(), touchTargetOutline, AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS (+31 more)

### Community 144 - "SQLiteKontoRepository"
Cohesion: 0.22
Nodes (10): konto_from_row(), Arc, EmailAdresse, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 145 - "UnitOfWork"
Cohesion: 0.17
Nodes (17): UnitOfWork, anlage_laden(), anmeldung(), benutzer_laden(), darf_anlage_lesen(), darf_anlage_verwalten(), darf_masterdaten_pflegen(), ist_nicht_gefunden() (+9 more)

### Community 146 - "Box"
Cohesion: 0.36
Nodes (3): Arc<T>, Box, RepositoryResult

### Community 147 - "openapi_service"
Cohesion: 0.39
Nodes (8): ApiTag, openapi_service(), openapi_service_from_api(), Into, IntoIterator, Item, OpenApiService, ServerObject

### Community 148 - "in_memory_throttle.rs"
Cohesion: 0.24
Nodes (17): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), fehlversuche_erschöpfen_das_budget_und_blockieren_weitere_anfragen(), gültige_anfragen_verbrauchen_das_budget_nicht(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now() (+9 more)

### Community 149 - "apply_up_migrations"
Cohesion: 0.17
Nodes (15): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationUp, MigrationError, MigrationTarget, Box, Future (+7 more)

### Community 150 - "instrumented.rs"
Cohesion: 0.24
Nodes (7): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option

### Community 151 - ".wertung"
Cohesion: 0.24
Nodes (8): ac8_die_summe_ist_die_summe_der_zielwertungen(), eine_leere_runde_hat_null(), IntoIterator, Item, Option, Schuss, Zielwertung, Rundenwertung

### Community 152 - "InMemorySigning"
Cohesion: 0.35
Nodes (5): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex()

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.11
Nodes (15): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+7 more)

### Community 154 - "LogandaimApiSpec"
Cohesion: 0.16
Nodes (12): AuthenticatedBenutzer, Cookie, LogandaimApiSpec, From, Option, RealIp, Self, Ok (+4 more)

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "AccessTokenClaims"
Cohesion: 0.19
Nodes (18): abgelaufenes_token_wird_abgelehnt(), AccessTokenClaims, claims(), fenster_mit_exp_vor_iat_wird_abgelehnt(), gültiges_fenster_wird_akzeptiert(), herkunft_muss_exakt_stimmen(), iat_in_der_zukunft_wird_abgelehnt(), iat_innerhalb_der_toleranz_wird_akzeptiert() (+10 more)

### Community 158 - "schema.ts"
Cohesion: 0.09
Nodes (20): ApiError, AnlageZustand, Aktiv, Archiviert, BenutzerRolle, Benutzer, Systemadministrator, components (+12 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "Erfasser"
Cohesion: 0.17
Nodes (5): Erfasser, DateTime, Option, Utc, Zielwertung

### Community 162 - "Deferred capabilities"
Cohesion: 0.17
Nodes (12): Account security, Administration, Auth audit, Deferred capabilities, Email change, Email delivery, Email verification, External identity providers (+4 more)

### Community 163 - "InstrumentedUnitOfWork"
Cohesion: 0.17
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 164 - "schema/schuss.rs"
Cohesion: 0.21
Nodes (12): AnonymisierterErfasser, BenutzerErfasser, domain::Wertungszone, Erfasser, DateTime, From, Self, Utc (+4 more)

### Community 166 - "Signing"
Cohesion: 0.24
Nodes (5): ResultReport, Send, Sync, Signing, SigningError

### Community 167 - "tests/throttle.rs"
Cohesion: 0.36
Nodes (9): ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac16_registrierung_ist_auf_fünf_pro_stunde_gedrosselt(), adresse(), app(), client_key(), passwort(), EmailAdresse, Report (+1 more)

### Community 168 - "user.rs"
Cohesion: 0.25
Nodes (8): Beförderung, BenutzerLöschen, BenutzerLöschenFehler, EmailAdresse, Error, Result, SystemadministratorBefördern, SystemadministratorBefördernFehler

### Community 169 - "client_key"
Cohesion: 0.30
Nodes (11): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), bearer_from_request(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), register_bearer(), Option (+3 more)

### Community 170 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 171 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 173 - ".auth_context"
Cohesion: 0.33
Nodes (4): bearer_token(), LogandaimApiSpec, Option, ResultReport

### Community 174 - "Specifications"
Cohesion: 0.29
Nodes (7): Language, Layout, Legacy, Lifecycle, Specifications, Traceability, Writing a feature spec

### Community 175 - "RundeStartenRequest"
Cohesion: 0.38
Nodes (5): From, Self, Uuid, RundeStarten, RundeStartenRequest

### Community 176 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 177 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 178 - "Wertungszone"
Cohesion: 0.40
Nodes (5): Wertungszone, Body, Kill, Miss, Spot

### Community 186 - "spec.rs"
Cohesion: 0.47
Nodes (8): bearer_schema_ist_veröffentlicht(), jede_operation_hat_genau_einen_veröffentlichten_tag(), jede_operation_und_jedes_schema_ist_beschrieben(), keine_internen_verweise_in_der_öffentlichen_spec(), operationen(), Value, Vec, spec()

### Community 189 - ".anlagen_anzeigen"
Cohesion: 0.22
Nodes (6): BearerAuthorization, AnlageZugriff, AnlageÖffentlich, ParkourÖffentlich, RundeBasic, Vec

## Knowledge Gaps
- **504 isolated node(s):** `ValidationError`, `LogandaimApiSpec`, `ApiTag`, `AnlageVerwaltungRolleUnbekannt`, `ZustandUnbekannt` (+499 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1128 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **14 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `format_datetime`, `StructuredError`, `StreamBinaryResponse`, `ExecutionContext`, `domain/adresse.rs`, `spend_open_recoveries`, `SQLiteRundeRepository`, `InMemorySigning`, `LogandaimApiSpec`, `ContextPorts`, `AccessTokenClaims`, `jwt.rs`, `neuer_benutzer`, `cases/konto.rs`, `Signing`, `tests/throttle.rs`, `KlartextPasswort`, `client_key`, `AnlageAdresse`, `ObjectStore`, `domain/common.rs`, `SQLiteSchussRepository`, `domain/kontakt.rs`, `Signature`, `Session`, `Self`, `Mail`, `AnlageId`, `Runde`, `preis.rs`, `ClientKey`, `Harness`, `StammdatenFehler`, `schema/stammdaten.rs`, `mailpit.rs`, `FakeUnitOfWork`, `ZielId`, `.new`, `e2e/konto.rs`, `config.rs`, `CookieConfig`, `NewSessionResponse`, `EmailAdresse`, `parse_uuid`, `AuthenticatedBenutzer`?**
  _High betweenness centrality (0.263) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `ExecutionContext`, `SQLiteRundeRepository`, `SQLiteKontoRepository`, `UnitOfWork`, `ParkourId`, `cases/stammdaten.rs`, `AccessTokenClaims`, `RundeIn<S>`, `jwt.rs`, `Erfasser`, `cases/konto.rs`, `SessionAusgestellt`, `domain/runde.rs`, `SQLiteSchussRepository`, `Session`, `NeuesKonto`, `application/auth.rs`, `SQLiteBenutzerRepository`, `AnlageId`, `FakeDatastore`, `SchussId`, `StammdatenFehler`, `Anlage`, `.new`, `revoke_all_sessions`, `anlage/mod.rs`, `Versioned`, `parse_uuid`, `RundeId`?**
  _High betweenness centrality (0.074) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `neuer_benutzer`, `cases/konto.rs`, `StructuredError`, `StreamBinaryResponse`, `tests/throttle.rs`, `.add_dyn`, `revoke_all_sessions`, `NewSessionResponse`, `apply_up_migrations`, `.contextualize_with`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 16 inferred relationships involving `AnlageId` (e.g. with `.anlage_anzeigen()` and `.anlage_deaktivieren()`) actually correct?**
  _`AnlageId` has 16 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `ApiTag` to the rest of the system?**
  _504 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09885057471264368 - nodes in this community are weakly interconnected._