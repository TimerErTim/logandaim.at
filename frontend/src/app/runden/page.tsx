"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useRundenQuery } from "@/api/hooks";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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
 * The caller's own Runden. Another Benutzer's Runden are never listed, not even for an
 * Anlage's staff (AC-13); a Systemadministrator reads one by identity alone.
 */
export default function RundenPage() {
  const phase = useAuth();
  const runden = useRundenQuery();
  const router = useRouter();

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label="Runden" />

      <header className="space-y-2">
        <p className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
          Logbuch
        </p>
        <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
          Ihre Runden
        </h1>
        <p className="text-sm text-secondary">
          Jede Runde hält die Aufstellung fest, mit der sie begonnen wurde.
          Spätere Änderungen an den Stammdaten verändern sie nicht.
        </p>
      </header>

      {runden.isError ? (
        <Alert variant="error">
          {apiErrorMessage(runden.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Runden konnten nicht geladen werden.",
          })}
        </Alert>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle>Übersicht</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {runden.isLoading ? (
            <p className="text-sm text-secondary">Runden werden geladen …</p>
          ) : (runden.data?.length ?? 0) === 0 ? (
            <p className="text-sm text-secondary">
              Noch keine Runde begonnen. Eine Runde starten Sie auf der
              Parcours-Seite.
            </p>
          ) : (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Parcours</TableHeader>
                  <TableHeader>Begonnen</TableHeader>
                  <TableHeader>Status</TableHeader>
                  <TableHeader>
                    <span className="sr-only">Öffnen</span>
                  </TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {runden.data?.map((runde) => (
                  <TableRow key={runde.id}>
                    <TableCell className="font-medium">
                      {runde.parcoursName}
                    </TableCell>
                    <TableCell className="tabular-nums text-secondary">
                      {new Date(runde.begonnenAm).toLocaleString("de-AT")}
                    </TableCell>
                    <TableCell>
                      <Badge
                        variant={
                          runde.zustand === "Abgeschlossen"
                            ? "default"
                            : "primary"
                        }
                      >
                        {runde.zustand === "Abgeschlossen"
                          ? "Abgeschlossen"
                          : "In Bearbeitung"}
                      </Badge>
                    </TableCell>
                    <TableCell className="text-right">
                      <Button variant="secondary" size="sm" asChild>
                        <Link href={`${paths.runden}/detail?id=${runde.id}`}>
                          Öffnen
                        </Link>
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}

          <Button variant="secondary" asChild>
            <Link href={paths.parcours}>Neue Runde starten</Link>
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
