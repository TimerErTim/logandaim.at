"use client";

import { Plus } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import {
  useAnlagenQuery,
  useBenutzerQuery,
  useStammdatenMutations,
} from "@/api/hooks";
import type { AnlageErstellungInput } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
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

const leer: AnlageErstellungInput = {
  name: "",
  adresse: { straße: "", postleitzahl: "", ort: "", ländercode: "AT" },
  kontakt: {},
};

/**
 * The admin entry page: the Anlagen the caller may administer, plus the create form a
 * Systemadministrator alone may use (AC-1). One primary action: creating an Anlage.
 */
export default function VerwaltungPage() {
  const phase = useAuth();
  const anlagen = useAnlagenQuery();
  const benutzer = useBenutzerQuery();
  const { anlageErstellen } = useStammdatenMutations();
  const router = useRouter();
  const [form, setForm] = useState<AnlageErstellungInput>(leer);

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  const istSystemadministrator = benutzer.data?.rolle === "Systemadministrator";

  function absenden(event: React.FormEvent) {
    event.preventDefault();
    anlageErstellen.mutate(form, {
      onSuccess: () => setForm(leer),
    });
  }

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
        <p className="text-sm text-secondary">
          Stammdaten der Anlagen, für die Sie zuständig sind. Deaktivierte
          Anlagen bleiben lesbar, nehmen aber keine Änderung mehr an.
        </p>
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

      {istSystemadministrator ? (
        <>
          <Separator />
          <Card>
            <CardHeader>
              <CardTitle>Neue Anlage</CardTitle>
            </CardHeader>
            <CardContent>
              <form className="space-y-4" onSubmit={absenden}>
                <div className="grid gap-4 sm:grid-cols-2">
                  <Field label="Name">
                    <Input
                      value={form.name}
                      onChange={(event) =>
                        setForm((bisher) => ({
                          ...bisher,
                          name: event.target.value,
                        }))
                      }
                      required
                    />
                  </Field>
                  <Field label="Straße">
                    <Input
                      value={form.adresse.straße}
                      onChange={(event) =>
                        setForm((bisher) => ({
                          ...bisher,
                          adresse: {
                            ...bisher.adresse,
                            straße: event.target.value,
                          },
                        }))
                      }
                      required
                    />
                  </Field>
                  <Field label="Postleitzahl">
                    <Input
                      value={form.adresse.postleitzahl}
                      onChange={(event) =>
                        setForm((bisher) => ({
                          ...bisher,
                          adresse: {
                            ...bisher.adresse,
                            postleitzahl: event.target.value,
                          },
                        }))
                      }
                      required
                    />
                  </Field>
                  <Field label="Ort">
                    <Input
                      value={form.adresse.ort}
                      onChange={(event) =>
                        setForm((bisher) => ({
                          ...bisher,
                          adresse: {
                            ...bisher.adresse,
                            ort: event.target.value,
                          },
                        }))
                      }
                      required
                    />
                  </Field>
                  <Field label="Ländercode" hint="AT, DE oder CH">
                    <Input
                      value={form.adresse.ländercode}
                      onChange={(event) =>
                        setForm((bisher) => ({
                          ...bisher,
                          adresse: {
                            ...bisher.adresse,
                            ländercode: event.target.value.toUpperCase(),
                          },
                        }))
                      }
                      maxLength={2}
                      required
                    />
                  </Field>
                  <Field label="Verein" optional>
                    <Input
                      value={form.kontakt.name ?? ""}
                      onChange={(event) =>
                        setForm((bisher) => ({
                          ...bisher,
                          kontakt: {
                            ...bisher.kontakt,
                            name: event.target.value,
                          },
                        }))
                      }
                    />
                  </Field>
                </div>

                {anlageErstellen.isError ? (
                  <Alert variant="error">
                    {apiErrorMessage(anlageErstellen.error, {
                      badRequest: "Bitte prüfen Sie die Eingaben.",
                      fallback: "Die Anlage konnte nicht angelegt werden.",
                    })}
                  </Alert>
                ) : null}

                <Button type="submit" disabled={anlageErstellen.isPending}>
                  <Plus />
                  {anlageErstellen.isPending
                    ? "Wird angelegt…"
                    : "Anlage anlegen"}
                </Button>
              </form>
            </CardContent>
          </Card>
        </>
      ) : null}
    </div>
  );
}
