# Graph Report - logandaim.at  (2026-10-26)

## Corpus Check
- 247 files · ~121,997 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2922 nodes · 6432 edges · 156 communities (134 shown, 11 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 108 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `672dff95`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PdfRenderer
- arc_up
- Specifications
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- data-figures.tsx
- SchussErstellung
- StatusCode
- cn
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- button.tsx
- in_memory_object_store.rs
- EmailBestätigung
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- domain/konto/passwort.rs
- Clock
- wertung.rs
- common.rs
- <Title>
- Make invalid state unrepresentable
- Betrieb — Operations and launch readiness
- check-release.mjs
- ContextPorts
- Wertung — Classic 3-arrow scoring
- .contextualize_with
- Konto — Benutzer, registration, and login
- KontoFehler
- App
- RepoStorage
- cases/konto.rs
- .prüfe_und_zähle
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- Harness
- .get_current_version
- SQLiteUnitOfWork
- Vertical slices
- domain/schuss.rs
- format_datetime
- components.json
- instrumented.rs
- e2e/konto.rs
- InstrumentedUnitOfWork
- kontakt.rs
- jwt.rs
- page.dev.tsx
- logandaim.at — Product Charter
- dependencies
- Session
- charter.md
- Json
- Visual identity and design system
- demo-page.tsx
- AuthContext
- LogandaimAppApi
- ExecutionContext
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- .schuss_registrieren
- FakeObjectStore
- check-version.mjs
- InMemoryDrossel
- sortable-table-section.tsx
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- UnitOfWorkImpl
- Next.js
- postcss.config.mjs
- toggle-group.tsx
- react
- Contributing and project management
- BenutzerAnsicht
- provider.tsx
- RepositoryResult
- SigningKeypair
- ObjectStore
- EmailAdresse
- PasswortWiederherstellung
- Rolle
- LogandaimApiSpec
- check-design.mjs
- File Icon
- Globe Icon
- Next.js Logo
- Vercel Logo
- Window Icon
- FakeDatastore
- SQLiteInstance
- UpMigration
- config.rs
- kunde.rs
- SitzungAntwort
- .add_dyn
- BenutzerId
- SQLiteEmailBestätigungRepository
- SQLiteKontoRepository
- Request
- KundeSchlüssel
- UseCase
- MailSender
- String
- navigation.ts
- UnitOfWork
- Mail
- LockedUnitOfWorkImpl
- pull_request_template.md
- preis.rs
- Ratio
- adresse.rs
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- BenutzerAntwort
- cookie.rs
- signing.rs
- Anlage-Anfrage — Requesting a new Anlage
- FixedClock
- Preis
- StructuredError
- tests/mail.rs
- .get_current_version
- Option
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- openapi_service
- dates.ts
- ports/mail.rs
- check-identifiers.mjs
- .finish
- StreamBody
- .rendern
- Menge
- Versioned<T>
- ADR-0002: Session cookie auth across frontend and API origins
- m1-overseer.md
- Modes
- package.json
- m1-reviewer.md
- mise

## God Nodes (most connected - your core abstractions)
1. `cn()` - 99 edges
2. `BenutzerId` - 61 edges
3. `EmailAdresse` - 43 edges
4. `Session` - 40 edges
5. `react` - 39 edges
6. `E` - 37 edges
7. `TokenHash` - 37 edges
8. `KundeSchlüssel` - 36 edges
9. `Konto` - 35 edges
10. `ExecutionContext` - 33 edges

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
- 2-file cycle: `crates/logandaim-core/src/domain/konto/mod.rs -> crates/logandaim-core/src/domain/konto/token.rs -> crates/logandaim-core/src/domain/konto/mod.rs`

## Hyperedges (group relationships)
- **YAMS Deployment Modes** — cursor_agents_md_deployment_server_mode, cursor_agents_md_deployment_embedded_mode, cursor_agents_md_yams_app_api, cursor_agents_md_app [EXTRACTED 1.00]
- **diffx Code Review Workflow** — agents_skills_diffx_start_review_skill, agents_skills_diffx_server, agents_skills_diffx_finish_review_skill [EXTRACTED 1.00]
- **YAMS Hexagonal Backend Layers** — cursor_agents_md_driving_adapters, cursor_agents_md_yams_api, cursor_agents_md_yams_core, cursor_agents_md_yams_persistence [EXTRACTED 1.00]

## Communities (156 total, 11 thin omitted)

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

### Community 6 - "data-figures.tsx"
Cohesion: 0.27
Nodes (9): GridPaper(), Legend(), LegendEntry, RingMark, RingPlot(), ScaleBar(), DataFiguresSection(), SeriesPlot() (+1 more)

### Community 7 - "SchussErstellung"
Cohesion: 0.27
Nodes (9): From, Self, Uuid, Vec, Wertungszone, SchussErstellung, SchussRegistrieren, WertungBerechnen (+1 more)

### Community 8 - "StatusCode"
Cohesion: 0.12
Nodes (17): HttpStatusMapping, KontoDienstFehler, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option (+9 more)

### Community 9 - "cn"
Cohesion: 0.08
Nodes (45): ThemePlate(), BadgeSection(), ButtonSection(), CardSection(), FormSection(), MiscSection(), zonen, Dialog (+37 more)

### Community 10 - "logandaim-filesystemstore/src/lib.rs"
Cohesion: 0.13
Nodes (27): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), FileStream, FileSystemObjectStore, key_to_path(), missing_is_none(), overwrites_existing() (+19 more)

