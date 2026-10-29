"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useRundeQuery, useStammdatenMutations } from "@/api/hooks";
import type { Runde } from "@/api/logandaim-api";
import { Wertungszone } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PflockMarker } from "@/components/stammdaten/pflock-swatch";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { RegisterLine, SheetNumber } from "@/components/ui/paper-rule";
import { Separator } from "@/components/ui/separator";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { paths } from "@/lib/navigation";
import { useQueryParam } from "@/lib/use-query-param";

/**
 * One Runde, read as it was captured. Everything on this page comes from the snapshot:
 * the Parcours name, the Pflock order and colours, every distance, and each animal's
 * name and target group. The Wertungen are folded from the logged Schüsse — there is no
 * separate Wertung request (SPEC-wertung rule 6).
 */
export default function RundeDetailPage() {
  const phase = useAuth();
  const router = useRouter();
  const id = useQueryParam("id");
  const runde = useRundeQuery(id);

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  if (id === null) {
    return (
      <div className="p-6">
        <p className="text-sm text-secondary">Runde wird geladen …</p>
      </div>
    );
  }

  if (!id) {
    return (
      <div className="p-6">
        <Alert variant="warning">
          Es wurde keine Runde ausgewählt. Bitte öffnen Sie sie aus der
          Übersicht.
        </Alert>
      </div>
    );
  }

  if (runde.isLoading) {
    return (
      <div className="p-6">
        <p className="text-sm text-secondary">Runde wird geladen …</p>
      </div>
    );
  }

  if (runde.isError || !runde.data) {
    return (
      <div className="p-6">
        <Alert variant="error">
          {apiErrorMessage(runde.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Runde konnte nicht geladen werden.",
          })}
        </Alert>
      </div>
    );
  }

  return <RundeInhalt runde={runde.data} />;
}

