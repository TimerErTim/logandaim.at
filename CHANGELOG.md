# Changelog

All notable user-visible changes to logandaim.at are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Draft spec [`specs/product/konto.md`](specs/product/konto.md) for accounts and login
  (registration, login, password reset, account deletion, admin role) and proposed
  [ADR-0002](specs/decisions/0002-session-cookie-auth.md) for session-cookie auth with
  explicit-origin CORS.

### Changed

- SPEC-konto draft: `Konto-Löschung` now anonymizes `Runden`/`Schüsse` instead of deleting them
  (aggregate statistics survive; PO decision Q-1), auth throttle thresholds are fixed exactly
  (PO decision Q-4), and registration answers generically so it cannot be used as an
  account-enumeration oracle.
- Remove the Tauri desktop shell; the app now runs as a web frontend with an HTTP backend.

[Unreleased]: https://github.com/TimerErTim/logandaim.at/commits/main
