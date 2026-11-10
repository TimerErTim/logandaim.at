"use client";

import {
  closestCenter,
  DndContext,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Plus, Trash2 } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import {
  useAnlageQuery,
  useErfolgsmeldung,
  useStammdatenMutations,
} from "@/api/hooks";
import type {
  Anlage,
  Parkour,
  ParkourSetupInput,
  Pflock,
  ZielPflöcke,
  Zielplatzierung,
} from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
import {
  freiePflockFarbe,
  neuerPflock,
} from "@/components/stammdaten/neuer-pflock";
import { PflockFarbeFeld } from "@/components/stammdaten/pflock-farbe-feld";
import { Alert } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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
 * One Parkour' shooting setup. A setup is one value: replacing it replaces every Pflock
 * and every placement at once (AC-6), which is why the editor works on a local copy and
 * sends the whole `ParkourSetupRequest`.
 */
export default function ParkourVerwaltungPage() {
  const phase = useAuth();
  const router = useRouter();
  const parkourId = useQueryParam("id");
  const anlageId = useQueryParam("anlage");
  const anlage = useAnlageQuery(anlageId);

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  if (parkourId === null || anlageId === null) {
    return <ParkourSkeleton />;
  }

  if (!parkourId || !anlageId) {
    return (
      <div className="p-6">
        <Alert variant="warning">
          Es wurde kein Parkour ausgewählt. Bitte öffnen Sie ihn aus der Anlage.
        </Alert>
      </div>
    );
  }

  if (anlage.isLoading) {
    return <ParkourSkeleton />;
  }

  const parkour = anlage.data?.parkour.find(
    (eintrag) => eintrag.id === parkourId,
  );
  if (anlage.isError || !anlage.data || !parkour) {
    return (
      <div className="p-6">
        <Alert variant="error">
          {apiErrorMessage(anlage.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Der Parkour konnte nicht geladen werden.",
          })}
        </Alert>
      </div>
    );
  }

  return <ParkourInhalt anlage={anlage.data} parkour={parkour} />;
}

/** A printed placeholder: the head and the ledgers, before the Parkour arrives. */
function ParkourSkeleton() {
  return (
    <div aria-busy="true" className="space-y-6 p-6">
      <div className="space-y-3">
        <Skeleton className="h-3 w-32" />
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-4 w-72" />
        <Skeleton className="h-px w-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="h-5 w-40" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
      <Skeleton className="h-24 w-full" />
    </div>
  );
}

