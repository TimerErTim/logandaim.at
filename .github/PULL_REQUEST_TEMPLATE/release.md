# Release pull request

This template is for merging `dev` into `main`. Merging it deploys the static frontend to GitHub
Pages and creates the GitHub release tagged `v<version>`. Read `CONTRIBUTING.md` before you start.

## Release description

One or two paragraphs written for end users and contributors reading the release page. This section
becomes the first part of the release body, so describe the release, not the pull request process.

## Ships

List the pull requests and issues that reach users with this release:

- #…

## Version

- [ ] `mise run release:version <patch|minor|major>` has been run, or the version was set explicitly.
- [ ] `Cargo.toml` (`[workspace.package] version`) and `frontend/package.json` carry the new version.
- [ ] `CHANGELOG.md` has a dated `## [<version>] - YYYY-MM-DD` section.
- [ ] `CHANGELOG.md` has a fresh `## [Unreleased]` section above it.

## Test evidence

- [ ] `mise run ci` passes (check, test, build, smoke-test)
- [ ] `mise run check:release` passes
- [ ] Repository variable `LOGANDAIM_API_URL` points at the production API (the Pages build fails without it)

## Checklist

- [ ] No direct commits to `main`; this branch is `dev` or `hotfix/*`
- [ ] Every shipped pull request is merged into `dev`
- [ ] Open issues for the next milestone are unchanged and still correctly prioritized
