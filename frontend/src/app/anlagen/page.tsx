"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useAnlagenVerzeichnisQuery } from "@/api/hooks";
import { PageHeader } from "@/components/layout/page-header";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import { HatchBand, MarginNote, SheetNumber } from "@/components/ui/paper-rule";
import { SearchInput } from "@/components/ui/search-input";
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
 * The archer's first screen: every active Anlage with the number of Parkours it offers.
 *
 * Public by design — the directory answers without a session, so the page has no auth
 * guard and no `enabled` gate on its read. Filtering is client-side over the whole
 * active set and mirrored into `?q=`, so a filtered view can be linked or reloaded.
 */
export default function AnlagenPage() {
  const phase = useAuth();
  const verzeichnis = useAnlagenVerzeichnisQuery();
  const q = useQueryParam("q");

  const [suche, setSuche] = useState("");

  // The parameter is read on the client after mount, and again on back/forward. Writing
  // it below uses `replaceState`, which fires no `popstate`, so this cannot fight what
  // the field is typing; a URL without `?q=` empties the field again.
  useEffect(() => {
    setSuche(q ?? "");
  }, [q]);

  const aendereSuche = (wert: string) => {
    setSuche(wert);

    const url = new URL(window.location.href);
    if (wert.trim()) {
      url.searchParams.set("q", wert);
    } else {
      url.searchParams.delete("q");
    }
    // `replaceState`: a search filter is a view of the page, not a new entry in the
    // logbook. It also keeps the static export's route untouched.
    window.history.replaceState(null, "", url);
  };

  const alle = verzeichnis.data ?? [];
  const nadel = suche.trim().toLocaleLowerCase("de-DE");
  const treffer = nadel
    ? alle.filter(
        (anlage) =>
          anlage.name.toLocaleLowerCase("de-DE").includes(nadel) ||
          anlage.ort.toLocaleLowerCase("de-DE").includes(nadel),
      )
    : alle;

  return (
    <div className="space-y-6 p-6">
      <PageHeader
        register={["Logbuch", "Anlagen"]}
        title="Anlagen"
        description="Die Anlagen des Logbuchs. Wählen Sie eine Anlage, um ihre Parkours zu sehen und eine Runde zu starten."
      />

      {verzeichnis.isError ? (
        <Alert variant="error">
          {apiErrorMessage(verzeichnis.error, {
            fallback: "Die Anlagen konnten nicht geladen werden.",
          })}
        </Alert>
      ) : null}

      {verzeichnis.isLoading ? (
        <VerzeichnisSkeleton />
      ) : alle.length === 0 ? (
        <MarginNote label="Kein Eintrag">
          Zurzeit ist keine Anlage freigegeben. Sobald eine Anlage aktiv ist,
          erscheint sie hier.
        </MarginNote>
      ) : (
        <>
          <div className="flex flex-wrap items-end justify-between gap-3">
            <div className="w-full max-w-xs">
              <Field label="Suche">
                <SearchInput
                  placeholder="Name oder Ort"
                  value={suche}
                  onChange={(event) => aendereSuche(event.target.value)}
                />
              </Field>
            </div>
            <SheetNumber>
              {treffer.length} von {alle.length}
            </SheetNumber>
          </div>

          {treffer.length === 0 ? (
            <div className="space-y-4">
              <MarginNote label="Keine Treffer">
                Keine Anlage passt zu „{suche}“. Prüfen Sie den Namen oder den
                Ort.
              </MarginNote>
              <Button variant="secondary" onClick={() => aendereSuche("")}>
                Suche zurücksetzen
              </Button>
            </div>
          ) : (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Anlage</TableHeader>
                  <TableHeader>Ort</TableHeader>
                  <TableHeader className="text-right">Parkour</TableHeader>
                  <TableHeader>
                    <span className="sr-only">Öffnen</span>
                  </TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {treffer.map((anlage) => {
                  const href = `${paths.anlagenDetail}?id=${anlage.anlageId}`;
                  return (
                    <TableRow key={anlage.anlageId}>
                      <TableCell className="font-medium">
                        <Link
                          href={href}
                          className="underline-offset-4 hover:underline"
                        >
                          {anlage.name}
                        </Link>
                      </TableCell>
                      <TableCell className="text-secondary">
                        {anlage.ort}
                      </TableCell>
                      <TableCell className="text-right tabular-nums">
                        {anlage.parkour.length}
                      </TableCell>
                      <TableCell className="text-right">
                        <Button variant="secondary" size="sm" asChild>
                          <Link href={href}>Öffnen</Link>
                        </Button>
                      </TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          )}
        </>
      )}

      {phase.status === "anonymous" ? (
        <>
          <HatchBand />
          <MarginNote label="Ohne Anmeldung">
            Das Verzeichnis ist öffentlich. Eine Runde starten Sie nach der{" "}
            <Link
              href="/anmeldung"
              className="text-primary underline-offset-4 hover:underline"
            >
              Anmeldung
            </Link>
            ; Ihre eigenen Runden stehen danach in der Historie.
          </MarginNote>
        </>
      ) : null}
    </div>
  );
}

/** The ledger's shape, printed while the directory arrives, so the page does not jump. */
function VerzeichnisSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6">
      <div className="space-y-2">
        <Skeleton className="h-4 w-48" />
        <Skeleton className="h-11 w-full max-w-xs" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-9 w-full" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
    </div>
  );
}