function ParkourInhalt({
  anlage,
  parkour,
}: {
  anlage: Anlage;
  parkour: Parkour;
}) {
  const { parkourSetupAendern, parkourUmbenennen, parkourEntfernen } =
    useStammdatenMutations();
  const umbenennenMeldung = useErfolgsmeldung();
  const aufstellungMeldung = useErfolgsmeldung();
  const pflockMeldung = useErfolgsmeldung();
  const router = useRouter();
  const [name, setName] = useState(parkour.name);
  const [entfernenOffen, setEntfernenOffen] = useState(false);
  const [setup, setSetup] = useState<ParkourSetupInput>(() => ({
    pflöcke: parkour.pflöcke.map((pflock) => ({ ...pflock })),
    platzierungen: parkour.platzierungen.map((platzierung) => ({
      zielId: platzierung.zielId,
      pflöcke:
        platzierung.pflöcke.art === "Explicit"
          ? {
              art: "Explicit",
              pflöcke: platzierung.pflöcke.pflöcke.map((pflock) => ({
                ...pflock,
              })),
            }
          : { art: "InheritFromParkour" },
      abstände: platzierung.abstände.map((abstand) => ({ ...abstand })),
    })),
  }));
  // The draft as it was last read from or written to the server, so the page can say
  // whether the setup on screen still is the one stored. A local Pflock row or a drag
  // changes only the draft; nothing is written until "Aufstellung speichern".
  const [gespeichert, setGespeichert] = useState(() => JSON.stringify(setup));
  const geaendert = JSON.stringify(setup) !== gespeichert;

  const gesperrt =
    anlage.zustand === "Archiviert" || parkour.zustand === "Archiviert";
  const ziele = useMemo(
    () => anlage.ziele.filter((ziel) => ziel.zustand === "Aktiv"),
    [anlage.ziele],
  );

  /** One Pflock cannot be removed: the domain refuses a setup without any (`KeinePflöcke`). */
  const letzterPflock = setup.pflöcke.length <= 1;
  /** No vocabulary colour is left for a new shared row, so the user must pick a custom one. */
  const keineFreieFarbe = freiePflockFarbe(setup.pflöcke) === null;

  function effektivePflöcke(platzierung: Zielplatzierung): Pflock[] {
    return platzierung.pflöcke.art === "Explicit"
      ? platzierung.pflöcke.pflöcke
      : setup.pflöcke;
  }

  function abständeBereinigen(
    abstände: Zielplatzierung["abstände"],
    pflöcke: readonly Pflock[],
  ) {
    const ids = new Set(pflöcke.map((pflock) => pflock.id));
    return abstände.filter((abstand) => ids.has(abstand.pflockId));
  }

  function pflockFarbeAendern(id: string, farbe: string) {
    setSetup((bisher) => ({
      ...bisher,
      pflöcke: bisher.pflöcke.map((pflock) =>
        pflock.id === id ? { ...pflock, farbe } : pflock,
      ),
    }));
  }

  /** The Pflock array order *is* the near-to-far order, so a drag rewrites it. */
  function pflockVerschieben(von: number, nach: number) {
    setSetup((bisher) => ({
      ...bisher,
      pflöcke: arrayMove(bisher.pflöcke, von, nach),
    }));
  }

  /**
   * A new Pflock joins the far end of the row: the user drags it into place, and the
   * array order stays the only statement of the near-to-far order (the first Pflock is
   * the nearest). Its colour is
   * the standard one, or the first free colour of the club's vocabulary when that is taken,
   * because the domain refuses two Pflöcke of one colour (AC-6).
   *
   * This edits the draft, not the Parkour: the row appearing *is* the confirmation,
   * and the dirty note beside "Aufstellung speichern" says the draft is not stored yet.
   */
  function pflockHinzufuegen() {
    if (freiePflockFarbe(setup.pflöcke) === null) return;
    setSetup((bisher) => {
      const neu = neuerPflock(bisher.pflöcke);
      // The new row has no distances yet, so no placement needs an entry for it.
      return { ...bisher, pflöcke: [...bisher.pflöcke, neu] };
    });
    // The row exists in the draft only; the "Nicht gespeichert" note below says so.
    pflockMeldung.melden("Pflock hinzugefügt.");
  }

  /**
   * Remove one Pflock *and* every distance measured from it, on every placement. The
   * domain rejects a distance keyed to a Pflock that is not in the Parkour
   * (`PflockNichtImParkour`), so leaving the entries behind would make the whole
   * setup unsaveable. Dropping entries only ever removes distances, which keeps the
   * near-to-far increase of the remaining ones intact.
   *
   * One Pflock always stays: `KeinePflöcke` is a domain error, so the row's remove
   * action is disabled at one and the list says why.
   */
  function pflockEntfernen(id: string) {
    if (setup.pflöcke.length <= 1) return;
    setSetup((bisher) => {
      if (bisher.pflöcke.length <= 1) return bisher;
      return {
        ...bisher,
        pflöcke: bisher.pflöcke.filter((pflock) => pflock.id !== id),
        platzierungen: bisher.platzierungen.map((platzierung) => {
          const eigene =
            platzierung.pflöcke.art === "Explicit"
              ? platzierung.pflöcke.pflöcke.filter((pflock) => pflock.id !== id)
              : null;
          const pflöcke = eigene
            ? ({ art: "Explicit", pflöcke: eigene } satisfies ZielPflöcke)
            : platzierung.pflöcke;
          const nächstePlatzierung = { ...platzierung, pflöcke };
          return {
            ...nächstePlatzierung,
            abstände: abständeBereinigen(
              platzierung.abstände.filter((abstand) => abstand.pflockId !== id),
              effektivePflöcke(nächstePlatzierung),
            ),
          };
        }),
      };
    });
    pflockMeldung.melden("Pflock entfernt.");
  }

  /**
   * An empty field means "no distance configured", which the API expresses by omitting
   * `meter` — never by `0` or `NaN`.
   */
  function abstandAendern(
    zielId: string,
    pflockId: string,
    meter: number | undefined,
  ) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) =>
        platzierung.zielId === zielId
          ? {
              ...platzierung,
              abstände: effektivePflöcke(platzierung).map((pflock) => {
                const bisheriger = platzierung.abstände.find(
                  (abstand) => abstand.pflockId === pflock.id,
                );
                return {
                  pflockId: pflock.id,
                  meter: pflock.id === pflockId ? meter : bisheriger?.meter,
                };
              }),
            }
          : platzierung,
      ),
    }));
  }

  function zielPflöckeAendern(zielId: string, pflöcke: ZielPflöcke) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) => {
        if (platzierung.zielId !== zielId) return platzierung;
        const bisherigePflöcke =
          platzierung.pflöcke.art === "Explicit"
            ? platzierung.pflöcke.pflöcke
            : bisher.pflöcke;
        const nächstePflöcke =
          pflöcke.art === "Explicit" ? pflöcke.pflöcke : bisher.pflöcke;
        const abstände = nächstePflöcke.flatMap((pflock) => {
          const bisheriger =
            platzierung.abstände.find(
              (abstand) => abstand.pflockId === pflock.id,
            ) ??
            (() => {
              const farbe = bisherigePflöcke.find(
                (eintrag) => eintrag.farbe === pflock.farbe,
              );
              return farbe
                ? platzierung.abstände.find(
                    (abstand) => abstand.pflockId === farbe.id,
                  )
                : undefined;
            })();
          return bisheriger
            ? [{ pflockId: pflock.id, meter: bisheriger.meter }]
            : [];
        });
        return { ...platzierung, pflöcke, abstände };
      }),
    }));
  }

  function zielPflockFarbeAendern(zielId: string, id: string, farbe: string) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) => {
        if (
          platzierung.zielId !== zielId ||
          platzierung.pflöcke.art !== "Explicit"
        )
          return platzierung;
        const pflöcke = platzierung.pflöcke.pflöcke.map((pflock) =>
          pflock.id === id ? { ...pflock, farbe } : pflock,
        );
        return { ...platzierung, pflöcke: { art: "Explicit", pflöcke } };
      }),
    }));
  }

  function zielPflockHinzufuegen(zielId: string) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) => {
        if (
          platzierung.zielId !== zielId ||
          platzierung.pflöcke.art !== "Explicit"
        )
          return platzierung;
        const pflöcke = platzierung.pflöcke.pflöcke;
        if (freiePflockFarbe(pflöcke) === null) return platzierung;
        return {
          ...platzierung,
          pflöcke: {
            art: "Explicit",
            pflöcke: [...pflöcke, neuerPflock(pflöcke)],
          },
        };
      }),
    }));
  }

  function zielPflockEntfernen(zielId: string, pflockId: string) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) => {
        if (
          platzierung.zielId !== zielId ||
          platzierung.pflöcke.art !== "Explicit"
        )
          return platzierung;
        const pflöcke = platzierung.pflöcke.pflöcke.filter(
          (pflock) => pflock.id !== pflockId,
        );
        if (pflöcke.length === 0) return platzierung;
        return {
          ...platzierung,
          pflöcke: { art: "Explicit", pflöcke },
          abstände: abständeBereinigen(platzierung.abstände, pflöcke),
        };
      }),
    }));
  }

  function zielPflockVerschieben(zielId: string, von: number, nach: number) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) =>
        platzierung.zielId === zielId && platzierung.pflöcke.art === "Explicit"
          ? {
              ...platzierung,
              pflöcke: {
                art: "Explicit",
                pflöcke: arrayMove(platzierung.pflöcke.pflöcke, von, nach),
              },
            }
          : platzierung,
      ),
    }));
  }

  /** Route order is the order the Ziele are shot in, so it is the array order. */
  function platzierungVerschieben(von: number, nach: number) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: arrayMove(bisher.platzierungen, von, nach),
    }));
  }

  const pointerSensor = useSensor(PointerSensor, {
    // A short press must not become a drag when the user meant to type a distance.
    activationConstraint: { distance: 4 },
  });
  const keyboardSensor = useSensor(KeyboardSensor, {
    coordinateGetter: sortableKeyboardCoordinates,
  });
  const sensors = useSensors(pointerSensor, keyboardSensor);

  return (
    <div className="space-y-6 p-6">
      <SetBreadcrumbLabel label={parkour.name} />

      <PageHeader
        register={["Verwaltung", "Parkour"]}
        title={parkour.name}
        description={`${anlage.name} · Aufstellung, Pflöcke und Abstände`}
        actions={
          <>
            <Badge
              variant={parkour.zustand === "Archiviert" ? "warning" : "success"}
            >
              {parkour.zustand === "Archiviert" ? "Archiviert" : "Aktiv"}
            </Badge>
            {parkour.zustand === "Archiviert" ? (
              <StampMark>Archiviert</StampMark>
            ) : null}
          </>
        }
      />

      <Card>
        <CardHeader>
          <CardTitle>Name</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {parkourUmbenennen.isError ? (
            <Alert variant="error">
              {apiErrorMessage(parkourUmbenennen.error, {
                badRequest: "Bitte prüfen Sie den Parkournamen.",
                fallback: "Der Parkour konnte nicht umbenannt werden.",
              })}
            </Alert>
          ) : null}
          <form
            onSubmit={(event) => {
              event.preventDefault();
              umbenennenMeldung.verwerfen();
              parkourUmbenennen.mutate([anlage.id, parkour.id, name], {
                onSuccess: () => umbenennenMeldung.melden("Parkour umbenannt."),
              });
            }}
          >
            <FormRow
              className="sm:grid-cols-3"
              actions={
                <Button
                  type="submit"
                  size="field"
                  disabled={gesperrt || parkourUmbenennen.isPending}
                >
                  Umbenennen
                </Button>
              }
            >
              <Field label="Parkourname">
                <Input
                  value={name}
                  onChange={(event) => setName(event.target.value)}
                  disabled={gesperrt}
                  required
                />
              </Field>
            </FormRow>
          </form>
        </CardContent>
      </Card>

      <MarginNote label="Name">
        Bereits gestartete Runden behalten den damaligen Namen.
      </MarginNote>

      <section aria-label="Pflöcke" className="space-y-2">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">Pflöcke</h2>
          <SheetNumber>{setup.pflöcke.length}</SheetNumber>
        </div>
        {setup.pflöcke.length === 0 ? (
          <p className="text-sm text-secondary">Noch kein Pflock erfasst.</p>
        ) : (
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={({ active, over }) => {
              if (!over || active.id === over.id) {
                return;
              }
              const von = setup.pflöcke.findIndex(
                (pflock) => pflock.id === active.id,
              );
              const nach = setup.pflöcke.findIndex(
                (pflock) => pflock.id === over.id,
              );
              if (von >= 0 && nach >= 0) {
                pflockVerschieben(von, nach);
              }
            }}
          >
            <SortableContext
              items={setup.pflöcke.map((pflock) => pflock.id)}
              strategy={verticalListSortingStrategy}
            >
              {setup.pflöcke.map((pflock, index) => (
                <SortablePflock
                  key={pflock.id}
                  pflock={pflock}
                  index={index}
                  gesperrt={gesperrt}
                  onFarbeAendern={pflockFarbeAendern}
                  onEntfernen={pflockEntfernen}
                  entfernenGesperrt={letzterPflock}
                />
              ))}
            </SortableContext>
          </DndContext>
        )}
        {gesperrt ? null : (
          <div className="space-y-2">
            <Button
              variant="secondary"
              size="sm"
              onClick={pflockHinzufuegen}
              disabled={keineFreieFarbe || parkourSetupAendern.isPending}
            >
              <Plus />
              Pflock hinzufügen
            </Button>
            {keineFreieFarbe ? (
              <MarginNote label="Farbe wählen">
                Alle Farben der Liste sind bereits vergeben. Geben Sie einem
                Pflock eine eigene Farbe, dann ist wieder eine Farbe für einen
                weiteren Pflock frei.
              </MarginNote>
            ) : null}
          </div>
        )}
        <MarginNote label="Reihenfolge">
          Die Reihe ist die Entfernung: oben steht der nächste Pflock, unten der
          weiteste. Ziehen am Griff ändert die Reihenfolge.
        </MarginNote>
        {!gesperrt && letzterPflock ? (
          <MarginNote label="Mindestens ein Pflock">
            Ein Parkour braucht mindestens einen Pflock. Der letzte Pflock lässt
            sich deshalb nicht entfernen; legen Sie zuerst einen weiteren an.
          </MarginNote>
        ) : null}
      </section>

      <HatchBand />

      <section aria-label="Platzierungen und Abstände" className="space-y-3">
        <div className="flex items-baseline gap-3">
          <h2 className="text-sm font-semibold text-ink">
            Platzierungen und Abstände
          </h2>
          <SheetNumber>{setup.platzierungen.length}</SheetNumber>
        </div>
        {setup.platzierungen.length === 0 ? (
          <p className="text-sm text-secondary">Noch kein Ziel platziert.</p>
        ) : (
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={({ active, over }) => {
              if (!over || active.id === over.id) {
                return;
              }
              const von = setup.platzierungen.findIndex(
                (eintrag) => eintrag.zielId === active.id,
              );
              const nach = setup.platzierungen.findIndex(
                (eintrag) => eintrag.zielId === over.id,
              );
              if (von >= 0 && nach >= 0) {
                platzierungVerschieben(von, nach);
              }
            }}
          >
            <SortableContext
              items={setup.platzierungen.map((eintrag) => eintrag.zielId)}
              strategy={verticalListSortingStrategy}
            >
              {setup.platzierungen.map((platzierung, index) => {
                const ziel = ziele.find(
                  (eintrag) => eintrag.id === platzierung.zielId,
                );
                return (
                  <SortablePlatzierung
                    key={platzierung.zielId}
                    platzierung={platzierung}
                    index={index}
                    tiername={ziel?.tiername ?? "Ziel"}
                    gesperrt={gesperrt}
                    pflöcke={setup.pflöcke}
                    onAbstandAendern={abstandAendern}
                    onZielPflöckeAendern={zielPflöckeAendern}
                    onZielPflockFarbeAendern={zielPflockFarbeAendern}
                    onZielPflockHinzufuegen={zielPflockHinzufuegen}
                    onZielPflockEntfernen={zielPflockEntfernen}
                    onZielPflockVerschieben={zielPflockVerschieben}
                  />
                );
              })}
            </SortableContext>
          </DndContext>
        )}
      </section>

      <MarginNote label="Schussfolge">
        Die Reihe ist die Schussfolge: oben steht das erste Ziel, unten das
        letzte.
      </MarginNote>

      <section aria-label="Aufstellung speichern" className="space-y-3">
        {parkourSetupAendern.isError ? (
          <Alert variant="error">
            {apiErrorMessage(parkourSetupAendern.error, {
              badRequest: "Bitte prüfen Sie die Aufstellung.",
              fallback: "Die Aufstellung konnte nicht gespeichert werden.",
            })}
          </Alert>
        ) : null}
        {/* Same slot as the failure: one result, in the section the save belongs to. */}
        <div className="flex flex-wrap gap-3">
          <Button
            disabled={gesperrt || parkourSetupAendern.isPending}
            onClick={() => {
              aufstellungMeldung.verwerfen();
              pflockMeldung.verwerfen();
              parkourSetupAendern.mutate([anlage.id, parkour.id, setup], {
                onSuccess: () => {
                  setGespeichert(JSON.stringify(setup));
                  aufstellungMeldung.melden("Aufstellung gespeichert.");
                },
              });
            }}
          >
            {parkourSetupAendern.isPending
              ? "Wird gespeichert…"
              : "Aufstellung speichern"}
          </Button>
          <Button variant="secondary" asChild>
            <Link href={`${paths.verwaltungAnlage}?id=${anlage.id}`}>
              Zurück zur Anlage
            </Link>
          </Button>
        </div>
        {geaendert ? (
          <MarginNote label="Nicht gespeichert">
            Die Änderungen an Pflöcken, Reihenfolge und Abständen gelten erst
            nach „Aufstellung speichern“.
          </MarginNote>
        ) : null}
      </section>

      {gesperrt ? null : (
        <>
          <HatchBand height="lg" />
          <Card>
            <CardHeader>
              <CardTitle>Parkour entfernen</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-sm text-secondary">
                Wurde der Parkour schon einmal gestartet, wird er archiviert und
                bleibt in alten Runden sichtbar. Sonst wird er gelöscht.
              </p>
              <Button
                variant="destructive"
                disabled={parkourEntfernen.isPending}
                onClick={() => {
                  parkourEntfernen.reset();
                  setEntfernenOffen(true);
                }}
              >
                Entfernen
              </Button>
            </CardContent>
          </Card>

          <Dialog open={entfernenOffen} onOpenChange={setEntfernenOffen}>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Parkour entfernen?</DialogTitle>
                <DialogDescription>
                  Wurde der Parkour schon einmal gestartet, wird er archiviert
                  und bleibt in alten Runden sichtbar. Sonst wird er endgültig
                  gelöscht.
                </DialogDescription>
              </DialogHeader>
              {parkourEntfernen.isError ? (
                <Alert variant="error">
                  {apiErrorMessage(parkourEntfernen.error, {
                    fallback: "Der Parkour konnte nicht entfernt werden.",
                  })}
                </Alert>
              ) : null}
              <DialogFooter>
                <Button
                  variant="secondary"
                  onClick={() => setEntfernenOffen(false)}
                  disabled={parkourEntfernen.isPending}
                >
                  Abbrechen
                </Button>
                <Button
                  variant="destructive"
                  disabled={parkourEntfernen.isPending}
                  onClick={() =>
                    parkourEntfernen.mutate([anlage.id, parkour.id], {
                      // The removed Parkour no longer exists for this page to read,
                      // so the confirmation is the Anlage's own ledger without it.
                      onSuccess: () =>
                        router.push(
                          `${paths.verwaltungAnlage}?id=${anlage.id}`,
                        ),
                    })
                  }
                >
                  {parkourEntfernen.isPending ? "Wird entfernt…" : "Entfernen"}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </>
      )}

      <ParkourZiele ziele={ziele} platzierungen={setup.platzierungen} />
    </div>
  );
}

