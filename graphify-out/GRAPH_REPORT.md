# Graph Report - logandaim.at  (2026-10-28)

## Corpus Check
- 251 files · ~126,945 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3029 nodes · 6800 edges · 163 communities (142 shown, 13 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 139 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `85459a34`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PdfRenderer
- arc_up
- Specifications
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- auth-pages.tsx
- SchussErstellungRequest
- http_status.rs
- cn
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- demo-page.tsx
- in_memory_object_store.rs
- jwt.rs
- preis.rs
- E
- Datenschutz — Data protection, privacy, and legal obligations
- passwort.rs
- Clock
- wertung.rs
- common.rs
- <Title>
- Make invalid state unrepresentable
- Betrieb — Operations and launch readiness
- check-release.mjs
- ExecutionContext
- Wertung — Classic 3-arrow scoring
- .contextualize_with
- Konto — Benutzer, registration, and login
- SecretTokenHash
- Benutzer
- RepoStorage
- cases/konto.rs
- package.json
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- PasswortWiederherstellung
- StreamBinaryResponse
- CookieConfig
- claims_for_session
- SQLiteUnitOfWork
- Vertical slices
- domain/schuss.rs
- SQLiteKontoRepository
- components.json
- instrumented.rs
- e2e/konto.rs
- InstrumentedUnitOfWork
- domain/kontakt.rs
- SigningKeypair
- token-swatches.tsx
- logandaim.at — Product Charter
- dependencies
- DateTime
- charter.md
- LogandaimApiTestClient
- Visual identity and design system
- Mail
- BenutzerId
- LogandaimAppApi
- react
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- Session
- Konto
- check-version.mjs
- Throttle
- sortable-table-section.tsx
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- Versioned
- Next.js
- postcss.config.mjs
- .finish
- sections.tsx
- Contributing and project management
- spend_open_recoveries
- http-client.ts
- SessionAusgestellt
- String
- .begin
- FakeUnitOfWork
- Harness
- .get_current_version
- LogandaimApiSpec
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- FakeObjectStore
- SQLiteInstance
- UpMigration
- config.rs
- ClientKey
- .add_dyn
- SQLiteBenutzerRepository
- ObjectStore
- e2e/main.rs
- UnitOfWorkImpl
- NewSessionResponse
- user.rs
- LogandaimApi
- EmailAdresse
- app-shell.tsx
- RepositoryResult
- auth-provider.tsx
- layout.tsx
- pull_request_template.md
- InMemorySigning
- konto/signing.rs
- adresse.rs
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- Benutzer
- tests/throttle.rs
- FakeDatastore
- Anlage-Anfrage — Requesting a new Anlage
- in_memory_throttle.rs
- revoke_all_sessions
- StructuredError
- toggle-group.tsx
- StreamBody
- provider.tsx
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- UnitOfWork
- SQLitePasswortWiederherstellungRepository
- HttpLogandaimApi
- dates.ts
- Signature
- schema/schuss.rs
- konto/session.rs
- App
- FixedClock
- .begin
- apply_up_migrations
- .partial_cmp
- Versioned<T>
- SessionIn<S>
- ADR-0002: Session cookie auth across frontend and API origins
- client_key
- m1-overseer.md
- Modes
- button.tsx
- Ok
- m1-reviewer.md
- session-store.ts
- KontoFehler
- mise

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 117 edges
2. `cn()` - 99 edges
3. `BenutzerId` - 62 edges
4. `E` - 45 edges
5. `Session` - 45 edges
6. `FakeDatastore` - 44 edges
7. `react` - 44 edges
8. `SecretTokenHash` - 41 edges
9. `EmailBestätigung` - 40 edges
10. `PasswortWiederherstellung` - 40 edges

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

## Communities (163 total, 13 thin omitted)

### Community 0 - "PdfRenderer"
Cohesion: 0.10
Nodes (18): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, Vec, PdfDokument, PdfRenderer, PdfRenderError (+10 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "Specifications"
Cohesion: 0.11
Nodes (15): Added, Changelog, [Unreleased], Checklist, Release description, Release pull request, Test evidence, Version (+7 more)

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "auth-pages.tsx"
Cohesion: 0.11
Nodes (13): LoginPage(), PasswordPage(), RegistrationPage(), useMailToken(), Card(), CardContent(), CardDescription(), CardFooter() (+5 more)

### Community 7 - "SchussErstellungRequest"
Cohesion: 0.18
Nodes (11): From, Self, Uuid, Vec, Wertungszone, SchussErstellungRequest, SchussRegistrieren, WertungBerechnen (+3 more)

### Community 8 - "http_status.rs"
Cohesion: 0.12
Nodes (16): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+8 more)

### Community 9 - "cn"
Cohesion: 0.14
Nodes (25): ComponentSection(), Example(), ThemePlate(), touchTargetOutline, GridPaper(), Legend(), LegendEntry, RingMark (+17 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "demo-page.tsx"
Cohesion: 0.10
Nodes (27): CHECKPOINTS, MESSREIHE, Row, START_ROWS, statusVariant, Zone, ZONEN, Dialog (+19 more)

### Community 14 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 15 - "jwt.rs"
Cohesion: 0.07
Nodes (46): bearer_token(), LogandaimApiSpec, AuthContext, Option, ResultReport, access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt() (+38 more)

### Community 16 - "preis.rs"
Cohesion: 0.05
Nodes (36): Menge, MengeFehler, Decimal, Result, Self, nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis() (+28 more)

### Community 17 - "E"
Cohesion: 0.33
Nodes (10): E, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+2 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "passwort.rs"
Cohesion: 0.13
Nodes (22): argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gültige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password(), hash_und_verify_stimmen_überein() (+14 more)

### Community 20 - "Clock"
Cohesion: 0.13
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (21): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), pfeil(), Punkte, Add (+13 more)

### Community 22 - "common.rs"
Cohesion: 0.19
Nodes (18): format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), parse_version() (+10 more)

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
Cohesion: 0.09
Nodes (19): ContextPorts, ExecutionContext, ExecutionContext<'a>, ExecutionSource, Arc, AuthContext, die_konfiguration_hält_nur_die_claim_identität(), IdentityProviderConfiguration (+11 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 30 - "Konto — Benutzer, registration, and login"
Cohesion: 0.07
Nodes (28): Acceptance criteria, Account security, Administration, Approval, Auth audit, Decision log, Deferred capabilities, Domain terms and business rules (+20 more)

