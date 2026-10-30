"use client";

import { Plus, Trash2 } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { type FormEvent, useEffect, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import {
  useAnlageQuery,
  useBenutzerNachEmailQuery,
  useErfolgsmeldung,
  useStammdatenMutations,
} from "@/api/hooks";
import {
  type Anlage,
  type AnlageAdresse,
  type AnlageKontakt,
  AnlageVerwaltungRolle,
  type ParkourInput,
  type Pflock,
  type ZielInput,
} from "@/api/logandaim-api";
import { Erfolgsmeldung } from "@/components/feedback/erfolgsmeldung";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
import {
  freiePflockFarbe,
  neuerPflock,
} from "@/components/stammdaten/neuer-pflock";
import { PflockFarbeFeld } from "@/components/stammdaten/pflock-farbe-feld";
import { PflockMarker } from "@/components/stammdaten/pflock-swatch";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Field, FormRow } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
  HatchBand,
  MarginNote,
  SheetNumber,
  StampMark,
} from "@/components/ui/paper-rule";
import { Select } from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
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
    return <AnlageSkeleton />;
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
    return <AnlageSkeleton />;
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

/** A printed placeholder: the head, then a ledger, before the Anlage arrives. */
function AnlageSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6 p-6">
      <div className="space-y-3">
        <Skeleton className="h-3 w-32" />
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-4 w-80" />
        <Skeleton className="h-px w-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-5 w-40" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
    </div>
  );
}

function AnlageInhalt({ anlage }: { anlage: Anlage }) {
  const mutations = useStammdatenMutations();
  const deaktivierenMeldung = useErfolgsmeldung();
  const deaktiviert = anlage.zustand === "Archiviert";

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label={anlage.name} />

      <PageHeader
        register={["Verwaltung", "Anlagen"]}
        title={anlage.name}
        description={`${anlage.adresse.straße}, ${anlage.adresse.postleitzahl} ${anlage.adresse.ort} (${anlage.adresse.ländercode})`}
        actions={
          <>
            <Badge variant={deaktiviert ? "warning" : "success"}>
              {deaktiviert ? "Deaktiviert" : "Aktiv"}
            </Badge>
            {deaktiviert ? (
              <StampMark className="text-impact">Deaktiviert</StampMark>
            ) : null}
          </>
        }
      />

      {deaktiviert ? (
        <Alert variant="warning">
          Diese Anlage ist deaktiviert. Sie ist nur noch für
          Systemadministratoren einsehbar, nimmt keine Änderung mehr an und
          startet keine neuen Runden. Eine Reaktivierung ist nicht vorgesehen.
        </Alert>
      ) : null}

      {/* The confirmation lives here, not in the card that deactivated: a success
          replaces that card with the archived state of the whole page. */}
      <Erfolgsmeldung meldung={deaktivierenMeldung.meldung} />

      <Tabs defaultValue="ziele">
        <TabsList>
          <TabsTrigger value="ziele">Ziele</TabsTrigger>
          <TabsTrigger value="parkour">Parkour</TabsTrigger>
          <TabsTrigger value="verwaltung">Verwaltung</TabsTrigger>
          <TabsTrigger value="stammdaten">Stammdaten</TabsTrigger>
        </TabsList>

        <TabsContent value="ziele">
          <Ziele anlage={anlage} gesperrt={deaktiviert} />
        </TabsContent>

        <TabsContent value="parkour">
          <ParkourListe anlage={anlage} gesperrt={deaktiviert} />
        </TabsContent>

        <TabsContent value="verwaltung">
          <Verwaltungen anlage={anlage} gesperrt={deaktiviert} />
        </TabsContent>

        <TabsContent value="stammdaten">
          <Stammdaten
            anlage={anlage}
            gesperrt={deaktiviert}
            onDeaktivieren={() => {
              deaktivierenMeldung.verwerfen();
              mutations.anlageDeaktivieren.mutate(anlage.id, {
                onSuccess: () =>
                  deaktivierenMeldung.melden("Anlage deaktiviert."),
              });
            }}
            laeuft={mutations.anlageDeaktivieren.isPending}
            fehler={mutations.anlageDeaktivieren.error}
            aenderungSpeichert={mutations.anlageAendern}
          />
        </TabsContent>
      </Tabs>
    </div>
  );
}

// --------------------------------------------------------------------------- Ziele

