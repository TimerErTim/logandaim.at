# Graph Report - logandaim.at  (2026-10-27)

## Corpus Check
- 246 files · ~126,903 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2963 nodes · 6714 edges · 144 communities (122 shown, 12 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 121 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `cbefcb03`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PdfRenderer
- arc_up
- Specifications
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- cn
- SchussErstellung
- http_status.rs
- demo-page.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- SQLitePasswortWiederherstellungRepository
- ObjectStore
- SecretTokenHash
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- BenutzerId
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
- token.rs
- App
- RepoStorage
- cases/konto.rs
- react
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- PasswortWiederherstellung
- .get_current_version
- revoke_all_sessions
- SQLiteUnitOfWork
- Vertical slices
- domain/schuss.rs
- SQLiteSessionRepository
- components.json
- instrumented.rs
- e2e/konto.rs
- InstrumentedUnitOfWork
- kontakt.rs
- jwt.rs
- token-swatches.tsx
- logandaim.at — Product Charter
- dependencies
- Session
- charter.md
- openapi_service
- Visual identity and design system
- MailSender
- AuthContext
- LogandaimAppApi
- SQLiteEmailBestätigungRepository
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- update_versioned
- Konto
- check-version.mjs
- InMemoryThrottle
- package.json
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- FakeDatastore
- Next.js
- postcss.config.mjs
- .finish
- schuss/page.tsx
- Contributing and project management
- UseCase
- provider.tsx
- Preis
- mailpit.rs
- export_spec.rs
- EmailAdresse
- tests/mail.rs
- .get_current_version
- LogandaimApiSpec
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- .begin
- SQLiteInstance
- UpMigration
- config.rs
- .partial_cmp
- String
- .add_dyn
- SQLiteBenutzerRepository
- UnitOfWorkImpl
- ClientKey
- user.rs
- Mail
- navigation.ts
- RepositoryResult
- UnitOfWork
- pull_request_template.md
- preis.rs
- Ratio
- adresse.rs
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- BenutzerAntwort
- Versioned
- Anlage-Anfrage — Requesting a new Anlage
- use_cases/session.rs
- StructuredError
- Option
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- kunde.rs
- dates.ts
- schema/schuss.rs
- Menge
- Versioned<T>
- ADR-0002: Session cookie auth across frontend and API origins
- m1-overseer.md
- Modes
- button.tsx
- m1-reviewer.md
- mise

## God Nodes (most connected - your core abstractions)
1. `Versioned` - 117 edges
2. `cn()` - 99 edges
3. `BenutzerId` - 62 edges
4. `EmailAdresse` - 47 edges
5. `Session` - 47 edges
6. `E` - 42 edges
7. `SecretTokenHash` - 41 edges
8. `EmailBestätigung` - 40 edges
9. `PasswortWiederherstellung` - 40 edges
10. `react` - 39 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `openapi_service()`  [INFERRED]
  backend/server/src/main.rs → crates/logandaim-api/src/spec.rs
- `logandaim-server` --depends_on--> `logandaim-api`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-api/Cargo.toml
- `logandaim-server` --depends_on--> `logandaim-filesystemstore`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-filesystemstore/Cargo.toml
- `logandaim-server` --depends_on--> `logandaim-smtp`  [EXTRACTED]
  backend/server/Cargo.toml → crates/logandaim-smtp/Cargo.toml
- `ConfigError` --references--> `String`  [EXTRACTED]
  backend/server/src/config.rs → crates/logandaim-api/src/errors/internal_error.rs

## Import Cycles
- 2-file cycle: `crates/logandaim-fakes/src/konto.rs -> crates/logandaim-fakes/src/repository.rs -> crates/logandaim-fakes/src/konto.rs`

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (144 total, 12 thin omitted)

### Community 0 - "PdfRenderer"
Cohesion: 0.09
Nodes (19): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, InstrumentedPdfRenderer, Vec, PdfDokument, PdfRenderer (+11 more)

### Community 1 - "arc_up"
Cohesion: 0.06
Nodes (42): AppliedLog, Cell, arc_down(), arc_up(), FakeDownMigration, FakeMigrationTarget, FakeUpMigration, Arc (+34 more)

### Community 2 - "Specifications"
Cohesion: 0.11
Nodes (14): Changelog, [Unreleased], Checklist, Release description, Release pull request, Test evidence, Version, Language (+6 more)

### Community 3 - "biome.json"
Cohesion: 0.08
Nodes (25): source, assist, actions, next, react, files, ignoreUnknown, includes (+17 more)

### Community 4 - "yams-persistence"
Cohesion: 0.08
Nodes (26): Hexagonal Architecture, yams-dto, yams-server, App, Embedded Deployment Mode, Server Deployment Mode, Domain-Driven Design, Driven Adapter (+18 more)

### Community 5 - "Seminar — Domain-Spezifikation"
Cohesion: 0.07
Nodes (25): Abrechnung — Domain-Spezifikation, Aggregates (Type-State), Geplant (nicht in diesem Slice), Invarianten, Stub-Umfang, Tagesabschluss-Ablauf, Use Cases, Wertobjekte (+17 more)

### Community 6 - "cn"
Cohesion: 0.17
Nodes (22): ComponentSection(), Example(), ThemePlate(), touchTargetOutline, GridPaper(), Legend(), LegendEntry, RingMark (+14 more)

### Community 7 - "SchussErstellung"
Cohesion: 0.18
Nodes (11): From, Self, Uuid, Vec, Wertungszone, SchussErstellung, SchussRegistrieren, WertungBerechnen (+3 more)

### Community 8 - "http_status.rs"
Cohesion: 0.12
Nodes (16): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+8 more)

