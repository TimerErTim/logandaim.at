# Graph Report - logandaim.at  (2026-10-30)

## Corpus Check
- 284 files · ~182,295 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4124 nodes · 10853 edges · 192 communities (172 shown, 12 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 250 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7b924348`
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
- neu/page.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- package.json
- e2e/konto.rs
- token.rs
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- KlartextPasswort
- Clock
- cases/stammdaten.rs
- NeuesKonto
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
- verwaltung/parcours/page.tsx
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- PasswortWiederherstellung
- Konto
- String
- spend_open_confirmations
- .get_current_version
- Vertical slices
- Ländercode
- in_memory_object_store.rs
- components.json
- Punkte
- domain/runde.rs
- SQLiteSchussRepository
- domain/kontakt.rs
- SigningKeypair
- sections.tsx
- logandaim.at — Product Charter
- dependencies
- DateTime
- charter.md
- Versioned
- Visual identity and design system
- Mail
- AnlageVerwaltungRolle
- AnlageId
- Runde
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- konto/session.rs
- preis.rs
- check-version.mjs
- ClientKey
- LockedUnitOfWorkImpl
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- FakeDatastore
- Next.js
- postcss.config.mjs
- AnlageKontakt
- StammdatenFehler
- Contributing and project management
- Anlage
- HttpLogandaimApi
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
- NewSessionResponse
- SecretTokenHash
- AnlageAktiv
- EmailAdresse
- app-shell.tsx
- RepositoryResult
- Benutzer
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
- InMemorySigning
- parcours_mit_einem_ziel
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- .authenticated
- ParcoursId
- requests/konto.rs
- dates.ts
- .commit
- spend_open_recoveries
- benutzer_laden
- .from_parts
- openapi_service
- in_memory_throttle.rs
- apply_up_migrations
- instrumented.rs
- FakeObjectStore
- Deferred capabilities
- ADR-0002: Session cookie auth across frontend and API origins
- .from
- m1-overseer.md
- Modes
- AccessTokenClaims
- Wertungszone
- ObjectStore
- m1-reviewer.md
- use_cases/runde.rs
- Parcours
- InstrumentedUnitOfWork
- Zuordnung
- AnlageName
- UnitOfWorkImpl
- anlage/mod.rs
- user.rs
- client_key
- Migration
- AnlageIn
- provider.tsx
- AccessError
- Box
- RundeStartenRequest
- .finish
- Release pull request
- [Unreleased]
- devDependencies
- ParcoursName
- mise
- claims_for_session
- data-figures.tsx
- .neu
- tests/throttle.rs
- spec.rs
- .poll_next
- scripts
- .anlagen_verzeichnis
- SessionIn<S>
- CountryCode

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 165 edges
2. `BenutzerId` - 112 edges
3. `cn()` - 107 edges
4. `AnlageId` - 92 edges
5. `ZielId` - 81 edges
6. `ParcoursId` - 62 edges
7. `react` - 56 edges
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

## Communities (192 total, 12 thin omitted)

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
Cohesion: 0.09
Nodes (42): useAuth(), apiErrorMessage(), useAnlageMutation(), useAuthMutations(), useStammdatenMutations(), useAnlagenQuery(), useAnlagenVerzeichnisQuery(), useAnlageQuery() (+34 more)

### Community 7 - "ExecutionContext"
Cohesion: 0.14
Nodes (37): ExecutionContext, UnitOfWork, UseCase, aktive_anlage(), aktive_anlage_ändern(), AnlageAnsicht, AnlageBeschreibendeDatenÄndern, AnlageDeaktivieren (+29 more)

### Community 8 - "http_status.rs"
Cohesion: 0.12
Nodes (14): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+6 more)

### Community 9 - "neu/page.tsx"
Cohesion: 0.11
Nodes (15): laender, leer, AuthError(), LoginPage(), PasswordPage(), RegistrationPage(), useMailToken(), Card() (+7 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.20
Nodes (19): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing(), put_get_roundtrip() (+11 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "package.json"
Cohesion: 0.04
Nodes (43): name, private, version, ComposedPageSection(), DemoPage(), CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS (+35 more)

### Community 14 - "e2e/konto.rs"
Cohesion: 0.14
Nodes (19): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+11 more)

### Community 15 - "token.rs"
Cohesion: 0.15
Nodes (18): AccessToken, basis64url(), basis64url_decode(), constant_time_eq(), debug_gibt_das_token_nicht_preis(), hash_ist_stabil_und_vergleicht_konstantzeitig(), konstantzeit_vergleich_erkennt_jedes_gekippte_byte(), parsing_ist_ein_roundtrip_über_die_bytes() (+10 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.23
Nodes (14): E, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+6 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "KlartextPasswort"
Cohesion: 0.15
Nodes (19): argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gültige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password(), hash_und_verify_stimmen_überein() (+11 more)

### Community 20 - "Clock"
Cohesion: 0.09
Nodes (17): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+9 more)

### Community 21 - "cases/stammdaten.rs"
Cohesion: 0.13
Nodes (55): anlage_ansicht(), runde_ansicht(), ac10_deaktivieren_verhindert_neue_runden_und_haelt_alte_lesbar(), ac10_ein_mitarbeiter_deaktiviert_nicht(), ac11_ein_archiviertes_ziel_verweigert_jede_aenderung(), ac11_ein_referenzierter_parcours_wird_archiviert(), ac11_ein_referenziertes_ziel_wird_archiviert_nicht_geloescht(), ac11_eine_deaktivierte_anlage_verweigert_schreibzugriffe() (+47 more)

### Community 22 - "NeuesKonto"
Cohesion: 0.17
Nodes (13): email(), from_parts_roundtrip(), hash(), jetzt(), neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime, EmailAdresse (+5 more)

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
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 30 - "Konto — Benutzer, registration, and login"
Cohesion: 0.12
Nodes (16): Acceptance criteria, Approval, Decision log, Domain terms and business rules, Implementation sketches, Konto — Benutzer, registration, and login, Outcome, Problem (+8 more)

### Community 31 - "EmailBestätigung"
Cohesion: 0.14
Nodes (16): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+8 more)

### Community 32 - "jwt.rs"
Cohesion: 0.17
Nodes (22): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), claims(), decode(), DecodedToken, encode(), encode_session() (+14 more)

### Community 33 - "neuer_benutzer"
Cohesion: 0.06
Nodes (71): assert_status_ok(), base_app_builder(), befördere_zum_systemadministrator(), cookie_pair(), cookie_response(), E2eHarness, json_response(), LogandaimApiTestClient (+63 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.10
Nodes (52): Benutzer, SessionAusgestellt, SessionPrüfen, build(), fremder_schluessel(), Harness, jetzt(), pruefen() (+44 more)

### Community 35 - "verwaltung/parcours/page.tsx"
Cohesion: 0.11
Nodes (30): RundenZeile, laender, ZONEN, PageChromeContext, PageChromeContextValue, SetBreadcrumbLabel(), usePageChrome(), PageHeader() (+22 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.12
Nodes (22): from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime, Option (+14 more)

### Community 40 - "Konto"
Cohesion: 0.14
Nodes (13): Konto, KontoId, FakeKontoRepository, konto_from_row(), Arc, EmailAdresse, Mutex, Option (+5 more)

### Community 42 - "String"
Cohesion: 0.17
Nodes (19): From, String, attribute(), clear_cookie_header(), CookieConfig, das_löschen_setzt_max_age_auch_ohne_lebensdauer(), das_löschen_trifft_dasselbe_cookie(), die_attribute_sind_vollständig() (+11 more)

### Community 44 - "spend_open_confirmations"
Cohesion: 0.23
Nodes (10): RegistrierungAbschließen, RegistrierungStarten, RegistrierungStartenFehler, DateTime, EmailAdresse, Error, Report, Result (+2 more)

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

### Community 51 - "Punkte"
Cohesion: 0.13
Nodes (9): IntoIterator, Item, Zielergebnis, Rundenergebnis, Punkte, Add, I, Zielergebnis (+1 more)

### Community 52 - "domain/runde.rs"
Cohesion: 0.27
Nodes (9): ac8_die_faltung_ist_unabhaengig_von_der_menge_reihenfolge(), ac8_die_summe_ist_die_summe_der_zielergebnisse(), ac8_ein_ziel_ausserhalb_des_snapshots_zaehlt_nicht(), ac8_ziele_ohne_schuss_werden_uebersprungen(), eine_leere_runde_hat_null(), menge(), Wertungszone, RundeZustandUnbekannt (+1 more)

### Community 53 - "SQLiteSchussRepository"
Cohesion: 0.17
Nodes (17): Arc, IntoParams, Mutex, Option, RepositoryError, RepositoryResult, Result, Row (+9 more)

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.13
Nodes (16): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe(), normalisiert_ist_idempotent() (+8 more)

### Community 55 - "SigningKeypair"
Cohesion: 0.14
Nodes (13): falscher_schlüssel_wird_abgelehnt(), gekipptes_byte_wird_abgelehnt(), jedes_byte_ist_signierbar(), kaputtes_pem_wird_abgelehnt(), keypair(), Debug, Formatter, KontoFehler (+5 more)

### Community 56 - "sections.tsx"
Cohesion: 0.07
Nodes (41): PLATES, ComponentSection(), Example(), ThemePlate(), touchTargetOutline, AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS (+33 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.07
Nodes (28): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+20 more)

### Community 59 - "DateTime"
Cohesion: 0.17
Nodes (7): NeueSession, RefreshedSession, RenewableSession, RevokedSession, DateTime, TryFrom, Utc

### Community 61 - "Versioned"
Cohesion: 0.10
Nodes (22): id(), NeueSession, SessionRepository, Versioned, BenutzerId, Session, SessionId, FakeSessionRepository (+14 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "Mail"
Cohesion: 0.06
Nodes (34): AsyncSmtpTransport, NullMailSender, ResultReport, adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), Mail, MailArt (+26 more)

### Community 64 - "AnlageVerwaltungRolle"
Cohesion: 0.17
Nodes (6): AnlagenSicht, AnlageVerwaltung, AnlageVerwaltungRolle, ist_systemadministrator_oder_verwalter(), Benutzer, FromStr

### Community 65 - "AnlageId"
Cohesion: 0.12
Nodes (15): LogandaimAppApi, Anlage, AnlagenZeile, AnlageÖffentlich, Arc, BenutzerSuchtreffer, ParcoursÖffentlich, ResultReport (+7 more)

### Community 66 - "Runde"
Cohesion: 0.19
Nodes (16): DateTime, From, Option, Self, Utc, Uuid, Vec, Zielergebnis (+8 more)

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
Cohesion: 0.20
Nodes (17): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), jetzt(), neue_session_ist_gültig_und_läuft_in_einem_monat_ab(), Refreshed (+9 more)

### Community 71 - "preis.rs"
Cohesion: 0.07
Nodes (25): Menge, MengeFehler, Decimal, Result, Self, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis() (+17 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "ClientKey"
Cohesion: 0.17
Nodes (13): ClientKey, DateTime, Duration, Into, ResultReport, Self, Send, Sync (+5 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "FakeDatastore"
Cohesion: 0.10
Nodes (22): FakeAnlageRepository, FakeBenutzerRepository, FakeDatastore, FakePasswortWiederherstellungRepository, FakeRundeRepository, FakeSchussRepository, Anlage, Arc (+14 more)

### Community 80 - "AnlageKontakt"
Cohesion: 0.16
Nodes (12): alle_felder_duerfen_fehlen(), AnlageKontakt, jedes_feld_ist_einzeln_optional(), AsRef, EmailAdresse, Option, Result, S (+4 more)

### Community 81 - "StammdatenFehler"
Cohesion: 0.15
Nodes (32): AnlageAdresseSchema, AnlageKontaktSchema, adresse(), AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest, kontakt(), name() (+24 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "Anlage"
Cohesion: 0.15
Nodes (6): adresse(), Anlage, NeueAnlage, AnlageAdresse, AnlageKontakt, From

### Community 84 - "HttpLogandaimApi"
Cohesion: 0.18
Nodes (3): HttpLogandaimApi, unwrap(), AuthPhase

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
Cohesion: 0.20
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
Cohesion: 0.13
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
Cohesion: 0.28
Nodes (8): Abstand, NeuerParcours, Pflock, PflockId, platzierung(), Option, Vec, Zielplatzierung

### Community 108 - "AnlageIn<S>"
Cohesion: 0.21
Nodes (4): AnlageIn<S>, Option, Parcours, Ziel

### Community 109 - "cn"
Cohesion: 0.06
Nodes (58): leeresZiel, Stammdaten(), CHECKPOINTS, MESSREIHE, Row, START_ROWS, statusVariant, Zone (+50 more)

### Community 110 - "NewSessionResponse"
Cohesion: 0.18
Nodes (17): InternalServerError, Self, EndSessionResponse, EndSessionResponse<T>, NewSessionResponse, refresh_cookie_config(), From, Json (+9 more)

### Community 111 - "SecretTokenHash"
Cohesion: 0.13
Nodes (9): hash(), RefreshToken, Debug, Formatter, From, TryFrom, SecretToken, SecretTokenHash (+1 more)

### Community 112 - "AnlageAktiv"
Cohesion: 0.27
Nodes (5): AnlageVerwaltung, AnlageAktiv, AnlageFehler, Err, Result

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "app-shell.tsx"
Cohesion: 0.14
Nodes (21): Sidebar(), SidebarLink(), SidebarProps, DropdownMenu, DropdownMenuContent(), DropdownMenuGroup, DropdownMenuItem(), DropdownMenuLabel() (+13 more)

### Community 115 - "RepositoryResult"
Cohesion: 0.05
Nodes (23): AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, RepositoryError, Anlage, Benutzer (+15 more)

### Community 116 - "Benutzer"
Cohesion: 0.15
Nodes (14): befoerderung_ist_eine_domain_mutation(), Benutzer, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle, DateTime (+6 more)

### Community 117 - "parse_uuid"
Cohesion: 0.08
Nodes (49): anlage_aus_zeile(), ersetze_kinder(), kinder_loeschen_sql(), lade_parcours(), lade_pflöcke(), lade_platzierungen(), lade_verwaltungen(), lade_ziele() (+41 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "LogandaimApi"
Cohesion: 0.06
Nodes (4): LogandaimApi, AnlageVerwaltungRolle, Administrator, Mitarbeiter

### Community 120 - "ZielId"
Cohesion: 0.07
Nodes (33): ZielId, ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), anonymisieren_entfernt_den_benutzer(), die_faltung_ist_unabhängig_von_der_eingabereihenfolge(), ergebnis(), from_parts_roundtrip(), menge() (+25 more)

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
Cohesion: 0.10
Nodes (16): abschliessen_ist_einmalig(), anonymisieren_laesst_den_snapshot_unberuehrt(), jetzt(), NeueRunde, DateTime, Err, FromStr, Option (+8 more)

### Community 127 - "logandaim-api.ts"
Cohesion: 0.05
Nodes (44): RFC-3339, ApiError, JsonClient, AnlageAdresse, AnlageKontakt, AnlagenZeile, AnlageVerwaltung, AnlageÖffentlich (+36 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "revoke_all_sessions"
Cohesion: 0.16
Nodes (12): Abmeldung, AbmeldungFehler, ErneuerungFehler, Erneuerungsausgang, revoke_all_sessions(), DateTime, Error, F (+4 more)

### Community 131 - "format_datetime"
Cohesion: 0.07
Nodes (40): benutzer_from_row(), Arc, Benutzer, Mutex, Option, RepositoryResult, Row, Transaction (+32 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "StreamBinaryResponse"
Cohesion: 0.17
Nodes (20): C, From, Json, ObjectStream, PlainText, Report, Result, Self (+12 more)

### Community 134 - "InMemorySigning"
Cohesion: 0.15
Nodes (11): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex(), ResultReport, Send, Sync (+3 more)

### Community 135 - "parcours_mit_einem_ziel"
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

### Community 139 - "ParcoursId"
Cohesion: 0.31
Nodes (10): ac11_archivieren_entfernt_das_ziel_aus_den_parcours(), ac12_entfernen_loescht_das_ziel_und_seine_platzierungen(), ac4_ein_parcours_verwendet_nur_ziele_dieser_anlage(), ac4_ein_ziel_kann_in_mehreren_parcours_liegen(), ac7_der_snapshot_ist_eine_wertkopie(), parcours_neu(), Pflock, Zielplatzierung (+2 more)

### Community 140 - "requests/konto.rs"
Cohesion: 0.19
Nodes (11): AnmeldungRequest, PasswortVergessenRequest, RegistrierungAbschlussRequest, RegistrierungStartRequest, EmailAdresse, Report, Result, AnmeldungFehler (+3 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - ".commit"
Cohesion: 0.60
Nodes (3): Box, RepositoryResult, Self

### Community 144 - "spend_open_recoveries"
Cohesion: 0.19
Nodes (10): PasswortWiederherstellenRequest, PasswortVergessenFehler, PasswortWiederherstellen, PasswortWiederherstellenFehler, DateTime, Error, Report, Result (+2 more)

### Community 145 - "benutzer_laden"
Cohesion: 0.20
Nodes (15): anlage_laden(), anmeldung(), benutzer_laden(), darf_anlage_lesen(), darf_anlage_verwalten(), darf_masterdaten_pflegen(), ist_nicht_gefunden(), Anlage (+7 more)

### Community 146 - ".from_parts"
Cohesion: 0.40
Nodes (5): AnlageArchiviert, Archiviert, DateTime, Self, Utc

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
Cohesion: 0.19
Nodes (11): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, InstrumentedThrottle, Arc, DateTime, ObjectStoreError, ObjectStream (+3 more)

### Community 151 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

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

### Community 159 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "use_cases/runde.rs"
Cohesion: 0.15
Nodes (18): RundenZeile, Error, Option, Result, Runde, Schuss, Self, Vec (+10 more)

### Community 162 - "Parcours"
Cohesion: 0.33
Nodes (4): Parcours, ParcoursFehler, Result, Self

### Community 163 - "InstrumentedUnitOfWork"
Cohesion: 0.17
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 164 - "Zuordnung"
Cohesion: 0.26
Nodes (9): domain::Wertungszone, From, Option, Self, Uuid, Schuss, Wertungszone, Zielergebnis (+1 more)

### Community 165 - "AnlageName"
Cohesion: 0.25
Nodes (6): AnlageName, AsRef, AnlageÖffentlich, ParcoursKurz, ParcoursÖffentlich, Parcours

### Community 166 - "UnitOfWorkImpl"
Cohesion: 0.13
Nodes (15): main(), Box, RepositoryResult, Unimplemented, Box, RepositoryResult, Unimplemented, ExecutionSource (+7 more)

### Community 167 - "anlage/mod.rs"
Cohesion: 0.21
Nodes (10): ac2_die_letzte_administrator_verwaltung_kann_nicht_entzogen_werden(), ac2_eine_verwaltung_existiert_nur_einmal_pro_benutzer(), Aktiv, AnlageVerwaltungRolleUnbekannt, benutzer(), deaktivieren_ist_eine_einbahnstrasse(), eine_mitarbeiter_verwaltung_kann_nicht_zum_null_administrator_fuehren(), pflock() (+2 more)

### Community 168 - "user.rs"
Cohesion: 0.22
Nodes (9): BenutzerLöschungRequest, Beförderung, BenutzerLöschen, BenutzerLöschenFehler, EmailAdresse, Error, Result, SystemadministratorBefördern (+1 more)

### Community 169 - "client_key"
Cohesion: 0.30
Nodes (11): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), bearer_from_request(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), register_bearer(), Option (+3 more)

### Community 170 - "Migration"
Cohesion: 0.28
Nodes (5): Migration, Error, Option, Result, Transaction

### Community 171 - "AnlageIn"
Cohesion: 0.18
Nodes (6): AnlageIn, AnlageNameLeer, S, Vec, Zielverwendung, Zielverwendung

### Community 172 - "provider.tsx"
Cohesion: 0.21
Nodes (7): createLogandaimApi(), AuthSession, LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config, @tanstack/react-query-devtools

### Community 173 - "AccessError"
Cohesion: 0.19
Nodes (6): bearer_token(), LogandaimApiSpec, Option, ResultReport, AccessError, ResultReport

### Community 174 - "Box"
Cohesion: 0.36
Nodes (3): Arc<T>, Box, RepositoryResult

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

### Community 179 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 180 - "ParcoursName"
Cohesion: 0.24
Nodes (5): ParcoursName, PflockFarbe, AsRef, S, Self

### Community 182 - "claims_for_session"
Cohesion: 0.23
Nodes (9): Anmeldung, EmailAdresse, Error, Result, claims_for_session(), new_session(), DateTime, NeueSession (+1 more)

### Community 183 - "data-figures.tsx"
Cohesion: 0.27
Nodes (9): GridPaper(), Legend(), LegendEntry, RingMark, RingPlot(), ScaleBar(), DataFiguresSection(), SeriesPlot() (+1 more)

### Community 184 - ".neu"
Cohesion: 0.22
Nodes (7): KontoFehler, Error, From, Into, Result, Self, SessionNotRenewable

### Community 185 - "tests/throttle.rs"
Cohesion: 0.36
Nodes (9): ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac16_registrierung_ist_auf_fünf_pro_stunde_gedrosselt(), adresse(), app(), client_key(), passwort(), EmailAdresse, Report (+1 more)

### Community 186 - "spec.rs"
Cohesion: 0.47
Nodes (8): bearer_schema_ist_veröffentlicht(), jede_operation_hat_genau_einen_veröffentlichten_tag(), jede_operation_und_jedes_schema_ist_beschrieben(), keine_internen_verweise_in_der_öffentlichen_spec(), operationen(), Value, Vec, spec()

### Community 187 - ".poll_next"
Cohesion: 0.22
Nodes (8): FileStream, Context, Item, Pin, Self, File, Poll, Stream

### Community 188 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 189 - ".anlagen_verzeichnis"
Cohesion: 0.40
Nodes (4): BearerAuthorization, AnlageÖffentlich, ParcoursÖffentlich, Vec

### Community 191 - "CountryCode"
Cohesion: 0.50
Nodes (4): CountryCode, AT, CH, DE

## Knowledge Gaps
- **505 isolated node(s):** `ValidationError`, `LogandaimApiSpec`, `ApiTag`, `AnlageVerwaltungRolleUnbekannt`, `ZustandUnbekannt` (+500 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1126 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `format_datetime`, `StructuredError`, `StreamBinaryResponse`, `InMemorySigning`, `requests/konto.rs`, `e2e/konto.rs`, `token.rs`, `spend_open_recoveries`, `KlartextPasswort`, `NeuesKonto`, `FakeObjectStore`, `.from`, `ContextPorts`, `AccessTokenClaims`, `jwt.rs`, `neuer_benutzer`, `Parcours`, `use_cases/runde.rs`, `Zuordnung`, `AnlageName`, `cases/konto.rs`, `user.rs`, `client_key`, `AnlageIn`, `Ländercode`, `in_memory_object_store.rs`, `ParcoursName`, `SQLiteSchussRepository`, `domain/kontakt.rs`, `SigningKeypair`, `.neu`, `tests/throttle.rs`, `DateTime`, `Versioned`, `Mail`, `AnlageId`, `Runde`, `konto/session.rs`, `preis.rs`, `ClientKey`, `AnlageKontakt`, `StammdatenFehler`, `schema/stammdaten.rs`, `mailpit.rs`, `FakeUnitOfWork`, `ziel.rs`, `ParcoursSnapshot`, `LogandaimApiSpec`, `config.rs`, `NewSessionResponse`, `SecretTokenHash`, `EmailAdresse`, `parse_uuid`, `Benutzer`?**
  _High betweenness centrality (0.256) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `Versioned` to `revoke_all_sessions`, `format_datetime`, `ExecutionContext`, `benutzer_laden`, `cases/stammdaten.rs`, `NeuesKonto`, `AccessTokenClaims`, `jwt.rs`, `cases/konto.rs`, `anlage/mod.rs`, `Konto`, `AccessError`, `domain/runde.rs`, `SQLiteSchussRepository`, `claims_for_session`, `.neu`, `DateTime`, `SessionIn<S>`, `AnlageVerwaltungRolle`, `AnlageId`, `konto/session.rs`, `FakeDatastore`, `StammdatenFehler`, `LogandaimApiSpec`, `AnlageIn<S>`, `AnlageAktiv`, `RepositoryResult`, `Benutzer`, `parse_uuid`, `ZielId`, `Runde`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **Why does `ZielId` connect `ZielId` to `ExecutionContext`, `parcours_mit_einem_ziel`, `ParcoursId`, `cases/stammdaten.rs`, `use_cases/runde.rs`, `Parcours`, `AnlageIn`, `domain/runde.rs`, `SQLiteSchussRepository`, `AnlageId`, `FakeDatastore`, `StammdatenFehler`, `SchussRegistrieren`, `ziel.rs`, `ParcoursSnapshot`, `LogandaimApiSpec`, `parcours.rs`, `PflockId`, `AnlageIn<S>`, `AnlageAktiv`, `RepositoryResult`, `parse_uuid`, `Runde`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 15 inferred relationships involving `AnlageId` (e.g. with `.anlage_anzeigen()` and `.anlage_deaktivieren()`) actually correct?**
  _`AnlageId` has 15 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `ApiTag` to the rest of the system?**
  _505 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09885057471264368 - nodes in this community are weakly interconnected._