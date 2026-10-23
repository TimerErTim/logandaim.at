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

### Changed

- Remove the Tauri desktop shell; the app now runs as a web frontend with an HTTP backend.

[Unreleased]: https://github.com/TimerErTim/logandaim.at/commits/main
