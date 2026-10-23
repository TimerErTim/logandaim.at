# logandaim.at

## [![HTL Grieskirchen](https://img.shields.io/badge/Education-HTL%20Grieskirchen-ffffff?style=flat&logo=internetarchive&logoColor=ffffff)](https://htl-grieskirchen.net)

logandaim.at is an Austria-wide online logbook for 3D archery, optimized for 3D targets and parcours.
Archers log rounds target by target, including the exact shooting distance between stakes, and see how
their accuracy develops over time. Parcours operators get a standard way to describe their courses.

## Introduction

The software records `Runden` (one pass through a `Parcours`) and the individual `Schüsse` per `Ziel`,
scoring them with the classic 3-arrow `Wertung`. Master data (`Anlage`, `Parcours`, `Ziel`, `Pflock`)
is seeded by a system administrator. See the [product charter](specs/product/charter.md) for the domain
glossary and MVP scope.

The underlying persistence model is implemented with [libsql](https://libsql.org) (SQLite) with a hexagonal architecture in Rust.

## Installation

The latest versions can be seen and downloaded on the
[releases page](https://github.com/TimerErTim/logandaim.at/releases).

## Usage

The server reads its configuration from `logandaim-server.json` in the working directory, overridable via
`--config-path` or `LOGANDAIM_CONFIG_PATH`. See [`backend/README.md`](backend/README.md) for the full
option and environment variable list.

## Technologies

We decided to use the following technologies:

- **Development**
  - Environment: [mise-en-place](https://github.com/jdx/mise)
- **Backend**
  - Language: Rust
  - Database: [libsql](https://libsql.org) (SQLite)
  - API: [poem-openapi](https://github.com/poem-web/poem)
- **Frontend**
  - Language: [TypeScript](https://www.typescriptlang.org/)
  - Framework: [Next.js](https://nextjs.org/)
  - State Management: [TanStack Query](https://tanstack.com/query) (Migrating from MobX)
  - Design: [TailwindCSS](https://tailwindcss.com/), [HeroUI](https://heroui.com/)
### Contributing

mise handles almost everything for you. Just make sure to define `env = ["dev"]` in your local-only `.miserc.toml` file to get the full development environment.
