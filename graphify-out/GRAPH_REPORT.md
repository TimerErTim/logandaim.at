# Graph Report - logandaim.at  (2026-10-27)

## Corpus Check
- 246 files · ~125,903 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2964 nodes · 6691 edges · 169 communities (144 shown, 15 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 110 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e180e93b`
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
- button.tsx
- in_memory_object_store.rs
- SecretTokenHash
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- passwort.rs
- Clock
- Punkte
- common.rs
- <Title>
- Make invalid state unrepresentable
- Betrieb — Operations and launch readiness
- check-release.mjs
- ContextPorts
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
- ExecutionContext
- SQLiteUnitOfWork
- Vertical slices
- SchussId
- SessionId
- components.json
- instrumented.rs
- e2e/konto.rs
- InstrumentedUnitOfWork
- kontakt.rs
- jwt.rs
- token-swatches.tsx
- logandaim.at — Product Charter
- dependencies
- DateTime
- charter.md
- Json
- Visual identity and design system
- SigningKeyId
- AuthContext
- ClientKey
- schuss/page.tsx
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- FakeObjectStore
- format_datetime
- check-version.mjs
- InMemoryThrottle
- sortable-table-section.tsx
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- FakeDatastore
- Next.js
- postcss.config.mjs
- InMemorySigning
- hooks/index.ts
- Contributing and project management
- Anmeldung
- http-client.ts
- AccessTokenClaims
- SigningKeypair
- Session
- Konto
- ZielId
- BenutzerId
- LogandaimApiSpec
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- EmailAdresse
- SQLiteInstance
- UpMigration
- config.rs
- ObjectStore
- SitzungAntwort
- .add_dyn
- SQLiteBenutzerRepository
- layout.tsx
- .perform
- UnitOfWorkProvider
- Throttle
- user.rs
- Mail
- SQLiteEmailBestätigungRepository
- navigation.ts
- RepositoryResult
- domain/schuss.rs
- UnitOfWorkImpl
- pull_request_template.md
- preis.rs
- Ratio
- adresse.rs
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- String
- FakeUnitOfWork
- Versioned
- Anlage-Anfrage — Requesting a new Anlage
- SQLitePasswortWiederherstellungRepository
- UseCase
- StructuredError
- schema.ts
- .get_current_version
- Option
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- openapi_service
- dates.ts
- konto/signing.rs
- schema/schuss.rs
- SchussRegistrieren
- .from_parts
- .rendern
- SessionIn
- BenutzerRepository
- Menge
- Versioned<T>
- SchussRepository
- ADR-0002: Session cookie auth across frontend and API origins
- wertung.rs
- m1-overseer.md
- Modes
- package.json
- devDependencies
- scripts
- m1-reviewer.md
- base_app_builder
- MailSender
- provider.tsx
- ApiError
- SmtpKonfiguration
- tests/mail.rs
- FakeMailSender
- mise

## God Nodes (most connected - your core abstractions)
1. `cn()` - 99 edges
2. `Versioned` - 84 edges
3. `BenutzerId` - 65 edges
4. `EmailAdresse` - 47 edges
5. `Session` - 45 edges
6. `E` - 43 edges
7. `SecretTokenHash` - 41 edges
8. `react` - 39 edges
9. `EmailBestätigung` - 37 edges
10. `PasswortWiederherstellung` - 37 edges

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

## Communities (169 total, 15 thin omitted)

### Community 0 - "PdfRenderer"
Cohesion: 0.13
Nodes (14): Vec, PdfDokument, PdfRenderer, PdfRenderError, ResultReport, Send, Sync, Vec (+6 more)

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
Cohesion: 0.27
Nodes (9): From, Self, Uuid, Vec, Wertungszone, SchussErstellung, SchussRegistrieren, WertungBerechnen (+1 more)

### Community 8 - "http_status.rs"
Cohesion: 0.12
Nodes (16): HttpStatusMapping, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option, Report (+8 more)

### Community 9 - "demo-page.tsx"
Cohesion: 0.07
Nodes (51): PLATES, CHECKPOINTS, MESSREIHE, Row, START_ROWS, statusVariant, Zone, ZONEN (+43 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "button.tsx"
Cohesion: 0.16
Nodes (11): SAMPLE_ROWS, TypeSection(), Alert(), AlertProps, alertVariants, iconColour, Button(), ButtonProps (+3 more)

### Community 14 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 15 - "SecretTokenHash"
Cohesion: 0.13
Nodes (16): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+8 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.20
Nodes (18): E, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+10 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "passwort.rs"
Cohesion: 0.15
Nodes (19): argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gültige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password(), hash_und_verify_stimmen_überein() (+11 more)

### Community 20 - "Clock"
Cohesion: 0.09
Nodes (17): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+9 more)

### Community 21 - "Punkte"
Cohesion: 0.14
Nodes (16): ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), Punkte, Add, I, IntoIterator, Item, Iterator (+8 more)

### Community 22 - "common.rs"
Cohesion: 0.16
Nodes (22): benutzer_from_row(), Row, format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid() (+14 more)

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
Nodes (8): ContextPorts, ExecutionContext<'a>, Arc, RepositoryResult, die_konfiguration_hält_nur_die_claim_identität(), KontoKonfiguration, Into, Self

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
Cohesion: 0.15
Nodes (11): App, Arc, Box, Context, F, ResultReport, Send, Sync (+3 more)

### Community 33 - "RepoStorage"
Cohesion: 0.20
Nodes (10): RepoStorage, Box, Pin, Self, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository, InstrumentedPasswortWiederherstellungRepository (+2 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.10
Nodes (54): Benutzer, SitzungAusgestellt, SitzungPrüfen, anonym(), build(), Harness, jetzt(), pruefen() (+46 more)

### Community 35 - "react"
Cohesion: 0.10
Nodes (21): InkedControlsSection(), Checkbox(), CheckboxProps, Field(), FieldProps, LabelledProps, Label(), RadioGroup() (+13 more)

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "PasswortWiederherstellung"
Cohesion: 0.18
Nodes (13): from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId, DateTime, Option (+5 more)

### Community 40 - ".get_current_version"
Cohesion: 0.19
Nodes (12): Box, Error, Future, Option, Output, Pin, Result, Send (+4 more)

### Community 44 - "ExecutionContext"
Cohesion: 0.14
Nodes (21): ExecutionContext, Error, Result, claims_for_session(), new_session(), revoke_all_sessions(), DateTime, F (+13 more)

### Community 46 - "SQLiteUnitOfWork"
Cohesion: 0.17
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "SchussId"
Cohesion: 0.17
Nodes (17): id(), SchussId, Arc, Mutex, Option, RepositoryError, RepositoryResult, Result (+9 more)

### Community 49 - "SessionId"
Cohesion: 0.14
Nodes (14): Uuid, SessionId, option_datetime(), Arc, DateTime, Mutex, Option, RepositoryResult (+6 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "instrumented.rs"
Cohesion: 0.24
Nodes (9): InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, InstrumentedThrottle, Arc, ObjectStoreError, ObjectStream, Option (+1 more)

### Community 52 - "e2e/konto.rs"
Cohesion: 0.07
Nodes (40): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sitzungen_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_signatur_wird_gegen_den_schlüssel_der_sitzungszeile_geprüft(), ac17_widerrufene_sitzung_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt() (+32 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.43
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - "kontakt.rs"
Cohesion: 0.14
Nodes (16): email_accepts_valid_address(), EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe(), normalisiert_ist_idempotent() (+8 more)

### Community 55 - "jwt.rs"
Cohesion: 0.15
Nodes (23): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), bearer_token(), claims(), decode(), DecodedToken, encode() (+15 more)

### Community 56 - "token-swatches.tsx"
Cohesion: 0.18
Nodes (14): AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour(), relativeLuminance(), COLOUR_TOKENS, CONTRAST_PAIRS, Measurement (+6 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.07
Nodes (28): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+20 more)

### Community 59 - "DateTime"
Cohesion: 0.15
Nodes (8): DateTime, Option, Utc, SessionIn<S>, SessionState, state::Ok, state::Refreshed, state::Revoked

### Community 61 - "Json"
Cohesion: 0.14
Nodes (19): C, From, Json, ObjectStream, PlainText, Report, Result, Schuss (+11 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "SigningKeyId"
Cohesion: 0.15
Nodes (9): kid(), NeueSession, Into, Self, SessionIn<state::Ok>, SigningKeyId, claims_fuer(), DateTime (+1 more)

### Community 64 - "AuthContext"
Cohesion: 0.18
Nodes (8): LogandaimAppApi, Request, ResultReport, AccessError, AuthContext, id(), Option, ResultReport

### Community 65 - "ClientKey"
Cohesion: 0.12
Nodes (16): AusgestellteSitzung, LogandaimAppApi, Arc, Option, ResultReport, Schuss, Self, Uuid (+8 more)

### Community 66 - "schuss/page.tsx"
Cohesion: 0.15
Nodes (18): zonen, Card(), CardContent(), CardDescription(), CardFooter(), CardHeader(), CardTitle(), Input() (+10 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 71 - "format_datetime"
Cohesion: 0.18
Nodes (10): format_datetime(), DateTime, Utc, Arc, Mutex, Option, RepositoryResult, Transaction (+2 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "InMemoryThrottle"
Cohesion: 0.24
Nodes (15): aktionen_und_clients_haben_getrennte_budgets(), client(), das_budget_wird_genau_einmal_ausgeschöpft(), InMemoryThrottle, nach_dem_fenster_ist_das_budget_wieder_frei(), now(), DateTime, Default (+7 more)

### Community 74 - "sortable-table-section.tsx"
Cohesion: 0.08
Nodes (19): ComposedPageSection(), DemoPage(), CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow(), SortableTableSection() (+11 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "FakeDatastore"
Cohesion: 0.12
Nodes (14): Arc, Self, FakeDatastore, FakeSchussRepository, Arc, Clone, Default, FxHashMap (+6 more)

### Community 80 - "InMemorySigning"
Cohesion: 0.15
Nodes (9): InMemorySigning, ResultReport, Self, verify_with_public_key_hex(), ResultReport, SigningError, Debug, Signature (+1 more)

### Community 81 - "hooks/index.ts"
Cohesion: 0.27
Nodes (8): useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), useLogandaimApi(), logandaimKeys, HomePage(), SchussPage(), DeploymentStatus()

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "Anmeldung"
Cohesion: 0.13
Nodes (14): AnmeldeAnfrage, PasswortWiederherstellenAnfrage, RegistrierungAbschlussAnfrage, RegistrierungStartAnfrage, Report, Result, Anmeldung, AnmeldungFehler (+6 more)

### Community 84 - "http-client.ts"
Cohesion: 0.20
Nodes (8): HttpLogandaimApi, JsonClient, unwrap(), LogandaimApi, Schuss, SchussErstellungInput, Wertung, openapi-fetch

### Community 86 - "AccessTokenClaims"
Cohesion: 0.26
Nodes (14): abgelaufenes_token_wird_abgelehnt(), AccessTokenClaims, claims(), fenster_mit_exp_vor_iat_wird_abgelehnt(), gültiges_fenster_wird_akzeptiert(), herkunft_muss_exakt_stimmen(), iat_in_der_zukunft_wird_abgelehnt(), iat_innerhalb_der_toleranz_wird_akzeptiert() (+6 more)

### Community 87 - "SigningKeypair"
Cohesion: 0.24
Nodes (8): kaputtes_pem_wird_abgelehnt(), Formatter, Into, KontoFehler, Result, Self, SigningKeypair, SigningKey

### Community 88 - "Session"
Cohesion: 0.19
Nodes (12): eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden(), eine_widerrufene_session_kann_nicht_erneuert_werden(), erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an(), from_parts_roundtrip_über_alle_drei_zustände(), jetzt(), Ok, Refreshed, Revoked (+4 more)

### Community 89 - "Konto"
Cohesion: 0.18
Nodes (14): email(), from_parts_roundtrip(), hash(), jetzt(), Konto, KontoId, neues_konto_bestätigt_die_adresse(), NeuesKonto (+6 more)

### Community 90 - "ZielId"
Cohesion: 0.26
Nodes (6): NeuerSchuss, NonZeroUsize, Uuid, Wertungszone, Schuss, ZielId

### Community 91 - "BenutzerId"
Cohesion: 0.14
Nodes (14): befoerderung_ist_eine_domain_mutation(), Benutzer, BenutzerId, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle (+6 more)

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.19
Nodes (20): BenutzerLöschungAnfrage, PasswortVergessenAnfrage, abmelden(), anmelden(), benutzer_anzeigen(), benutzer_löschen(), erneuerung(), passwort_vergessen() (+12 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "EmailAdresse"
Cohesion: 0.19
Nodes (7): adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), MailArt, Into, Self, EmailAdresse

### Community 100 - "SQLiteInstance"
Cohesion: 0.16
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 104 - "SitzungAntwort"
Cohesion: 0.20
Nodes (15): BeendendeAntwort<T>, KontoAntwort, KontoAntwort<T>, From, Json, PlainText, Report, Result (+7 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 106 - "SQLiteBenutzerRepository"
Cohesion: 0.24
Nodes (7): Arc, Benutzer, Mutex, Option, RepositoryResult, Transaction, SQLiteBenutzerRepository

### Community 107 - "layout.tsx"
Cohesion: 0.16
Nodes (10): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, Providers(), createQueryClient() (+2 more)

### Community 108 - ".perform"
Cohesion: 0.27
Nodes (9): delete_confirmations(), RegistrierungAbschließen, RegistrierungAbschliessenFehler, DateTime, Error, Report, Result, Utc (+1 more)

### Community 109 - "UnitOfWorkProvider"
Cohesion: 0.14
Nodes (11): Box, RepositoryResult, Unimplemented, ExecutionSource, Arc<T>, Send, Sync, UnitOfWorkProvider (+3 more)

### Community 110 - "Throttle"
Cohesion: 0.10
Nodes (16): main(), NoThrottle, DateTime, ResultReport, Utc, DateTime, Utc, DateTime (+8 more)

### Community 111 - "user.rs"
Cohesion: 0.18
Nodes (11): Beförderung, BenutzerAnsicht, BenutzerAnzeigen, BenutzerLoeschenFehler, BenutzerLöschen, Benutzer, Error, Option (+3 more)

### Community 112 - "Mail"
Cohesion: 0.18
Nodes (9): AsyncSmtpTransport, NoMailSender, Mail, MailError, ResultReport, ResultReport, SmtpMailSender, Message (+1 more)

### Community 113 - "SQLiteEmailBestätigungRepository"
Cohesion: 0.23
Nodes (9): bestätigung_from_row(), Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec (+1 more)

### Community 114 - "navigation.ts"
Cohesion: 0.12
Nodes (23): AppChrome(), AppShell(), PageChromeContext, PageChromeContextValue, PageChromeProvider(), SetBreadcrumbLabel(), usePageChrome(), Sidebar() (+15 more)

### Community 115 - "RepositoryResult"
Cohesion: 0.08
Nodes (10): EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, RepositoryError, Option, RepositoryResult, Send, Sync (+2 more)

### Community 116 - "domain/schuss.rs"
Cohesion: 0.36
Nodes (8): wertung_ab_pfeil_vier_zählt_null(), wertung_aus_schüssen(), wertung_hält_ziele_auseinander(), wertung_ordnet_auch_bei_unsortierter_eingabe(), wertung_summiert_je_ziel_und_ordnung_nach_pfeil(), Wertungsergebnis, Vec, WertungBerechnen

### Community 117 - "UnitOfWorkImpl"
Cohesion: 0.15
Nodes (10): LockedUnitOfWorkImpl, Box, Option, RepositoryResult, Self, UnitOfWork, UnitOfWork<'a>, UnitOfWorkImpl (+2 more)

### Community 118 - "pull_request_template.md"
Cohesion: 0.33
Nodes (5): Acceptance criteria, Behavior, Checklist, Notes, Test evidence

### Community 119 - "preis.rs"
Cohesion: 0.15
Nodes (14): nach_rabatt_full_is_zero(), nach_rabatt_twenty_percent(), nach_rabatt_zero_keeps_basis(), Preis, preis_add_sums_values(), preis_times_menge_scales_value(), preis_times_ratio_scales_value(), PreisFehler (+6 more)

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

### Community 125 - "String"
Cohesion: 0.10
Nodes (19): From, Self, String, AnmeldeAntwort, BenutzerAntwort, Benutzer, From, Self (+11 more)

### Community 126 - "FakeUnitOfWork"
Cohesion: 0.20
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 127 - "Versioned"
Cohesion: 0.12
Nodes (19): Versioned, FakeBenutzerRepository, FakeEmailBestätigungRepository, FakeKontoRepository, FakePasswortWiederherstellungRepository, FakeSessionRepository, KontoDatastore, Benutzer (+11 more)

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "SQLitePasswortWiederherstellungRepository"
Cohesion: 0.24
Nodes (9): Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec, SQLitePasswortWiederherstellungRepository (+1 more)

### Community 131 - "UseCase"
Cohesion: 0.19
Nodes (11): Context, Output, ResultReport, UseCase, Abmeldung, AbmeldungFehler, Erneuerung, ErneuerungAusgestellt (+3 more)

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "schema.ts"
Cohesion: 0.14
Nodes (13): components, $defs, operations, paths, SchemaRolle, Benutzer, Systemadministrator, webhooks (+5 more)

### Community 134 - ".get_current_version"
Cohesion: 0.40
Nodes (4): Future, Output, Pin, Send

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.40
Nodes (5): ADR-0001: Baseline architecture, Appendix: spelling German identifiers, Consequences, Context, Decision

### Community 140 - "openapi_service"
Cohesion: 0.13
Nodes (16): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), client_key(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), IpAddr, Option, openapi_service() (+8 more)

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - "konto/signing.rs"
Cohesion: 0.29
Nodes (7): debug_gibt_den_privatschlüssel_nicht_preis(), falscher_schlüssel_wird_abgelehnt(), gekipptes_byte_wird_abgelehnt(), jedes_byte_ist_signierbar(), keypair(), public_key_verifikation_über_die_gespeicherte_zeile(), signieren_und_verifizieren_roundtrip()

### Community 144 - "schema/schuss.rs"
Cohesion: 0.27
Nodes (9): domain::Wertungszone, From, Self, Uuid, schema_schuss_from_domain(), schema_wertung_from_domain(), Schuss, Wertung (+1 more)

### Community 145 - "SchussRegistrieren"
Cohesion: 0.25
Nodes (7): Error, Result, Schuss, Wertungszone, SchussRegistrieren, SchussRegistrierenFehler, WertungFehler

### Community 146 - ".from_parts"
Cohesion: 0.33
Nodes (8): from_parts_roundtrip(), pfeil_ist_eins_basiert(), pfeil_null_wird_abgelehnt(), pfeil_vier_und_später_sind_erlaubt(), Result, Self, SchussFehler, ziel()

### Community 147 - ".rendern"
Cohesion: 0.38
Nodes (4): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec

### Community 148 - "SessionIn"
Cohesion: 0.28
Nodes (6): KontoFehler, From, Result, S, SessionIn, SessionNotLive

### Community 150 - "Menge"
Cohesion: 0.22
Nodes (6): Menge, MengeFehler, Decimal, Result, Self, menge_to_str()

### Community 151 - "Versioned<T>"
Cohesion: 0.10
Nodes (16): C, Clone, Debug, Deref, F, Formatter, Future, Output (+8 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.20
Nodes (10): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+2 more)

### Community 154 - "wertung.rs"
Cohesion: 0.29
Nodes (5): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), pfeil(), ziel()

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "package.json"
Cohesion: 0.09
Nodes (21): name, private, version, Progress(), babel-plugin-react-compiler, @biomejs/biome, @dnd-kit/sortable, openapi-typescript (+13 more)

### Community 158 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 159 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

### Community 161 - "base_app_builder"
Cohesion: 0.31
Nodes (8): base_app_builder(), AppBuilder, SetUowProvider, schuss_registrieren_persists_and_can_be_loaded(), wertung_leere_liste_ist_fehler(), wertung_summiert_nach_pfeil_innerhalb_eines_ziels(), wertung_unbekannte_id_ist_fehler(), ziel_id()

### Community 162 - "MailSender"
Cohesion: 0.29
Nodes (5): MailSender, ResultReport, Send, Sync, KeinVersand

### Community 163 - "provider.tsx"
Cohesion: 0.36
Nodes (6): createLogandaimApi(), LogandaimApiContext, LogandaimApiContextValue, LogandaimApiProvider(), config, @tanstack/react-query-devtools

### Community 165 - "SmtpKonfiguration"
Cohesion: 0.40
Nodes (4): Option, Self, SmtpKonfiguration, Verschlüsselung

### Community 166 - "tests/mail.rs"
Cohesion: 0.60
Nodes (5): absender(), die_nachricht_trägt_empfänger_betreff_und_klartext_rumpf(), eine_ungültige_adresse_ist_ein_zustellungsfehler(), entfalte_quoted_printable(), sender()

## Knowledge Gaps
- **473 isolated node(s):** `BeendendeAntwort<T>`, `ValidationError`, `KontoFehler`, `SchussRegistrierenFehler`, `WertungFehler` (+468 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 875 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `StructuredError`, `in_memory_object_store.rs`, `konto/signing.rs`, `SecretTokenHash`, `passwort.rs`, `SessionIn`, `common.rs`, `Menge`, `ContextPorts`, `token.rs`, `App`, `cases/konto.rs`, `SmtpKonfiguration`, `tests/mail.rs`, `SessionId`, `e2e/konto.rs`, `kontakt.rs`, `jwt.rs`, `Json`, `SigningKeyId`, `ClientKey`, `FakeObjectStore`, `format_datetime`, `InMemorySigning`, `Anmeldung`, `AccessTokenClaims`, `SigningKeypair`, `Session`, `Konto`, `LogandaimApiSpec`, `EmailAdresse`, `config.rs`, `SitzungAntwort`, `Mail`, `adresse.rs`, `FakeUnitOfWork`?**
  _High betweenness centrality (0.199) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `AuthContext`, `cases/konto.rs`, `format_datetime`, `SQLiteBenutzerRepository`, `ExecutionContext`, `user.rs`, `SessionId`, `RepositoryResult`, `SessionIn`, `BenutzerRepository`, `AccessTokenClaims`, `jwt.rs`, `Session`, `Konto`, `common.rs`, `Versioned`, `SigningKeyId`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `EmailAdresse` connect `EmailAdresse` to `cases/konto.rs`, `SmtpKonfiguration`, `tests/mail.rs`, `format_datetime`, `.perform`, `SecretTokenHash`, `Mail`, `user.rs`, `SQLiteEmailBestätigungRepository`, `RepositoryResult`, `Anmeldung`, `kontakt.rs`, `Konto`, `String`, `Versioned`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `BeendendeAntwort<T>`, `ValidationError`, `KontoFehler` to the rest of the system?**
  _473 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.12648221343873517 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._