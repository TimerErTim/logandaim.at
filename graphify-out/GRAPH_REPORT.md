# Graph Report - logandaim.at  (2026-10-29)

## Corpus Check
- 278 files · ~169,924 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4054 nodes · 10672 edges · 169 communities (146 shown, 15 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 208 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `aedffcfc`
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
- Session
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- token-swatches.tsx
- ObjectStore
- token.rs
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- KlartextPasswort
- Clock
- ParcoursHinzufügen
- parse_uuid
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
- RepoStorage
- cases/konto.rs
- anlage/page.tsx
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- Versioned
- format_datetime
- CookieConfig
- claims_for_session
- .get_current_version
- Vertical slices
- RundeId
- konto/session.rs
- components.json
- domain/mod.rs
- App
- SQLiteSchussRepository
- domain/kontakt.rs
- InMemorySigning
- cn
- logandaim.at — Product Charter
- dependencies
- DateTime
- charter.md
- cases/stammdaten.rs
- Visual identity and design system
- Mail
- layout.tsx
- AnlageId
- use_cases/runde.rs
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- SessionAusgestellt
- preis.rs
- check-version.mjs
- ClientKey
- package.json
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- FakeDatastore
- Next.js
- postcss.config.mjs
- .finish
- StammdatenFehler
- Contributing and project management
- .anmelden
- unwrap
- schema/stammdaten.rs
- mailpit.rs
- .begin
- FakeUnitOfWork
- ziel.rs
- ParcoursSnapshot
- String
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- domain/runde.rs
- SQLiteInstance
- UpMigration
- config.rs
- Harness
- .add_dyn
- LockedUnitOfWorkImpl
- parcours.rs
- SQLiteBenutzerRepository
- UnitOfWorkImpl
- NewSessionResponse
- SecretTokenHash
- ZielId
- EmailAdresse
- app-shell.tsx
- RepositoryResult
- BenutzerId
- anlage_aus_zeile
- pull_request_template.md
- LogandaimApi
- Runde
- scripts
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- Benutzer
- Runde
- logandaim-api.ts
- Anlage-Anfrage — Requesting a new Anlage
- KontoFehler
- type-section.tsx
- StructuredError
- StreamBinaryResponse
- e2e/konto.rs
- cases/runde.rs
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- HttpLogandaimApi
- ParcoursId
- AnlageName
- dates.ts
- SQLiteUnitOfWork
- SQLiteRundeRepository
- in_memory_throttle.rs
- apply_up_migrations
- instrumented.rs
- Versioned<T>
- Deferred capabilities
- ADR-0002: Session cookie auth across frontend and API origins
- .from
- m1-overseer.md
- Modes
- AccessTokenClaims
- schema.ts
- m1-reviewer.md
- InstrumentedUnitOfWork
- Zuordnung
- LoginPage
- .window
- PasswordPage
- user.rs
- Migration
- provider.tsx
- .auth_context
- RundeStartenRequest
- mise
- .begin

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 165 edges
2. `BenutzerId` - 112 edges
3. `cn()` - 101 edges
4. `AnlageId` - 91 edges
5. `ZielId` - 81 edges
6. `ParcoursId` - 61 edges
7. `ExecutionContext` - 55 edges
8. `AuthContext` - 54 edges
9. `FakeDatastore` - 54 edges
10. `react` - 53 edges

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

## Communities (169 total, 15 thin omitted)

### Community 0 - "PdfRenderer"
Cohesion: 0.10
Nodes (18): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, Vec, PdfDokument, PdfRenderer, PdfRenderError (+10 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "Specifications"
Cohesion: 0.10
Nodes (16): Added, Changelog, Removed, [Unreleased], Checklist, Release description, Release pull request, Test evidence (+8 more)

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
Cohesion: 0.10
Nodes (38): AuthProvider(), useAuth(), apiErrorMessage(), useAnlageMutation(), useAuthMutations(), useStammdatenMutations(), useAnlagenQuery(), useAnlageQuery() (+30 more)

### Community 7 - "ExecutionContext"
Cohesion: 0.12
Nodes (45): ExecutionContext, UnitOfWork, Context, Output, ResultReport, UseCase, aktive_anlage(), aktive_anlage_ändern() (+37 more)

### Community 8 - "http_status.rs"
Cohesion: 0.07
Nodes (28): Report, Result, Uuid, Wertungszone, SchussErstellungRequest, HttpStatusMapping, mapped(), nested_http_status() (+20 more)

### Community 9 - "Session"
Cohesion: 0.11
Nodes (17): Option, Uuid, Session, SessionId, option_datetime(), Arc, DateTime, Mutex (+9 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "token-swatches.tsx"
Cohesion: 0.23
Nodes (11): AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour(), relativeLuminance(), COLOUR_TOKENS, CONTRAST_PAIRS, Measurement (+3 more)

### Community 14 - "ObjectStore"
Cohesion: 0.07
Nodes (36): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+28 more)

### Community 15 - "token.rs"
Cohesion: 0.12
Nodes (23): AccessToken, basis64url(), basis64url_decode(), constant_time_eq(), debug_gibt_das_token_nicht_preis(), hash_ist_stabil_und_vergleicht_konstantzeitig(), konstantzeit_vergleich_erkennt_jedes_gekippte_byte(), parsing_ist_ein_roundtrip_über_die_bytes() (+15 more)

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
Cohesion: 0.06
Nodes (42): email(), from_parts_roundtrip(), hash(), jetzt(), neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime, EmailAdresse (+34 more)

### Community 20 - "Clock"
Cohesion: 0.09
Nodes (17): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+9 more)

### Community 21 - "ParcoursHinzufügen"
Cohesion: 0.33
Nodes (8): ParcoursAnzeigen, ParcoursHinzufügen, ParcoursSetupÄndern, ParcoursÖffentlich, Parcours, Pflock, Vec, Zielplatzierung

### Community 22 - "parse_uuid"
Cohesion: 0.18
Nodes (19): delete_versioned(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), parse_version() (+11 more)

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
Cohesion: 0.09
Nodes (26): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+18 more)

