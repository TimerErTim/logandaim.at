"use client";

import Link from "next/link";

import { useAuth } from "@/api/auth-provider";
import { useRundenQuery, useRundeQuery } from "@/api/hooks";
import type { RundeBasic } from "@/api/logandaim-api";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import {
  HatchBand,
  MarginNote,
  SheetNumber,
  StampMark,
} from "@/components/ui/paper-rule";
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
 * The logbook's first page: what the caller should do next. An open Runde is picked up
 * again, the last three closed Runden are read as figures, and the directory of Anlagen
 * is the way to begin something new.
 *
 * A guest gets the directory and the way in, and nothing that would need a session:
 * the Runden hooks are gated on the authenticated phase, so no read is attempted for
 * an anonymous visitor.
 */
export default function HomePage() {
  const phase = useAuth();

  return (
    <div className="space-y-6 p-6">
      <PageHeader
        register={["Logbuch"]}
        title="Logbuch"
        description="Ihre Runden, vom ersten Pfeil bis zum letzten Ergebnis. Was offen ist, setzen Sie fort; was abgeschlossen ist, bleibt als Aufzeichnung stehen."
      />

      {phase.status === "authenticated" ? (
        <EingetragenesLogbuch />
      ) : phase.status === "anonymous" ? (
        <GastEinstieg />
      ) : (
        <LogbuchSkeleton />
      )}
    </div>
  );
}

/** A guest sees the directory and the way in, never a record. */
function GastEinstieg() {
  return (
    <div className="space-y-4">
      <MarginNote label="Ohne Anmeldung">
        Das Anlagen-Verzeichnis ist öffentlich. Ihre eigenen Runden sehen Sie
        nach der Anmeldung.
      </MarginNote>
      <div className="flex flex-wrap gap-3">
        <Button asChild>
          <Link href="/anmeldung">Anmelden</Link>
        </Button>
        <Button variant="secondary" asChild>
          <Link href={paths.anlagen}>Zu den Anlagen</Link>
        </Button>
      </div>
    </div>
  );
}

function EingetragenesLogbuch() {
  const runden = useRundenQuery();
  const alle = runden.data ?? [];

  if (runden.isLoading) {
    return <LogbuchSkeleton />;
  }

  const offene = alle
    .filter((runde) => runde.zustand === "InBearbeitung")
    .sort((a, b) => Date.parse(b.begonnenAm) - Date.parse(a.begonnenAm));
  const aktive = offene[0];
  const letzteDrei = [...alle]
    .sort((a, b) => Date.parse(b.begonnenAm) - Date.parse(a.begonnenAm))
    .slice(0, 3);

  if (alle.length === 0) {
    return <Leerzustand />;
  }

  return (
    <>
      {aktive ? (
        <>
          <AktiveRunde runde={aktive} />
          <HatchBand />
        </>
      ) : null}

      <section aria-label="Letzte Runden" className="space-y-2">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">Letzte Runden</h2>
          <SheetNumber>{letzteDrei.length}</SheetNumber>
        </div>
        <Table>
          <TableHead>
            <TableRow>
              <TableHeader>Parkour</TableHeader>
              <TableHeader>Begonnen</TableHeader>
              <TableHeader className="text-right">Erster Treffer</TableHeader>
              <TableHeader className="text-right">Mehrschuss</TableHeader>
              <TableHeader>
                <span className="sr-only">Status</span>
              </TableHeader>
              <TableHeader>
                <span className="sr-only">Öffnen</span>
              </TableHeader>
            </TableRow>
          </TableHead>
          <TableBody>
            {letzteDrei.map((runde) => (
              <LetzteRundeZeile key={runde.id} runde={runde} />
            ))}
          </TableBody>
        </Table>
        <Button variant={aktive ? "secondary" : "primary"} asChild>
          <Link href={paths.anlagen}>Zu den Anlagen</Link>
        </Button>
      </section>
    </>
  );
}

/**
 * One row of the short history. The list endpoint carries no Wertung, so the figures are
 * read from the Runde itself — three detail reads, each gated on the session like every
 * other authenticated query.
 */
function LetzteRundeZeile({ runde }: { runde: RundeBasic }) {
  const detail = useRundeQuery(runde.id);
  const offen = runde.zustand === "InBearbeitung";

  return (
    <TableRow>
      <TableCell className="font-medium">{runde.parkourName}</TableCell>
      <TableCell className="tabular-nums text-secondary">
        {new Date(runde.begonnenAm).toLocaleString("de-AT")}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {detail.data ? (
          detail.data.gesamtErsterTreffer
        ) : (
          <Skeleton className="ml-auto h-4 w-8" />
        )}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {detail.data ? (
          detail.data.gesamtMehrschuss
        ) : (
          <Skeleton className="ml-auto h-4 w-8" />
        )}
      </TableCell>
      <TableCell>
        {offen ? (
          <SheetNumber>Offen</SheetNumber>
        ) : (
          <StampMark>Abgeschlossen</StampMark>
        )}
      </TableCell>
      <TableCell className="text-right">
        <Button variant="secondary" size="sm" asChild>
          <Link href={`${paths.runden}/detail?id=${runde.id}`}>
            {offen ? "Fortsetzen" : "Öffnen"}
          </Link>
        </Button>
      </TableCell>
    </TableRow>
  );
}

/** The one Runde the caller should pick up, with the single primary action of the page. */
function AktiveRunde({ runde }: { runde: RundeBasic }) {
  return (
    <section
      aria-label="Offene Runde"
      className="space-y-3 border-l-2 border-primary py-1 pl-4"
    >
      <div className="flex flex-wrap items-center gap-3">
        <SheetNumber>Offen</SheetNumber>
        <h2 className="text-sm font-semibold text-ink">{runde.parkourName}</h2>
        <span className="text-xs text-secondary tabular-nums">
          begonnen {new Date(runde.begonnenAm).toLocaleString("de-AT")}
        </span>
      </div>
      <Button asChild>
        <Link href={`${paths.runden}/detail?id=${runde.id}`}>Fortsetzen</Link>
      </Button>
    </section>
  );
}

function LogbuchSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6">
      <div className="space-y-2">
        <Skeleton className="h-5 w-40" />
        <Skeleton className="h-16 w-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-5 w-40" />
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-11 w-full" />
        <Skeleton className="h-11 w-full" />
      </div>
    </div>
  );
}

/** The next action for a caller with no Runde yet: choose an Anlage in the directory. */
function Leerzustand() {
  return (
    <div className="space-y-4">
      <MarginNote label="Noch keine Runde">
        Sie haben noch keine Runde begonnen. Im Anlagen-Verzeichnis wählen Sie
        eine Anlage und starten dort einen Parkour.
      </MarginNote>
      <Button asChild>
        <Link href={paths.anlagen}>Zu den Anlagen</Link>
      </Button>
    </div>
  );
}