/**
 * One Ziel in the route, draggable by its handle only. The row shows the Ziel's
 * position in the shot order and every configured distance.
 */
function SortablePlatzierung({
  platzierung,
  index,
  tiername,
  gesperrt,
  pflöcke,
  onAbstandAendern,
  onZielPflöckeAendern,
  onZielPflockFarbeAendern,
  onZielPflockHinzufuegen,
  onZielPflockEntfernen,
  onZielPflockVerschieben,
}: {
  platzierung: Zielplatzierung;
  index: number;
  tiername: string;
  gesperrt: boolean;
  pflöcke: Pflock[];
  onAbstandAendern: (
    zielId: string,
    pflockId: string,
    meter: number | undefined,
  ) => void;
  onZielPflöckeAendern: (zielId: string, pflöcke: ZielPflöcke) => void;
  onZielPflockFarbeAendern: (zielId: string, id: string, farbe: string) => void;
  onZielPflockHinzufuegen: (zielId: string) => void;
  onZielPflockEntfernen: (zielId: string, pflockId: string) => void;
  onZielPflockVerschieben: (zielId: string, von: number, nach: number) => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: platzierung.zielId });
  const zielPflockSensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 4 } }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  );

  const effektive =
    platzierung.pflöcke.art === "Explicit"
      ? platzierung.pflöcke.pflöcke
      : pflöcke;
  const abstandById = new Map(
    platzierung.abstände.map((abstand) => [abstand.pflockId, abstand.meter]),
  );

  return (
    <div
      ref={setNodeRef}
      style={{ transform: CSS.Translate.toString(transform), transition }}
      className={
        isDragging
          ? "relative z-10 space-y-3 rounded-sm border border-subtle bg-surface p-3 ring-1 ring-primary"
          : "relative space-y-3 rounded-sm border border-subtle bg-surface p-3"
      }
    >
      <div className="flex items-center gap-2">
        <Button
          variant="ghost"
          size="icon"
          aria-label={`Verschieben: ${tiername}`}
          className="cursor-grab touch-none text-secondary active:cursor-grabbing"
          disabled={gesperrt}
          {...attributes}
          {...listeners}
        >
          <GripVertical />
        </Button>
        <span className="font-mono text-xs tabular-nums text-secondary">
          {index + 1}
        </span>
        <h3 className="text-sm font-semibold text-ink">{tiername}</h3>
        <span className="font-mono text-xs text-secondary">
          {platzierung.zielId}
        </span>
      </div>
      <fieldset className="space-y-2">
        <legend className="text-xs font-semibold tracking-[0.09em] text-secondary uppercase">
          Pflöcke für dieses Ziel
        </legend>
        <div className="flex flex-wrap gap-2">
          <Button
            type="button"
            variant="secondary"
            className={
              platzierung.pflöcke.art === "InheritFromParkour"
                ? "border-primary bg-primary-subtle"
                : undefined
            }
            size="sm"
            aria-pressed={platzierung.pflöcke.art === "InheritFromParkour"}
            disabled={gesperrt}
            onClick={() =>
              onZielPflöckeAendern(platzierung.zielId, {
                art: "InheritFromParkour",
              })
            }
          >
            Parkour-Pflöcke
          </Button>
          <Button
            type="button"
            variant={
              platzierung.pflöcke.art === "Explicit" ? "primary" : "secondary"
            }
            size="sm"
            aria-pressed={platzierung.pflöcke.art === "Explicit"}
            disabled={gesperrt}
            onClick={() => {
              if (platzierung.pflöcke.art === "Explicit") return;
              onZielPflöckeAendern(platzierung.zielId, {
                art: "Explicit",
                pflöcke: effektive.map((pflock) => ({
                  ...pflock,
                  id: crypto.randomUUID(),
                })),
              });
            }}
          >
            Eigene Pflöcke
          </Button>
        </div>
        {platzierung.pflöcke.art === "Explicit" ? (
          <DndContext
            sensors={zielPflockSensors}
            collisionDetection={closestCenter}
            onDragEnd={({ active, over }) => {
              if (!over || active.id === over.id) return;
              const von = effektive.findIndex(
                (pflock) => pflock.id === active.id,
              );
              const nach = effektive.findIndex(
                (pflock) => pflock.id === over.id,
              );
              if (von >= 0 && nach >= 0)
                onZielPflockVerschieben(platzierung.zielId, von, nach);
            }}
          >
            <SortableContext
              items={effektive.map((pflock) => pflock.id)}
              strategy={verticalListSortingStrategy}
            >
              {effektive.map((pflock, pflockIndex) => (
                <SortablePflock
                  key={pflock.id}
                  pflock={pflock}
                  index={pflockIndex}
                  gesperrt={gesperrt}
                  onFarbeAendern={(id, farbe) =>
                    onZielPflockFarbeAendern(platzierung.zielId, id, farbe)
                  }
                  onEntfernen={(id) =>
                    onZielPflockEntfernen(platzierung.zielId, id)
                  }
                  entfernenGesperrt={effektive.length <= 1}
                />
              ))}
            </SortableContext>
          </DndContext>
        ) : null}
        {platzierung.pflöcke.art === "Explicit" ? (
          <Button
            type="button"
            variant="secondary"
            size="sm"
            onClick={() => onZielPflockHinzufuegen(platzierung.zielId)}
            disabled={gesperrt || freiePflockFarbe(effektive) === null}
          >
            <Plus /> Pflock hinzufügen
          </Button>
        ) : null}
      </fieldset>
      <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
        {effektive.map((pflock, nummer) => (
          <Field
            key={pflock.id}
            label={`Abstand Pflock ${nummer + 1} · ${pflock.farbe}`}
            hint="Meter, leer = nicht erfasst"
          >
            <Input
              type="number"
              min={1}
              value={abstandById.get(pflock.id) ?? ""}
              onChange={(event) => {
                const eingabe = event.target.value;
                const meter = eingabe === "" ? undefined : Number(eingabe);
                onAbstandAendern(
                  platzierung.zielId,
                  pflock.id,
                  meter !== undefined && Number.isFinite(meter)
                    ? meter
                    : undefined,
                );
              }}
              disabled={gesperrt}
            />
          </Field>
        ))}
      </div>
    </div>
  );
}

