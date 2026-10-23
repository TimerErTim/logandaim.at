Closes #

**Spec:** `specs/product/<feature>.md#<section>`

This template is for pull requests into `dev`. For a release pull request from `dev` into `main`,
use the release template: add `?template=release.md` to the compare URL.

## Behavior

What changes for the user or for the system.

## Acceptance criteria

- [ ] AC-…: covered by …

## Test evidence

- [ ] `mise run ci` (check, test, build, smoke-test; the same command the `pr/base` check runs)
- [ ] `mise run check:release` only if this pull request targets `main` (the `release-check` check)

`mise run ci` covers formatting, linting, type checking, the OpenAPI drift check, the version
check, the test suite, both builds, and the daemon-based smoke tests. Report failures with the task
name, not the underlying tool.

## Checklist

- [ ] `CHANGELOG.md` updated under `Unreleased` for user-visible changes (see the changelog scope in `CONTRIBUTING.md`)
- [ ] Spec updated if behavior differs from it
- [ ] No new legacy names, assets, or veterinary domain terms
- [ ] Migration or data notes below, if any

## Notes