### Community 12 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, esModuleInterop, incremental, isolatedModules, jsx, lib, module (+10 more)

### Community 13 - "button.tsx"
Cohesion: 0.22
Nodes (9): Alert(), AlertProps, alertVariants, iconColour, Button(), ButtonProps, buttonVariants, class-variance-authority (+1 more)

### Community 14 - "in_memory_object_store.rs"
Cohesion: 0.21
Nodes (15): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+7 more)

### Community 15 - "EmailBestätigung"
Cohesion: 0.17
Nodes (14): bestätigung(), BestätigungId, EmailBestätigung, from_parts_roundtrip(), jetzt(), neue_bestätigung_gilt_24_stunden(), NeueBestätigung, DateTime (+6 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.20
Nodes (18): E, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+10 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "domain/konto/passwort.rs"
Cohesion: 0.14
Nodes (19): argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gültige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password(), hash_und_verify_stimmen_überein() (+11 more)

### Community 20 - "Clock"
Cohesion: 0.13
Nodes (10): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+2 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (21): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), pfeil(), Punkte, Add (+13 more)

### Community 22 - "common.rs"
Cohesion: 0.22
Nodes (15): format_naive_date(), parse_datetime(), parse_decimal(), parse_menge(), parse_naive_date(), parse_uuid(), DateTime, Decimal (+7 more)

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
Nodes (9): ContextPorts, ExecutionContext<'a>, Arc, RepositoryResult, Self, die_konfiguration_hält_nur_die_claim_identität(), KontoKonfiguration, Into (+1 more)

### Community 28 - "Wertung — Classic 3-arrow scoring"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Domain terms and business rules, Outcome, Problem, Requirements, Revision history, Risks and open questions (+4 more)

### Community 29 - ".contextualize_with"
Cohesion: 0.24
Nodes (11): ErrorReportExt, Result<T, E>, C, Error, Report, Result, Send, Sync (+3 more)

### Community 30 - "Konto — Benutzer, registration, and login"
Cohesion: 0.07
Nodes (28): Acceptance criteria, Account security, Administration, Approval, Auth audit, Decision log, Deferred capabilities, Domain terms and business rules (+20 more)

### Community 31 - "KontoFehler"
Cohesion: 0.12
Nodes (20): KontoFehler, AccessToken, basis64url(), debug_gibt_das_token_nicht_preis(), generate_secret_token(), hash_ist_stabil_und_vergleicht_konstantzeitig(), hash_token(), refresh_token_roundtrip() (+12 more)

### Community 32 - "App"
Cohesion: 0.21
Nodes (9): App, Arc, Box, Context, F, RepositoryError, ResultReport, O (+1 more)