/**
 * One Pflock of the setup, draggable by its handle only. The row position *is* the
 * near-to-far order, so the number is the record and the drag is the edit; the
 * colour itself is chosen on the picker, which names every option in words.
 *
 * Removing the row removes the Pflock from the setup — and with it every distance
 * measured from it (see `pflockEntfernen`). The last remaining row cannot be removed,
 * because the domain refuses a Parkour without a Pflock; the section says why beside
 * the list, so the reason is visible whether or not the button is reachable.
 */
function SortablePflock({
  pflock,
  index,
  gesperrt,
  onFarbeAendern,
  onEntfernen,
  entfernenGesperrt,
}: {
  pflock: Pflock;
  index: number;
  gesperrt: boolean;
  onFarbeAendern: (id: string, farbe: string) => void;
  onEntfernen: (id: string) => void;
  entfernenGesperrt: boolean;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: pflock.id });

  return (
    <div
      ref={setNodeRef}
      style={{ transform: CSS.Translate.toString(transform), transition }}
      className={
        isDragging
          ? "relative z-10 flex flex-wrap items-start gap-3 rounded-sm border border-subtle bg-surface p-3 ring-1 ring-primary"
          : "relative flex flex-wrap items-start gap-3 rounded-sm border border-subtle bg-surface p-3"
      }
    >
      <Button
        variant="ghost"
        size="icon"
        aria-label={`Pflock ${index + 1} verschieben`}
        className="cursor-grab touch-none text-secondary active:cursor-grabbing"
        disabled={gesperrt}
        {...attributes}
        {...listeners}
      >
        <GripVertical />
      </Button>
      <span className="flex h-11 w-6 shrink-0 items-center justify-center">
        <SheetNumber className="text-xs">{index + 1}</SheetNumber>
      </span>
      <PflockFarbeFeld
        label={`Farbe Pflock ${index + 1}`}
        value={pflock.farbe}
        onValueChange={(farbe) => onFarbeAendern(pflock.id, farbe)}
        disabled={gesperrt}
        className="min-w-0 flex-1"
      />
      <Button
        variant="ghost"
        size="icon"
        aria-label={`Pflock ${index + 1} entfernen`}
        className="text-secondary"
        disabled={gesperrt || entfernenGesperrt}
        onClick={() => onEntfernen(pflock.id)}
      >
        <Trash2 />
      </Button>
    </div>
  );
}

