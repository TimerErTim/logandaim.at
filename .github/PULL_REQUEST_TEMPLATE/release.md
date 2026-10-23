# Release pull request

<!--
`dev` -> `main`. Merging deploys to GitHub Pages and creates the release tagged `v<version>`.
See `CONTRIBUTING.md`.
-->

## Release description

<!--
Becomes the release body. Write for end users: what the release does, not the pull request process.
-->

## Ships

<!--
One line per shipped issue or pull request: a dash, a hash, then the number. `pr/main` requires one.
-->

- #

## Version

- [ ] `mise run release:version <patch|minor|major>` has been run, or the version was set explicitly.
- [ ] `Cargo.toml` (`[workspace.package] version`) and `frontend/package.json` carry the new version.
- [ ] `CHANGELOG.md` has a dated `## [<version>] - YYYY-MM-DD` section.
- [ ] `CHANGELOG.md` has a fresh `## [Unreleased]` section above it.

## Test evidence

- [ ] `mise run ci`
- [ ] `mise run check:release`

## Checklist

- [ ] No direct commits to `main`; this branch is `dev` or `hotfix/*`
- [ ] Every shipped pull request is merged into `dev`
