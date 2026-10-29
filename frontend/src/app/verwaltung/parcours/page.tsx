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
import { GripVertical } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";

import { useAuth } from "@/api/auth-provider";
import { apiErrorMessage } from "@/api/errors";
import { useAnlageQuery, useStammdatenMutations } from "@/api/hooks";
import type {
  Anlage,
  Parcours,
  ParcoursSetupInput,
  Pflock,
  Zielplatzierung,
} from "@/api/logandaim-api";
import { SetBreadcrumbLabel } from "@/components/layout/page-chrome";
import { PageHeader } from "@/components/layout/page-header";
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
 * One Parcours' shooting setup. A setup is one value: replacing it replaces every Pflock
 * and every placement at once (AC-6), which is why the editor works on a local copy and
 * sends the whole `ParcoursSetupRequest`.
 */
export default function ParcoursVerwaltungPage() {
  const phase = useAuth();
  const router = useRouter();
  const parcoursId = useQueryParam("id");
  const anlageId = useQueryParam("anlage");
  const anlage = useAnlageQuery(anlageId);

  useEffect(() => {
    if (phase.status === "anonymous") router.replace("/anmeldung");
  }, [phase.status, router]);

  if (parcoursId === null || anlageId === null) {
    return <ParcoursSkeleton />;
  }

  if (!parcoursId || !anlageId) {
    return (
      <div className="p-6">
        <Alert variant="warning">
          Es wurde kein Parcours ausgewählt. Bitte öffnen Sie ihn aus der
          Anlage.
        </Alert>
      </div>
    );
  }

  if (anlage.isLoading) {
    return <ParcoursSkeleton />;
  }

  const parcours = anlage.data?.parcours.find(
    (eintrag) => eintrag.id === parcoursId,
  );
  if (anlage.isError || !anlage.data || !parcours) {
    return (
      <div className="p-6">
        <Alert variant="error">
          {apiErrorMessage(anlage.error, {
            unauthorized: "Bitte melden Sie sich an.",
            fallback: "Der Parcours konnte nicht geladen werden.",
          })}
        </Alert>
      </div>
    );
  }

  return <ParcoursInhalt anlage={anlage.data} parcours={parcours} />;
}

