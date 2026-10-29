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
import { PflockMarker } from "@/components/stammdaten/pflock-swatch";
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
    return (
      <div className="p-6">
        <p className="text-sm text-secondary">Parcours wird geladen …</p>
      </div>
    );
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
    return (
      <div className="p-6">
        <p className="text-sm text-secondary">Parcours wird geladen …</p>
      </div>
    );
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

  function pflockAendern(id: string, feld: keyof Pflock, wert: string) {
    setSetup((bisher) => ({
      ...bisher,
      pflöcke: bisher.pflöcke.map((pflock) =>
        pflock.id === id ? { ...pflock, [feld]: wert } : pflock,
      ),
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

      <header className="space-y-2">
        <div className="flex flex-wrap items-center gap-3">
          <h1 className="font-serif text-2xl font-semibold tracking-tight text-ink">
            {parcours.name}
          </h1>
          <Badge
            variant={parcours.zustand === "Archiviert" ? "warning" : "success"}
          >
            {parcours.zustand === "Archiviert" ? "Archiviert" : "Aktiv"}
          </Badge>
        </div>
        <p className="text-sm text-secondary">
          {anlage.name} · Aufstellung, Pflöcke und Abstände
        </p>
      </header>

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
            className="grid items-end gap-4 sm:grid-cols-3"
            onSubmit={(event) => {
              event.preventDefault();
              parcoursUmbenennen.mutate([anlage.id, parcours.id, name]);
            }}
          >
            <Field label="Parcoursname">
              <Input
                value={name}
                onChange={(event) => setName(event.target.value)}
                disabled={gesperrt}
                required
              />
            </Field>
            <Button
              type="submit"
              disabled={gesperrt || parcoursUmbenennen.isPending}
            >
              Umbenennen
            </Button>
          </form>
          <p className="mt-2 text-xs text-secondary">
            Bereits gestartete Runden behalten den damaligen Namen.
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Pflöcke</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader className="w-16">Reihe</TableHeader>
                <TableHeader>Bezeichnung</TableHeader>
                <TableHeader>Farbe</TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {setup.pflöcke.map((pflock, index) => (
                <TableRow key={pflock.id}>
                  <TableCell className="font-mono text-xs tabular-nums text-secondary">
                    {index + 1}
                  </TableCell>
                  <TableCell>
                    <Input
                      value={pflock.bezeichnung}
                      onChange={(event) =>
                        pflockAendern(
                          pflock.id,
                          "bezeichnung",
                          event.target.value,
                        )
                      }
                      disabled={gesperrt}
                      aria-label={`Bezeichnung Pflock ${index + 1}`}
                    />
                  </TableCell>
                  <TableCell>
                    <div className="flex items-center gap-3">
                      <Input
                        value={pflock.farbe}
                        onChange={(event) =>
                          pflockAendern(pflock.id, "farbe", event.target.value)
                        }
                        disabled={gesperrt}
                        aria-label={`Farbe Pflock ${index + 1}`}
                      />
                      <PflockMarker farbe={pflock.farbe} />
                    </div>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          <p className="text-xs text-secondary">
            Die Reihe ist die Entfernung: oben steht der weiteste Pflock, unten
            der nächste.
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Platzierungen und Abstände</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
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
          <p className="text-xs text-secondary">
            Die Reihe ist die Schussfolge: oben steht das erste Ziel, unten das
            letzte.
          </p>
        </CardContent>
      </Card>

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
          <Separator />
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
                onClick={() =>
                  parcoursEntfernen.mutate([anlage.id, parcours.id])
                }
              >
                Entfernen
              </Button>
            </CardContent>
          </Card>
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
        {platzierung.abstände.map((abstand) => {
          const pflock = pflöcke.find(
            (eintrag) => eintrag.id === abstand.pflockId,
          );
          return (
            <Field
              key={abstand.pflockId}
              label={`Abstand ${pflock?.bezeichnung ?? ""}`}
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
    <Card>
      <CardHeader>
        <CardTitle>Ziele der Anlage</CardTitle>
      </CardHeader>
      <CardContent>
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
      </CardContent>
    </Card>
  );
}
