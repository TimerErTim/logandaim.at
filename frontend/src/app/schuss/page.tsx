"use client";

import { useState } from "react";
import { useSchussRegistrierenMutation, useWertungMutation } from "@/api/hooks";
import { type Schuss, type Wertung, Wertungszone } from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { Alert } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

const zonen: Wertungszone[] = [
  Wertungszone.Spot,
  Wertungszone.Kill,
  Wertungszone.Body,
  Wertungszone.Miss,
];

/**
 * Reference slice UI: log Schüsse at one Ziel and summarize their Wertung.
 *
 * The list below is a user *selection* (ids the user just created), not a copy of
 * server state — that is why it lives in component state while everything else goes
 * through TanStack Query hooks.
 */
export default function SchussPage() {
  const [zielId, setZielId] = useState(() => crypto.randomUUID());
  const [pfeil, setPfeil] = useState(1);
  const [zone, setZone] = useState<Wertungszone>(Wertungszone.Kill);
  const [schüsse, setSchüsse] = useState<Schuss[]>([]);
  const [wertung, setWertung] = useState<Wertung | null>(null);

  const registrieren = useSchussRegistrierenMutation();
  const berechnen = useWertungMutation();

  function onRegistrieren(event: React.FormEvent) {
    event.preventDefault();
    registrieren.mutate(
      { zielId, pfeil, wertungszone: zone },
      {
        onSuccess: (schuss) => {
          setSchüsse((bisher) => [...bisher, schuss]);
          setPfeil((bisher) => bisher + 1);
          setWertung(null);
        },
      },
    );
  }

  function onBerechnen() {
    berechnen.mutate(
      schüsse.map((schuss) => schuss.id),
      { onSuccess: setWertung },
    );
  }

  const fehler = registrieren.error ?? berechnen.error;

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label="Schuss" />

      <Card>
        <CardHeader>
          <CardTitle>Schuss erfassen</CardTitle>
          <CardDescription>
            Jeden Pfeil an einem Ziel registrieren. Der erste Treffer und der
            Mehrschuss werden daraus berechnet.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form
            className="grid gap-4 sm:grid-cols-4 sm:items-end"
            onSubmit={onRegistrieren}
          >
            <Field label="Ziel" hint="Kennung des Ziels">
              <Input
                value={zielId}
                onChange={(event) => setZielId(event.target.value)}
                required
              />
            </Field>
            <Field label="Pfeil" hint="1-basiert">
              <Input
                type="number"
                min={1}
                value={pfeil}
                onChange={(event) => setPfeil(Number(event.target.value))}
                required
              />
            </Field>
            <Field label="Wertungszone">
              <Select
                value={zone}
                onChange={(event) =>
                  setZone(event.target.value as Wertungszone)
                }
              >
                {zonen.map((wert) => (
                  <option key={wert} value={wert}>
                    {wert}
                  </option>
                ))}
              </Select>
            </Field>
            <Button type="submit" disabled={registrieren.isPending}>
              {registrieren.isPending ? "Wird erfasst…" : "Schuss erfassen"}
            </Button>
          </form>
        </CardContent>
      </Card>

      {fehler ? <Alert variant="error">{String(fehler)}</Alert> : null}

      <Card>
        <CardHeader>
          <CardTitle>Erfasste Schüsse</CardTitle>
          <CardDescription>
            Auswahl der Schüsse, für die die Wertung berechnet wird.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {schüsse.length === 0 ? (
            <p className="text-sm text-muted-foreground">
              Noch kein Schuss erfasst.
            </p>
          ) : (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Pfeil</TableHeader>
                  <TableHeader>Zone</TableHeader>
                  <TableHeader>Ziel</TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {schüsse.map((schuss) => (
                  <TableRow key={schuss.id}>
                    <TableCell>{schuss.pfeil}</TableCell>
                    <TableCell>{schuss.wertungszone}</TableCell>
                    <TableCell className="font-mono text-xs">
                      {schuss.zielId}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}

          <div className="flex items-center gap-3">
            <Button
              variant="secondary"
              onClick={onBerechnen}
              disabled={schüsse.length === 0 || berechnen.isPending}
            >
              {berechnen.isPending ? "Wird berechnet…" : "Wertung berechnen"}
            </Button>
            {schüsse.length > 0 ? (
              <Button
                variant="ghost"
                onClick={() => {
                  setSchüsse([]);
                  setWertung(null);
                }}
              >
                Auswahl leeren
              </Button>
            ) : null}
          </div>

          {wertung ? (
            <dl className="grid gap-3 text-sm sm:grid-cols-3">
              <div>
                <dt className="font-medium text-muted-foreground">
                  Erster Treffer
                </dt>
                <dd className="mt-0.5 text-lg font-semibold">
                  {wertung.ersterTreffer}
                </dd>
              </div>
              <div>
                <dt className="font-medium text-muted-foreground">
                  Mehrschuss
                </dt>
                <dd className="mt-0.5 text-lg font-semibold">
                  {wertung.mehrschuss}
                </dd>
              </div>
              <div>
                <dt className="font-medium text-muted-foreground">
                  Erster Pfeil getroffen
                </dt>
                <dd className="mt-0.5 text-lg font-semibold">
                  {wertung.ersterPfeilGetroffen ? "Ja" : "Nein"}
                </dd>
              </div>
            </dl>
          ) : null}
        </CardContent>
      </Card>
    </div>
  );
}
