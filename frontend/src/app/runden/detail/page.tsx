"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useRundeQuery, useStammdatenMutations } from "@/api/hooks";
import type { Runde } from "@/api/logandaim-api";
import { Wertungszone } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
import { PflockMarker } from "@/components/stammdaten/pflock-swatch";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  HatchBand,
  MarginNote,
  Perforation,
  RulerEdge,
  SheetNumber,
  StampMark,
} from "@/components/ui/paper-rule";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
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

  // `null` means the query parameter has not been read yet; both cases show the same
  // placeholder, because the page has no shape to draw before the id is known.
  if (id === null || runde.isLoading) {
    return <RundeSkeleton />;
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

  if (runde.isError || !runde.data) {
    return (
      <div className="space-y-4 p-6">
        <Alert variant="error">
          {apiErrorMessage(runde.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Runde konnte nicht geladen werden.",
          })}
        </Alert>
        <MarginNote label="Nächster Schritt">
          Öffnen Sie die Runde erneut aus der Historie. Wenn der Fehler bleibt,
          ist der Eintrag nicht mehr vorhanden.
        </MarginNote>
      </div>
    );
  }

  return <RundeInhalt runde={runde.data} />;
}

/** A printed placeholder: the same blocks the sheet will carry, before they arrive. */
function RundeSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6 p-6">
      <div className="space-y-3">
        <Skeleton className="h-3 w-32" />
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-px w-full" />
      </div>
      <Skeleton className="h-40 w-full" />
      <div className="space-y-2">
        <Skeleton className="h-5 w-24" />
        <Skeleton className="h-24 w-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-5 w-20" />
        <Skeleton className="h-16 w-full" />
      </div>
    </div>
  );
}

