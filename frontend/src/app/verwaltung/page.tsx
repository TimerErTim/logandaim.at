"use client";

import { Plus } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useAnlagenQuery, useBenutzerQuery } from "@/api/hooks";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { MarginNote } from "@/components/ui/paper-rule";
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

/** The admin entry page lists Anlagen; creation is a separate administrator-only journey. */
export default function VerwaltungPage() {
  const phase = useAuth();
  const anlagen = useAnlagenQuery();
  const benutzer = useBenutzerQuery();
  const router = useRouter();

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  const istSystemadministrator = benutzer.data?.rolle === "Systemadministrator";
  const leer = !anlagen.isLoading && (anlagen.data?.length ?? 0) === 0;

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label="Anlagen" />

      <PageHeader
        register={["Verwaltung", "Anlagen"]}
        title="Anlagen"
        description="Stammdaten der Anlagen, für die Sie zuständig sind. Deaktivierte Anlagen sind nur für Systemadministratoren einsehbar und nehmen keine Änderung mehr an."
        actions={
          istSystemadministrator ? (
            // Exactly one primary action per screen: while the ledger is empty the
            // empty state carries it, so the header action steps back to secondary.
            <Button variant={leer ? "secondary" : "primary"} asChild>
              <Link href={paths.verwaltungAnlageNeu}>
                <Plus />
                Anlage anlegen
              </Link>
            </Button>
          ) : null
        }
      />

      {anlagen.isError ? (
        <Alert variant="error">
          {apiErrorMessage(anlagen.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Anlagen konnten nicht geladen werden.",
          })}
        </Alert>
      ) : null}

      {anlagen.isLoading ? (
        <AnlagenSkeleton />
      ) : (anlagen.data?.length ?? 0) === 0 ? (
        <Leerzustand istSystemadministrator={istSystemadministrator} />
      ) : (
        <Table>
          <TableHead>
            <TableRow>
              <TableHeader>Name</TableHeader>
              <TableHeader>Ort</TableHeader>
              <TableHeader>Ihre Rolle</TableHeader>
              <TableHeader>Status</TableHeader>
              <TableHeader>
                <span className="sr-only">Öffnen</span>
              </TableHeader>
            </TableRow>
          </TableHead>
          <TableBody>
            {anlagen.data?.map((zeile) => (
              <TableRow key={zeile.anlage.id}>
                <TableCell className="font-medium">
                  {zeile.anlage.name}
                </TableCell>
                <TableCell className="text-secondary">
                  {zeile.anlage.adresse.ort}
                </TableCell>
                <TableCell>
                  {zeile.istSystemadministrator ? (
                    <Badge variant="primary">Systemadministrator</Badge>
                  ) : zeile.rolle === "Administrator" ? (
                    <Badge>Administrator</Badge>
                  ) : (
                    <Badge>Mitarbeiter</Badge>
                  )}
                </TableCell>
                <TableCell>
                  <Badge
                    variant={
                      zeile.anlage.zustand === "Archiviert"
                        ? "warning"
                        : "success"
                    }
                  >
                    {zeile.anlage.zustand === "Archiviert"
                      ? "Deaktiviert"
                      : "Aktiv"}
                  </Badge>
                </TableCell>
                <TableCell className="text-right">
                  <Button variant="secondary" size="sm" asChild>
                    <Link
                      href={`${paths.verwaltungAnlage}?id=${zeile.anlage.id}`}
                    >
                      Öffnen
                    </Link>
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}

/** The ledger's shape, printed while the list arrives, so the page does not jump. */
function AnlagenSkeleton() {
  return (
    <div aria-busy="true" className="space-y-2">
      <Skeleton className="h-9 w-full" />
      <Skeleton className="h-12 w-full" />
      <Skeleton className="h-12 w-full" />
      <Skeleton className="h-12 w-full" />
    </div>
  );
}

/**
 * The empty state names the next action. A Systemadministrator creates the missing
 * Anlage with the header's primary action; everyone else has no way to give themselves
 * one, so the note names who can.
 */
function Leerzustand({
  istSystemadministrator,
}: {
  istSystemadministrator: boolean;
}) {
  if (istSystemadministrator) {
    return (
      <div className="space-y-4">
        <MarginNote label="Noch keine Anlage">
          Es ist noch keine Anlage angelegt. Legen Sie die erste Anlage an; sie
          erscheint danach als Blatt in dieser Liste.
        </MarginNote>
        <Button asChild>
          <Link href={paths.verwaltungAnlageNeu}>
            <Plus />
            Anlage anlegen
          </Link>
        </Button>
      </div>
    );
  }

  return (
    <MarginNote label="Keine Anlage zugewiesen">
      Ihnen ist noch keine Anlage zugewiesen. Wenden Sie sich an einen
      Systemadministrator, wenn Sie eine Anlage verwalten sollen.
    </MarginNote>
  );
}