/** Which active Ziele of the Anlage are already in this route, and which are not. */
function ParkourZiele({
  ziele,
  platzierungen,
}: {
  ziele: Anlage["ziele"];
  platzierungen: Zielplatzierung[];
}) {
  if (ziele.length === 0) return null;
  return (
    <section aria-label="Ziele der Anlage" className="space-y-2">
      <div className="flex items-baseline gap-3">
        <h2 className="text-sm font-semibold text-ink">Ziele der Anlage</h2>
        <SheetNumber>{ziele.length}</SheetNumber>
      </div>
      <Table>
        <TableHead>
          <TableRow>
            <TableHeader>Tiername</TableHeader>
            <TableHeader>Nummer</TableHeader>
            <TableHeader>In diesem Parkour</TableHeader>
          </TableRow>
        </TableHead>
        <TableBody>
          {ziele.map((ziel) => {
            const platziert = platzierungen.some(
              (platzierung) => platzierung.zielId === ziel.id,
            );
            return (
              <TableRow key={ziel.id}>
                <TableCell className="font-medium">{ziel.tiername}</TableCell>
                <TableCell className="tabular-nums">
                  {ziel.zielnummer}
                </TableCell>
                <TableCell>
                  <Badge variant={platziert ? "success" : "default"}>
                    {platziert ? "Platziert" : "Nicht platziert"}
                  </Badge>
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </section>
  );
}
