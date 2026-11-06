## Sections

`Anlagen` holds master data: an Anlage with its address, contact details, role
assignments, Ziele, and Parkour, plus the public directory. `Parkour` covers the
shooting setup itself: Pflöcke, their order, and the distances to the Ziele placed
among them. `Runden` covers shooting a Parkour and logging Schüsse. `Konto` covers
registration, sign-in, sessions, password recovery, and the caller's own Benutzer.

## Authentication

A protected route takes an access token as `Authorization: Bearer <token>`. Sign-in,
registration step 2, and session renewal hand that token back in the response body
and place the refresh token in the `__Host-logandaim_refresh` cookie. Every session
response is `Cache-Control: no-store`; the refresh token never appears in a body.
Routes that accept a token also work without one, as an anonymous caller, and then
usually answer `401`.

## Errors

Every failed request answers the same envelope: `message` says what failed,
`attachments` carries the free-text detail attached to it, and `sources` nests the
causes beneath it. A `500` answers a constant body and never reveals its cause. The
HTTP status carries the meaning: `400` for an invalid request, `401` for a missing or
unusable token, `403` for a caller that may not act, `404` for what is not there, and
`409` for a write an archived object refuses.