### Community 31 - "SecretTokenHash"
Cohesion: 0.05
Nodes (49): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+41 more)

### Community 32 - "Benutzer"
Cohesion: 0.18
Nodes (12): befoerderung_ist_eine_domain_mutation(), Benutzer, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle, DateTime (+4 more)

### Community 33 - "RepoStorage"
Cohesion: 0.20
Nodes (10): RepoStorage, Box, Pin, Self, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository, InstrumentedPasswortWiederherstellungRepository (+2 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.18
Nodes (39): pruefen(), ac10_wiederherstellung_antwortet_für_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac13_anonymer_zugriff_wird_verweigert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_fremder_schlüssel_passt_nicht_zur_session_zeile() (+31 more)

### Community 35 - "package.json"
Cohesion: 0.09
Nodes (20): name, private, version, Progress(), babel-plugin-react-compiler, @biomejs/biome, @dnd-kit/core, openapi-typescript (+12 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.12
Nodes (16): KontoId, from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime (+8 more)

### Community 40 - "StreamBinaryResponse"
Cohesion: 0.22
Nodes (18): InternalServerError, Self, C, From, Json, ObjectStream, PlainText, Report (+10 more)

### Community 42 - "CookieConfig"
Cohesion: 0.19
Nodes (16): attribute(), clear_cookie_header(), CookieConfig, das_löschen_setzt_max_age_auch_ohne_lebensdauer(), das_löschen_trifft_dasselbe_cookie(), die_attribute_sind_vollständig(), die_lebensdauer_und_die_session_variante_setzen_max_age(), die_vorgaben_schreiben_nur_das_nötigste() (+8 more)

### Community 44 - "claims_for_session"
Cohesion: 0.15
Nodes (14): Error, Result, claims_for_session(), new_session(), DateTime, NeueSession, Utc, DateTime (+6 more)

### Community 46 - "SQLiteUnitOfWork"
Cohesion: 0.09
Nodes (19): Box, Error, Future, Option, Output, Pin, Result, Send (+11 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.06
Nodes (54): from_parts_roundtrip(), id(), NeuerSchuss, pfeil_ist_eins_basiert(), pfeil_null_wird_abgelehnt(), pfeil_vier_und_später_sind_erlaubt(), NonZeroUsize, Result (+46 more)

### Community 49 - "SQLiteKontoRepository"
Cohesion: 0.14
Nodes (14): delete_versioned(), RepositoryResult, Transaction, konto_from_row(), Arc, EmailAdresse, Mutex, Option (+6 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "instrumented.rs"
Cohesion: 0.24
Nodes (9): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, InstrumentedThrottle, Arc, ObjectStoreError, ObjectStream, Option (+1 more)

### Community 52 - "e2e/konto.rs"
Cohesion: 0.13
Nodes (19): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+11 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.27
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.13
Nodes (17): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe() (+9 more)

### Community 55 - "SigningKeypair"
Cohesion: 0.24
Nodes (7): kaputtes_pem_wird_abgelehnt(), Formatter, KontoFehler, Result, Self, SigningKeypair, SigningKey

### Community 56 - "token-swatches.tsx"
Cohesion: 0.23
Nodes (11): AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour(), relativeLuminance(), COLOUR_TOKENS, CONTRAST_PAIRS, Measurement (+3 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.04
Nodes (47): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+39 more)

### Community 59 - "DateTime"
Cohesion: 0.15
Nodes (11): NeueSession, RefreshedSession, RenewableSession, RevokedSession, DateTime, Error, Into, Result (+3 more)

### Community 61 - "LogandaimApiTestClient"
Cohesion: 0.29
Nodes (10): cookie_response(), json_response(), LogandaimApiTestClient, Option, StatusCode, Value, Vec, Route (+2 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "Mail"
Cohesion: 0.07
Nodes (31): AsyncSmtpTransport, NullMailSender, ResultReport, adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), Mail, MailArt (+23 more)

### Community 64 - "BenutzerId"
Cohesion: 0.18
Nodes (6): AuthContext, id(), Option, ResultReport, BenutzerId, Uuid

### Community 65 - "LogandaimAppApi"
Cohesion: 0.12
Nodes (15): LogandaimAppApi, Arc, AuthContext, Benutzer, Option, ResultReport, Schuss, Uuid (+7 more)

### Community 66 - "react"
Cohesion: 0.11
Nodes (20): InkedControlsSection(), Checkbox(), CheckboxProps, FieldProps, LabelledProps, Label(), RadioGroup(), RadioGroupItem() (+12 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "Session"
Cohesion: 0.13
Nodes (16): Uuid, Session, SessionId, option_datetime(), Arc, DateTime, Mutex, NeueSession (+8 more)

### Community 71 - "Konto"
Cohesion: 0.18
Nodes (12): email(), from_parts_roundtrip(), hash(), jetzt(), Konto, neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime (+4 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "Throttle"
Cohesion: 0.11
Nodes (14): DateTime, Utc, DateTime, Duration, Into, ResultReport, Self, Send (+6 more)

### Community 74 - "sortable-table-section.tsx"
Cohesion: 0.09
Nodes (23): zonen, CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow(), SortableTableSection(), State (+15 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "Versioned"
Cohesion: 0.13
Nodes (18): Versioned, FakeBenutzerRepository, FakeEmailBestätigungRepository, FakeKontoRepository, FakeSchussRepository, FakeSessionRepository, Benutzer, EmailAdresse (+10 more)

### Community 80 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 81 - "sections.tsx"
Cohesion: 0.12
Nodes (22): PLATES, PaperFurnitureSection(), AlertSection(), BadgeSection(), ButtonSection(), CardSection(), DataSection(), FormSection() (+14 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "spend_open_recoveries"
Cohesion: 0.20
Nodes (9): PasswortWiederherstellenRequest, PasswortWiederherstellen, PasswortWiederherstellenFehler, DateTime, Error, Report, Result, Utc (+1 more)

### Community 84 - "http-client.ts"
Cohesion: 0.13
Nodes (18): ApiError, JsonClient, Benutzer, Schuss, SchussErstellungInput, Wertung, components, $defs (+10 more)

### Community 86 - "SessionAusgestellt"
Cohesion: 0.18
Nodes (10): Self, Benutzer, SessionAusgestellt, Abmeldung, Erneuerung, ErneuerungFehler, Erneuerungsausgang, SessionPrüfen (+2 more)

### Community 87 - "String"
Cohesion: 0.21
Nodes (8): From, String, basis_url(), letzter_text_an(), nachrichten(), Value, Vec, token_aus()

### Community 88 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 89 - "FakeUnitOfWork"
Cohesion: 0.24
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 90 - "Harness"
Cohesion: 0.14
Nodes (14): anonym(), build(), fremder_schluessel(), Harness, jetzt(), AppBuilder, Arc, AuthContext (+6 more)

### Community 91 - ".get_current_version"
Cohesion: 0.29
Nodes (6): ApplyMigrationUp, Box, Future, Output, Pin, Send

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.17
Nodes (17): Cookie, LogandaimApiSpec, openapi_service(), Benutzer, From, Into, IntoIterator, Item (+9 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "ClientKey"
Cohesion: 0.14
Nodes (19): AnmeldungRequest, PasswortVergessenRequest, RegistrierungAbschlussRequest, RegistrierungStartRequest, EmailAdresse, Report, Result, ClientKey (+11 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 106 - "SQLiteBenutzerRepository"
Cohesion: 0.26
Nodes (9): benutzer_from_row(), Arc, Benutzer, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 107 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 108 - "e2e/main.rs"
Cohesion: 0.22
Nodes (11): assert_status_ok(), base_app_builder(), E2eHarness, Arc, Duration, Self, test_konfiguration(), test_signing_key() (+3 more)

### Community 109 - "UnitOfWorkImpl"
Cohesion: 0.12
Nodes (6): Arc<T>, LockedUnitOfWorkImpl, Send, Sync, UnitOfWorkImpl, UnitOfWorkProvider

### Community 110 - "NewSessionResponse"
Cohesion: 0.24
Nodes (15): EndSessionResponse, EndSessionResponse<T>, NewSessionResponse, refresh_cookie_config(), From, Json, NeueSession, Option (+7 more)

### Community 111 - "user.rs"
Cohesion: 0.20
Nodes (11): Beförderung, BenutzerAnsicht, BenutzerAnzeigen, BenutzerLöschen, Benutzer, EmailAdresse, Error, Option (+3 more)

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "app-shell.tsx"
Cohesion: 0.12
Nodes (23): AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar() (+15 more)

### Community 115 - "RepositoryResult"
Cohesion: 0.07
Nodes (16): BenutzerRepository, EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, RepositoryError, Benutzer, EmailAdresse, NeueSession (+8 more)

### Community 116 - "auth-provider.tsx"
Cohesion: 0.20
Nodes (17): AuthContext, AuthContextValue, AuthProvider(), useAuth(), useAuthSession(), useAuthMutations(), useSchussRegistrierenMutation(), useWertungMutation() (+9 more)

### Community 117 - "layout.tsx"
Cohesion: 0.16
Nodes (9): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, Providers(), createQueryClient() (+1 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "InMemorySigning"
Cohesion: 0.35
Nodes (5): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex()

### Community 120 - "konto/signing.rs"
Cohesion: 0.30
Nodes (5): falscher_schlüssel_wird_abgelehnt(), gekipptes_byte_wird_abgelehnt(), jedes_byte_ist_signierbar(), keypair(), signieren_und_verifizieren_roundtrip()

### Community 121 - "adresse.rs"
Cohesion: 0.22
Nodes (5): Adresse, Ländercode, LändercodeValidierungsfehler, Result, Self

### Community 122 - "Appendix: implementation sketch (non-normative)"
Cohesion: 0.17
Nodes (12): ADR-0004: JWT access tokens with rotating refresh tokens, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Crypto is domain code (sketch) (+4 more)

### Community 123 - "Stammdaten — Master data and `Anlage` administration"
Cohesion: 0.17
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Requirements, Revision history (+4 more)

### Community 124 - "YAMS Frontend Migration Specification"
Cohesion: 0.33
Nodes (5): Backend Abstraction, Developer Experience, State Management Transition, UI Framework Upgrade, YAMS Frontend Migration Specification

### Community 125 - "Benutzer"
Cohesion: 0.36
Nodes (6): Benutzer, NeueSession, EmailAdresse, From, Self, Uuid

### Community 126 - "tests/throttle.rs"
Cohesion: 0.36
Nodes (9): ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client(), ac16_registrierung_ist_auf_fünf_pro_stunde_gedrosselt(), adresse(), app(), client_key(), passwort(), EmailAdresse, Report (+1 more)

### Community 127 - "FakeDatastore"
Cohesion: 0.32
Nodes (5): FakeDatastore, Arc, Clone, Default, Self

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "in_memory_throttle.rs"
Cohesion: 0.24
Nodes (15): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now(), DateTime, Default (+7 more)

### Community 131 - "revoke_all_sessions"
Cohesion: 0.24
Nodes (8): revoke_all_sessions(), AuthContext, DateTime, Error, F, Report, Result, Utc

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "toggle-group.tsx"
Cohesion: 0.31
Nodes (6): ComposedPageSection(), DemoPage(), ToggleGroup(), ToggleGroupItem(), toggleGroupItemVariants, @radix-ui/react-toggle-group

### Community 134 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 135 - "provider.tsx"
Cohesion: 0.36
Nodes (6): createLogandaimApi(), LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config, @tanstack/react-query-devtools

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.40
Nodes (5): ADR-0001: Baseline architecture, Appendix: spelling German identifiers, Consequences, Context, Decision

### Community 138 - "UnitOfWork"
Cohesion: 0.23
Nodes (5): RepositoryResult, Box, RepositoryResult, UnitOfWork, Drop

### Community 139 - "SQLitePasswortWiederherstellungRepository"
Cohesion: 0.24
Nodes (8): Arc, Mutex, Option, RepositoryResult, Row, Transaction, SQLitePasswortWiederherstellungRepository, wiederherstellung_from_row()

### Community 140 - "HttpLogandaimApi"
Cohesion: 0.24
Nodes (3): HttpLogandaimApi, unwrap(), setAccessToken()

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - "Signature"
Cohesion: 0.32
Nodes (4): ResultReport, SigningError, Debug, Signature

### Community 144 - "schema/schuss.rs"
Cohesion: 0.27
Nodes (9): domain::Wertungszone, From, Self, Uuid, schema_schuss_from_domain(), schema_wertung_from_domain(), Schuss, Wertung (+1 more)

### Community 145 - "konto/session.rs"
Cohesion: 0.22
Nodes (17): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), hash(), jetzt(), neue_session_ist_gültig_und_läuft_in_einem_monat_ab() (+9 more)

### Community 146 - "App"
Cohesion: 0.26
Nodes (9): App, Arc, AuthContext, Box, Context, F, ResultReport, O (+1 more)

### Community 147 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Duration, Mutex, NaiveDate, Self, Utc

### Community 148 - ".begin"
Cohesion: 0.50
Nodes (3): RepositoryError, ResultReport, SQLiteInstance

### Community 149 - "apply_up_migrations"
Cohesion: 0.23
Nodes (9): AppliableMigration, apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option (+1 more)

### Community 151 - "Versioned<T>"
Cohesion: 0.11
Nodes (12): Clone, Debug, Deref, Self, T, Target, UnitOfWork<'a>, Versioned<T> (+4 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.20
Nodes (10): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+2 more)

### Community 154 - "client_key"
Cohesion: 0.57
Nodes (7): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), Option, RealIp

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "button.tsx"
Cohesion: 0.14
Nodes (14): SAMPLE_ROWS, TypeSection(), Alert(), AlertProps, alertVariants, iconColour, Badge(), BadgeProps (+6 more)

### Community 158 - "Ok"
Cohesion: 0.38
Nodes (4): Ok, Default, Example, Self

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "session-store.ts"
Cohesion: 0.38
Nodes (6): AuthSnapshot, getAuthSnapshot(), initialSnapshot, listeners, subscribeToAuth(), useAuthSnapshot()

### Community 162 - "KontoFehler"
Cohesion: 0.67
Nodes (3): KontoFehler, From, SessionNotRenewable

## Knowledge Gaps
- **476 isolated node(s):** `ValidationError`, `LogandaimApiSpec`, `KontoFehler`, `SchussRegistrierenFehler`, `WertungFehler` (+471 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 930 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **13 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `StructuredError`, `in_memory_object_store.rs`, `jwt.rs`, `preis.rs`, `konto/session.rs`, `passwort.rs`, `common.rs`, `ExecutionContext`, `SecretTokenHash`, `cases/konto.rs`, `StreamBinaryResponse`, `CookieConfig`, `e2e/konto.rs`, `domain/kontakt.rs`, `DateTime`, `LogandaimApiTestClient`, `Mail`, `LogandaimAppApi`, `Session`, `Throttle`, `spend_open_recoveries`, `SessionAusgestellt`, `FakeUnitOfWork`, `Harness`, `LogandaimApiSpec`, `FakeObjectStore`, `config.rs`, `ClientKey`, `NewSessionResponse`, `EmailAdresse`, `InMemorySigning`, `konto/signing.rs`, `adresse.rs`, `Benutzer`, `tests/throttle.rs`?**
  _High betweenness centrality (0.218) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `Benutzer`, `cases/konto.rs`, `revoke_all_sessions`, `Session`, `Konto`, `SQLiteBenutzerRepository`, `claims_for_session`, `Versioned`, `jwt.rs`, `user.rs`, `konto/session.rs`, `SQLiteKontoRepository`, `RepositoryResult`, `SessionIn<S>`, `DateTime`, `FakeDatastore`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Why does `E` connect `E` to `cases/konto.rs`, `revoke_all_sessions`, `StructuredError`, `StreamBinaryResponse`, `.add_dyn`, `NewSessionResponse`, `App`, `apply_up_migrations`, `.get_current_version`, `.contextualize_with`, `tests/throttle.rs`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `KontoFehler` to the rest of the system?**
  _476 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09885057471264368 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._