"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useAnlageQuery, useStammdatenMutations } from "@/api/hooks";
import {
  type Anlage,
  AnlageVerwaltungRolle,
  type ZielInput,
} from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PflockMarker } from "@/components/stammdaten/pflock-swatch";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { paths } from "@/lib/navigation";
import { useQueryParam } from "@/lib/use-query-param";

/**
 * One Anlage's master data. Everything a Mitarbeiter may do is here; the destructive
 * and structural actions additionally ask for an Administrator's confirmation by
 * simply being refused by the API when the role is not enough.
 */
export default function AnlagePage() {
  const phase = useAuth();
  const router = useRouter();
  const id = useQueryParam("id");
  const anlage = useAnlageQuery(id);

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  if (id === null) {
    return (
      <div className="p-6">
        <p className="text-sm text-secondary">Anlage wird geladen …</p>
      </div>
    );
  }

  if (!id) {
    return (
      <div className="p-6">
        <Alert variant="warning">
          Es wurde keine Anlage ausgewählt. Bitte öffnen Sie sie aus der
          Übersicht.
        </Alert>
      </div>
    );
  }

  if (anlage.isLoading) {
    return (
      <div className="p-6">
        <p className="text-sm text-secondary">Anlage wird geladen …</p>
      </div>
    );
  }

  if (anlage.isError || !anlage.data) {
    return (
      <div className="p-6">
        <Alert variant="error">
          {apiErrorMessage(anlage.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Die Anlage konnte nicht geladen werden.",
          })}
        </Alert>
      </div>
    );
  }

  return <AnlageInhalt anlage={anlage.data} />;
}

function AnlageInhalt({ anlage }: { anlage: Anlage }) {
  const mutations = useStammdatenMutations();
  const deaktiviert = anlage.zustand === "Archiviert";

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label={anlage.name} />

      <header className="space-y-2">
        <div className="flex flex-wrap items-center gap-3">
          <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
            {anlage.name}
          </h1>
          <Badge variant={deaktiviert ? "warning" : "success"}>
            {deaktiviert ? "Deaktiviert" : "Aktiv"}
          </Badge>
        </div>
        <p className="text-sm text-secondary">
          {anlage.adresse.straße}, {anlage.adresse.postleitzahl}{" "}
          {anlage.adresse.ort} ({anlage.adresse.ländercode})
        </p>
      </header>

      {deaktiviert ? (
        <Alert variant="warning">
          Diese Anlage ist deaktiviert. Alle Stammdaten bleiben lesbar,
          Änderungen werden jedoch abgelehnt. Eine Reaktivierung ist nicht
          vorgesehen.
        </Alert>
      ) : null}

      <Tabs defaultValue="ziele">
        <TabsList>
          <TabsTrigger value="ziele">Ziele</TabsTrigger>
          <TabsTrigger value="parcours">Parcours</TabsTrigger>
          <TabsTrigger value="verwaltung">Verwaltung</TabsTrigger>
          <TabsTrigger value="stammdaten">Stammdaten</TabsTrigger>
        </TabsList>

        <TabsContent value="ziele">
          <Ziele anlage={anlage} gesperrt={deaktiviert} />
        </TabsContent>

        <TabsContent value="parcours">
          <ParcoursListe anlage={anlage} />
        </TabsContent>

        <TabsContent value="verwaltung">
          <Verwaltungen anlage={anlage} gesperrt={deaktiviert} />
        </TabsContent>

        <TabsContent value="stammdaten">
          <Stammdaten
            anlage={anlage}
            gesperrt={deaktiviert}
            onDeaktivieren={() =>
              mutations.anlageDeaktivieren.mutate(anlage.id)
            }
            laeuft={mutations.anlageDeaktivieren.isPending}
            fehler={mutations.anlageDeaktivieren.error}
          />
        </TabsContent>
      </Tabs>
    </div>
  );
}

// --------------------------------------------------------------------------- Ziele

const leeresZiel: ZielInput = { tiername: "", zielgruppe: 1 };