### Community 33 - "RepoStorage"
Cohesion: 0.18
Nodes (11): RepoStorage, Box, Pin, Self, InstrumentedBenutzerRepository, InstrumentedEmailBestätigungRepository, InstrumentedKontoRepository, InstrumentedPasswortWiederherstellungRepository (+3 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.12
Nodes (45): base_app_builder(), AppBuilder, SetUowProvider, ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sitzungen_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac13_anonymer_zugriff_wird_verweigert() (+37 more)

### Community 35 - ".prüfe_und_zähle"
Cohesion: 0.27
Nodes (7): KeineDrossel, KeinVersand, main(), DateTime, Report, Result, Utc

### Community 36 - "base_app_builder"
Cohesion: 0.50
Nodes (3): base_app_builder(), AppBuilder, SetUowProvider

### Community 37 - "wal_connection_race.rs"
Cohesion: 0.83
Nodes (3): app(), parallel_execute_fn_survives_wal_connection_init(), Arc

### Community 39 - "Harness"
Cohesion: 0.16
Nodes (12): anonym(), build(), Harness, jetzt(), AppBuilder, Arc, DateTime, Duration (+4 more)

### Community 40 - ".get_current_version"
Cohesion: 0.19
Nodes (12): Box, Error, Future, Option, Output, Pin, Result, Send (+4 more)

### Community 46 - "SQLiteUnitOfWork"
Cohesion: 0.26
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.05
Nodes (54): RepositoryError, Option, RepositoryResult, Schuss, Send, Sync, Vec, SchussRepository (+46 more)

### Community 49 - "format_datetime"
Cohesion: 0.23
Nodes (12): format_datetime(), option_datetime(), Arc, DateTime, Mutex, Option, RepositoryResult, Row (+4 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "instrumented.rs"
Cohesion: 0.18
Nodes (11): InstrumentedDrossel, InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, DateTime, ObjectStoreError, ObjectStream (+3 more)

### Community 52 - "e2e/konto.rs"
Cohesion: 0.06
Nodes (42): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sitzungen_und_ist_einmalig(), ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei(), ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_außer_betrieb_gesetzter_oder_unbekannter_kid_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt(), ac2_vergebene_adresse_antwortet_gleich_und_mailt_nicht() (+34 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.22
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - "kontakt.rs"
Cohesion: 0.15
Nodes (15): email_accepts_valid_address(), EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe(), normalisiert_ist_idempotent() (+7 more)

### Community 55 - "jwt.rs"
Cohesion: 0.11
Nodes (35): access_token_lebensdauer_fließt_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), bearer_token(), claims(), decode(), encode(), encode_und_decode_roundtrip() (+27 more)

### Community 56 - "page.dev.tsx"
Cohesion: 0.09
Nodes (29): PLATES, ComponentSection(), Example(), touchTargetOutline, AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour() (+21 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.04
Nodes (47): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+39 more)

### Community 59 - "Session"
Cohesion: 0.06
Nodes (39): Option, erneuerung_erzeugt_eine_neue_session(), from_parts_roundtrip(), hash(), jetzt(), kid(), NeueSession, DateTime (+31 more)

### Community 61 - "Json"
Cohesion: 0.21
Nodes (15): C, From, Json, ObjectStream, PlainText, Report, Result, Schuss (+7 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "demo-page.tsx"
Cohesion: 0.14
Nodes (21): CHECKPOINTS, MESSREIHE, Row, START_ROWS, statusVariant, Zone, ZONEN, DogEaredSheet() (+13 more)

### Community 64 - "AuthContext"
Cohesion: 0.18
Nodes (8): LogandaimAppApi, Request, ResultReport, AuthContext, id(), Option, ResultReport, ZugriffFehler

### Community 65 - "LogandaimAppApi"
Cohesion: 0.23
Nodes (8): AusgestellteSitzung, LogandaimAppApi, Arc, KontoDienstFehler, Option, ResultReport, Self, Uuid

### Community 66 - "ExecutionContext"
Cohesion: 0.14
Nodes (21): ExecutionContext, Anmeldung, claims_fuer(), drossle(), KontoDienstFehler, Benutzer, DateTime, From (+13 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - ".schuss_registrieren"
Cohesion: 0.16
Nodes (13): Schuss, Wertung, domain::Wertungszone, From, Self, Uuid, schema_schuss_from_domain(), schema_wertung_from_domain() (+5 more)

### Community 71 - "FakeObjectStore"
Cohesion: 0.18
Nodes (10): FakeObjectStore, Arc, FxHashMap, Mutex, ObjectStoreError, ObjectStream, Option, ResultReport (+2 more)

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "InMemoryDrossel"
Cohesion: 0.24
Nodes (15): aktionen_und_clients_haben_getrennte_budgets(), das_budget_wird_genau_einmal_ausgeschöpft(), Fenster, InMemoryDrossel, jetzt(), kunde(), nach_dem_fenster_ist_das_budget_wieder_frei(), DateTime (+7 more)

### Community 74 - "sortable-table-section.tsx"
Cohesion: 0.09
Nodes (15): CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow(), SortableTableSection(), State, STATES (+7 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "UnitOfWorkImpl"
Cohesion: 0.19
Nodes (11): Box, RepositoryResult, Unimplemented, ExecutionSource, Send, Sync, UnitOfWorkImpl, UnitOfWorkProvider (+3 more)

### Community 80 - "toggle-group.tsx"
Cohesion: 0.31
Nodes (6): ComposedPageSection(), DemoPage(), ToggleGroup(), ToggleGroupItem(), toggleGroupItemVariants, @radix-ui/react-toggle-group

### Community 81 - "react"
Cohesion: 0.17
Nodes (16): zonen, Card(), CardContent(), CardDescription(), CardFooter(), CardHeader(), CardTitle(), Field() (+8 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "BenutzerAnsicht"
Cohesion: 0.21
Nodes (9): Beförderung, BenutzerAnsicht, BenutzerAnzeigen, BenutzerLöschen, Benutzer, Error, Option, Result (+1 more)

### Community 84 - "provider.tsx"
Cohesion: 0.06
Nodes (40): ApiError, useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), HttpLogandaimApi, JsonClient, unwrap(), createLogandaimApi() (+32 more)

### Community 86 - "RepositoryResult"
Cohesion: 0.09
Nodes (11): EmailBestätigungRepository, KontoRepository, PasswortWiederherstellungRepository, DateTime, RepositoryResult, Send, Sync, Utc (+3 more)

### Community 87 - "SigningKeypair"
Cohesion: 0.27
Nodes (6): Debug, Formatter, Result, Self, Signature, SigningKeypair

### Community 88 - "ObjectStore"
Cohesion: 0.23
Nodes (11): collect_object(), ObjectStore, ObjectStoreError, once_stream(), ObjectStream, Option, Result, ResultReport (+3 more)

### Community 89 - "EmailAdresse"
Cohesion: 0.16
Nodes (14): EmailAdresse, email(), from_parts_roundtrip(), hash(), jetzt(), Konto, KontoId, neues_konto_bestätigt_die_adresse() (+6 more)

### Community 90 - "PasswortWiederherstellung"
Cohesion: 0.11
Nodes (24): from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeueWiederherstellung, PasswortWiederherstellung, DateTime, Option, Result (+16 more)

### Community 91 - "Rolle"
Cohesion: 0.19
Nodes (11): Benutzer, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle, DateTime, Result (+3 more)

### Community 92 - "LogandaimApiSpec"
Cohesion: 0.25
Nodes (20): abmelden(), anmelden(), benutzer_anzeigen(), benutzer_löschen(), erneuerung(), nicht_angemeldet(), passwort_vergessen(), passwort_wiederherstellen() (+12 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "FakeDatastore"
Cohesion: 0.06
Nodes (43): Uuid, SessionId, FakeBenutzerRepository, FakeEmailBestätigungRepository, FakeKontoRepository, FakePasswortWiederherstellungRepository, FakeSessionRepository, FakeSigningKeyRepository (+35 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (45): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+37 more)

### Community 103 - "kunde.rs"
Cohesion: 0.44
Nodes (7): andere_ip_oder_anderer_client_ergeben_andere_schlüssel(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schlüssel(), ip(), kunde(), IpAddr, Option

### Community 104 - "SitzungAntwort"
Cohesion: 0.25
Nodes (15): BeendendeAntwort, BeendendeAntwort<T>, KontoAntwort, KontoAntwort<T>, From, Json, KontoDienstFehler, PlainText (+7 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 106 - "BenutzerId"
Cohesion: 0.12
Nodes (13): BenutzerRepository, Benutzer, BenutzerId, Uuid, benutzer_from_row(), Arc, Benutzer, Mutex (+5 more)

### Community 107 - "SQLiteEmailBestätigungRepository"
Cohesion: 0.22
Nodes (10): bestätigung_from_row(), Arc, DateTime, Mutex, Option, RepositoryResult, Row, Transaction (+2 more)

### Community 108 - "SQLiteKontoRepository"
Cohesion: 0.23
Nodes (9): konto_from_row(), Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec (+1 more)

### Community 109 - "Request"
Cohesion: 0.17
Nodes (7): AnmeldeAnfrage, BenutzerLöschungAnfrage, Kundenkennung, PasswortVergessenAnfrage, RegistrierungAbschlussAnfrage, RegistrierungStartAnfrage, Request

### Community 110 - "KundeSchlüssel"
Cohesion: 0.11
Nodes (15): Drossel, DrosselAktion, DrosselFehler, KundeSchlüssel, DateTime, Duration, Into, IpAddr (+7 more)

### Community 111 - "UseCase"
Cohesion: 0.19
Nodes (11): Context, Output, ResultReport, UseCase, Abmeldung, Erneuerung, ErneuerungAusgestellt, Erneuerungsausgang (+3 more)

### Community 112 - "MailSender"
Cohesion: 0.14
Nodes (15): AsyncSmtpTransport, MailFehler, MailSender, ResultReport, Send, Sync, KeinVersand, Option (+7 more)

### Community 113 - "String"
Cohesion: 0.11
Nodes (13): PasswortWiederherstellenAnfrage, From, Self, String, basis_url(), letzter_text_an(), nachrichten(), Value (+5 more)

### Community 114 - "navigation.ts"
Cohesion: 0.08
Nodes (30): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, AppChrome(), AppShell() (+22 more)

### Community 115 - "UnitOfWork"
Cohesion: 0.10
Nodes (3): UnitOfWork, UnitOfWork<'a>, Drop

### Community 116 - "Mail"
Cohesion: 0.26
Nodes (5): Mail, FakeMailSender, Mutex, ResultReport, Vec

### Community 117 - "LockedUnitOfWorkImpl"
Cohesion: 0.30
Nodes (4): Arc<T>, LockedUnitOfWorkImpl, Box, RepositoryResult

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

### Community 126 - "cookie.rs"
Cohesion: 0.50
Nodes (7): das_löschen_trifft_dasselbe_cookie(), das_token_steht_nur_im_cookie_und_hat_256_bit(), die_attribute_sind_vollständig(), löschen(), setzen(), setzen_raw(), token()

### Community 127 - "signing.rs"
Cohesion: 0.26
Nodes (8): debug_gibt_den_privatschlüssel_nicht_preis(), falscher_schlüssel_wird_abgelehnt(), gekipptes_byte_wird_abgelehnt(), jedes_byte_ist_signierbar(), kaputtes_pem_wird_abgelehnt(), keypair(), public_key_verifikation_über_die_schlüsselzeile(), signieren_und_verifizieren_roundtrip()

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "FixedClock"
Cohesion: 0.24
Nodes (7): FixedClock, DateTime, Duration, Mutex, NaiveDate, Self, Utc

### Community 131 - "Preis"
Cohesion: 0.29
Nodes (5): Preis, Add, Output, Self, Mul

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "tests/mail.rs"
Cohesion: 0.60
Nodes (5): absender(), die_nachricht_trägt_empfänger_betreff_und_klartext_rumpf(), eine_ungültige_adresse_ist_ein_zustellungsfehler(), entfalte_quoted_printable(), sender()

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
Cohesion: 0.22
Nodes (8): openapi_service(), Into, IntoIterator, Item, Self, LogandaimAppApi, OpenApiService, ServerObject

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

### Community 142 - "ports/mail.rs"
Cohesion: 0.27
Nodes (6): adresse(), der_link_wird_aus_der_basis_url_und_der_route_gebaut(), ein_schrägstrich_am_ende_erzeugt_keinen_doppelten(), MailArt, Into, Self

### Community 144 - "check-identifiers.mjs"
Cohesion: 0.20
Nodes (9): ALLOWED, EXTENSIONS, files, findings, moduleNames, ROOTS, SELF, SKIP (+1 more)

### Community 145 - ".finish"
Cohesion: 0.29
Nodes (7): C, F, Formatter, Future, Output, Report, Result

### Community 146 - "StreamBody"
Cohesion: 0.29
Nodes (5): StreamBody, IntoResponse, MetaSchemaRef, Payload, Response

### Community 147 - ".rendern"
Cohesion: 0.38
Nodes (4): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec

### Community 150 - "Menge"
Cohesion: 0.22
Nodes (6): Menge, MengeFehler, Decimal, Result, Self, menge_to_str()

### Community 151 - "Versioned<T>"
Cohesion: 0.11
Nodes (13): Clone, Debug, Deref, Option, Self, T, Target, Versioned (+5 more)

### Community 153 - "ADR-0002: Session cookie auth across frontend and API origins"
Cohesion: 0.20
Nodes (10): ADR-0002: Session cookie auth across frontend and API origins, Alternatives considered, API surface (sketch), Appendix: implementation sketch (non-normative), Authorisation (sketch), Consequences, Context, Data model (sketch) (+2 more)

### Community 155 - "m1-overseer.md"
Cohesion: 0.29
Nodes (6): Corrective measures, Final report (normal English, not caveman), Fleet, Goal, Hard rules, Loop

### Community 156 - "Modes"
Cohesion: 0.33
Nodes (5): address-review, fix-ci, implement, merge-base, Modes

### Community 157 - "package.json"
Cohesion: 0.07
Nodes (27): name, private, version, Progress(), Slider(), Switch(), babel-plugin-react-compiler, @biomejs/biome (+19 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

## Knowledge Gaps
- **477 isolated node(s):** `BeendendeAntwort<T>`, `ValidationError`, `SchussRegistrierenFehler`, `WertungFehler`, `logandaim-fakes` (+472 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 874 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `StructuredError`, `tests/mail.rs`, `in_memory_object_store.rs`, `ports/mail.rs`, `domain/konto/passwort.rs`, `common.rs`, `Menge`, `ContextPorts`, `KontoFehler`, `cases/konto.rs`, `Harness`, `format_datetime`, `e2e/konto.rs`, `kontakt.rs`, `jwt.rs`, `Session`, `LogandaimAppApi`, `ExecutionContext`, `FakeObjectStore`, `BenutzerAnsicht`, `EmailAdresse`, `LogandaimApiSpec`, `FakeDatastore`, `config.rs`, `SitzungAntwort`, `Request`, `KundeSchlüssel`, `UseCase`, `MailSender`, `Mail`, `adresse.rs`, `BenutzerAntwort`, `cookie.rs`, `signing.rs`?**
  _High betweenness centrality (0.217) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `AuthContext`, `ExecutionContext`, `cases/konto.rs`, `FakeDatastore`, `Session`, `SQLiteKontoRepository`, `format_datetime`, `BenutzerAnsicht`, `RepositoryResult`, `jwt.rs`, `EmailAdresse`, `Rolle`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Why does `App` connect `App` to `PdfRenderer`, `LogandaimAppApi`, `config.rs`, `Harness`, `openapi_service`, `UnitOfWorkImpl`, `KundeSchlüssel`, `MailSender`, `e2e/konto.rs`, `Clock`, `SigningKeypair`, `ObjectStore`, `ContextPorts`, `LogandaimApiSpec`?**
  _High betweenness centrality (0.044) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `BeendendeAntwort<T>`, `ValidationError`, `SchussRegistrierenFehler` to the rest of the system?**
  _477 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.12648221343873517 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._