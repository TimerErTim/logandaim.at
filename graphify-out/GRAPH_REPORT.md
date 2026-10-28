# Graph Report - logandaim.at  (2026-10-28)

## Corpus Check
- 251 files · ~127,397 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 3038 nodes · 6856 edges · 156 communities (137 shown, 11 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 139 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `14492809`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FakePdfRenderer
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
- Zeitraum
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
- EmailBestätigung
- CookieConfig
- claims_for_session
- SQLiteUnitOfWork
- Vertical slices
- domain/schuss.rs
- Konto
- components.json
- BestätigungId
- e2e/konto.rs
- cn.ts
- domain/kontakt.rs
- InMemorySigning
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
- Menge
- check-version.mjs
- ClientKey
- sortable-table-section.tsx
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- FakeDatastore
- Next.js
- postcss.config.mjs
- .finish
- preis.rs
- Contributing and project management
- UseCase
- http-client.ts
- Ratio
- String
- .begin
- FakeUnitOfWork
- spend_open_confirmations
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
- ResultReport
- .add_dyn
- SQLiteBenutzerRepository
- ObjectStore
- e2e/main.rs
- UnitOfWorkImpl
- NewSessionResponse
- user.rs
- LogandaimApi
- EmailAdresse
- navigation.ts
- Versioned
- auth-provider.tsx
- layout.tsx
- pull_request_template.md
- Preis
- devDependencies
- adresse.rs
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- Benutzer
- scripts
- .neu
- Anlage-Anfrage — Requesting a new Anlage
- delete_versioned
- revoke_all_sessions
- StructuredError
- toggle-group.tsx
- provider.tsx
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- SQLitePasswortWiederherstellungRepository
- HttpLogandaimApi
- dates.ts
- .schuss_registrieren
- konto/session.rs
- .execute
- FixedClock
- apply_up_migrations
- Versioned<T>
- ADR-0002: Session cookie auth across frontend and API origins
- client_key
- m1-overseer.md
- Modes
- button.tsx
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

## Communities (156 total, 11 thin omitted)

### Community 0 - "FakePdfRenderer"
Cohesion: 0.11
Nodes (15): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, Vec, PdfDokument, PdfRenderError, ResultReport (+7 more)

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
Nodes (12): LoginPage(), PasswordPage(), RegistrationPage(), useMailToken(), Card(), CardContent(), CardDescription(), CardFooter() (+4 more)

### Community 7 - "SchussErstellungRequest"
Cohesion: 0.27
Nodes (9): From, Self, Uuid, Vec, Wertungszone, SchussErstellungRequest, SchussRegistrieren, WertungBerechnen (+1 more)

### Community 8 - "http_status.rs"
Cohesion: 0.06
Nodes (39): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+31 more)

