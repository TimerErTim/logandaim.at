"use client";

import { Plus } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useAnlagenQuery, useBenutzerQuery } from "@/api/hooks";
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

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label="Anlagen" />

      <header className="space-y-2">
        <p className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
          Verwaltung
        </p>
        <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
          Anlagen
        </h1>
        <div className="flex flex-wrap items-end justify-between gap-4">
          <p className="max-w-2xl text-sm text-secondary">
            Stammdaten der Anlagen, für die Sie zuständig sind. Deaktivierte
            Anlagen bleiben lesbar, nehmen aber keine Änderung mehr an.
          </p>
          {istSystemadministrator ? (
            <Button asChild>
              <Link href={paths.verwaltungAnlageNeu}>
                <Plus />
                Anlage anlegen
              </Link>
            </Button>
          ) : null}
        </div>
      </header>

      {anlagen.isError ? (
        <Alert variant="error">
          {apiErrorMessage(anlagen.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Anlagen konnten nicht geladen werden.",
          })}
        </Alert>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle>Ihre Anlagen</CardTitle>
        </CardHeader>
        <CardContent>
          {anlagen.isLoading ? (
            <p className="text-sm text-secondary">Anlagen werden geladen …</p>
          ) : (anlagen.data?.length ?? 0) === 0 ? (
            <p className="text-sm text-secondary">
              Ihnen ist noch keine Anlage zugewiesen.
            </p>
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
        </CardContent>
      </Card>
    </div>
  );
}