/** A printed placeholder: the head and the ledgers, before the Parcours arrives. */
function ParcoursSkeleton() {
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

function ParcoursInhalt({
  anlage,
  parcours,
}: {
  anlage: Anlage;
  parcours: Parcours;
}) {
  const { parcoursSetupAendern, parcoursUmbenennen, parcoursEntfernen } =
    useStammdatenMutations();
  const [name, setName] = useState(parcours.name);
  const [entfernenOffen, setEntfernenOffen] = useState(false);
  const [setup, setSetup] = useState<ParcoursSetupInput>(() => ({
    pflöcke: parcours.pflöcke.map((pflock) => ({ ...pflock })),
    platzierungen: parcours.platzierungen.map((platzierung) => ({
      zielId: platzierung.zielId,
      abstände: platzierung.abstände.map((abstand) => ({ ...abstand })),
    })),
  }));

  const gesperrt =
    anlage.zustand === "Archiviert" || parcours.zustand === "Archiviert";
  const ziele = useMemo(
    () => anlage.ziele.filter((ziel) => ziel.zustand === "Aktiv"),
    [anlage.ziele],
  );

  const fehler =
    parcoursSetupAendern.error ??
    parcoursUmbenennen.error ??
    parcoursEntfernen.error;

  function pflockFarbeAendern(id: string, farbe: string) {
    setSetup((bisher) => ({
      ...bisher,
      pflöcke: bisher.pflöcke.map((pflock) =>
        pflock.id === id ? { ...pflock, farbe } : pflock,
      ),
    }));
  }

  /** The Pflock array order *is* the far-to-near order, so a drag rewrites it. */
  function pflockVerschieben(von: number, nach: number) {
    setSetup((bisher) => ({
      ...bisher,
      pflöcke: arrayMove(bisher.pflöcke, von, nach),
    }));
  }

  function abstandAendern(zielId: string, pflockId: string, meter: number) {
    setSetup((bisher) => ({
      ...bisher,
      platzierungen: bisher.platzierungen.map((platzierung) =>
        platzierung.zielId === zielId
          ? {
              ...platzierung,
              abstände: platzierung.abstände.map((abstand) =>
                abstand.pflockId === pflockId ? { ...abstand, meter } : abstand,
              ),
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
      <SetBreadcrumbLabel label={parcours.name} />

      <PageHeader
        register={["Verwaltung", "Parcours"]}
        title={parcours.name}
        description={`${anlage.name} · Aufstellung, Pflöcke und Abstände`}
        actions={
          <>
            <Badge
              variant={
                parcours.zustand === "Archiviert" ? "warning" : "success"
              }
            >
              {parcours.zustand === "Archiviert" ? "Archiviert" : "Aktiv"}
            </Badge>
            {parcours.zustand === "Archiviert" ? (
              <StampMark>Archiviert</StampMark>
            ) : null}
          </>
        }
      />

      {fehler ? (
        <Alert variant="error">
          {apiErrorMessage(fehler, {
            badRequest: "Bitte prüfen Sie die Aufstellung.",
            fallback: "Die Änderung konnte nicht gespeichert werden.",
          })}
        </Alert>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle>Name</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={(event) => {
              event.preventDefault();
              parcoursUmbenennen.mutate([anlage.id, parcours.id, name]);
            }}
          >
            <FormRow
              className="sm:grid-cols-3"
              actions={
                <Button
                  type="submit"
                  size="field"
                  disabled={gesperrt || parcoursUmbenennen.isPending}
                >
                  Umbenennen
                </Button>
              }
            >
              <Field label="Parcoursname">
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
                />
              ))}
            </SortableContext>
          </DndContext>
        )}
        <MarginNote label="Reihenfolge">
          Die Reihe ist die Entfernung: oben steht der weiteste Pflock, unten
          der nächste. Ziehen am Griff ändert die Reihenfolge.
        </MarginNote>
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

      <div className="flex flex-wrap gap-3">
        <Button
          disabled={gesperrt || parcoursSetupAendern.isPending}
          onClick={() =>
            parcoursSetupAendern.mutate([anlage.id, parcours.id, setup])
          }
        >
          {parcoursSetupAendern.isPending
            ? "Wird gespeichert…"
            : "Aufstellung speichern"}
        </Button>
        <Button variant="secondary" asChild>
          <Link href={`${paths.verwaltungAnlage}?id=${anlage.id}`}>
            Zurück zur Anlage
          </Link>
        </Button>
      </div>

      {gesperrt ? null : (
        <>
          <HatchBand height="lg" />
          <Card>
            <CardHeader>
              <CardTitle>Parcours entfernen</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-sm text-secondary">
                Wurde der Parcours schon einmal gestartet, wird er archiviert
                und bleibt in alten Runden sichtbar. Sonst wird er gelöscht.
              </p>
              <Button
                variant="destructive"
                disabled={parcoursEntfernen.isPending}
                onClick={() => {
                  parcoursEntfernen.reset();
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
                <DialogTitle>Parcours entfernen?</DialogTitle>
                <DialogDescription>
                  Wurde der Parcours schon einmal gestartet, wird er archiviert
                  und bleibt in alten Runden sichtbar. Sonst wird er endgültig
                  gelöscht.
                </DialogDescription>
              </DialogHeader>
              {parcoursEntfernen.isError ? (
                <Alert variant="error">
                  {apiErrorMessage(parcoursEntfernen.error, {
                    fallback: "Der Parcours konnte nicht entfernt werden.",
                  })}
                </Alert>
              ) : null}
              <DialogFooter>
                <Button
                  variant="secondary"
                  onClick={() => setEntfernenOffen(false)}
                  disabled={parcoursEntfernen.isPending}
                >
                  Abbrechen
                </Button>
                <Button
                  variant="destructive"
                  disabled={parcoursEntfernen.isPending}
                  onClick={() =>
                    parcoursEntfernen.mutate([anlage.id, parcours.id], {
                      onSuccess: () => setEntfernenOffen(false),
                    })
                  }
                >
                  {parcoursEntfernen.isPending ? "Wird entfernt…" : "Entfernen"}
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </>
      )}

      <ParcoursZiele ziele={ziele} platzierungen={setup.platzierungen} />
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
}: {
  platzierung: Zielplatzierung;
  index: number;
  tiername: string;
  gesperrt: boolean;
  pflöcke: Pflock[];
  onAbstandAendern: (zielId: string, pflockId: string, meter: number) => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: platzierung.zielId });

  // Distances follow the Pflock list, so a reordered setup reads far to near here too.
  const abständeInReihenfolge = [...platzierung.abstände].sort(
    (a, b) =>
      pflöcke.findIndex((pflock) => pflock.id === a.pflockId) -
      pflöcke.findIndex((pflock) => pflock.id === b.pflockId),
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
      <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
        {abständeInReihenfolge.map((abstand) => {
          const pflock = pflöcke.find(
            (eintrag) => eintrag.id === abstand.pflockId,
          );
          const nummer = pflöcke.findIndex(
            (eintrag) => eintrag.id === abstand.pflockId,
          );
          return (
            <Field
              key={abstand.pflockId}
              label={`Abstand Pflock ${nummer + 1} · ${pflock?.farbe ?? ""}`}
              hint="Meter"
            >
              <Input
                type="number"
                min={1}
                value={abstand.meter}
                onChange={(event) =>
                  onAbstandAendern(
                    platzierung.zielId,
                    abstand.pflockId,
                    Number(event.target.value),
                  )
                }
                disabled={gesperrt}
              />
            </Field>
          );
        })}
      </div>
    </div>
  );
}

/**
 * One Pflock of the setup, draggable by its handle only. The row position *is* the
 * far-to-near order, so the number is the record and the drag is the edit; the
 * colour itself is chosen on the picker, which names every option in words.
 */
function SortablePflock({
  pflock,
  index,
  gesperrt,
  onFarbeAendern,
}: {
  pflock: Pflock;
  index: number;
  gesperrt: boolean;
  onFarbeAendern: (id: string, farbe: string) => void;
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
    </div>
  );
}

/** Which active Ziele of the Anlage are already in this route, and which are not. */
function ParcoursZiele({
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
            <TableHeader>Zielgruppe</TableHeader>
            <TableHeader>In diesem Parcours</TableHeader>
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
                  {ziel.zielgruppe}
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