### Community 9 - "cn"
Cohesion: 0.17
Nodes (22): ComponentSection(), Example(), ThemePlate(), touchTargetOutline, GridPaper(), Legend(), LegendEntry, RingMark (+14 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "demo-page.tsx"
Cohesion: 0.07
Nodes (50): PLATES, CHECKPOINTS, MESSREIHE, Row, START_ROWS, statusVariant, Zone, ZONEN (+42 more)

### Community 14 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 15 - "jwt.rs"
Cohesion: 0.07
Nodes (46): bearer_token(), LogandaimApiSpec, AuthContext, Option, ResultReport, access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt() (+38 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.33
Nodes (10): E, ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>>, Arc<dyn UpMigration<T, E>>, Box<dyn UpMigration<T, E>>, DownMigration (+2 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "passwort.rs"
Cohesion: 0.07
Nodes (41): email(), from_parts_roundtrip(), hash(), jetzt(), neues_konto_bestätigt_die_adresse(), NeuesKonto, DateTime, EmailAdresse (+33 more)

### Community 20 - "Clock"
Cohesion: 0.13
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (21): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), pfeil(), Punkte, Add (+13 more)

### Community 22 - "common.rs"
Cohesion: 0.17
Nodes (21): format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), parse_version() (+13 more)

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
Cohesion: 0.11
Nodes (20): ContextPorts, ExecutionContext, ExecutionContext<'a>, ExecutionSource, Arc, AuthContext, IdentityProviderConfiguration, App (+12 more)

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
Cohesion: 0.09
Nodes (27): hash(), AccessToken, basis64url(), basis64url_decode(), constant_time_eq(), debug_gibt_das_token_nicht_preis(), hash_ist_stabil_und_vergleicht_konstantzeitig(), konstantzeit_vergleich_erkennt_jedes_gekippte_byte() (+19 more)

### Community 32 - "Benutzer"
Cohesion: 0.18
Nodes (12): befoerderung_ist_eine_domain_mutation(), Benutzer, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle, DateTime (+4 more)

### Community 33 - "RepoStorage"
Cohesion: 0.20
Nodes (10): RepoStorage, Box, Pin, Self, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository, InstrumentedPasswortWiederherstellungRepository (+2 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.09
Nodes (56): Benutzer, SessionAusgestellt, anonym(), build(), fremder_schluessel(), Harness, jetzt(), pruefen() (+48 more)

### Community 35 - "package.json"
Cohesion: 0.08
Nodes (22): name, private, version, Progress(), babel-plugin-react-compiler, @biomejs/biome, @dnd-kit/sortable, openapi-typescript (+14 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.15
Nodes (14): from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime, Option (+6 more)

### Community 40 - "EmailBestätigung"
Cohesion: 0.21
Nodes (13): bestätigung(), EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime, EmailAdresse (+5 more)

### Community 42 - "CookieConfig"
Cohesion: 0.19
Nodes (16): attribute(), clear_cookie_header(), CookieConfig, das_löschen_setzt_max_age_auch_ohne_lebensdauer(), das_löschen_trifft_dasselbe_cookie(), die_attribute_sind_vollständig(), die_lebensdauer_und_die_session_variante_setzen_max_age(), die_vorgaben_schreiben_nur_das_nötigste() (+8 more)

### Community 44 - "claims_for_session"
Cohesion: 0.20
Nodes (10): Anmeldung, AnmeldungFehler, EmailAdresse, Error, Result, claims_for_session(), new_session(), DateTime (+2 more)

### Community 46 - "SQLiteUnitOfWork"
Cohesion: 0.08
Nodes (22): Box, Error, Future, Option, Output, Pin, Result, Send (+14 more)

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.06
Nodes (54): from_parts_roundtrip(), id(), NeuerSchuss, pfeil_ist_eins_basiert(), pfeil_null_wird_abgelehnt(), pfeil_vier_und_später_sind_erlaubt(), NonZeroUsize, Result (+46 more)

### Community 49 - "Konto"
Cohesion: 0.15
Nodes (13): KontoRepository, Konto, KontoId, konto_from_row(), Arc, EmailAdresse, Mutex, Option (+5 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "BestätigungId"
Cohesion: 0.18
Nodes (11): BestätigungId, bestätigung_from_row(), Arc, EmailAdresse, Mutex, Option, RepositoryResult, Row (+3 more)

### Community 52 - "e2e/konto.rs"
Cohesion: 0.13
Nodes (19): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+11 more)

### Community 53 - "cn.ts"
Cohesion: 0.14
Nodes (11): Field(), FieldProps, LabelledProps, Label(), Textarea(), TextareaProps, Step, WorkflowSteps() (+3 more)

### Community 54 - "domain/kontakt.rs"
Cohesion: 0.13
Nodes (17): email_accepts_valid_address(), EmailAdresse, EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe() (+9 more)

### Community 55 - "InMemorySigning"
Cohesion: 0.09
Nodes (21): InMemorySigning, Default, ResultReport, Self, verify_with_public_key_hex(), ResultReport, SigningError, falscher_schlüssel_wird_abgelehnt() (+13 more)

### Community 56 - "token-swatches.tsx"
Cohesion: 0.23
Nodes (11): AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour(), relativeLuminance(), COLOUR_TOKENS, CONTRAST_PAIRS, Measurement (+3 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.07
Nodes (28): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+20 more)

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
Cohesion: 0.12
Nodes (9): AuthContext, id(), Option, ResultReport, BenutzerId, Uuid, Uuid, SessionId (+1 more)

### Community 65 - "LogandaimAppApi"
Cohesion: 0.16
Nodes (10): LogandaimAppApi, Arc, AuthContext, Benutzer, Option, Self, Uuid, AbmeldungFehler (+2 more)

### Community 66 - "react"
Cohesion: 0.20
Nodes (11): InkedControlsSection(), Checkbox(), CheckboxProps, RadioGroup(), RadioGroupItem(), RadioGroupOption(), Slider(), Switch() (+3 more)

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
Nodes (12): Session, option_datetime(), Arc, DateTime, Mutex, NeueSession, Option, RepositoryResult (+4 more)

### Community 71 - "Menge"
Cohesion: 0.22
Nodes (6): Menge, MengeFehler, Decimal, Result, Self, menge_to_str()

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "ClientKey"
Cohesion: 0.06
Nodes (46): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), fehlversuche_erschöpfen_das_budget_und_blockieren_weitere_anfragen(), gültige_anfragen_verbrauchen_das_budget_nicht(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now() (+38 more)

### Community 74 - "sortable-table-section.tsx"
Cohesion: 0.07
Nodes (27): zonen, CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow(), SortableTableSection(), State (+19 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "FakeDatastore"
Cohesion: 0.10
Nodes (22): FakeBenutzerRepository, FakeDatastore, FakeEmailBestätigungRepository, FakeKontoRepository, FakeSchussRepository, FakeSessionRepository, Arc, Benutzer (+14 more)

### Community 80 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 81 - "preis.rs"
Cohesion: 0.26
Nodes (9): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal (+1 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "UseCase"
Cohesion: 0.13
Nodes (15): PasswortWiederherstellenRequest, Context, Output, ResultReport, UseCase, PasswortVergessen, PasswortWiederherstellen, PasswortWiederherstellenFehler (+7 more)

### Community 84 - "http-client.ts"
Cohesion: 0.13
Nodes (18): ApiError, JsonClient, Benutzer, Schuss, SchussErstellungInput, Wertung, components, $defs (+10 more)

### Community 86 - "Ratio"
Cohesion: 0.24
Nodes (5): Ratio, RatioFehler, Decimal, Result, Self

### Community 87 - "String"
Cohesion: 0.21
Nodes (8): From, String, basis_url(), letzter_text_an(), nachrichten(), Value, Vec, token_aus()

### Community 88 - ".begin"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 89 - "FakeUnitOfWork"
Cohesion: 0.19
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 90 - "spend_open_confirmations"
Cohesion: 0.24
Nodes (9): RegistrierungAbschließen, RegistrierungStarten, DateTime, EmailAdresse, Error, Report, Result, Utc (+1 more)

### Community 91 - ".get_current_version"
Cohesion: 0.29
Nodes (6): ApplyMigrationUp, Box, Future, Output, Pin, Send

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.11
Nodes (23): Cookie, LogandaimApiSpec, openapi_service(), Benutzer, From, Into, IntoIterator, Item (+15 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "ResultReport"
Cohesion: 0.16
Nodes (13): ResultReport, AnmeldungRequest, BenutzerLöschungRequest, PasswortVergessenRequest, RegistrierungAbschlussRequest, RegistrierungStartRequest, EmailAdresse, Report (+5 more)

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
Cohesion: 0.20
Nodes (12): assert_status_ok(), base_app_builder(), cookie_pair(), E2eHarness, Arc, Duration, Self, test_konfiguration() (+4 more)

### Community 109 - "UnitOfWorkImpl"
Cohesion: 0.09
Nodes (12): RepositoryResult, Arc<T>, LockedUnitOfWorkImpl, Box, RepositoryResult, Self, Send, Sync (+4 more)

### Community 110 - "NewSessionResponse"
Cohesion: 0.24
Nodes (15): EndSessionResponse, EndSessionResponse<T>, NewSessionResponse, refresh_cookie_config(), From, Json, NeueSession, Option (+7 more)

### Community 111 - "user.rs"
Cohesion: 0.20
Nodes (11): Beförderung, BenutzerAnsicht, BenutzerAnzeigen, BenutzerLöschen, Benutzer, EmailAdresse, Error, Option (+3 more)

### Community 113 - "EmailAdresse"
Cohesion: 0.21
Nodes (8): domain::EmailAdresse, EmailAdresse, Error, Example, From, Result, Self, TryFrom

### Community 114 - "navigation.ts"
Cohesion: 0.17
Nodes (14): Sidebar(), SidebarLink(), SidebarProps, BreadcrumbSegment, homeNavItem, isExactPath(), isNavItemActive(), NavAction (+6 more)

### Community 115 - "Versioned"
Cohesion: 0.10
Nodes (16): BenutzerRepository, EmailBestätigungRepository, PasswortWiederherstellungRepository, RepositoryError, Benutzer, EmailAdresse, NeueSession, Option (+8 more)

### Community 116 - "auth-provider.tsx"
Cohesion: 0.18
Nodes (20): AuthContext, AuthContextValue, AuthProvider(), useAuth(), useAuthSession(), useAuthMutations(), useSchussRegistrierenMutation(), useWertungMutation() (+12 more)

### Community 117 - "layout.tsx"
Cohesion: 0.14
Nodes (11): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, Providers(), AppChrome() (+3 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "Preis"
Cohesion: 0.29
Nodes (5): Preis, Add, Output, Self, Mul

### Community 120 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 121 - "adresse.rs"
Cohesion: 0.20
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

### Community 126 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 127 - ".neu"
Cohesion: 0.40
Nodes (3): die_konfiguration_hält_nur_die_claim_identität(), Into, Self

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "delete_versioned"
Cohesion: 0.40
Nodes (4): delete_versioned(), RepositoryResult, Transaction, IntoParams

### Community 131 - "revoke_all_sessions"
Cohesion: 0.17
Nodes (12): Abmeldung, Erneuerungsausgang, revoke_all_sessions(), AuthContext, DateTime, Error, F, Report (+4 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "toggle-group.tsx"
Cohesion: 0.31
Nodes (6): ComposedPageSection(), DemoPage(), ToggleGroup(), ToggleGroupItem(), toggleGroupItemVariants, @radix-ui/react-toggle-group

### Community 135 - "provider.tsx"
Cohesion: 0.43
Nodes (5): createLogandaimApi(), LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.40
Nodes (5): ADR-0001: Baseline architecture, Appendix: spelling German identifiers, Consequences, Context, Decision

### Community 139 - "SQLitePasswortWiederherstellungRepository"
Cohesion: 0.21
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec, SQLitePasswortWiederherstellungRepository (+1 more)

### Community 140 - "HttpLogandaimApi"
Cohesion: 0.24
Nodes (3): HttpLogandaimApi, unwrap(), setAccessToken()

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 144 - ".schuss_registrieren"
Cohesion: 0.16
Nodes (13): Schuss, Wertung, domain::Wertungszone, From, Self, Uuid, schema_schuss_from_domain(), schema_wertung_from_domain() (+5 more)

### Community 145 - "konto/session.rs"
Cohesion: 0.23
Nodes (16): abgelaufene_session_ist_ungültig(), eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), jetzt(), neue_session_ist_gültig_und_läuft_in_einem_monat_ab(), Refreshed (+8 more)

### Community 146 - ".execute"
Cohesion: 0.33
Nodes (6): AuthContext, Context, F, ResultReport, O, U

### Community 147 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Duration, Mutex, NaiveDate, Self, Utc

### Community 149 - "apply_up_migrations"
Cohesion: 0.23
Nodes (9): AppliableMigration, apply_down_migrations(), apply_up_migrations(), MigrationError, MigrationTarget, Item, Iterator, Option (+1 more)

### Community 151 - "Versioned<T>"
Cohesion: 0.10
Nodes (12): Clone, Debug, Deref, Option, T, Target, Versioned<T>, DerefMut (+4 more)

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
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 926 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `revoke_all_sessions`, `StructuredError`, `http_status.rs`, `in_memory_object_store.rs`, `jwt.rs`, `konto/session.rs`, `passwort.rs`, `common.rs`, `ExecutionContext`, `SecretTokenHash`, `cases/konto.rs`, `CookieConfig`, `e2e/konto.rs`, `domain/kontakt.rs`, `InMemorySigning`, `DateTime`, `LogandaimApiTestClient`, `Mail`, `LogandaimAppApi`, `Session`, `Menge`, `ClientKey`, `UseCase`, `FakeUnitOfWork`, `LogandaimApiSpec`, `FakeObjectStore`, `config.rs`, `ResultReport`, `e2e/main.rs`, `NewSessionResponse`, `EmailAdresse`, `adresse.rs`, `Benutzer`, `.neu`?**
  _High betweenness centrality (0.258) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `Benutzer`, `cases/konto.rs`, `revoke_all_sessions`, `Session`, `SQLiteBenutzerRepository`, `claims_for_session`, `FakeDatastore`, `jwt.rs`, `user.rs`, `Konto`, `konto/session.rs`, `Versioned`, `passwort.rs`, `common.rs`, `DateTime`?**
  _High betweenness centrality (0.056) - this node is a cross-community bridge._
- **Why does `SecretTokenHash` connect `SecretTokenHash` to `BenutzerId`, `Session`, `PasswortWiederherstellung`, `EmailBestätigung`, `SQLitePasswortWiederherstellungRepository`, `FakeDatastore`, `konto/session.rs`, `Versioned`, `BestätigungId`, `String`, `DateTime`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ValidationError`, `LogandaimApiSpec`, `KontoFehler` to the rest of the system?**
  _476 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `FakePdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.11076923076923077 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._