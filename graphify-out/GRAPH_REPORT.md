# Graph Report - logandaim.at  (2026-10-26)

## Corpus Check
- 246 files · ~120,863 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2905 nodes · 6412 edges · 149 communities (127 shown, 12 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 108 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `207d1fe1`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- FakePdfRenderer
- arc_up
- Specifications
- biome.json
- yams-persistence
- Seminar — Domain-Spezifikation
- cn
- SchussErstellung
- StatusCode
- demo-page.tsx
- logandaim-filesystemstore/src/lib.rs
- compilerOptions
- button.tsx
- ObjectStore
- FakeUnitOfWork
- Zeitraum
- E
- Datenschutz — Data protection, privacy, and legal obligations
- KontoFehler
- Clock
- wertung.rs
- EmailBestaetigung
- <Title>
- Make invalid state unrepresentable
- Betrieb — Operations and launch readiness
- check-release.mjs
- AuthContext
- Wertung — Classic 3-arrow scoring
- .contextualize_with
- Konto — Benutzer, registration, and login
- token.rs
- App
- RepoStorage
- cases/konto.rs
- .pruefe_und_zaehle
- base_app_builder
- wal_connection_race.rs
- validation_error.rs
- Harness
- .get_current_version
- SQLiteUnitOfWork
- Vertical slices
- domain/schuss.rs
- SessionId
- components.json
- Drossel
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
- SigningKey
- BenutzerId
- KundeSchluessel
- ExecutionContext
- release-version.mjs
- Product Requirements Document (PRD) & Konzeptbericht
- logandaim-core
- react
- SchussRepository
- check-version.mjs
- InMemoryDrossel
- sortable-table-section.tsx
- diffx-finish-review
- ADR-0003: Design system and dev-only gallery
- UnitOfWorkImpl
- Next.js
- postcss.config.mjs
- toggle-group.tsx
- card.tsx
- Contributing and project management
- BenutzerAnsicht
- provider.tsx
- RepositoryResult
- signing.rs
- RepositoryResult
- EmailAdresse
- KontoId
- Rolle
- routes/konto.rs
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
- BenutzerRepository
- devDependencies
- SQLiteKontoRepository
- LogandaimApiSpec
- DrosselAktion
- UseCase
- MailSender
- String
- navigation.ts
- EmailBestaetigungRepository
- Mail
- UnitOfWork
- pull_request_template.md
- preis.rs
- Ratio
- adresse.rs
- Appendix: implementation sketch (non-normative)
- Stammdaten — Master data and `Anlage` administration
- YAMS Frontend Migration Specification
- BenutzerAntwort
- RefreshToken
- KontoKonfiguration
- Anlage-Anfrage — Requesting a new Anlage
- scripts
- Preis
- StructuredError
- tests/mail.rs
- .get_current_version
- Option
- migration_error_to_persistence_error
- ADR-0001: Baseline architecture
- dates.ts
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
3. `EmailAdresse` - 41 edges
4. `Session` - 40 edges
5. `react` - 39 edges
6. `E` - 37 edges
7. `TokenHash` - 37 edges
8. `KundeSchluessel` - 36 edges
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

## Communities (149 total, 12 thin omitted)

### Community 0 - "FakePdfRenderer"
Cohesion: 0.11
Nodes (15): blank_renderer_returns_pdf_prefix(), BlankPdfRenderer, ResultReport, Vec, Vec, PdfDokument, PdfRenderError, ResultReport (+7 more)

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

### Community 8 - "StatusCode"
Cohesion: 0.12
Nodes (17): HttpStatusMapping, KontoDienstFehler, mapped(), nested_http_status(), ObjectStoreError, RepositoryError, C, Option (+9 more)

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
Cohesion: 0.14
Nodes (14): SAMPLE_ROWS, TypeSection(), Alert(), AlertProps, alertVariants, iconColour, Badge(), BadgeProps (+6 more)

### Community 14 - "ObjectStore"
Cohesion: 0.07
Nodes (36): delete_missing_is_already_deleted(), delete_removes_object(), ensure_deleted_swallows_already_deleted(), get_missing_is_none(), InMemoryObjectStore, put_overwrites(), put_then_get_roundtrip(), FxHashMap (+28 more)

### Community 15 - "FakeUnitOfWork"
Cohesion: 0.26
Nodes (10): FakeUnitOfWork, FakeUnitOfWorkProvider, Arc, Box, Default, Mutex, RepositoryResult, Self (+2 more)

### Community 16 - "Zeitraum"
Cohesion: 0.21
Nodes (11): DateTime, Display, Formatter, Result, Self, Utc, utc(), Zeitraum (+3 more)

### Community 17 - "E"
Cohesion: 0.20
Nodes (18): E, apply_down_migrations(), apply_up_migrations(), ApplyMigrationDown, ApplyMigrationDown<T, E>, ApplyMigrationUp, ApplyMigrationUp<T, E>, Arc<dyn DownMigration<T, E>> (+10 more)

### Community 18 - "Datenschutz — Data protection, privacy, and legal obligations"
Cohesion: 0.11
Nodes (19): Analytics, Benchmarks, Consent, Data export, Datenschutz — Data protection, privacy, and legal obligations, Decision log, Domain terms and business rules, Incident response (+11 more)

### Community 19 - "KontoFehler"
Cohesion: 0.14
Nodes (20): KontoFehler, argon2(), debug_gibt_das_passwort_nicht_preis(), dummy_verify(), dummy_verify_akzeptiert_jedes_gueltige_passwort(), falsches_passwort_wird_abgelehnt(), gleiches_passwort_ergibt_verschiedene_hashes(), hash_password() (+12 more)

### Community 20 - "Clock"
Cohesion: 0.09
Nodes (17): DateTime, NaiveDate, Utc, SystemClock, Clock, DateTime, NaiveDate, Send (+9 more)

### Community 21 - "wertung.rs"
Cohesion: 0.11
Nodes (21): ac2_drei_fehlschüsse_null(), ac5_ab_viertem_pfeil_null(), ac6_leer_ohne_ergebnis_sonst_beliebig_lang(), ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis(), aus_schüssen_sortiert_nach_pfeil(), pfeil(), Punkte, Add (+13 more)

### Community 22 - "EmailBestaetigung"
Cohesion: 0.06
Nodes (39): bestaetigung(), BestaetigungId, EmailBestaetigung, from_parts_roundtrip(), jetzt(), neue_bestaetigung_gilt_24_stunden(), NeueBestaetigung, DateTime (+31 more)

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

### Community 27 - "AuthContext"
Cohesion: 0.10
Nodes (11): AuthContext, Option, ResultReport, ContextPorts, ExecutionContext<'a>, Arc, RepositoryResult, Self (+3 more)

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
Cohesion: 0.10
Nodes (20): AccessToken, basis64url(), debug_gibt_das_token_nicht_preis(), generate_secret_token(), hash_ist_stabil_und_vergleicht_konstantzeitig(), hash_token(), refresh_token_roundtrip(), Debug (+12 more)

### Community 32 - "App"
Cohesion: 0.19
Nodes (10): App, Arc, Box, Context, F, RepositoryError, ResultReport, SigningKeypair (+2 more)

### Community 33 - "RepoStorage"
Cohesion: 0.18
Nodes (11): RepoStorage, Box, Pin, Self, InstrumentedBenutzerRepository, InstrumentedEmailBestaetigungRepository, InstrumentedKontoRepository, InstrumentedPasswortWiederherstellungRepository (+3 more)

### Community 34 - "cases/konto.rs"
Cohesion: 0.11
Nodes (47): base_app_builder(), AppBuilder, SetUowProvider, ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sitzungen_und_ist_einmalig(), ac12_loeschung_entfernt_identitaet_und_gibt_die_adresse_frei(), ac12_loeschung_ohne_bestaetigung_oder_mit_falschem_passwort_scheitert(), ac13_anonymer_zugriff_wird_verweigert() (+39 more)

### Community 35 - ".pruefe_und_zaehle"
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
Cohesion: 0.23
Nodes (8): Arc, Box, Mutex, Option, RepositoryResult, Self, Transaction, SQLiteUnitOfWork

### Community 47 - "Vertical slices"
Cohesion: 0.25
Nodes (7): File map, leaf to root, Frontend API seam, Frontend state rules, Order to work in, Vertical slices, Where things belong, Why `RepoStorage` has that pointer wiring

### Community 48 - "domain/schuss.rs"
Cohesion: 0.06
Nodes (55): domain::Wertungszone, From, Self, Uuid, schema_schuss_from_domain(), schema_wertung_from_domain(), Schuss, Wertung (+47 more)

### Community 49 - "SessionId"
Cohesion: 0.18
Nodes (14): id(), SessionId, format_datetime(), option_datetime(), Arc, DateTime, Mutex, Option (+6 more)

### Community 50 - "components.json"
Cohesion: 0.11
Nodes (17): aliases, components, hooks, lib, ui, utils, iconLibrary, rsc (+9 more)

### Community 51 - "Drossel"
Cohesion: 0.16
Nodes (12): InstrumentedDrossel, InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer, Arc, ObjectStoreError, ObjectStream, Option (+4 more)

### Community 52 - "e2e/konto.rs"
Cohesion: 0.06
Nodes (42): ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich(), ac11_neues_passwort_beendet_alle_sitzungen_und_ist_einmalig(), ac12_loeschung_entfernt_identitaet_und_gibt_die_adresse_frei(), ac12_loeschung_ohne_bestaetigung_oder_mit_falschem_passwort_scheitert(), ac14_fremden_benutzer_liest_nur_ein_systemadministrator(), ac17_ausser_betrieb_gesetzter_oder_unbekannter_kid_wird_abgelehnt(), ac18_abgelaufenes_access_token_wird_abgelehnt(), ac2_vergebene_adresse_antwortet_gleich_und_mailt_nicht() (+34 more)

### Community 53 - "InstrumentedUnitOfWork"
Cohesion: 0.22
Nodes (5): InstrumentedUnitOfWork, Box, Pin, RepositoryResult, Self

### Community 54 - "kontakt.rs"
Cohesion: 0.15
Nodes (15): email_accepts_valid_address(), EmailAdresseValidierungsfehler, Mobilnummer, mobilnummer_accepts_digits(), mobilnummer_accepts_plus_prefix(), MobilnummerValidierungsfehler, normalisiert_faltet_weder_punkte_noch_plus_suffixe(), normalisiert_ist_idempotent() (+7 more)

### Community 55 - "jwt.rs"
Cohesion: 0.09
Nodes (39): LogandaimAppApi, Request, ResultReport, access_token_lebensdauer_fliesst_in_exp(), AccessTokenClaimsDto, alg_none_wird_abgelehnt(), bearer_token(), claims() (+31 more)

### Community 56 - "token-swatches.tsx"
Cohesion: 0.23
Nodes (11): AA_NORMAL_TEXT, contrastRatio(), NAMED_COLOURS, parseColour(), relativeLuminance(), COLOUR_TOKENS, CONTRAST_PAIRS, Measurement (+3 more)

### Community 57 - "logandaim.at — Product Charter"
Cohesion: 0.13
Nodes (15): Accessibility, Decision log, Domain terms, Import, Language and copy, logandaim.at — Product Charter, Milestones, MVP scope (+7 more)

### Community 58 - "dependencies"
Cohesion: 0.07
Nodes (28): dependencies, class-variance-authority, clsx, @dnd-kit/core, @dnd-kit/sortable, @dnd-kit/utilities, lucide-react, next (+20 more)

### Community 59 - "Session"
Cohesion: 0.10
Nodes (19): Option, erneuerung_erzeugt_eine_neue_session(), from_parts_roundtrip(), hash(), jetzt(), kid(), NeueSession, DateTime (+11 more)

### Community 61 - "openapi_service"
Cohesion: 0.10
Nodes (25): openapi_service(), C, From, Into, IntoIterator, Item, ObjectStream, PlainText (+17 more)

### Community 62 - "Visual identity and design system"
Cohesion: 0.15
Nodes (12): Acceptance criteria, Decision log, Open questions, Outcome, Problem, Requirements, Revision history, Roles (+4 more)

### Community 63 - "SigningKey"
Cohesion: 0.12
Nodes (21): ausser_betrieb_nehmen_ist_idempotent(), from_parts_roundtrip(), jetzt(), neue_schluessel_sind_aktiv(), DateTime, Into, Option, Result (+13 more)

### Community 64 - "BenutzerId"
Cohesion: 0.19
Nodes (9): BenutzerId, FakeBenutzerRepository, FakeKontoRepository, KontoDatastore, Benutzer, Clone, Default, FxHashMap (+1 more)

### Community 65 - "KundeSchluessel"
Cohesion: 0.13
Nodes (16): AusgestellteSitzung, LogandaimAppApi, Arc, KontoDienstFehler, Option, ResultReport, Schuss, Self (+8 more)

### Community 66 - "ExecutionContext"
Cohesion: 0.11
Nodes (23): ExecutionContext, Anmeldung, Error, Result, claims_fuer(), drossle(), KontoDienstFehler, Benutzer (+15 more)

### Community 67 - "release-version.mjs"
Cohesion: 0.25
Nodes (5): current, [major, minor, patch], manifest, packageJson, today

### Community 68 - "Product Requirements Document (PRD) & Konzeptbericht"
Cohesion: 0.12
Nodes (15): 1. Executive Summary & Vision, 2. Stakeholder & Rollenmodell, 3.1 Domain-Modell & Begrifflichkeiten (Domain-Driven Design), 3.2 Wertungssystem & Pfeillogik, 3.3 Distanzerfassung mittels stufenlosem Farbslider, 3.4 Progressions- und Benchmark-Berechnungen, 3. Kernanforderungen (Funktionale Spezifikation), 3D-Bogensport Accuracy & Progress Tracking WebApp (Österreich) (+7 more)

### Community 69 - "logandaim-core"
Cohesion: 0.39
Nodes (8): logandaim-api, logandaim-core, logandaim-fakes, logandaim-filesystemstore, logandaim-server, logandaim-smtp, logandaim-sqlite, molting

### Community 70 - "react"
Cohesion: 0.11
Nodes (20): InkedControlsSection(), Checkbox(), CheckboxProps, Field(), FieldProps, LabelledProps, Label(), RadioGroup() (+12 more)

### Community 71 - "SchussRepository"
Cohesion: 0.19
Nodes (6): RepositoryResult, Schuss, Send, Sync, Vec, SchussRepository

### Community 72 - "check-version.mjs"
Cohesion: 0.40
Nodes (4): cargoVersion, changelog, read(), workspaceVersion()

### Community 73 - "InMemoryDrossel"
Cohesion: 0.24
Nodes (15): aktionen_und_clients_haben_getrennte_budgets(), das_budget_wird_genau_einmal_ausgeschoepft(), Fenster, InMemoryDrossel, jetzt(), kunde(), nach_dem_fenster_ist_das_budget_wieder_frei(), DateTime (+7 more)

### Community 74 - "sortable-table-section.tsx"
Cohesion: 0.07
Nodes (27): zonen, CHECKPOINTS, CheckpointSliderSection(), INITIAL_ROWS, Row, SortableRow(), SortableTableSection(), State (+19 more)

### Community 75 - "diffx-finish-review"
Cohesion: 1.00
Nodes (3): diffx-finish-review, diffx server, diffx-start-review

### Community 76 - "ADR-0003: Design system and dev-only gallery"
Cohesion: 0.40
Nodes (4): ADR-0003: Design system and dev-only gallery, Consequences, Context, Decision

### Community 77 - "UnitOfWorkImpl"
Cohesion: 0.15
Nodes (13): Box, RepositoryResult, Unimplemented, ExecutionSource, Arc<T>, Send, Sync, UnitOfWork<'a> (+5 more)

### Community 80 - "toggle-group.tsx"
Cohesion: 0.31
Nodes (6): ComposedPageSection(), DemoPage(), ToggleGroup(), ToggleGroupItem(), toggleGroupItemVariants, @radix-ui/react-toggle-group

### Community 81 - "card.tsx"
Cohesion: 0.19
Nodes (14): useSchussRegistrierenMutation(), useWertungMutation(), useHealthQuery(), useLogandaimApi(), logandaimKeys, HomePage(), SchussPage(), Card() (+6 more)

### Community 82 - "Contributing and project management"
Cohesion: 0.12
Nodes (16): Agents, Before the first release, Branches, CHANGELOG scope, Checks, Code comments, Commits, Contributing and project management (+8 more)

### Community 83 - "BenutzerAnsicht"
Cohesion: 0.21
Nodes (9): Befoerderung, BenutzerAnsicht, BenutzerAnzeigen, BenutzerLoeschen, Benutzer, Error, Option, Result (+1 more)

### Community 84 - "provider.tsx"
Cohesion: 0.08
Nodes (27): ApiError, HttpLogandaimApi, JsonClient, unwrap(), createLogandaimApi(), LogandaimApi, Schuss, SchussErstellungInput (+19 more)

### Community 86 - "RepositoryResult"
Cohesion: 0.12
Nodes (6): KontoRepository, DateTime, RepositoryResult, Utc, Vec, SessionRepository

### Community 87 - "signing.rs"
Cohesion: 0.15
Nodes (13): debug_gibt_den_privatschluessel_nicht_preis(), falscher_schluessel_wird_abgelehnt(), gekipptes_byte_wird_abgelehnt(), jedes_byte_ist_signierbar(), kaputtes_pem_wird_abgelehnt(), keypair(), public_key_verifikation_ueber_die_schluesselzeile(), Debug (+5 more)

### Community 88 - "RepositoryResult"
Cohesion: 0.18
Nodes (7): FakeEmailBestaetigungRepository, FakePasswortWiederherstellungRepository, FakeSessionRepository, DateTime, Option, RepositoryResult, Utc

### Community 89 - "EmailAdresse"
Cohesion: 0.16
Nodes (13): EmailAdresse, email(), from_parts_roundtrip(), hash(), jetzt(), Konto, neues_konto_bestaetigt_die_adresse(), NeuesKonto (+5 more)

### Community 90 - "KontoId"
Cohesion: 0.11
Nodes (25): KontoId, from_parts_roundtrip(), gilt_60_minuten(), jetzt(), NeueWiederherstellung, PasswortWiederherstellung, DateTime, Option (+17 more)

### Community 91 - "Rolle"
Cohesion: 0.10
Nodes (21): Benutzer, from_parts_roundtrip(), jetzt(), NeuerBenutzer, registrierung_ist_immer_ein_benutzer(), Rolle, DateTime, Result (+13 more)

### Community 92 - "routes/konto.rs"
Cohesion: 0.24
Nodes (18): abmelden(), anmelden(), benutzer_anzeigen(), benutzer_loeschen(), erneuerung(), nicht_angemeldet(), passwort_vergessen(), passwort_wiederherstellen() (+10 more)

### Community 93 - "check-design.mjs"
Cohesion: 0.20
Nodes (8): CSS, failures, ROOT, RULE, SCANNED, SKIP_DIRS, SRC, tokens

### Community 99 - "FakeDatastore"
Cohesion: 0.13
Nodes (16): FakeSigningKeyRepository, Arc, Self, FakeDatastore, FakeSchussRepository, KontoRepositories, Arc, Clone (+8 more)

### Community 100 - "SQLiteInstance"
Cohesion: 0.17
Nodes (17): AtomicBool, Connection, InstanceType, Arc, AsRef, Deref, Mutex, Path (+9 more)

### Community 101 - "UpMigration"
Cohesion: 0.12
Nodes (12): Migration, Error, Option, Result, Transaction, Migration, Error, Option (+4 more)

### Community 102 - "config.rs"
Cohesion: 0.09
Nodes (46): Cli, cli_overlay(), ConfigError, env_or_cli_overlay_beats_file(), file_values_used_when_no_overlay(), load(), load_file(), loads_json_file() (+38 more)

### Community 103 - "kunde.rs"
Cohesion: 0.54
Nodes (7): andere_ip_oder_anderer_client_ergeben_andere_schluessel(), fehlende_header_sind_kein_fehler(), gleiche_kennung_ergibt_gleichen_schluessel(), ip(), kunde(), IpAddr, Option

### Community 104 - "SitzungAntwort"
Cohesion: 0.22
Nodes (16): BeendendeAntwort, BeendendeAntwort<T>, KontoAntwort, KontoAntwort<T>, From, Json, KontoDienstFehler, PlainText (+8 more)

### Community 105 - ".add_dyn"
Cohesion: 0.23
Nodes (12): MigrationRegistry, MigrationRegistry<dyn DownMigration<T, E>>, MigrationRegistry<dyn UpMigration<T, E>>, MigrationRegistry<M>, Arc, Default, From, I (+4 more)

### Community 107 - "devDependencies"
Cohesion: 0.20
Nodes (10): devDependencies, babel-plugin-react-compiler, @biomejs/biome, openapi-typescript, tailwindcss, @tailwindcss/postcss, @types/node, @types/react (+2 more)

### Community 108 - "SQLiteKontoRepository"
Cohesion: 0.23
Nodes (9): konto_from_row(), Arc, Mutex, Option, RepositoryResult, Row, Transaction, Vec (+1 more)

### Community 109 - "LogandaimApiSpec"
Cohesion: 0.19
Nodes (9): OkAntwort, LogandaimApiSpec, Json, Option, Request, Schuss, Uuid, Wertung (+1 more)

### Community 110 - "DrosselAktion"
Cohesion: 0.17
Nodes (8): DateTime, Utc, DrosselAktion, DrosselFehler, DateTime, Duration, ResultReport, Utc

### Community 111 - "UseCase"
Cohesion: 0.24
Nodes (9): Context, Output, ResultReport, UseCase, Abmeldung, Erneuerung, ErneuerungAusgestellt, Erneuerungsausgang (+1 more)

### Community 112 - "MailSender"
Cohesion: 0.14
Nodes (15): AsyncSmtpTransport, MailFehler, MailSender, ResultReport, Send, Sync, KeinVersand, Option (+7 more)

### Community 113 - "String"
Cohesion: 0.12
Nodes (16): AnmeldeAnfrage, BenutzerLoeschungAnfrage, Kundenkennung, PasswortVergessenAnfrage, PasswortWiederherstellenAnfrage, RegistrierungAbschlussAnfrage, RegistrierungStartAnfrage, From (+8 more)

### Community 114 - "navigation.ts"
Cohesion: 0.07
Nodes (33): DEV_PAGE_EXTENSIONS, PROD_PAGE_EXTENSIONS, geistMono, inter, lora, metadata, Providers(), AppChrome() (+25 more)

### Community 115 - "EmailBestaetigungRepository"
Cohesion: 0.09
Nodes (7): EmailBestaetigungRepository, PasswortWiederherstellungRepository, Send, Sync, SigningKeyRepository, RepositoryError, Option

### Community 116 - "Mail"
Cohesion: 0.23
Nodes (7): Mail, Into, Self, FakeMailSender, Mutex, ResultReport, Vec

### Community 117 - "UnitOfWork"
Cohesion: 0.16
Nodes (12): LockedUnitOfWorkImpl, Box, C, F, Formatter, Future, Output, Report (+4 more)

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
Cohesion: 0.33
Nodes (8): AnmeldeAntwort, BenutzerAntwort, Benutzer, From, Self, Uuid, SchemaBenutzer, SchemaRolle

### Community 126 - "RefreshToken"
Cohesion: 0.38
Nodes (8): das_loeschen_trifft_dasselbe_cookie(), das_token_steht_nur_im_cookie_und_hat_256_bit(), die_attribute_sind_vollstaendig(), loeschen(), setzen(), setzen_raw(), token(), RefreshToken

### Community 127 - "KontoKonfiguration"
Cohesion: 0.27
Nodes (5): konfiguration(), KontoKonfiguration, links_tragen_das_token_als_query_parameter(), Into, Self

### Community 128 - "Anlage-Anfrage — Requesting a new Anlage"
Cohesion: 0.18
Nodes (11): Acceptance criteria, Anlage-Anfrage — Requesting a new Anlage, Decision log, Domain terms and business rules, Open questions, Outcome, Problem, Revision history (+3 more)

### Community 129 - "scripts"
Cohesion: 0.22
Nodes (9): scripts, build, check:format, check:lint, check:ts, dev, fix:format, fix:lint (+1 more)

### Community 131 - "Preis"
Cohesion: 0.29
Nodes (5): Preis, Add, Output, Self, Mul

### Community 132 - "StructuredError"
Cohesion: 0.33
Nodes (7): into_structured_error_from_frame(), From, Report, Self, Vec, StructuredError, Frame

### Community 133 - "tests/mail.rs"
Cohesion: 0.60
Nodes (5): absender(), die_nachricht_traegt_empfaenger_betreff_und_klartext_rumpf(), eine_ungueltige_adresse_ist_ein_zustellungsfehler(), entfalte_quoted_printable(), sender()

### Community 134 - ".get_current_version"
Cohesion: 0.40
Nodes (4): Future, Output, Pin, Send

### Community 136 - "migration_error_to_persistence_error"
Cohesion: 0.80
Nodes (4): libsql_error_to_persistence_error(), migration_error_to_persistence_error(), Error, RepositoryError

### Community 137 - "ADR-0001: Baseline architecture"
Cohesion: 0.50
Nodes (4): ADR-0001: Baseline architecture, Consequences, Context, Decision

### Community 141 - "dates.ts"
Cohesion: 0.23
Nodes (5): datetimeLocalForDate(), defaultDatetimeLocal(), isoDateFromInstant(), todayIsoDate(), toIsoDate()

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
Cohesion: 0.08
Nodes (22): name, private, version, Progress(), babel-plugin-react-compiler, @biomejs/biome, openapi-fetch, openapi-typescript (+14 more)

### Community 160 - "m1-reviewer.md"
Cohesion: 0.40
Nodes (4): Do not touch the working copy, Mode re-review, Mode review, Reply (caveman, ≤3 lines)

## Knowledge Gaps
- **469 isolated node(s):** `BeendendeAntwort<T>`, `ValidationError`, `SchussRegistrierenFehler`, `WertungFehler`, `logandaim-fakes` (+464 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 864 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **12 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `String` connect `String` to `arc_up`, `StructuredError`, `tests/mail.rs`, `ObjectStore`, `FakeUnitOfWork`, `KontoFehler`, `EmailBestaetigung`, `Menge`, `token.rs`, `cases/konto.rs`, `Harness`, `SessionId`, `e2e/konto.rs`, `kontakt.rs`, `jwt.rs`, `Session`, `SigningKey`, `KundeSchluessel`, `ExecutionContext`, `BenutzerAnsicht`, `signing.rs`, `EmailAdresse`, `routes/konto.rs`, `config.rs`, `SitzungAntwort`, `LogandaimApiSpec`, `UseCase`, `MailSender`, `Mail`, `adresse.rs`, `BenutzerAntwort`, `RefreshToken`, `KontoKonfiguration`?**
  _High betweenness centrality (0.221) - this node is a cross-community bridge._
- **Why does `BenutzerId` connect `BenutzerId` to `ExecutionContext`, `cases/konto.rs`, `Session`, `BenutzerRepository`, `Rolle`, `SQLiteKontoRepository`, `SessionId`, `BenutzerAnsicht`, `RepositoryResult`, `jwt.rs`, `RepositoryResult`, `EmailAdresse`, `AuthContext`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Why does `EmailAdresse` connect `EmailAdresse` to `BenutzerId`, `tests/mail.rs`, `SQLiteKontoRepository`, `MailSender`, `String`, `EmailBestaetigungRepository`, `Mail`, `BenutzerAnsicht`, `kontakt.rs`, `RepositoryResult`, `EmailBestaetigung`, `RepositoryResult`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `BenutzerId` (e.g. with `claims()` and `decode()`) actually correct?**
  _`BenutzerId` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `BeendendeAntwort<T>`, `ValidationError`, `SchussRegistrierenFehler` to the rest of the system?**
  _469 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `FakePdfRenderer` be split into smaller, more focused modules?**
  _Cohesion score 0.11076923076923077 - nodes in this community are weakly interconnected._
- **Should `arc_up` be split into smaller, more focused modules?**
  _Cohesion score 0.05827505827505827 - nodes in this community are weakly interconnected._