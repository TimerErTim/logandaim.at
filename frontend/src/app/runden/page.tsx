"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useRundenQuery } from "@/api/hooks";
import type { RundenZeile } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { HatchBand, SheetNumber, StampMark } from "@/components/ui/paper-rule";
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

/**
 * The caller's own Runden, split into what is still open and what is closed. Another
 * Benutzer's Runden are never listed, not even for an Anlage's staff (AC-13); a
 * Systemadministrator reads one by identity alone.
 */
export default function RundenPage() {
  const phase = useAuth();
  const runden = useRundenQuery();
  const router = useRouter();

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  const alle = runden.data ?? [];
  const offen = alle.filter((runde) => runde.zustand === "InBearbeitung");
  const abgeschlossen = alle.filter(
    (runde) => runde.zustand === "Abgeschlossen",
  );

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label="Historie" />

      <PageHeader
        register={["Logbuch", "Historie"]}
        title="Historie"
        description="Jede Runde hält die Aufstellung fest, mit der sie begonnen wurde. Spätere Änderungen an den Stammdaten verändern sie nicht."
      />

      {runden.isError ? (
        <Alert variant="error">
          {apiErrorMessage(runden.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Runden konnten nicht geladen werden.",
          })}
        </Alert>
      ) : null}

      {runden.isLoading ? (
        <HistorieSkeleton />
      ) : alle.length === 0 ? (
        <Leerzustand />
      ) : (
        <>
          <RundenAbschnitt
            titel="In Bearbeitung"
            runden={offen}
            offen
            leer="Keine Runde ist offen."
          />

          <HatchBand />

          <RundenAbschnitt
            titel="Abgeschlossen"
            runden={abgeschlossen}
            leer="Noch keine Runde abgeschlossen."
          />
        </>
      )}
    </div>
  );
}

function HistorieSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6">
      <div className="space-y-2">
        <Skeleton className="h-5 w-40" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-5 w-40" />
        <Skeleton className="h-12 w-full" />
      </div>
    </div>
  );
}

/** A ruled section: a serif-free caption with its count, then a top-level ledger. */
function RundenAbschnitt({
  titel,
  runden,
  offen = false,
  leer,
}: {
  titel: string;
  runden: RundenZeile[];
  offen?: boolean;
  leer: string;
}) {
  return (
    <section aria-label={titel} className="space-y-2">
      <div className="flex items-baseline gap-3">
        <h2 className="text-sm font-semibold text-ink">{titel}</h2>
        <SheetNumber>{runden.length}</SheetNumber>
      </div>

      {runden.length === 0 ? (
        <p className="text-sm text-secondary">{leer}</p>
      ) : (
        <Table>
          <TableHead>
            <TableRow>
              <TableHeader>Parcours</TableHeader>
              <TableHeader>Begonnen</TableHeader>
              <TableHeader>
                <span className="sr-only">Status</span>
              </TableHeader>
              <TableHeader>
                <span className="sr-only">Aktion</span>
              </TableHeader>
            </TableRow>
          </TableHead>
          <TableBody>
            {runden.map((runde) => (
              <TableRow key={runde.id}>
                <TableCell className="font-medium">
                  {runde.parcoursName}
                </TableCell>
                <TableCell className="tabular-nums text-secondary">
                  {new Date(runde.begonnenAm).toLocaleString("de-AT")}
                </TableCell>
                <TableCell>
                  {offen ? null : <StampMark>Abgeschlossen</StampMark>}
                </TableCell>
                <TableCell className="text-right">
                  {offen ? (
                    <Button size="sm" asChild>
                      <Link href={`${paths.runden}/detail?id=${runde.id}`}>
                        Fortsetzen
                      </Link>
                    </Button>
                  ) : (
                    <Button variant="secondary" size="sm" asChild>
                      <Link href={`${paths.runden}/detail?id=${runde.id}`}>
                        Öffnen
                      </Link>
                    </Button>
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </section>
  );
}

/** The next action for an empty history is to pick an Anlage and start a Runde. */
function Leerzustand() {
  return (
    <div className="space-y-4">
      <p className="text-sm text-secondary">
        Noch keine Runde begonnen. Wählen Sie im Anlagen-Verzeichnis einen
        Parcours, um die erste Runde zu starten.
      </p>
      <Button asChild>
        <Link href={paths.anlagen}>Zu den Anlagen</Link>
      </Button>
    </div>
  );
}