### Community 9 - "demo-page.tsx"
Cohesion: 0.07
Nodes (52): PLATES, CHECKPOINTS, MESSREIHE, Row, START_ROWS, statusVariant, Zone, ZONEN (+44 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "SQLitePasswortWiederherstellungRepository"
Cohesion: 0.23
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec, SQLitePasswortWiederherstellungRepository (+1 more)

### Community 14 - "ObjectStore"
Cohesion: 0.07
Nodes (36): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+28 more)

### Community 15 - "SecretTokenHash"
Cohesion: 0.14
Nodes (15): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+7 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.20
Nodes (18): E, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+10 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "BenutzerId"
Cohesion: 0.06
Nodes (46): befoerderung_ist_eine_domain_mutation(), Benutzer, BenutzerId, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle (+38 more)

### Community 20 - "Clock"
Cohesion: 0.13
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (21): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), pfeil(), Punkte, Add (+13 more)

### Community 22 - "common.rs"
Cohesion: 0.21
Nodes (17): format_datetime(), format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), parse_version() (+9 more)

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
Cohesion: 0.10
Nodes (15): ContextPorts, ExecutionContext, ExecutionContext<'a>, ExecutionSource, Arc, die_konfiguration_hält_nur_die_claim_identität(), KontoKonfiguration, Into (+7 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 30 - "Konto — Benutzer, registration, and login"
Cohesion: 0.07
Nodes (28): Acceptance criteria, Account security, Administration, Approval, Auth audit, Decision log, Deferred capabilities, Domain terms and business rules (+20 more)

### Community 31 - "token.rs"
Cohesion: 0.09
Nodes (31): das_löschen_trifft_dasselbe_cookie(), das_token_steht_nur_im_cookie_und_hat_256_bit(), die_attribute_sind_vollständig(), löschen(), setzen(), setzen_raw(), token(), KontoFehler (+23 more)

### Community 32 - "App"
Cohesion: 0.26
Nodes (8): App, Arc, Box, Context, F, ResultReport, O, U