function RundeInhalt({ runde }: { runde: Runde }) {
  const { rundeAbschliessen, schussRegistrieren } = useStammdatenMutations();
  const offen = runde.zustand === "InBearbeitung";

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label={runde.parcoursName} />

      <header className="space-y-3">
        <RegisterLine
          items={[
            "Runde",
            new Date(runde.begonnenAm).toLocaleString("de-AT"),
            runde.zustand === "InBearbeitung"
              ? "In Bearbeitung"
              : "Abgeschlossen",
          ]}
        />
        <div className="flex flex-wrap items-center gap-3">
          <SheetNumber>{runde.id.slice(0, 8)}</SheetNumber>
          <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
            {runde.parcoursName}
          </h1>
          <Badge variant={offen ? "primary" : "default"}>
            {offen ? "In Bearbeitung" : "Abgeschlossen"}
          </Badge>
        </div>
        <p className="text-sm text-secondary">
          Aufstellung zum Start der Runde. Spätere Änderungen an den Stammdaten
          ändern diese Runde nicht.
        </p>
      </header>

      {rundeAbschliessen.isError ? (
        <Alert variant="error">
          {apiErrorMessage(rundeAbschliessen.error, {
            fallback: "Die Runde konnte nicht abgeschlossen werden.",
          })}
        </Alert>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle>Wertung</CardTitle>
        </CardHeader>
        <CardContent>
          <dl className="grid gap-4 sm:grid-cols-3">
            <div className="space-y-1">
              <dt className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
                Erster Treffer
              </dt>
              <dd className="font-serif text-2xl font-semibold tabular-nums text-ink">
                {runde.gesamtErsterTreffer}
              </dd>
            </div>
            <div className="space-y-1">
              <dt className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
                Mehrschuss
              </dt>
              <dd className="font-serif text-2xl font-semibold tabular-nums text-ink">
                {runde.gesamtMehrschuss}
              </dd>
            </div>
            <div className="space-y-1">
              <dt className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
                Ziele mit Ergebnis
              </dt>
              <dd className="font-serif text-2xl font-semibold tabular-nums text-ink">
                {runde.zieleMitErgebnis}
              </dd>
            </div>
          </dl>
          <p className="mt-3 text-xs text-secondary">
            Summiert über die Ziele, die mindestens einen Schuss tragen. Ziele
            ohne Schuss bleiben in der Runde, zählen aber nicht mit.
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Pflöcke</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader className="w-16">Reihe</TableHeader>
                <TableHeader>Bezeichnung</TableHeader>
                <TableHeader>Farbe</TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {runde.pflöcke.map((pflock) => (
                <TableRow key={`${pflock.position}-${pflock.bezeichnung}`}>
                  <TableCell className="font-mono text-xs tabular-nums text-secondary">
                    {pflock.position + 1}
                  </TableCell>
                  <TableCell>{pflock.bezeichnung}</TableCell>
                  <TableCell>
                    <PflockMarker farbe={pflock.farbe} />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Ziele</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          {runde.ziele.length === 0 ? (
            <p className="text-sm text-secondary">
              Diese Runde hat keine Ziele.
            </p>
          ) : (
            runde.ziele.map((ziel) => (
              <div key={ziel.zielId} className="space-y-3">
                <div className="flex flex-wrap items-baseline gap-3">
                  <h3 className="text-sm font-semibold text-ink">
                    {ziel.tiername}
                  </h3>
                  <Badge>Zielgruppe {ziel.zielgruppe}</Badge>
                  <span className="font-mono text-xs text-secondary">
                    {ziel.zielId.slice(0, 8)}
                  </span>
                </div>

                <Table>
                  <TableHead>
                    <TableRow>
                      <TableHeader>Pflock</TableHeader>
                      <TableHeader className="text-right">Abstand</TableHeader>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {ziel.abstände.map((abstand) => (
                      <TableRow key={abstand.pflock}>
                        <TableCell>{abstand.pflock}</TableCell>
                        <TableCell className="text-right tabular-nums">
                          {abstand.meter} m
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>

                {ziel.ergebnis ? (
                  <dl className="grid gap-3 text-sm sm:grid-cols-4">
                    <div className="flex items-baseline justify-between gap-2 sm:block">
                      <dt className="text-secondary">Erster Treffer</dt>
                      <dd className="font-medium tabular-nums">
                        {ziel.ergebnis.ersterTreffer}
                      </dd>
                    </div>
                    <div className="flex items-baseline justify-between gap-2 sm:block">
                      <dt className="text-secondary">Mehrschuss</dt>
                      <dd className="font-medium tabular-nums">
                        {ziel.ergebnis.mehrschuss}
                      </dd>
                    </div>
                    <div className="flex items-baseline justify-between gap-2 sm:block">
                      <dt className="text-secondary">Erster Pfeil</dt>
                      <dd className="font-medium">
                        {ziel.ergebnis.ersterPfeilGetroffen
                          ? "getroffen"
                          : "nicht getroffen"}
                      </dd>
                    </div>
                    <div className="flex items-baseline justify-between gap-2 sm:block">
                      <dt className="text-secondary">Schüsse</dt>
                      <dd className="font-medium tabular-nums">
                        {ziel.ergebnis.anzahlSchüsse}
                      </dd>
                    </div>
                  </dl>
                ) : (
                  <p className="text-sm text-secondary">
                    Noch kein Schuss auf dieses Ziel.
                  </p>
                )}

                {offen ? (
                  <SchussErfassung
                    tiername={ziel.tiername}
                    naechsterPfeil={(ziel.ergebnis?.anzahlSchüsse ?? 0) + 1}
                    ausstehend={schussRegistrieren.isPending}
                    onErfassen={(wertungszone) =>
                      schussRegistrieren.mutate({
                        rundeId: runde.id,
                        schuss: {
                          zielId: ziel.zielId,
                          pfeil: (ziel.ergebnis?.anzahlSchüsse ?? 0) + 1,
                          wertungszone,
                        },
                      })
                    }
                  />
                ) : null}

                <Separator />
              </div>
            ))
          )}
        </CardContent>
      </Card>

      <div className="flex flex-wrap gap-3">
        {offen ? (
          <Button
            disabled={rundeAbschliessen.isPending}
            onClick={() => rundeAbschliessen.mutate(runde.id)}
          >
            {rundeAbschliessen.isPending
              ? "Wird abgeschlossen…"
              : "Runde abschließen"}
          </Button>
        ) : null}
        <Button variant="secondary" asChild>
          <Link href={paths.runden}>Zurück zu den Runden</Link>
        </Button>
      </div>
    </div>
  );
}

/**
 * Log the next arrow at one Ziel. The zone is picked in the same gesture as the
 * commit, so a shot is never recorded with a stale zone: each button carries the
 * zone it submits (SPEC-wertung).
 */
function SchussErfassung({
  tiername,
  naechsterPfeil,
  ausstehend,
  onErfassen,
}: {
  tiername: string;
  naechsterPfeil: number;
  ausstehend: boolean;
  onErfassen: (zone: Wertungszone) => void;
}) {
  return (
    <div className="space-y-2">
      <p className="text-xs font-medium text-secondary">
        Pfeil {naechsterPfeil} für {tiername} erfassen
      </p>
      <div className="flex flex-wrap gap-2">
        {ZONEN.map(({ zone, label }) => (
          <Button
            key={zone}
            variant="secondary"
            size="sm"
            disabled={ausstehend}
            aria-label={`${label} für ${tiername}, Pfeil ${naechsterPfeil}`}
            onClick={() => onErfassen(zone)}
          >
            {label}
          </Button>
        ))}
      </div>
    </div>
  );
}

/** The four zones in the order an archer names them, with their German labels. */
const ZONEN: { zone: Wertungszone; label: string }[] = [
  { zone: Wertungszone.Spot, label: "Spot" },
  { zone: Wertungszone.Kill, label: "Kill" },
  { zone: Wertungszone.Body, label: "Body" },
  { zone: Wertungszone.Miss, label: "Fehlschuss" },
];
