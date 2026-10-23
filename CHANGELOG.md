# Changelog

All notable user-visible changes to logandaim.at are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Draft spec [`specs/product/stammdaten.md`](specs/product/stammdaten.md) for master data
  (`Anlage`, `Parcours`, `Ziel`, `Pflock`) with admin-UI seeding and the version-snapshot rules:
  every `Runde` freezes the `Parcours` layout at log time while shot history follows `Ziel`
  identity across `Runden` and layout changes.
- Draft spec [`specs/product/konto.md`](specs/product/konto.md) for accounts and login
  (registration, login, password reset, account deletion, admin role) and proposed
  [ADR-0002](specs/decisions/0002-session-cookie-auth.md) for session-cookie auth with
  explicit-origin CORS.
- Draft spec [`specs/product/datenschutz.md`](specs/product/datenschutz.md) covering GDPR duties:
  export, retention, consent, legal pages, rights process, processors, analytics, incident
  response, and future benchmarks.
- Draft spec [`specs/product/betrieb.md`](specs/product/betrieb.md) covering security hardening,
  backups and restore drills, abuse handling, and support.
- `konto.md`: Deferred capabilities part (email delivery, verification, email change, account
  security, MFA, external identity providers, passkeys, administration, auth audit, notifications),
  each linked to its delivery issue.
- `charter.md`: Draft "Quality bars" (accessibility, language and copy) and "Import" sections.

### Changed

- SPEC-konto draft: `Konto-Löschung` now anonymizes `Runden`/`Schüsse` instead of deleting them
  (aggregate statistics survive; PO decision Q-1), auth throttle thresholds are fixed exactly
  (PO decision Q-4), and registration answers generically so it cannot be used as an
  account-enumeration oracle.
- Remove the Tauri desktop shell; the app now runs as a web frontend with an HTTP backend.

[Unreleased]: https://github.com/TimerErTim/logandaim.at/commits/main