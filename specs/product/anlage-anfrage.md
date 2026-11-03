---
id: SPEC-anlage-anfrage
title: Requesting a new Anlage
status: Draft
owner: TimerErTim
issues: [68]
superseded_by: null
---

# Anlage-Anfrage — Requesting a new Anlage

## Problem

Only a `Systemadministrator` creates an `Anlage` (see [`stammdaten.md`](stammdaten.md),
[`konto.md`](konto.md) rule 15). A club that wants its parkour in the product therefore has no
way in unless someone already knows the operator. Manual seeding covers the 2–3 pilot Anlagen and
nothing after them.

## Outcome

A person with authority over a real 3D parkour can ask for it to be added, from inside the
product, and a `Systemadministrator` can accept or reject that request with a recorded reason. The
requester never gains authority over an `Anlage` by asking.

## Roles

- [`Benutzer`](charter.md#roles): submits a request for an `Anlage`.
- [`Systemadministrator`](charter.md#roles): accepts or rejects a request.

## Scope

**In scope**

- What a request contains.
- The states a request passes through.
- Who sees which state.
- What acceptance creates, and whether it grants the requester an `AnlageVerwaltung`.

**Out of scope**

- The master data itself ([`stammdaten.md`](stammdaten.md)).
- Abuse handling beyond throttling; a moderation queue is not designed here.
- Public listing of Anlagen before acceptance.

## Domain terms and business rules

| Term | Meaning |
|------|---------|
| `Anlage-Anfrage` | A recorded request to add one `Anlage`, with a requester and a state. |
| `Annahme` | Acceptance: creates the `Anlage`, links it to the request, ends the request. |
| `Ablehnung` | Rejection: ends the request with a recorded reason; creates nothing. |

Rules:

1. A request requires a logged-in `Benutzer` and names the `Anlage` (name, location), a contact, and
   the requester's relationship to the site. It is readable only by its requester and by
   `Systemadministratoren`.
2. A request is in exactly one state: `offen`, `angenommen`, or `abgelehnt`. Acceptance and
   rejection are final; a rejected request can be resubmitted as a new request.
3. Acceptance creates exactly one `Anlage` and records who accepted it and when.
4. A request alone grants no authority. Whether acceptance makes the requester `Administrator` of
   the created `Anlage` is Q-2 and applies to every accepted request the same way.
5. A `Benutzer` holds at most one `offen` request at a time; submitting again while one is open is
   refused with 409.
6. Rejection states a reason. The reason is visible to the requester.
7. Requests are throttled per `Benutzer` and per client; exceeding the limit fails with 429.

## Open questions

- [ ] Q-1: Must the requester already hold a `Benutzer`, or may a site owner apply without one?
- [ ] Q-2: On acceptance, is the requester automatically `Administrator` of the created
  `Anlage`, or does a `Systemadministrator` assign `AnlageVerwaltungen` manually?
- [ ] Q-3: Is this flow MVP, before the pilot Anlagen go public, or `Later`?
- [ ] Q-4: What evidence of authority over a site is required, and who checks it?

## Acceptance criteria

- AC-1: Given a logged-in `Benutzer`, when submitting a complete request, then it is stored as
  `offen` and is readable by that `Benutzer` and by every `Systemadministrator`.
- AC-2: When an unauthenticated visitor attempts to submit, then the request fails with 401.
- AC-3: Given an `offen` request, when the same `Benutzer` submits another, then the request fails
  with 409 and the first request stays `offen`.
- AC-4: When a `Systemadministrator` accepts a request, then one `Anlage` exists, the request is
  `angenommen`, and the accepting `Systemadministrator` and timestamp are recorded.
- AC-5: When a `Systemadministrator` rejects a request with a reason, then the request is `abgelehnt`,
  no `Anlage` is created, and the requester can read the reason.
- AC-6: When a request is accepted or rejected, then it can no longer be changed, and any further
  transition on it fails with 409.

## Verification

API e2e tests, one per AC, including the duplicate-open-request and transition-after-final paths.
Domain unit tests for the state machine.

## Decision log

| Date | Decision | By |
|------|----------|----|
| 2026-10-26 | A `Benutzer` requests an `Anlage`; only a `Systemadministrator` creates one. | TimerErTim (PO) |

## Revision history

| Date | Change | By |
|------|--------|----|
| 2026-10-26 | Initial draft from issue #68. | Agent (Cursor) |