### Community 32 - "jwt.rs"
Cohesion: 0.12
Nodes (25): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), claims(), decode(), DecodedToken, encode(), encode_session() (+17 more)

### Community 33 - "RepoStorage"
Cohesion: 0.16
Nodes (12): RepoStorage, Box, Pin, Self, InstrumentedAnlageRepository, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository (+4 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.20
Nodes (36): pruefen(), ac10_wiederherstellung_antwortet_für_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac17_fremder_schlüssel_passt_nicht_zur_session_zeile(), ac17_unbekannte_session_wird_abgelehnt(), ac17_widerrufene_session_wird_trotz_gültigem_schlüssel_abgelehnt() (+28 more)

### Community 35 - "anlage/page.tsx"
Cohesion: 0.12
Nodes (38): HomePage(), ZONEN, laender, leer, leeresZiel, AuthError(), PageChromeContext, PageChromeContextValue (+30 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "Versioned"
Cohesion: 0.09
Nodes (26): PasswortWiederherstellungRepository, Versioned, KontoId, from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung (+18 more)

### Community 40 - "format_datetime"
Cohesion: 0.16
Nodes (13): format_datetime(), DateTime, Utc, konto_from_row(), Arc, EmailAdresse, Mutex, Option (+5 more)

### Community 42 - "CookieConfig"
Cohesion: 0.19
Nodes (16): attribute(), clear_cookie_header(), CookieConfig, das_löschen_setzt_max_age_auch_ohne_lebensdauer(), das_löschen_trifft_dasselbe_cookie(), die_attribute_sind_vollständig(), die_lebensdauer_und_die_session_variante_setzen_max_age(), die_vorgaben_schreiben_nur_das_nötigste() (+8 more)

### Community 44 - "claims_for_session"
Cohesion: 0.11
Nodes (20): Anmeldung, EmailAdresse, Error, Result, claims_for_session(), new_session(), DateTime, NeueSession (+12 more)

### Community 46 - ".get_current_version"
Cohesion: 0.21
Nodes (11): Box, Error, Future, Option, Output, Pin, Result, Send (+3 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "RundeId"
Cohesion: 0.07
Nodes (32): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), anonymisieren_entfernt_den_benutzer(), die_faltung_ist_unabhängig_von_der_eingabereihenfolge(), ergebnis(), from_parts_roundtrip(), menge(), NeuerSchuss (+24 more)

### Community 49 - "konto/session.rs"
Cohesion: 0.23
Nodes (16): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), jetzt(), neue_session_ist_gültig_und_läuft_in_einem_monat_ab(), Refreshed (+8 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "domain/mod.rs"
Cohesion: 0.15
Nodes (9): AnlageVerwaltung, IntoIterator, Item, Zielergebnis, Rundenergebnis, Punkte, Add, Zielergebnis (+1 more)

### Community 52 - "App"
Cohesion: 0.07
Nodes (60): assert_status_ok(), base_app_builder(), befördere_zum_systemadministrator(), cookie_response(), E2eHarness, json_response(), LogandaimApiTestClient, neuer_benutzer() (+52 more)

### Community 53 - "SQLiteSchussRepository"
Cohesion: 0.17
Nodes (17): Arc, IntoParams, Mutex, Option, RepositoryError, RepositoryResult, Result, Row (+9 more)

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.08
Nodes (28): alle_felder_duerfen_fehlen(), AnlageKontakt, jedes_feld_ist_einzeln_optional(), AsRef, EmailAdresse, Option, Result, S (+20 more)

### Community 55 - "InMemorySigning"
Cohesion: 0.08
Nodes (24): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex(), ResultReport, Send, Sync (+16 more)

### Community 56 - "cn"
Cohesion: 0.05
Nodes (97): PLATES, ComponentSection(), Example(), ThemePlate(), touchTargetOutline, ComposedPageSection(), GridPaper(), Legend() (+89 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.05
Nodes (38): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+30 more)

### Community 59 - "DateTime"
Cohesion: 0.15
Nodes (11): NeueSession, RefreshedSession, RenewableSession, RevokedSession, DateTime, Error, Into, Result (+3 more)

### Community 61 - "cases/stammdaten.rs"
Cohesion: 0.13
Nodes (55): ac10_deaktivieren_verhindert_neue_runden_und_haelt_alte_lesbar(), ac10_ein_mitarbeiter_deaktiviert_nicht(), ac11_ein_archiviertes_ziel_verweigert_jede_aenderung(), ac11_ein_referenzierter_parcours_wird_archiviert(), ac11_ein_referenziertes_ziel_wird_archiviert_nicht_geloescht(), ac11_eine_deaktivierte_anlage_verweigert_schreibzugriffe(), ac12_ein_unreferenzierter_parcours_wird_geloescht(), ac12_ein_unreferenziertes_ziel_wird_geloescht() (+47 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "Mail"
Cohesion: 0.06
Nodes (34): AsyncSmtpTransport, NullMailSender, ResultReport, adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), Mail, MailArt (+26 more)

### Community 64 - "layout.tsx"
Cohesion: 0.13
Nodes (12): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, Providers(), AppChrome() (+4 more)

### Community 65 - "AnlageId"
Cohesion: 0.12
Nodes (14): LogandaimAppApi, Anlage, AnlagenZeile, Arc, BenutzerSuchtreffer, ParcoursÖffentlich, ResultReport, Runde (+6 more)

### Community 66 - "use_cases/runde.rs"
Cohesion: 0.15
Nodes (20): ist_nicht_gefunden(), C, Error, Option, Report, Result, Runde, Schuss (+12 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "SessionAusgestellt"
Cohesion: 0.13
Nodes (17): Benutzer, SessionAusgestellt, Abmeldung, AbmeldungFehler, Erneuerung, ErneuerungFehler, Erneuerungsausgang, revoke_all_sessions() (+9 more)

### Community 71 - "preis.rs"
Cohesion: 0.07
Nodes (25): Menge, MengeFehler, Decimal, Result, Self, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis() (+17 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "ClientKey"
Cohesion: 0.17
Nodes (16): InstrumentedThrottle, DateTime, ResultReport, Utc, ClientKey, DateTime, Into, ResultReport (+8 more)

### Community 74 - "package.json"
Cohesion: 0.05
Nodes (35): name, private, version, CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow() (+27 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "FakeDatastore"
Cohesion: 0.09
Nodes (24): FakeAnlageRepository, FakeBenutzerRepository, FakeDatastore, FakeKontoRepository, FakeRundeRepository, FakeSchussRepository, FakeSessionRepository, Anlage (+16 more)

### Community 80 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 81 - "StammdatenFehler"
Cohesion: 0.15
Nodes (32): AnlageAdresseSchema, AnlageKontaktSchema, adresse(), AnlageErstellungRequest, AnlageVerwaltungRequest, AnlageÄnderungRequest, kontakt(), name() (+24 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - ".anmelden"
Cohesion: 0.11
Nodes (19): AnmeldungRequest, BenutzerLöschungRequest, PasswortVergessenRequest, RegistrierungStartRequest, EmailAdresse, Report, Result, AnmeldungFehler (+11 more)

### Community 86 - "schema/stammdaten.rs"
Cohesion: 0.06
Nodes (43): AnlagenZeileAnsicht, Anlage, AnlageAdresse, AnlageKontakt, AnlagenZeile, AnlageVerwaltung, AnlageVerwaltungRolle, AnlageZustand (+35 more)

### Community 87 - "mailpit.rs"
Cohesion: 0.28
Nodes (5): basis_url(), letzter_text_an(), nachrichten(), Value, Vec

### Community 88 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 89 - "FakeUnitOfWork"
Cohesion: 0.18
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 90 - "ziel.rs"
Cohesion: 0.10
Nodes (17): aendern_haelt_die_identitaet(), archiviertes_ziel_verweigert_jede_aenderung(), from_parts_roundtrip(), NeuesZiel, AsRef, Err, FromStr, Result (+9 more)

### Community 91 - "ParcoursSnapshot"
Cohesion: 0.10
Nodes (22): from_parts_roundtrip(), leerer_name_wird_abgelehnt(), ParcoursSnapshot, pflock(), quelle(), AsRef, Into, Option (+14 more)

### Community 92 - "String"
Cohesion: 0.21
Nodes (20): From, String, LogandaimApiSpec, Anlage, AnlagenZeile, Benutzer, BenutzerSuchtreffer, From (+12 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "domain/runde.rs"
Cohesion: 0.27
Nodes (9): ac8_die_faltung_ist_unabhaengig_von_der_menge_reihenfolge(), ac8_die_summe_ist_die_summe_der_zielergebnisse(), ac8_ein_ziel_ausserhalb_des_snapshots_zaehlt_nicht(), ac8_ziele_ohne_schuss_werden_uebersprungen(), eine_leere_runde_hat_null(), menge(), Wertungszone, RundeZustandUnbekannt (+1 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (13): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+5 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "Harness"
Cohesion: 0.16
Nodes (12): build(), fremder_schluessel(), Harness, jetzt(), AppBuilder, Arc, DateTime, Duration (+4 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 107 - "parcours.rs"
Cohesion: 0.09
Nodes (30): Abstand, ac11_archivierter_parcours_verweigert_jede_aenderung(), ac4_dasselbe_ziel_darf_nur_einmal_vorkommen(), ac6_abstaende_nehmen_von_fern_nach_nah_streng_ab(), ac6_fehlende_abstaende_sind_erlaubt(), ac6_pflock_bezeichnungen_sind_eindeutig(), ac6_reihenfolge_ist_unabhaengig_von_der_farbe(), drei_pflöcke() (+22 more)

### Community 108 - "SQLiteBenutzerRepository"
Cohesion: 0.24
Nodes (9): benutzer_from_row(), Arc, Benutzer, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 109 - "UnitOfWorkImpl"
Cohesion: 0.23
Nodes (8): ExecutionSource, Arc<T>, Box, RepositoryResult, Send, Sync, UnitOfWorkImpl, UnitOfWorkProvider

### Community 110 - "NewSessionResponse"
Cohesion: 0.18
Nodes (17): InternalServerError, Self, EndSessionResponse, EndSessionResponse<T>, NewSessionResponse, refresh_cookie_config(), From, Json (+9 more)

### Community 111 - "SecretTokenHash"
Cohesion: 0.17
Nodes (4): hash(), SessionIn<S>, From, SecretTokenHash

### Community 112 - "ZielId"
Cohesion: 0.12
Nodes (26): ac11_archivieren_entfernt_das_ziel_aus_den_parcours(), ac12_entfernen_loescht_das_ziel_und_seine_platzierungen(), ac2_die_letzte_administrator_verwaltung_kann_nicht_entzogen_werden(), ac2_eine_verwaltung_existiert_nur_einmal_pro_benutzer(), ac4_ein_parcours_verwendet_nur_ziele_dieser_anlage(), ac4_ein_ziel_kann_in_mehreren_parcours_liegen(), ac7_der_snapshot_ist_eine_wertkopie(), Aktiv (+18 more)

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "app-shell.tsx"
Cohesion: 0.13
Nodes (22): Sidebar(), SidebarLink(), SidebarProps, DropdownMenu, DropdownMenuContent(), DropdownMenuGroup, DropdownMenuItem(), DropdownMenuLabel() (+14 more)

### Community 115 - "RepositoryResult"
Cohesion: 0.07
Nodes (20): AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository, RepositoryError, Anlage, Benutzer, EmailAdresse (+12 more)

### Community 116 - "BenutzerId"
Cohesion: 0.14
Nodes (15): befoerderung_ist_eine_domain_mutation(), Benutzer, BenutzerId, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle (+7 more)

### Community 117 - "anlage_aus_zeile"
Cohesion: 0.16
Nodes (27): anlage_aus_zeile(), ersetze_kinder(), kinder_loeschen_sql(), lade_parcours(), lade_pflöcke(), lade_platzierungen(), lade_verwaltungen(), lade_ziele() (+19 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "LogandaimApi"
Cohesion: 0.06
Nodes (4): LogandaimApi, AnlageVerwaltungRolle, Administrator, Mitarbeiter

### Community 120 - "Runde"
Cohesion: 0.19
Nodes (16): DateTime, From, Option, Self, Utc, Uuid, Vec, Zielergebnis (+8 more)

### Community 121 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 122 - "Appendix: implementation sketch (non-normative)"
Cohesion: 0.17
Nodes (12): ADR-0004: JWT access tokens with rotating refresh tokens, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Crypto is domain code (sketch) (+4 more)

### Community 123 - "Stammdaten — Master data and `Anlage` administration"
Cohesion: 0.17
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 125 - "Benutzer"
Cohesion: 0.33
Nodes (9): Benutzer, BenutzerRolle, BenutzerSuchtreffer, NeueSession, EmailAdresse, From, Self, Uuid (+1 more)

### Community 126 - "Runde"
Cohesion: 0.11
Nodes (16): abschliessen_ist_einmalig(), anonymisieren_laesst_den_snapshot_unberuehrt(), jetzt(), NeueRunde, DateTime, Err, FromStr, Option (+8 more)

### Community 127 - "logandaim-api.ts"
Cohesion: 0.09
Nodes (27): JsonClient, AnlageAdresse, AnlageErstellungInput, AnlageKontakt, AnlagenZeile, AnlageVerwaltung, Benutzer, BenutzerSuchtreffer (+19 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "KontoFehler"
Cohesion: 0.67
Nodes (3): KontoFehler, From, SessionNotRenewable

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "StreamBinaryResponse"
Cohesion: 0.17
Nodes (20): C, From, Json, ObjectStream, PlainText, Report, Result, Self (+12 more)

### Community 134 - "e2e/konto.rs"
Cohesion: 0.13
Nodes (20): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+12 more)

### Community 135 - "cases/runde.rs"
Cohesion: 0.24
Nodes (24): base_app_builder(), AppBuilder, SetUowProvider, abschliessen_ist_einmalig_und_setzt_den_zustand(), ac9_eine_runde_zeigt_den_schnappschuss_wie_beim_start(), adresse(), benutzer_mit_ip(), client_key() (+16 more)

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
Cohesion: 0.09
Nodes (13): Anlage, AnlageIn<S>, AnlagenSicht, AnlageVerwaltung, AnlageVerwaltungRolle, ist_systemadministrator_oder_verwalter(), Benutzer, From (+5 more)

### Community 140 - "AnlageName"
Cohesion: 0.10
Nodes (17): adresse(), AnlageArchiviert, AnlageIn, AnlageName, AnlageNameLeer, Archiviert, NeueAnlage, AnlageAdresse (+9 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - "SQLiteUnitOfWork"
Cohesion: 0.19
Nodes (5): Arc, Mutex, Option, Transaction, SQLiteUnitOfWork

### Community 147 - "SQLiteRundeRepository"
Cohesion: 0.15
Nodes (21): ersetze_referenzen(), PflockDto, Arc, Mutex, Option, RepositoryError, RepositoryResult, Result (+13 more)

### Community 148 - "in_memory_throttle.rs"
Cohesion: 0.25
Nodes (17): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), fehlversuche_erschöpfen_das_budget_und_blockieren_weitere_anfragen(), gültige_anfragen_verbrauchen_das_budget_nicht(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now() (+9 more)

### Community 149 - "apply_up_migrations"
Cohesion: 0.20
Nodes (10): AppliableMigration, apply_down_migrations(), apply_up_migrations(), ApplyMigrationUp, MigrationError, MigrationTarget, Item, Option (+2 more)

### Community 150 - "instrumented.rs"
Cohesion: 0.22
Nodes (7): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option

### Community 151 - "Versioned<T>"
Cohesion: 0.11
Nodes (13): Clone, Debug, Deref, Option, Self, T, Target, Versioned<T> (+5 more)

### Community 152 - "Deferred capabilities"
Cohesion: 0.17
Nodes (12): Account security, Administration, Auth audit, Deferred capabilities, Email change, Email delivery, Email verification, External identity providers (+4 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.20
Nodes (10): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+2 more)

### Community 154 - ".from"
Cohesion: 0.11
Nodes (23): Cookie, PasswortWiederherstellenRequest, RegistrierungAbschlussRequest, andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip() (+15 more)

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "AccessTokenClaims"
Cohesion: 0.22
Nodes (17): abgelaufenes_token_wird_abgelehnt(), AccessTokenClaims, claims(), fenster_mit_exp_vor_iat_wird_abgelehnt(), gültiges_fenster_wird_akzeptiert(), herkunft_muss_exakt_stimmen(), iat_in_der_zukunft_wird_abgelehnt(), iat_innerhalb_der_toleranz_wird_akzeptiert() (+9 more)

### Community 158 - "schema.ts"
Cohesion: 0.08
Nodes (25): ApiError, AnlageZustand, Aktiv, Archiviert, BenutzerRolle, Benutzer, Systemadministrator, components (+17 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 163 - "InstrumentedUnitOfWork"
Cohesion: 0.17
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 164 - "Zuordnung"
Cohesion: 0.26
Nodes (9): domain::Wertungszone, From, Option, Self, Uuid, Schuss, Wertungszone, Zielergebnis (+1 more)

### Community 167 - "PasswordPage"
Cohesion: 0.18
Nodes (3): PasswordPage(), RegistrationPage(), useMailToken()

### Community 168 - "user.rs"
Cohesion: 0.24
Nodes (8): Beförderung, BenutzerLöschen, BenutzerLöschenFehler, EmailAdresse, Error, Result, SystemadministratorBefördern, SystemadministratorBefördernFehler

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

### Community 184 - ".begin"
Cohesion: 0.28
Nodes (6): Box, RepositoryError, RepositoryResult, ResultReport, Self, SQLiteInstance

## Knowledge Gaps
- **498 isolated node(s):** `ValidationError`, `LogandaimApiSpec`, `AnlageVerwaltungRolleUnbekannt`, `ZustandUnbekannt`, `KontoFehler` (+493 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1097 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `StructuredError`, `StreamBinaryResponse`, `e2e/konto.rs`, `Session`, `AnlageName`, `ObjectStore`, `token.rs`, `KlartextPasswort`, `SQLiteRundeRepository`, `parse_uuid`, `.from`, `ContextPorts`, `AccessTokenClaims`, `jwt.rs`, `cases/konto.rs`, `Zuordnung`, `format_datetime`, `CookieConfig`, `konto/session.rs`, `App`, `SQLiteSchussRepository`, `domain/kontakt.rs`, `InMemorySigning`, `DateTime`, `Mail`, `AnlageId`, `use_cases/runde.rs`, `SessionAusgestellt`, `preis.rs`, `ClientKey`, `StammdatenFehler`, `.anmelden`, `schema/stammdaten.rs`, `mailpit.rs`, `FakeUnitOfWork`, `ziel.rs`, `ParcoursSnapshot`, `config.rs`, `Harness`, `parcours.rs`, `NewSessionResponse`, `SecretTokenHash`, `EmailAdresse`, `anlage_aus_zeile`, `Runde`, `Benutzer`?**
  _High betweenness centrality (0.284) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `ExecutionContext`, `Session`, `ParcoursId`, `KlartextPasswort`, `SQLiteRundeRepository`, `AccessTokenClaims`, `jwt.rs`, `cases/konto.rs`, `format_datetime`, `claims_for_session`, `RundeId`, `konto/session.rs`, `SQLiteSchussRepository`, `DateTime`, `cases/stammdaten.rs`, `AnlageId`, `SessionAusgestellt`, `FakeDatastore`, `StammdatenFehler`, `String`, `domain/runde.rs`, `SQLiteBenutzerRepository`, `SecretTokenHash`, `ZielId`, `RepositoryResult`, `anlage_aus_zeile`, `Runde`?**
  _High betweenness centrality (0.093) - this node is a cross-community bridge._
- **Why does `Versioned` connect `Versioned` to `ExecutionContext`, `format_datetime`, `Session`, `SQLiteBenutzerRepository`, `UnitOfWorkImpl`, `FakeDatastore`, `SecretTokenHash`, `RepositoryResult`, `BenutzerId`, `anlage_aus_zeile`, `parse_uuid`, `Versioned<T>`, `SQLiteRundeRepository`, `SQLiteSchussRepository`, `EmailBestätigung`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 15 inferred relationships involving `AnlageId` (e.g. with `.anlage_anzeigen()` and `.anlage_deaktivieren()`) actually correct?**
  _`AnlageId` has 15 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `AnlageVerwaltungRolleUnbekannt` to the rest of the system?**
  _498 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09885057471264368 - nodes in this community are weakly interconnected._