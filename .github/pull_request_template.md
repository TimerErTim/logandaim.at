Closes #

**Spec:** `specs/product/<feature>.md#<section>`

## Behavior

What changes for the user or for the system.

## Acceptance criteria

- [ ] AC-…: covered by …

## Test evidence

- [ ] `mise run check`
- [ ] `mise run test`
- [ ] Builds affected by this change (`mise run build:frontend`, `build:server`)
- [ ] `mise run sync:openapi` run if the API surface changed

## Checklist

- [ ] `CHANGELOG.md` updated under `Unreleased` (user-visible changes only)
- [ ] Spec updated if behavior differs from it
- [ ] No new YAMS names, assets, or domain terms
- [ ] Migration or data notes below, if any

## Notes