function RundeInhalt({ runde }: { runde: Runde }) {
  const { rundeAbschliessen, schussRegistrieren } = useStammdatenMutations();
  const [abschliessenOffen, setAbschliessenOffen] = useState(false);
  const offen = runde.zustand === "InBearbeitung";
  const zieleGesamt = runde.ziele.length;
  const fortschritt =
    zieleGesamt > 0 ? (runde.zieleMitErgebnis / zieleGesamt) * 100 : 0;

  return (
    <div className="flex min-h-full flex-col">
      {/* The bottom padding keeps the last Ziel clear of the sticky action bar. */}
      <div className="flex-1 space-y-6 p-6 pb-24">
        <SetBreadcrumbLabel label={runde.parcoursName} />

        <PageHeader
          register={["Logbuch", "Historie"]}
          title={runde.parcoursName}
          plate={runde.id.slice(0, 8)}
          description="Aufstellung zum Start der Runde. Spätere Änderungen an den Stammdaten ändern diese Runde nicht."
          actions={
            <>
              <Badge variant={offen ? "primary" : "default"}>
                {offen ? "In Bearbeitung" : "Abgeschlossen"}
              </Badge>
              {offen ? null : <StampMark>Abgeschlossen</StampMark>}
              <SheetNumber>
                Begonnen {new Date(runde.begonnenAm).toLocaleString("de-AT")}
              </SheetNumber>
            </>
          }
        />

        <Card>
          <CardHeader>
            <CardTitle>Wertung</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex items-stretch gap-4">
              <RulerEdge />
              <div className="min-w-0 flex-1 space-y-4">
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
                      {runde.zieleMitErgebnis} / {zieleGesamt}
                      <Progress
                        value={fortschritt}
                        aria-label={`Ziele mit Ergebnis: ${runde.zieleMitErgebnis} von ${zieleGesamt}`}
                        className="mt-2"
                      />
                    </dd>
                  </div>
                </dl>

                <MarginNote label="Zählweise">
                  Summiert über die Ziele, die mindestens einen Schuss tragen.
                  Ziele ohne Schuss bleiben in der Runde, zählen aber nicht mit.
                </MarginNote>
              </div>
            </div>
          </CardContent>
        </Card>

        <HatchBand />

        <section aria-label="Pflöcke" className="space-y-2">
          <div className="flex items-baseline gap-3">
            <h2 className="text-sm font-semibold text-ink">Pflöcke</h2>
            <SheetNumber>{runde.pflöcke.length}</SheetNumber>
          </div>

          {runde.pflöcke.length === 0 ? (
            <p className="text-sm text-secondary">
              Diese Runde hat keine Pflöcke.
            </p>
          ) : (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader className="w-16">Reihe</TableHeader>
                  <TableHeader>Farbe</TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {runde.pflöcke.map((pflock) => (
                  <TableRow key={`${pflock.position}-${pflock.farbe}`}>
                    <TableCell>
                      <SheetNumber className="text-xs">
                        {pflock.position + 1}
                      </SheetNumber>
                    </TableCell>
                    <TableCell>
                      <PflockMarker farbe={pflock.farbe} />
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </section>

        <Perforation />

        <section aria-label="Ziele" className="space-y-4">
          <div className="flex items-baseline gap-3">
            <h2 className="text-sm font-semibold text-ink">Ziele</h2>
            <SheetNumber>{zieleGesamt}</SheetNumber>
          </div>

          {zieleGesamt === 0 ? (
            <p className="text-sm text-secondary">
              Diese Runde hat keine Ziele.
            </p>
          ) : (
            runde.ziele.map((ziel, index) => (
              <div key={ziel.zielId} className="space-y-4">
                <Zielabschnitt
                  ziel={ziel}
                  reihenfolge={index + 1}
                  offen={offen}
                  ausstehend={schussRegistrieren.isPending}
                  onErfassen={(wertungszone, pfeil) =>
                    schussRegistrieren.mutate({
                      rundeId: runde.id,
                      schuss: {
                        zielId: ziel.zielId,
                        pfeil,
                        wertungszone,
                      },
                    })
                  }
                />
                {index < runde.ziele.length - 1 ? <Separator /> : null}
              </div>
            ))
          )}
        </section>

        <HatchBand height="lg" />
      </div>

      <div className="sticky bottom-0 z-20 border-t border-subtle bg-surface">
        <div className="flex flex-wrap items-center justify-between gap-4 px-6 py-3">
          <div className="flex flex-wrap items-baseline gap-x-6 gap-y-1">
            <span className="text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase">
              Gesamtstand
            </span>
            <span className="inline-flex items-baseline gap-2">
              <span className="text-xs text-secondary">Erster Treffer</span>
              <span className="text-sm font-semibold tabular-nums text-ink">
                {runde.gesamtErsterTreffer}
              </span>
            </span>
            <span className="inline-flex items-baseline gap-2">
              <span className="text-xs text-secondary">Mehrschuss</span>
              <span className="text-sm font-semibold tabular-nums text-ink">
                {runde.gesamtMehrschuss}
              </span>
            </span>
            <span className="inline-flex items-baseline gap-2">
              <span className="text-xs text-secondary">Ziele mit Ergebnis</span>
              <span className="text-sm font-semibold tabular-nums text-ink">
                {runde.zieleMitErgebnis} / {zieleGesamt}
              </span>
            </span>
          </div>

          <div className="flex flex-wrap items-center gap-3">
            {offen ? (
              <Button
                variant="destructive"
                onClick={() => {
                  rundeAbschliessen.reset();
                  setAbschliessenOffen(true);
                }}
              >
                Runde abschließen
              </Button>
            ) : null}
            <Button variant="secondary" asChild>
              <Link href={paths.runden}>Zurück zur Historie</Link>
            </Button>
          </div>
        </div>
      </div>

      <Dialog open={abschliessenOffen} onOpenChange={setAbschliessenOffen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Runde abschließen?</DialogTitle>
            <DialogDescription>
              Die Runde wird damit geschlossen. Weitere Pfeile lassen sich
              danach nicht mehr erfassen; die bisherige Wertung bleibt erhalten.
            </DialogDescription>
          </DialogHeader>
          {rundeAbschliessen.isError ? (
            <Alert variant="error">
              {apiErrorMessage(rundeAbschliessen.error, {
                fallback: "Die Runde konnte nicht abgeschlossen werden.",
              })}
            </Alert>
          ) : null}
          <DialogFooter>
            <Button
              variant="secondary"
              onClick={() => setAbschliessenOffen(false)}
              disabled={rundeAbschliessen.isPending}
            >
              Abbrechen
            </Button>
            <Button
              variant="destructive"
              disabled={rundeAbschliessen.isPending}
              onClick={() =>
                rundeAbschliessen.mutate(runde.id, {
                  onSuccess: () => setAbschliessenOffen(false),
                })
              }
            >
              {rundeAbschliessen.isPending
                ? "Wird abgeschlossen…"
                : "Runde abschließen"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

/**
 * One Ziel of the annex: its captured distances folded into a single line, its folded
 * result, and — while the Runde is open — the zone buttons that log the next arrow.
 */
function Zielabschnitt({
  ziel,
  reihenfolge,
  offen,
  ausstehend,
  onErfassen,
}: {
  ziel: Runde["ziele"][number];
  reihenfolge: number;
  offen: boolean;
  ausstehend: boolean;
  onErfassen: (zone: Wertungszone, pfeil: number) => void;
}) {
  return (
    <article className="space-y-3">
      <div className="flex flex-wrap items-baseline gap-3">
        <SheetNumber className="text-xs">
          {String(reihenfolge).padStart(2, "0")}
        </SheetNumber>
        <h3 className="text-sm font-semibold text-ink">{ziel.tiername}</h3>
        <Badge>Zielgruppe {ziel.zielgruppe}</Badge>
        <SheetNumber>{ziel.zielId.slice(0, 8)}</SheetNumber>
      </div>

      <div className="flex flex-wrap items-baseline gap-x-4 gap-y-1 text-sm">
        {ziel.abstände.length === 0 ? (
          <span className="text-secondary">Keine Abstände erfasst.</span>
        ) : (
          ziel.abstände.map((abstand) => (
            <span
              key={abstand.pflock}
              className="inline-flex items-baseline gap-1.5"
            >
              <span className="text-secondary">{abstand.pflock}</span>
              <span aria-hidden="true" className="text-secondary">
                ·
              </span>
              <span className="font-medium tabular-nums text-ink">
                {abstand.meter} m
              </span>
            </span>
          ))
        )}
      </div>

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
          ausstehend={ausstehend}
          onErfassen={(zone) =>
            onErfassen(zone, (ziel.ergebnis?.anzahlSchüsse ?? 0) + 1)
          }
        />
      ) : null}
    </article>
  );
}

/**
 * Log the next arrow at one Ziel. The zone is picked in the same gesture as the
 * commit, so a shot is never recorded with a stale zone: each button carries the
 * zone it submits (SPEC-wertung).
 *
 * The buttons stay in the order an archer names the zones — Spot · Kill · Body ·
 * Fehlschuss — and share one labelled group, so the row reads as a choice of zone
 * rather than four unrelated actions.
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
      <fieldset className="min-w-0 space-y-2">
        <legend className="text-[10px] font-semibold tracking-[0.09em] text-secondary uppercase">
          Wertungszone
        </legend>
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
      </fieldset>
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
