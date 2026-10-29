"use client";

import { Play } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import {
  useAnlagenVerzeichnisQuery,
  useStammdatenMutations,
} from "@/api/hooks";
import type {
  AnlageÖffentlich,
  AuthPhase,
  ParcoursKurz,
} from "@/api/logandaim-api";
import { Ländercode } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import {
  MarginNote,
  Perforation,
  SheetNumber,
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
import { useQueryParam } from "@/lib/use-query-param";

const laender: Record<Ländercode, string> = {
  [Ländercode.AT]: "Österreich",
  [Ländercode.DE]: "Deutschland",
  [Ländercode.CH]: "Schweiz",
};

/**
 * One Anlage's sheet: the Parcours it offers, and the way to start a Runde on one.
 *
 * The page reads the public directory and picks its entry out of it — there is no
 * single-Anlage read without a session, and the directory entry is exactly what a
 * visitor may see. A signed-in caller gets a start button per row; a guest gets the
 * way to sign in instead, because starting a Runde is the one thing that needs an
 * account.
 */
export default function AnlageDetailPage() {
  const id = useQueryParam("id");
  const verzeichnis = useAnlagenVerzeichnisQuery();

  // `null` means the query parameter has not been read yet; both cases show the same
  // placeholder, because the sheet has no shape to draw before its id is known.
  if (id === null || verzeichnis.isLoading) {
    return <AnlageSkeleton />;
  }

  if (!id) {
    return (
      <div className="space-y-4 p-6">
        <MarginNote label="Keine Anlage gewählt">
          Es wurde keine Anlage ausgewählt. Bitte öffnen Sie sie aus dem
          Verzeichnis.
        </MarginNote>
        <Button variant="secondary" asChild>
          <Link href={paths.anlagen}>Zum Verzeichnis</Link>
        </Button>
      </div>
    );
  }

  if (verzeichnis.isError) {
    return (
      <div className="space-y-4 p-6">
        <Alert variant="error">
          {apiErrorMessage(verzeichnis.error, {
            fallback: "Die Anlagen konnten nicht geladen werden.",
          })}
        </Alert>
        <MarginNote label="Nächster Schritt">
          Öffnen Sie die Anlage erneut aus dem Verzeichnis. Wenn der Fehler
          bleibt, ist der Eintrag nicht mehr vorhanden.
        </MarginNote>
      </div>
    );
  }

  const anlage = verzeichnis.data?.find((eintrag) => eintrag.anlageId === id);

  if (!anlage) {
    return (
      <div className="space-y-4 p-6">
        <Alert variant="warning">
          Diese Anlage ist nicht im Verzeichnis. Sie wurde womöglich deaktiviert
          oder der Link ist veraltet.
        </Alert>
        <Button variant="secondary" asChild>
          <Link href={paths.anlagen}>Zum Verzeichnis</Link>
        </Button>
      </div>
    );
  }

  return <AnlageBlatt anlage={anlage} />;
}

function AnlageBlatt({ anlage }: { anlage: AnlageÖffentlich }) {
  const phase = useAuth();

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label={anlage.name} />

      <PageHeader
        register={["Logbuch", "Anlagen"]}
        title={anlage.name}
        plate={anlage.anlageId.slice(0, 8)}
        description={anlage.ort}
      />

      <section aria-label="Parcours" className="space-y-2">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">Parcours</h2>
          <SheetNumber>{anlage.parcours.length}</SheetNumber>
        </div>

        {anlage.parcours.length === 0 ? (
          <MarginNote label="Kein Parcours">
            Diese Anlage bietet zurzeit keinen Parcours an. Sobald einer
            freigegeben ist, erscheint er hier.
          </MarginNote>
        ) : (
          <ParcoursLedger parcours={anlage.parcours} phase={phase.status} />
        )}
      </section>

      {phase.status === "anonymous" ? (
        <MarginNote label="Ohne Anmeldung">
          Zum Starten einer Runde{" "}
          <Link
            href="/anmeldung"
            className="text-primary underline-offset-4 hover:underline"
          >
            melden Sie sich an
          </Link>
          . Die Aufstellung können Sie auch ohne Konto lesen.
        </MarginNote>
      ) : null}

      <Perforation />

      <MarginNote label="Anschrift">
        {anlage.ort} · {laender[anlage.ländercode]}
      </MarginNote>
    </div>
  );
}

/**
 * The offered Parcours as a ruled ledger. The last column is the action: each row
 * starts its own Runde, so the pending label belongs to the row that was clicked —
 * one shared `isPending` flag would make every row read "Wird gestartet…".
 */
function ParcoursLedger({
  parcours,
  phase,
}: {
  parcours: ParcoursKurz[];
  phase: AuthPhase["status"];
}) {
  const { rundeStarten } = useStammdatenMutations();
  const router = useRouter();
  const [startend, setStartend] = useState<string | null>(null);

  return (
    <>
      {rundeStarten.isError ? (
        <Alert variant="error" className="mb-2">
          {apiErrorMessage(rundeStarten.error, {
            unauthorized: "Zum Starten einer Runde ist eine Anmeldung nötig.",
            fallback: "Die Runde konnte nicht gestartet werden.",
          })}
        </Alert>
      ) : null}

      <Table>
        <TableHead>
          <TableRow>
            <TableHeader>Name</TableHeader>
            <TableHeader className="text-right">Ziele</TableHeader>
            <TableHeader className="text-right">Pflöcke</TableHeader>
            <TableHeader>
              <span className="sr-only">
                {phase === "authenticated" ? "Runde starten" : "Anmeldung"}
              </span>
            </TableHeader>
          </TableRow>
        </TableHead>
        <TableBody>
          {parcours.map((eintrag) => {
            const laeuft = startend === eintrag.id;
            return (
              <TableRow key={eintrag.id}>
                <TableCell className="font-medium">{eintrag.name}</TableCell>
                <TableCell className="text-right tabular-nums">
                  {eintrag.anzahlZiele}
                </TableCell>
                <TableCell className="text-right tabular-nums">
                  {eintrag.anzahlPflöcke}
                </TableCell>
                <TableCell className="text-right">
                  {phase === "authenticated" ? (
                    <Button
                      size="sm"
                      disabled={rundeStarten.isPending}
                      onClick={() => {
                        setStartend(eintrag.id);
                        rundeStarten.mutate(eintrag.id, {
                          onSuccess: (runde) => {
                            router.push(
                              `${paths.runden}/detail?id=${runde.id}`,
                            );
                          },
                          onError: () => setStartend(null),
                        });
                      }}
                    >
                      <Play />
                      {laeuft ? "Wird gestartet…" : "Runde starten"}
                    </Button>
                  ) : phase === "anonymous" ? (
                    <Button variant="secondary" size="sm" asChild>
                      <Link href="/anmeldung">Anmelden</Link>
                    </Button>
                  ) : (
                    // The session is still being checked; the action is not decided yet.
                    <Skeleton className="ml-auto h-8 w-28" />
                  )}
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </>
  );
}

/** A printed placeholder: the head, then a ledger, before the Anlage arrives. */
function AnlageSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6 p-6">
      <div className="space-y-3">
        <Skeleton className="h-3 w-32" />
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-4 w-40" />
        <Skeleton className="h-px w-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-5 w-24" />
        <Skeleton className="h-9 w-full" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
    </div>
  );
}