### Community 33 - "RepoStorage"
Cohesion: 0.20
Nodes (10): RepoStorage, Box, Pin, Self, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository, InstrumentedPasswortWiederherstellungRepository (+2 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.07
Nodes (70): abgelaufenes_token_wird_abgelehnt(), AccessTokenClaims, claims(), fenster_mit_exp_vor_iat_wird_abgelehnt(), gültiges_fenster_wird_akzeptiert(), herkunft_muss_exakt_stimmen(), iat_in_der_zukunft_wird_abgelehnt(), iat_innerhalb_der_toleranz_wird_akzeptiert() (+62 more)

### Community 35 - "react"
Cohesion: 0.08
Nodes (27): InkedControlsSection(), Checkbox(), CheckboxProps, Field(), FieldProps, LabelledProps, Input(), InputProps (+19 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.17
Nodes (13): from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime, Option (+5 more)

### Community 40 - ".get_current_version"
Cohesion: 0.19
Nodes (12): Box, Error, Future, Option, Output, Pin, Result, Send (+4 more)

### Community 44 - "revoke_all_sessions"
Cohesion: 0.21
Nodes (11): Anmeldung, Error, Result, claims_for_session(), new_session(), revoke_all_sessions(), DateTime, F (+3 more)

### Community 46 - "SQLiteUnitOfWork"
Cohesion: 0.17
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.06
Nodes (54): from_parts_roundtrip(), id(), NeuerSchuss, pfeil_ist_eins_basiert(), pfeil_null_wird_abgelehnt(), pfeil_vier_und_später_sind_erlaubt(), NonZeroUsize, Result (+46 more)

### Community 49 - "SQLiteSessionRepository"
Cohesion: 0.21
Nodes (12): option_datetime(), Arc, DateTime, Mutex, Option, RepositoryResult, Row, Transaction (+4 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "instrumented.rs"
Cohesion: 0.26
Nodes (8): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedThrottle, Arc, ObjectStoreError, ObjectStream, Option, ResultReport

### Community 52 - "e2e/konto.rs"
Cohesion: 0.05
Nodes (47): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft(), ac17_widerrufene_session_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+39 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.20
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - "kontakt.rs"
Cohesion: 0.14
Nodes (16): email_accepts_valid_address(), EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe(), normalisiert_ist_idempotent() (+8 more)

### Community 55 - "jwt.rs"
Cohesion: 0.06
Nodes (46): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), bearer_token(), claims(), decode(), DecodedToken, encode() (+38 more)

### Community 56 - "token-swatches.tsx"
Cohesion: 0.23
Nodes (11): AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour(), relativeLuminance(), COLOUR_TOKENS, CONTRAST_PAIRS, Measurement (+3 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.04
Nodes (47): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+39 more)

### Community 59 - "Session"
Cohesion: 0.06
Nodes (38): claims_for(), DateTime, Utc, eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), hash() (+30 more)

### Community 61 - "openapi_service"
Cohesion: 0.10
Nodes (25): openapi_service(), C, From, Into, IntoIterator, Item, ObjectStream, PlainText (+17 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "MailSender"
Cohesion: 0.16
Nodes (12): AsyncSmtpTransport, NoMailSender, MailSender, Send, Sync, KeinVersand, Option, Self (+4 more)

### Community 64 - "AuthContext"
Cohesion: 0.18
Nodes (8): LogandaimAppApi, Request, ResultReport, AccessError, AuthContext, id(), Option, ResultReport

### Community 65 - "LogandaimAppApi"
Cohesion: 0.11
Nodes (17): AusgestellteSession, LogandaimAppApi, Arc, Option, ResultReport, Schuss, Self, Uuid (+9 more)

### Community 66 - "SQLiteEmailBestätigungRepository"
Cohesion: 0.23
Nodes (9): bestätigung_from_row(), Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec (+1 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "update_versioned"
Cohesion: 0.33
Nodes (5): delete_versioned(), RepositoryResult, Transaction, update_versioned(), IntoParams

### Community 71 - "Konto"
Cohesion: 0.19
Nodes (11): Konto, KontoId, konto_from_row(), Arc, Mutex, Option, RepositoryResult, Row (+3 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "InMemoryThrottle"
Cohesion: 0.24
Nodes (15): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now(), DateTime, Default (+7 more)

### Community 74 - "package.json"
Cohesion: 0.05
Nodes (37): name, private, version, ComposedPageSection(), DemoPage(), CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS (+29 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "FakeDatastore"
Cohesion: 0.10
Nodes (22): FakeDatastore, FakeSchussRepository, Arc, Clone, Default, FxHashMap, Mutex, RepositoryResult (+14 more)

### Community 80 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 81 - "schuss/page.tsx"
Cohesion: 0.14
Nodes (22): useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), useLogandaimApi(), logandaimKeys, HomePage(), SchussPage(), zonen (+14 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "UseCase"
Cohesion: 0.10
Nodes (26): Report, Result, UseCase, AnmeldungFehler, send_mail(), PasswortVergessen, PasswortVergessenFehler, PasswortWiederherstellen (+18 more)

### Community 84 - "provider.tsx"
Cohesion: 0.05
Nodes (39): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, ApiError, HttpLogandaimApi, JsonClient, unwrap(), createLogandaimApi(), LogandaimApi (+31 more)

### Community 86 - "Preis"
Cohesion: 0.29
Nodes (5): Preis, Add, Output, Self, Mul

### Community 87 - "mailpit.rs"
Cohesion: 0.28
Nodes (5): basis_url(), letzter_text_an(), nachrichten(), Value, Vec

### Community 88 - "export_spec.rs"
Cohesion: 0.33
Nodes (4): main(), Box, RepositoryResult, Unimplemented

### Community 89 - "EmailAdresse"
Cohesion: 0.19
Nodes (8): adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), MailArt, Into, ResultReport, Self, EmailAdresse

### Community 90 - "tests/mail.rs"
Cohesion: 0.60
Nodes (5): absender(), die_nachricht_trägt_empfänger_betreff_und_klartext_rumpf(), eine_ungültige_adresse_ist_ein_zustellungsfehler(), entfalte_quoted_printable(), sender()

### Community 91 - ".get_current_version"
Cohesion: 0.40
Nodes (4): Future, Output, Pin, Send

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.18
Nodes (19): PasswortWiederherstellenAnfrage, abmelden(), anmelden(), benutzer_anzeigen(), benutzer_löschen(), erneuerung(), passwort_vergessen(), passwort_wiederherstellen() (+11 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - ".begin"
Cohesion: 0.50
Nodes (3): RepositoryError, ResultReport, SQLiteInstance

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 104 - "String"
Cohesion: 0.18
Nodes (19): BeendendeAntwort, BeendendeAntwort<T>, KontoAntwort, KontoAntwort<T>, From, Json, PlainText, Report (+11 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 106 - "SQLiteBenutzerRepository"
Cohesion: 0.24
Nodes (9): benutzer_from_row(), Arc, Benutzer, Mutex, Option, RepositoryResult, Row, Transaction (+1 more)

### Community 109 - "UnitOfWorkImpl"
Cohesion: 0.12
Nodes (6): Arc<T>, LockedUnitOfWorkImpl, Send, Sync, UnitOfWorkImpl, UnitOfWorkProvider

### Community 110 - "ClientKey"
Cohesion: 0.10
Nodes (19): NoThrottle, DateTime, ResultReport, Utc, DateTime, Utc, ClientKey, DateTime (+11 more)

### Community 111 - "user.rs"
Cohesion: 0.18
Nodes (11): Beförderung, BenutzerAnsicht, BenutzerAnzeigen, BenutzerLöschen, BenutzerLöschenFehler, Benutzer, Error, Option (+3 more)

### Community 112 - "Mail"
Cohesion: 0.22
Nodes (5): Mail, MailError, ResultReport, ResultReport, Message

### Community 114 - "navigation.ts"
Cohesion: 0.12
Nodes (23): AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar() (+15 more)

### Community 115 - "RepositoryResult"
Cohesion: 0.09
Nodes (14): BenutzerRepository, EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, RepositoryError, Benutzer, Option, RepositoryResult (+6 more)

### Community 117 - "UnitOfWork"
Cohesion: 0.23
Nodes (5): RepositoryResult, Box, RepositoryResult, UnitOfWork, Drop

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "preis.rs"
Cohesion: 0.26
Nodes (9): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler, Decimal (+1 more)

### Community 120 - "Ratio"
Cohesion: 0.24
Nodes (5): Ratio, RatioFehler, Decimal, Result, Self

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

### Community 125 - "BenutzerAntwort"
Cohesion: 0.21
Nodes (11): AnmeldeAntwort, BenutzerAntwort, Benutzer, From, Self, Uuid, SchemaBenutzer, SchemaRolle (+3 more)

### Community 127 - "Versioned"
Cohesion: 0.10
Nodes (24): Versioned, FakeBenutzerRepository, FakeEmailBestätigungRepository, FakeKontoRepository, FakeMailSender, FakePasswortWiederherstellungRepository, FakeSessionRepository, KontoDatastore (+16 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 131 - "use_cases/session.rs"
Cohesion: 0.23
Nodes (9): Abmeldung, AbmeldungFehler, Erneuerung, ErneuerungAusgestellt, ErneuerungFehler, Erneuerungsausgang, Error, Result (+1 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.40
Nodes (5): ADR-0001: Baseline architecture, Appendix: spelling German identifiers, Consequences, Context, Decision

### Community 140 - "kunde.rs"
Cohesion: 0.54
Nodes (7): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), IpAddr, Option

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 144 - "schema/schuss.rs"
Cohesion: 0.27
Nodes (9): domain::Wertungszone, From, Self, Uuid, schema_schuss_from_domain(), schema_wertung_from_domain(), Schuss, Wertung (+1 more)

### Community 150 - "Menge"
Cohesion: 0.22
Nodes (6): Menge, MengeFehler, Decimal, Result, Self, menge_to_str()

### Community 151 - "Versioned<T>"
Cohesion: 0.11
Nodes (12): Clone, Debug, Deref, Self, T, Target, UnitOfWork<'a>, Versioned<T> (+4 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.20
Nodes (10): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+2 more)

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

## Knowledge Gaps
- **473 isolated node(s):** `BeendendeAntwort<T>`, `ValidationError`, `KontoFehler`, `SchussRegistrierenFehler`, `WertungFehler` (+468 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 881 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `StructuredError`, `ObjectStore`, `SecretTokenHash`, `BenutzerId`, `common.rs`, `Menge`, `ExecutionContext`, `token.rs`, `cases/konto.rs`, `SQLiteSessionRepository`, `e2e/konto.rs`, `kontakt.rs`, `jwt.rs`, `Session`, `MailSender`, `LogandaimAppApi`, `FakeDatastore`, `mailpit.rs`, `EmailAdresse`, `tests/mail.rs`, `LogandaimApiSpec`, `config.rs`, `ClientKey`, `Mail`, `adresse.rs`, `BenutzerAntwort`?**
  _High betweenness centrality (0.189) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `AuthContext`, `cases/konto.rs`, `Konto`, `SQLiteBenutzerRepository`, `revoke_all_sessions`, `user.rs`, `SQLiteSessionRepository`, `RepositoryResult`, `jwt.rs`, `Session`, `Versioned`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `Versioned` connect `Versioned` to `SQLiteEmailBestätigungRepository`, `update_versioned`, `PasswortWiederherstellung`, `Konto`, `SQLiteBenutzerRepository`, `UnitOfWorkImpl`, `FakeDatastore`, `SecretTokenHash`, `SQLitePasswortWiederherstellungRepository`, `domain/schuss.rs`, `SQLiteSessionRepository`, `RepositoryResult`, `Versioned<T>`, `Session`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `BeendendeAntwort<T>`, `ValidationError`, `KontoFehler` to the rest of the system?**
  _473 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.09475806451612903 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._