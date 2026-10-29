"use client";

import { Plus } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useBenutzerQuery, useStammdatenMutations } from "@/api/hooks";
import { type AnlageErstellungInput, Ländercode } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { paths } from "@/lib/navigation";

const leer: AnlageErstellungInput = {
  name: "",
  adresse: {
    straße: "",
    postleitzahl: "",
    ort: "",
    ländercode: Ländercode.AT,
  },
  kontakt: {},
};

const laender: Array<{ code: Ländercode; name: string }> = [
  { code: Ländercode.AT, name: "Österreich" },
  { code: Ländercode.DE, name: "Deutschland" },
  { code: Ländercode.CH, name: "Schweiz" },
];

export default function AnlageNeuPage() {
  const phase = useAuth();
  const benutzer = useBenutzerQuery();
  const router = useRouter();
  const { anlageErstellen } = useStammdatenMutations();
  const [form, setForm] = useState<AnlageErstellungInput>(leer);

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  if (phase.status === "checking" || benutzer.isLoading) {
    return (
      <div className="p-6 text-sm text-secondary">
        Berechtigung wird geprüft …
      </div>
    );
  }

  if (benutzer.data?.rolle !== "Systemadministrator") {
    return (
      <div className="space-y-4 p-6">
        <Alert variant="warning">
          Nur Systemadministratoren können eine Anlage anlegen.
        </Alert>
        <Button variant="secondary" asChild>
          <Link href={paths.verwaltung}>Zur Anlagenübersicht</Link>
        </Button>
      </div>
    );
  }

  function absenden(event: React.FormEvent) {
    event.preventDefault();
    anlageErstellen.mutate(form, {
      onSuccess: (anlage) =>
        router.push(`${paths.verwaltungAnlage}?id=${anlage.id}`),
    });
  }

  return (
    <div className="mx-auto max-w-4xl space-y-6 p-6">
      <SetBreadcrumbLabel label="Anlage anlegen" />
      <header className="space-y-2">
        <p className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
          Verwaltung · Neue Stammdaten
        </p>
        <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
          Anlage anlegen
        </h1>
        <p className="max-w-2xl text-sm text-secondary">
          Erfassen Sie Standort und optionale Kontaktdaten. Die Anlage wird
          aktiv angelegt; eine Verwaltung kann später zugewiesen werden.
        </p>
      </header>

      <form className="space-y-6" onSubmit={absenden}>
        <Card>
          <CardHeader>
            <CardTitle>Standort</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4 sm:grid-cols-2">
            <Field label="Anlagenname" className="sm:col-span-2">
              <Input
                value={form.name}
                onChange={(event) =>
                  setForm((old) => ({ ...old, name: event.target.value }))
                }
                required
              />
            </Field>
            <Field label="Straße und Hausnummer">
              <Input
                value={form.adresse.straße}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    adresse: { ...old.adresse, straße: event.target.value },
                  }))
                }
                required
              />
            </Field>
            <Field label="Postleitzahl">
              <Input
                value={form.adresse.postleitzahl}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    adresse: {
                      ...old.adresse,
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
                  setForm((old) => ({
                    ...old,
                    adresse: { ...old.adresse, ort: event.target.value },
                  }))
                }
                required
              />
            </Field>
            <Field label="Land">
              <Select
                value={form.adresse.ländercode}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    adresse: {
                      ...old.adresse,
                      ländercode: event.target.value as Ländercode,
                    },
                  }))
                }
                required
              >
                {laender.map(({ code, name }) => (
                  <option key={code} value={code}>
                    {name}
                  </option>
                ))}
              </Select>
            </Field>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Kontaktdaten</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4 sm:grid-cols-2">
            <Field label="Verein oder Betreiber" optional>
              <Input
                value={form.kontakt.name ?? ""}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    kontakt: {
                      ...old.kontakt,
                      name: event.target.value || undefined,
                    },
                  }))
                }
              />
            </Field>
            <Field label="E-Mail-Adresse" optional>
              <Input
                type="email"
                value={form.kontakt.email ?? ""}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    kontakt: {
                      ...old.kontakt,
                      email: event.target.value || undefined,
                    },
                  }))
                }
              />
            </Field>
            <Field label="Telefon" optional>
              <Input
                type="tel"
                value={form.kontakt.telefon ?? ""}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    kontakt: {
                      ...old.kontakt,
                      telefon: event.target.value || undefined,
                    },
                  }))
                }
              />
            </Field>
            <Field label="Website" optional>
              <Input
                type="url"
                value={form.kontakt.website ?? ""}
                onChange={(event) =>
                  setForm((old) => ({
                    ...old,
                    kontakt: {
                      ...old.kontakt,
                      website: event.target.value || undefined,
                    },
                  }))
                }
              />
            </Field>
          </CardContent>
        </Card>

        {anlageErstellen.isError ? (
          <Alert variant="error">
            {apiErrorMessage(anlageErstellen.error, {
              badRequest: "Bitte prüfen Sie die Eingaben.",
              fallback: "Die Anlage konnte nicht angelegt werden.",
            })}
          </Alert>
        ) : null}

        <Separator />
        <div className="flex flex-wrap gap-3">
          <Button type="submit" disabled={anlageErstellen.isPending}>
            <Plus />
            {anlageErstellen.isPending ? "Wird angelegt…" : "Anlage anlegen"}
          </Button>
          <Button variant="secondary" asChild>
            <Link href={paths.verwaltung}>Abbrechen</Link>
          </Button>
        </div>
      </form>
    </div>
  );
}