const leeresZiel: ZielInput = { tiername: "" };

function Ziele({ anlage, gesperrt }: { anlage: Anlage; gesperrt: boolean }) {
  const { zielHinzufuegen, zielAendern, zielErsetzen, zielEntfernen } =
    useStammdatenMutations();
  const meldung = useErfolgsmeldung();
  const [form, setForm] = useState<ZielInput>(leeresZiel);
  const [bearbeitet, setBearbeitet] = useState<string | null>(null);

  const fehler =
    zielHinzufuegen.error ??
    zielAendern.error ??
    zielErsetzen.error ??
    zielEntfernen.error;

  const aktive = anlage.ziele.filter((ziel) => ziel.zustand === "Aktiv");
  const archivierte = anlage.ziele.filter(
    (ziel) => ziel.zustand === "Archiviert",
  );

  return (
    <div className="space-y-4">
      <section aria-label="Ziele der Anlage" className="space-y-2">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">Ziele der Anlage</h2>
          <SheetNumber>{anlage.ziele.length}</SheetNumber>
        </div>
        {anlage.ziele.length === 0 ? (
          <p className="text-sm text-secondary">Noch kein Ziel erfasst.</p>
        ) : (
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader>Tiername</TableHeader>
                <TableHeader>Nummer</TableHeader>
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
                  <TableCell className="font-medium">{ziel.tiername}</TableCell>
                  <TableCell className="tabular-nums">
                    {ziel.zielnummer}
                  </TableCell>
                  <TableCell className="text-secondary">
                    {ziel.verwendungen.length === 0
                      ? "nicht platziert"
                      : ziel.verwendungen
                          .map(
                            (verwendung) =>
                              anlage.parkour.find(
                                (parkour) =>
                                  parkour.id === verwendung.parkourId,
                              )?.name ?? "Parkour",
                          )
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
                            setForm({ tiername: ziel.tiername });
                          }}
                        >
                          Bearbeiten
                        </Button>
                        <Button
                          variant="secondary"
                          size="sm"
                          disabled={zielErsetzen.isPending}
                          onClick={() => {
                            meldung.verwerfen();
                            zielErsetzen.mutate(
                              [anlage.id, ziel.id, { tiername: ziel.tiername }],
                              {
                                onSuccess: () =>
                                  meldung.melden("Ziel ersetzt."),
                              },
                            );
                          }}
                        >
                          Ersetzen
                        </Button>
                        <Button
                          variant="destructive"
                          size="sm"
                          disabled={zielEntfernen.isPending}
                          onClick={() => {
                            meldung.verwerfen();
                            zielEntfernen.mutate([anlage.id, ziel.id], {
                              onSuccess: () => meldung.melden("Ziel entfernt."),
                            });
                          }}
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

        <Erfolgsmeldung meldung={meldung.meldung} />
      </section>

      {gesperrt ? null : (
        <>
          <HatchBand />
          <section
            aria-label={bearbeitet ? "Ziel bearbeiten" : "Ziel hinzufügen"}
            className="space-y-3"
          >
            <h2 className="text-sm font-semibold text-ink">
              {bearbeitet ? "Ziel bearbeiten" : "Ziel hinzufügen"}
            </h2>
            <form
              onSubmit={(event) => {
                event.preventDefault();
                meldung.verwerfen();
                if (bearbeitet) {
                  zielAendern.mutate([anlage.id, bearbeitet, form], {
                    onSuccess: () => {
                      setBearbeitet(null);
                      meldung.melden("Ziel geändert.");
                    },
                  });
                } else {
                  zielHinzufuegen.mutate([anlage.id, form], {
                    onSuccess: () => {
                      setForm(leeresZiel);
                      meldung.melden("Ziel hinzugefügt.");
                    },
                  });
                }
              }}
            >
              <FormRow
                className="sm:grid-cols-3"
                actions={
                  <>
                    <Button
                      type="submit"
                      size="field"
                      disabled={
                        zielHinzufuegen.isPending || zielAendern.isPending
                      }
                    >
                      {bearbeitet ? "Speichern" : "Hinzufügen"}
                    </Button>
                    {bearbeitet ? (
                      <Button
                        variant="ghost"
                        size="field"
                        onClick={() => {
                          setBearbeitet(null);
                          setForm(leeresZiel);
                        }}
                      >
                        Abbrechen
                      </Button>
                    ) : null}
                  </>
                }
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
              </FormRow>
            </form>
          </section>
        </>
      )}
    </div>
  );
}

// ------------------------------------------------------------------------ Parkour

function ParkourListe({
  anlage,
  gesperrt,
}: {
  anlage: Anlage;
  gesperrt: boolean;
}) {
  const { parkourHinzufuegen } = useStammdatenMutations();
  const meldung = useErfolgsmeldung();
  const aktiveZiele = anlage.ziele.filter((ziel) => ziel.zustand === "Aktiv");
  const [name, setName] = useState("");
  const [pfloecke, setPfloecke] = useState<Pflock[]>(() => [neuerPflock([])]);
  const [ausgewaehlteZiele, setAusgewaehlteZiele] = useState<string[]>([]);

  function toggleZiel(zielId: string, ausgewaehlt: boolean) {
    setAusgewaehlteZiele((aktuell) =>
      ausgewaehlt
        ? [...aktuell, zielId]
        : aktuell.filter((id) => id !== zielId),
    );
  }

  /** A row joins the draft; the list itself is the record, so the note only confirms it. */
  function pflockHinzufuegen() {
    if (freiePflockFarbe(pfloecke) === null) return;
    setPfloecke((aktuell) => [...aktuell, neuerPflock(aktuell)]);
    meldung.melden("Pflock hinzugefügt.");
  }

  /** One Pflock stays: the domain refuses a Parkour without any (`KeinePflöcke`). */
  function pflockEntfernen(id: string) {
    if (pfloecke.length <= 1) return;
    setPfloecke((aktuell) => aktuell.filter((pflock) => pflock.id !== id));
    meldung.melden("Pflock entfernt.");
  }

  function parkourErstellen(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const eingabe: ParkourInput = {
      name,
      pflöcke: pfloecke,
      // A new Parkour has no distances yet, so every placement starts empty.
      platzierungen: ausgewaehlteZiele.map((zielId) => ({
        zielId,
        abstände: [],
      })),
    };
    meldung.verwerfen();
    parkourHinzufuegen.mutate([anlage.id, eingabe], {
      onSuccess: () => {
        setName("");
        setPfloecke([neuerPflock([])]);
        setAusgewaehlteZiele([]);
        meldung.melden("Parkour angelegt.");
      },
    });
  }

  return (
    <div className="space-y-6">
      {!gesperrt ? (
        <section aria-label="Parkour anlegen" className="space-y-3">
          <h2 className="text-sm font-semibold text-ink">Parkour anlegen</h2>
          <div className="space-y-4">
            {aktiveZiele.length === 0 ? (
              <Alert variant="warning">
                Erfassen Sie zuerst mindestens ein aktives Ziel, damit Sie es
                dem Parkour zuordnen können.
              </Alert>
            ) : null}
            {parkourHinzufuegen.isError ? (
              <Alert variant="error">
                {apiErrorMessage(parkourHinzufuegen.error, {
                  unauthorized:
                    "Sie dürfen an dieser Anlage keine Parkours anlegen.",
                  badRequest:
                    "Bitte prüfen Sie den Parkournamen und die Zielauswahl.",
                  fallback: "Der Parkour konnte nicht angelegt werden.",
                })}
              </Alert>
            ) : null}
            {/* One slot for the result of this form: failure above, success here. */}
            <Erfolgsmeldung meldung={meldung.meldung} />
            <form className="space-y-4" onSubmit={parkourErstellen}>
              <Field label="Parkourname" className="sm:max-w-md">
                <Input
                  value={name}
                  onChange={(event) => setName(event.target.value)}
                  disabled={parkourHinzufuegen.isPending}
                  required
                />
              </Field>
              <section
                aria-label="Pflöcke des neuen Parkours"
                className="space-y-3"
              >
                <h3 className="text-sm font-semibold text-ink">Pflöcke</h3>
                <ul className="space-y-3">
                  {pfloecke.map((pflock, index) => (
                    <li
                      key={pflock.id}
                      className="rounded-sm border border-subtle bg-surface p-3"
                    >
                      {/* A row of the print form: the picker, and its action aligned
                          to the control row rather than to the row's bottom edge. */}
                      <FormRow
                        className="sm:grid-cols-[minmax(0,1fr)_auto]"
                        actions={
                          <Button
                            variant="ghost"
                            size="icon"
                            aria-label={`Pflock ${index + 1} entfernen`}
                            disabled={
                              parkourHinzufuegen.isPending ||
                              pfloecke.length <= 1
                            }
                            onClick={() => pflockEntfernen(pflock.id)}
                          >
                            <Trash2 />
                          </Button>
                        }
                      >
                        <PflockFarbeFeld
                          label={`Farbe Pflock ${index + 1}`}
                          value={pflock.farbe}
                          onValueChange={(farbe) =>
                            setPfloecke((aktuell) =>
                              aktuell.map((eintrag) =>
                                eintrag.id === pflock.id
                                  ? { ...eintrag, farbe }
                                  : eintrag,
                              ),
                            )
                          }
                          disabled={parkourHinzufuegen.isPending}
                        />
                      </FormRow>
                    </li>
                  ))}
                </ul>
                <Button
                  variant="secondary"
                  size="sm"
                  onClick={pflockHinzufuegen}
                  disabled={
                    parkourHinzufuegen.isPending ||
                    freiePflockFarbe(pfloecke) === null
                  }
                >
                  <Plus />
                  Pflock hinzufügen
                </Button>
                {freiePflockFarbe(pfloecke) === null ? (
                  <MarginNote label="Farbe wählen">
                    Alle Farben der Liste sind bereits vergeben. Geben Sie einem
                    Pflock eine eigene Farbe, dann ist wieder eine Farbe für
                    einen weiteren Pflock frei.
                  </MarginNote>
                ) : null}
                {pfloecke.length <= 1 ? (
                  <MarginNote label="Mindestens ein Pflock">
                    Ein Parkour braucht mindestens einen Pflock. Der letzte
                    Pflock lässt sich deshalb nicht entfernen; legen Sie zuerst
                    einen weiteren an.
                  </MarginNote>
                ) : null}
              </section>
              <section className="space-y-2">
                <h3 className="text-sm font-semibold text-ink">
                  Ziele im Parkour
                </h3>
                <div className="grid gap-x-4 sm:grid-cols-2">
                  {aktiveZiele.map((ziel) => (
                    <Checkbox
                      key={ziel.id}
                      label={`${ziel.tiername} · Nr. ${ziel.zielnummer}`}
                      checked={ausgewaehlteZiele.includes(ziel.id)}
                      onChange={(event) =>
                        toggleZiel(ziel.id, event.target.checked)
                      }
                      disabled={parkourHinzufuegen.isPending}
                    />
                  ))}
                </div>
              </section>
              <MarginNote label="Aufbau">
                Die Pflöcke stehen von nah nach fern: der erste Pflock ist der
                nächste. Distanzen tragen Sie nach dem Anlegen im Parkour ein.
              </MarginNote>
              <Button
                type="submit"
                size="field"
                disabled={
                  !aktiveZiele.length ||
                  !ausgewaehlteZiele.length ||
                  parkourHinzufuegen.isPending
                }
              >
                {parkourHinzufuegen.isPending
                  ? "Parkour wird angelegt…"
                  : "Parkour anlegen"}
              </Button>
            </form>
          </div>
        </section>
      ) : null}
      {!gesperrt ? <HatchBand /> : null}
      <section aria-label="Parkours der Anlage" className="space-y-2">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">
            Parkours der Anlage
          </h2>
          <SheetNumber>{anlage.parkour.length}</SheetNumber>
        </div>
        {anlage.parkour.length === 0 ? (
          <p className="text-sm text-secondary">Noch kein Parkour erfasst.</p>
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
              {anlage.parkour.map((parkour) => (
                <TableRow key={parkour.id}>
                  <TableCell className="font-medium">{parkour.name}</TableCell>
                  <TableCell>
                    <span className="flex flex-wrap gap-3 text-sm">
                      {parkour.pflöcke.map((pflock) => (
                        <PflockMarker key={pflock.id} farbe={pflock.farbe} />
                      ))}
                    </span>
                  </TableCell>
                  <TableCell className="tabular-nums">
                    {parkour.platzierungen.length}
                  </TableCell>
                  <TableCell>
                    <Badge
                      variant={
                        parkour.zustand === "Archiviert" ? "warning" : "success"
                      }
                    >
                      {parkour.zustand === "Archiviert"
                        ? "Archiviert"
                        : "Aktiv"}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-right">
                    <Button variant="secondary" size="sm" asChild>
                      <Link
                        href={`${paths.verwaltungParkour}?id=${parkour.id}&anlage=${anlage.id}`}
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
      </section>
    </div>
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
  const meldung = useErfolgsmeldung();
  const [emailEingabe, setEmailEingabe] = useState("");
  const [emailAbfrage, setEmailAbfrage] = useState("");
  const [ausgewaehlteBenutzerId, setAusgewaehlteBenutzerId] = useState("");
  const [rolle, setRolle] = useState<AnlageVerwaltungRolle>(
    AnlageVerwaltungRolle.Mitarbeiter,
  );
  const benutzerSuche = useBenutzerNachEmailQuery(
    anlage.id,
    emailAbfrage,
    emailAbfrage.length > 0,
  );

  const fehler =
    verwaltungGewaehren.error ??
    verwaltungEntziehen.error ??
    verwaltungAendern.error;
  const suchfehler = benutzerSuche.error;

  return (
    <div className="space-y-6">
      <section aria-label="Verwaltungen" className="space-y-2">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">Verwaltungen</h2>
          <SheetNumber>{anlage.verwaltungen.length}</SheetNumber>
        </div>
        {anlage.verwaltungen.length === 0 ? (
          <p className="text-sm text-secondary">
            Noch keine Verwaltung vergeben.
          </p>
        ) : (
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader>Benutzer-ID</TableHeader>
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
                          onClick={() => {
                            meldung.verwerfen();
                            verwaltungAendern.mutate(
                              [
                                anlage.id,
                                verwaltung.benutzerId,
                                verwaltung.rolle ===
                                AnlageVerwaltungRolle.Administrator
                                  ? AnlageVerwaltungRolle.Mitarbeiter
                                  : AnlageVerwaltungRolle.Administrator,
                              ],
                              {
                                onSuccess: () =>
                                  meldung.melden("Verwaltung geändert."),
                              },
                            );
                          }}
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
                          onClick={() => {
                            meldung.verwerfen();
                            verwaltungEntziehen.mutate(
                              [anlage.id, verwaltung.benutzerId],
                              {
                                onSuccess: () =>
                                  meldung.melden("Verwaltung entzogen."),
                              },
                            );
                          }}
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
      </section>

      {fehler ? (
        <Alert variant="error">
          {apiErrorMessage(fehler, {
            badRequest:
              "Die letzte Verwaltung mit der Rolle Administrator kann nicht entzogen werden.",
            fallback: "Die Verwaltung konnte nicht geändert werden.",
          })}
        </Alert>
      ) : null}

      <Erfolgsmeldung meldung={meldung.meldung} />

      <MarginNote label="Rollen">
        Eine Anlage wird von Mitarbeitern gepflegt; ein Administrator darf
        zusätzlich Verwaltungen vergeben. Die letzte Verwaltung mit der Rolle
        Administrator kann nicht entzogen werden.
      </MarginNote>

      {gesperrt ? null : (
        <form
          onSubmit={(event) => {
            event.preventDefault();
            const email = emailEingabe.trim();
            setEmailAbfrage(email);
            setAusgewaehlteBenutzerId("");
          }}
        >
          <FormRow
            className="sm:grid-cols-[minmax(0,1fr)_auto]"
            actions={
              <Button type="submit" variant="secondary" size="field">
                Suchen
              </Button>
            }
          >
            <Field
              label="Benutzerin oder Benutzer per E-Mail finden"
              hint="Es wird nur nach der vollständigen E-Mail-Adresse gesucht."
            >
              <Input
                type="email"
                autoComplete="off"
                value={emailEingabe}
                onChange={(event) => {
                  setEmailEingabe(event.target.value);
                  setEmailAbfrage("");
                  setAusgewaehlteBenutzerId("");
                }}
                required
              />
            </Field>
          </FormRow>
        </form>
      )}

      {emailAbfrage ? (
        <div className="space-y-3">
          {benutzerSuche.isFetching ? (
            <p className="text-sm text-secondary">Suche läuft …</p>
          ) : benutzerSuche.data?.length ? (
            <Field label="Gefundenes Konto auswählen">
              <Select
                value={ausgewaehlteBenutzerId}
                onChange={(event) =>
                  setAusgewaehlteBenutzerId(event.target.value)
                }
              >
                <option value="">Konto auswählen …</option>
                {benutzerSuche.data.map((benutzer) => (
                  <option key={benutzer.id} value={benutzer.id}>
                    {benutzer.eMailAdresse}
                  </option>
                ))}
              </Select>
            </Field>
          ) : benutzerSuche.isSuccess ? (
            <p className="text-sm text-secondary">
              Für diese E-Mail-Adresse wurde kein Konto gefunden.
            </p>
          ) : null}
          {suchfehler ? (
            <Alert variant="error">
              {apiErrorMessage(suchfehler, {
                unauthorized: "Bitte melden Sie sich an.",
                badRequest: "Bitte geben Sie eine gültige E-Mail-Adresse ein.",
                fallback: "Die Suche konnte nicht durchgeführt werden.",
              })}
            </Alert>
          ) : null}
          {benutzerSuche.data?.length && !gesperrt ? (
            <form
              onSubmit={(event) => {
                event.preventDefault();
                if (!ausgewaehlteBenutzerId) return;
                meldung.verwerfen();
                verwaltungGewaehren.mutate(
                  [anlage.id, ausgewaehlteBenutzerId, rolle],
                  {
                    onSuccess: () => {
                      setEmailEingabe("");
                      setEmailAbfrage("");
                      setAusgewaehlteBenutzerId("");
                      meldung.melden("Verwaltung vergeben.");
                    },
                  },
                );
              }}
            >
              <FormRow
                className="sm:grid-cols-[minmax(0,1fr)_auto]"
                actions={
                  <Button
                    type="submit"
                    size="field"
                    disabled={
                      !ausgewaehlteBenutzerId || verwaltungGewaehren.isPending
                    }
                  >
                    Verwaltung vergeben
                  </Button>
                }
              >
                <Field label="Rolle für dieses Konto">
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
              </FormRow>
            </form>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

// ----------------------------------------------------------------------- Stammdaten

function Stammdaten({
  anlage,
  gesperrt,
  onDeaktivieren,
  laeuft,
  fehler,
  aenderungSpeichert,
}: {
  anlage: Anlage;
  gesperrt: boolean;
  onDeaktivieren: () => void;
  laeuft: boolean;
  fehler: unknown;
  aenderungSpeichert: ReturnType<
    typeof useStammdatenMutations
  >["anlageAendern"];
}) {
  const [name, setName] = useState(anlage.name);
  const [adresse, setAdresse] = useState<AnlageAdresse>(anlage.adresse);
  const [kontakt, setKontakt] = useState<AnlageKontakt>(anlage.kontakt);
  const [deaktivierenOffen, setDeaktivierenOffen] = useState(false);
  const meldung = useErfolgsmeldung();

  return (
    <div className="space-y-0">
      <Card>
        <CardHeader>
          <CardTitle>Beschreibende Daten</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            className="space-y-6"
            onSubmit={(event) => {
              event.preventDefault();
              meldung.verwerfen();
              aenderungSpeichert.mutate(
                [anlage.id, { ...anlage, name, adresse, kontakt }],
                {
                  onSuccess: () => meldung.melden("Stammdaten gespeichert."),
                },
              );
            }}
          >
            <div className="grid gap-4 sm:grid-cols-2">
              <Field label="Anlagenname" className="sm:col-span-2">
                <Input
                  value={name}
                  onChange={(event) => setName(event.target.value)}
                  disabled={gesperrt}
                  required
                />
              </Field>
              <Field label="Straße und Hausnummer">
                <Input
                  value={adresse.straße}
                  onChange={(event) =>
                    setAdresse((old) => ({
                      ...old,
                      straße: event.target.value,
                    }))
                  }
                  disabled={gesperrt}
                  required
                />
              </Field>
              <Field label="Postleitzahl">
                <Input
                  value={adresse.postleitzahl}
                  onChange={(event) =>
                    setAdresse((old) => ({
                      ...old,
                      postleitzahl: event.target.value,
                    }))
                  }
                  disabled={gesperrt}
                  required
                />
              </Field>
              <Field label="Ort">
                <Input
                  value={adresse.ort}
                  onChange={(event) =>
                    setAdresse((old) => ({ ...old, ort: event.target.value }))
                  }
                  disabled={gesperrt}
                  required
                />
              </Field>
              <Field label="Land">
                <Select
                  value={adresse.ländercode}
                  onChange={(event) =>
                    setAdresse((old) => ({
                      ...old,
                      ländercode: event.target
                        .value as AnlageAdresse["ländercode"],
                    }))
                  }
                  disabled={gesperrt}
                  required
                >
                  <option value="AT">Österreich</option>
                  <option value="DE">Deutschland</option>
                  <option value="CH">Schweiz</option>
                </Select>
              </Field>
            </div>

            <section className="space-y-4">
              <div>
                <h3 className="font-serif text-lg font-semibold text-ink">
                  Kontaktdaten
                </h3>
                <p className="text-sm text-secondary">
                  Alle Angaben sind optional.
                </p>
              </div>
              <div className="grid gap-4 sm:grid-cols-2">
                <Field label="Verein oder Betreiber" optional>
                  <Input
                    value={kontakt.name ?? ""}
                    onChange={(event) =>
                      setKontakt((old) => ({
                        ...old,
                        name: event.target.value || undefined,
                      }))
                    }
                    disabled={gesperrt}
                  />
                </Field>
                <Field label="E-Mail-Adresse" optional>
                  <Input
                    type="email"
                    value={kontakt.email ?? ""}
                    onChange={(event) =>
                      setKontakt((old) => ({
                        ...old,
                        email: event.target.value || undefined,
                      }))
                    }
                    disabled={gesperrt}
                  />
                </Field>
                <Field label="Telefon" optional>
                  <Input
                    type="tel"
                    value={kontakt.telefon ?? ""}
                    onChange={(event) =>
                      setKontakt((old) => ({
                        ...old,
                        telefon: event.target.value || undefined,
                      }))
                    }
                    disabled={gesperrt}
                  />
                </Field>
                <Field label="Website" optional>
                  <Input
                    type="url"
                    value={kontakt.website ?? ""}
                    onChange={(event) =>
                      setKontakt((old) => ({
                        ...old,
                        website: event.target.value || undefined,
                      }))
                    }
                    disabled={gesperrt}
                  />
                </Field>
              </div>
            </section>

            <Button
              type="submit"
              disabled={gesperrt || aenderungSpeichert.isPending}
            >
              {aenderungSpeichert.isPending
                ? "Wird gespeichert…"
                : "Stammdaten speichern"}
            </Button>
          </form>
          {aenderungSpeichert.isError ? (
            <Alert variant="error" className="mt-4">
              {apiErrorMessage(aenderungSpeichert.error, {
                badRequest: "Bitte prüfen Sie die Eingaben.",
                fallback: "Die Änderung konnte nicht gespeichert werden.",
              })}
            </Alert>
          ) : null}
          <Erfolgsmeldung meldung={meldung.meldung} className="mt-4" />
        </CardContent>
      </Card>

      {gesperrt ? null : (
        <>
          <HatchBand height="sm" className="my-4" />
          <Card>
            <CardHeader>
              <CardTitle>Anlage deaktivieren</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-sm text-secondary">
                Eine deaktivierte Anlage ist nur noch für Systemadministratoren
                einsehbar, nimmt keine Änderung mehr an, und ihre Parkours
                starten keine neuen Runden. Die Deaktivierung ist endgültig.
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
                onClick={() => setDeaktivierenOffen(true)}
                disabled={laeuft}
              >
                Endgültig deaktivieren
              </Button>
            </CardContent>
          </Card>

          <Dialog open={deaktivierenOffen} onOpenChange={setDeaktivierenOffen}>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Anlage endgültig deaktivieren?</DialogTitle>
                <DialogDescription>
                  Die Anlage ist danach nur noch für Systemadministratoren
                  einsehbar, nimmt keine Änderung mehr an und startet keine
                  neuen Runden. Dieser Schritt kann nicht rückgängig gemacht
                  werden.
                </DialogDescription>
              </DialogHeader>
              {fehler ? (
                <Alert variant="error">
                  {apiErrorMessage(fehler, {
                    fallback: "Die Anlage konnte nicht deaktiviert werden.",
                  })}
                </Alert>
              ) : null}
              <DialogFooter>
                <Button
                  variant="secondary"
                  onClick={() => setDeaktivierenOffen(false)}
                  disabled={laeuft}
                >
                  Abbrechen
                </Button>
                <Button
                  variant="destructive"
                  onClick={onDeaktivieren}
                  disabled={laeuft}
                >
                  {laeuft ? "Wird deaktiviert…" : "Endgültig deaktivieren"}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </>
      )}
    </div>
  );
}