function Ziele({ anlage, gesperrt }: { anlage: Anlage; gesperrt: boolean }) {
  const { zielHinzufuegen, zielAendern, zielErsetzen, zielEntfernen } =
    useStammdatenMutations();
  const [form, setForm] = useState<ZielInput>(leeresZiel);
  const [bearbeitet, setBearbeitet] = useState<string | null>(null);

  const fehler =
    zielHinzufuegen.error ?? zielAendern.error ?? zielEntfernen.error;

  const aktive = anlage.ziele.filter((ziel) => ziel.zustand === "Aktiv");
  const archivierte = anlage.ziele.filter(
    (ziel) => ziel.zustand === "Archiviert",
  );

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Ziele der Anlage</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {anlage.ziele.length === 0 ? (
            <p className="text-sm text-secondary">Noch kein Ziel erfasst.</p>
          ) : (
            <Table>
              <TableHead>
                <TableRow>
                  <TableHeader>Tiername</TableHeader>
                  <TableHeader>Zielgruppe</TableHeader>
                  <TableHeader>Verwendung</TableHeader>
                  <TableHeader>Status</TableHeader>
                  <TableHeader>
                    <span className="sr-only">Aktionen</span>
                  </TableHeader>
                </TableRow>
              </TableHead>
              <TableBody>
                {[...aktive, ...archivierte].map((ziel) => (
                  <TableRow key={ziel.id}>
                    <TableCell className="font-medium">
                      {ziel.tiername}
                    </TableCell>
                    <TableCell className="tabular-nums">
                      {ziel.zielgruppe}
                    </TableCell>
                    <TableCell className="text-secondary">
                      {ziel.verwendungen.length === 0
                        ? "nicht platziert"
                        : ziel.verwendungen
                            .map((verwendung) => verwendung.parcoursName)
                            .join(", ")}
                    </TableCell>
                    <TableCell>
                      <Badge
                        variant={
                          ziel.zustand === "Archiviert" ? "warning" : "success"
                        }
                      >
                        {ziel.zustand === "Archiviert" ? "Archiviert" : "Aktiv"}
                      </Badge>
                    </TableCell>
                    <TableCell className="text-right">
                      {ziel.zustand === "Aktiv" && !gesperrt ? (
                        <div className="flex justify-end gap-2">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => {
                              setBearbeitet(ziel.id);
                              setForm({
                                tiername: ziel.tiername,
                                zielgruppe: ziel.zielgruppe,
                              });
                            }}
                          >
                            Bearbeiten
                          </Button>
                          <Button
                            variant="secondary"
                            size="sm"
                            disabled={zielErsetzen.isPending}
                            onClick={() =>
                              zielErsetzen.mutate([
                                anlage.id,
                                ziel.id,
                                { ...ziel, zielgruppe: ziel.zielgruppe },
                              ])
                            }
                          >
                            Ersetzen
                          </Button>
                          <Button
                            variant="destructive"
                            size="sm"
                            disabled={zielEntfernen.isPending}
                            onClick={() =>
                              zielEntfernen.mutate([anlage.id, ziel.id])
                            }
                          >
                            Entfernen
                          </Button>
                        </div>
                      ) : (
                        <span className="text-xs text-secondary">
                          unveränderlich
                        </span>
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}

          {fehler ? (
            <Alert variant="error">
              {apiErrorMessage(fehler, {
                badRequest: "Bitte prüfen Sie die Eingaben.",
                fallback: "Die Änderung konnte nicht gespeichert werden.",
              })}
            </Alert>
          ) : null}
        </CardContent>
      </Card>

      {gesperrt ? null : (
        <Card>
          <CardHeader>
            <CardTitle>
              {bearbeitet ? "Ziel bearbeiten" : "Ziel hinzufügen"}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <form
              className="grid items-end gap-4 sm:grid-cols-3"
              onSubmit={(event) => {
                event.preventDefault();
                if (bearbeitet) {
                  zielAendern.mutate([anlage.id, bearbeitet, form], {
                    onSuccess: () => setBearbeitet(null),
                  });
                } else {
                  zielHinzufuegen.mutate([anlage.id, form], {
                    onSuccess: () => setForm(leeresZiel),
                  });
                }
              }}
            >
              <Field label="Tiername">
                <Input
                  value={form.tiername}
                  onChange={(event) =>
                    setForm((bisher) => ({
                      ...bisher,
                      tiername: event.target.value,
                    }))
                  }
                  required
                />
              </Field>
              <Field label="Zielgruppe" hint="1 bis 4">
                <Select
                  value={String(form.zielgruppe)}
                  onChange={(event) =>
                    setForm((bisher) => ({
                      ...bisher,
                      zielgruppe: Number(event.target.value),
                    }))
                  }
                >
                  {[1, 2, 3, 4].map((wert) => (
                    <option key={wert} value={wert}>
                      {wert}
                    </option>
                  ))}
                </Select>
              </Field>
              <div className="flex gap-2">
                <Button
                  type="submit"
                  disabled={zielHinzufuegen.isPending || zielAendern.isPending}
                >
                  {bearbeitet ? "Speichern" : "Hinzufügen"}
                </Button>
                {bearbeitet ? (
                  <Button
                    variant="ghost"
                    onClick={() => {
                      setBearbeitet(null);
                      setForm(leeresZiel);
                    }}
                  >
                    Abbrechen
                  </Button>
                ) : null}
              </div>
            </form>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

// ------------------------------------------------------------------------ Parcours

function ParcoursListe({ anlage }: { anlage: Anlage }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Parcours der Anlage</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {anlage.parcours.length === 0 ? (
          <p className="text-sm text-secondary">Noch kein Parcours erfasst.</p>
        ) : (
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader>Name</TableHeader>
                <TableHeader>Pflöcke</TableHeader>
                <TableHeader>Platzierungen</TableHeader>
                <TableHeader>Status</TableHeader>
                <TableHeader>
                  <span className="sr-only">Öffnen</span>
                </TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {anlage.parcours.map((parcours) => (
                <TableRow key={parcours.id}>
                  <TableCell className="font-medium">{parcours.name}</TableCell>
                  <TableCell>
                    <span className="flex flex-wrap gap-3 text-sm">
                      {parcours.pflöcke.map((pflock) => (
                        <PflockMarker key={pflock.id} farbe={pflock.farbe} />
                      ))}
                    </span>
                    <span className="sr-only">
                      {parcours.pflöcke
                        .map((pflock) => pflock.bezeichnung)
                        .join(", ")}
                    </span>
                  </TableCell>
                  <TableCell className="tabular-nums">
                    {parcours.platzierungen.length}
                  </TableCell>
                  <TableCell>
                    <Badge
                      variant={
                        parcours.zustand === "Archiviert"
                          ? "warning"
                          : "success"
                      }
                    >
                      {parcours.zustand === "Archiviert"
                        ? "Archiviert"
                        : "Aktiv"}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-right">
                    <Button variant="secondary" size="sm" asChild>
                      <Link
                        href={`${paths.verwaltungParcours}?id=${parcours.id}&anlage=${anlage.id}`}
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
  );
}

// --------------------------------------------------------------------- Verwaltungen

function Verwaltungen({
  anlage,
  gesperrt,
}: {
  anlage: Anlage;
  gesperrt: boolean;
}) {
  const { verwaltungGewaehren, verwaltungEntziehen, verwaltungAendern } =
    useStammdatenMutations();
  const [benutzerId, setBenutzerId] = useState("");
  const [rolle, setRolle] = useState<AnlageVerwaltungRolle>(
    AnlageVerwaltungRolle.Mitarbeiter,
  );

  const fehler =
    verwaltungGewaehren.error ??
    verwaltungEntziehen.error ??
    verwaltungAendern.error;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Verwaltungen</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {anlage.verwaltungen.length === 0 ? (
          <p className="text-sm text-secondary">
            Noch keine Verwaltung vergeben.
          </p>
        ) : (
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader>Benutzer</TableHeader>
                <TableHeader>Rolle</TableHeader>
                <TableHeader>
                  <span className="sr-only">Aktionen</span>
                </TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {anlage.verwaltungen.map((verwaltung) => (
                <TableRow key={verwaltung.benutzerId}>
                  <TableCell className="font-mono text-xs">
                    {verwaltung.benutzerId}
                  </TableCell>
                  <TableCell>{verwaltung.rolle}</TableCell>
                  <TableCell className="text-right">
                    {gesperrt ? null : (
                      <div className="flex justify-end gap-2">
                        <Button
                          variant="ghost"
                          size="sm"
                          disabled={verwaltungAendern.isPending}
                          onClick={() =>
                            verwaltungAendern.mutate([
                              anlage.id,
                              verwaltung.benutzerId,
                              verwaltung.rolle ===
                              AnlageVerwaltungRolle.Administrator
                                ? AnlageVerwaltungRolle.Mitarbeiter
                                : AnlageVerwaltungRolle.Administrator,
                            ])
                          }
                        >
                          {verwaltung.rolle ===
                          AnlageVerwaltungRolle.Administrator
                            ? "Zu Mitarbeiter"
                            : "Zu Administrator"}
                        </Button>
                        <Button
                          variant="destructive"
                          size="sm"
                          disabled={verwaltungEntziehen.isPending}
                          onClick={() =>
                            verwaltungEntziehen.mutate([
                              anlage.id,
                              verwaltung.benutzerId,
                            ])
                          }
                        >
                          Entziehen
                        </Button>
                      </div>
                    )}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}

        {fehler ? (
          <Alert variant="error">
            {apiErrorMessage(fehler, {
              badRequest:
                "Die letzte Verwaltung mit der Rolle Administrator kann nicht entzogen werden.",
              fallback: "Die Verwaltung konnte nicht geändert werden.",
            })}
          </Alert>
        ) : null}

        {gesperrt ? null : (
          <form
            className="grid items-end gap-4 sm:grid-cols-3"
            onSubmit={(event) => {
              event.preventDefault();
              verwaltungGewaehren.mutate([anlage.id, benutzerId, rolle], {
                onSuccess: () => setBenutzerId(""),
              });
            }}
          >
            <Field label="Benutzer-ID" hint="Kennung des Benutzers">
              <Input
                value={benutzerId}
                onChange={(event) => setBenutzerId(event.target.value)}
                required
              />
            </Field>
            <Field label="Rolle">
              <Select
                value={rolle}
                onChange={(event) =>
                  setRolle(event.target.value as AnlageVerwaltungRolle)
                }
              >
                <option value={AnlageVerwaltungRolle.Mitarbeiter}>
                  Mitarbeiter
                </option>
                <option value={AnlageVerwaltungRolle.Administrator}>
                  Administrator
                </option>
              </Select>
            </Field>
            <Button type="submit" disabled={verwaltungGewaehren.isPending}>
              Vergeben
            </Button>
          </form>
        )}
      </CardContent>
    </Card>
  );
}

// ----------------------------------------------------------------------- Stammdaten

function Stammdaten({
  anlage,
  gesperrt,
  onDeaktivieren,
  laeuft,
  fehler,
}: {
  anlage: Anlage;
  gesperrt: boolean;
  onDeaktivieren: () => void;
  laeuft: boolean;
  fehler: unknown;
}) {
  const { anlageAendern } = useStammdatenMutations();
  const [name, setName] = useState(anlage.name);

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Beschreibende Daten</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            className="grid items-end gap-4 sm:grid-cols-3"
            onSubmit={(event) => {
              event.preventDefault();
              anlageAendern.mutate([
                anlage.id,
                {
                  ...anlage,
                  name,
                  adresse: anlage.adresse,
                  kontakt: anlage.kontakt,
                },
              ]);
            }}
          >
            <Field label="Name">
              <Input
                value={name}
                onChange={(event) => setName(event.target.value)}
                disabled={gesperrt}
                required
              />
            </Field>
            <Button
              type="submit"
              disabled={gesperrt || anlageAendern.isPending}
            >
              Speichern
            </Button>
          </form>
          {anlageAendern.isError ? (
            <Alert variant="error" className="mt-4">
              {apiErrorMessage(anlageAendern.error, {
                badRequest: "Bitte prüfen Sie die Eingaben.",
                fallback: "Die Änderung konnte nicht gespeichert werden.",
              })}
            </Alert>
          ) : null}
        </CardContent>
      </Card>

      {gesperrt ? null : (
        <>
          <Separator />
          <Card>
            <CardHeader>
              <CardTitle>Anlage deaktivieren</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-sm text-secondary">
                Eine deaktivierte Anlage bleibt lesbar, nimmt aber keine
                Änderung mehr an, und ihre Parcours starten keine neuen Runden.
                Die Deaktivierung ist endgültig.
              </p>
              {fehler ? (
                <Alert variant="error">
                  {apiErrorMessage(fehler, {
                    fallback: "Die Anlage konnte nicht deaktiviert werden.",
                  })}
                </Alert>
              ) : null}
              <Button
                variant="destructive"
                onClick={onDeaktivieren}
                disabled={laeuft}
              >
                {laeuft ? "Wird deaktiviert…" : "Endgültig deaktivieren"}
              </Button>
            </CardContent>
          </Card>
        </>
      )}
    </div>
  );
}
